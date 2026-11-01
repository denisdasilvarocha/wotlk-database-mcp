use std::{path::Path, sync::Arc};

use rmcp::{
    ErrorData, ServerHandler,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;

use crate::{
    db::{self, Database, QueryError},
    kinds::{Kind, Relation},
};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchParams {
    /// Name or words to find (case-insensitive; prefixes match).
    pub text: String,
    /// Only return this kind.
    pub kind: Option<Kind>,
    /// Maximum hits, 1 to 100. Default 20.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct GetParams {
    /// Record kind.
    pub kind: Kind,
    /// Numeric id, or the exact name for offset, `lua_function`, `lua_event`, widget,
    /// cvar, `lua_type`, `combatlog_event`, or `secure_template`. Not used for meta.
    pub id: Option<RecordId>,
    /// Extra field names to add to the summary, or `["all"]` for every field.
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RelatedParams {
    /// Kind of the starting record.
    pub kind: Kind,
    /// Id of the starting record.
    pub id: RecordId,
    /// Relation to follow; see the server instructions for the relations of each kind.
    pub relation: Relation,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SqlParams {
    /// One read-only SQLite statement.
    pub query: String,
    /// Maximum rows, 1 to 1000. Default 200.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub(crate) enum RecordId {
    Number(i64),
    Text(String),
}

/// MCP server that exposes the WotLK database tools.
#[derive(Debug, Clone)]
pub struct WotlkServer {
    db: Arc<Database>,
}

impl WotlkServer {
    /// Opens an existing database in read-only mode.
    ///
    /// # Errors
    /// Returns an error if the database cannot be opened or lacks build metadata.
    pub fn open(path: &Path) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            db: Arc::new(Database::open(path)?),
        })
    }

    async fn query<F>(&self, f: F) -> Result<CallToolResult, ErrorData>
    where
        F: FnOnce(&mut rusqlite::Connection) -> Result<String, QueryError> + Send + 'static,
    {
        let db = Arc::clone(&self.db);
        let result = tokio::task::spawn_blocking(move || db.run(f))
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        match result {
            Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
            Err(QueryError::Invalid(message)) => Err(ErrorData::invalid_params(message, None)),
            Err(QueryError::Tool(message)) => Ok(CallToolResult::error(vec![ContentBlock::text(message)])),
            Err(QueryError::Internal(message)) => Err(ErrorData::internal_error(message, None)),
        }
    }
}

#[tool_router]
impl WotlkServer {
    #[tool(
        description = "Find records by name across all kinds (items, spells, creatures, quests, offsets, Lua API, ...). Use it first when you only know a name. Returns one line per hit: kind id name (subtext). Exact and prefix name matches come first."
    )]
    async fn search(&self, Parameters(args): Parameters<SearchParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::search(conn, &args.text, args.kind, args.limit))
            .await
    }

    #[tool(
        description = "Return one record as key=value lines. Without fields you get the summary fields; pass field names to add them, or [\"all\"] for every field. Zero, empty, and default fields are always left out. kind=meta returns build provenance and needs no id."
    )]
    async fn get(&self, Parameters(args): Parameters<GetParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::get(conn, args.kind, args.id, args.fields.as_deref()))
            .await
    }

    #[tool(
        description = "List records linked to one record: where an item drops or is sold, what a creature sells or trains, a quest chain, spell ranks, and more. The server instructions list the relations of each kind."
    )]
    async fn related(&self, Parameters(args): Parameters<RelatedParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::related(conn, args.kind, args.id, args.relation))
            .await
    }

    #[tool(
        description = "Run one read-only SQLite statement. Returns a tab-separated header line and rows. Default 200 rows, maximum 1000, 2 second limit; a last line says when rows were cut. Read the schema (with column comments) by running: SELECT sql FROM sqlite_schema WHERE type = 'table'"
    )]
    async fn sql(&self, Parameters(args): Parameters<SqlParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::sql(conn, &args.query, args.limit)).await
    }
}

#[tool_handler(
    name = "wotlk-database-mcp",
    instructions = "Offline reference data for World of Warcraft 3.3.5a (build 12340). The client DBC files are the truth; TrinityCore TDB 335.26091 supplies server data (loot, spawns, vendors, trainers, quests). Output is compact key=value text; zero, empty, and default fields are left out; flags and enums are names. Money is in copper; times are in milliseconds unless the schema comment says seconds.\n\nTools: search(text, kind?, limit?) finds records by name. get(kind, id, fields?) returns one record. related(kind, id, relation) lists linked records. sql(query, limit?) runs one read-only SQLite statement; read the schema with SELECT sql FROM sqlite_schema WHERE type = 'table'.\n\nKinds with numeric ids: item, creature, spell, itemset, quest, gameobject, map, area, faction, talent, skill, glyph, achievement.\nKinds with name ids: offset (qualified name, for example WoWOffsets::Lua::FrameScript::Execute), lua_function, lua_event, widget, cvar, lua_type, combatlog_event, secure_template.\nmeta: build provenance; get(kind=\"meta\") needs no id.\n\nRelations:\nitem: dropped_by, sold_by, reward_of, set, created_by, disenchant\ncreature: loot, sells, trains, starts_quests, ends_quests, spawns, spells\nquest: givers, enders, chain, rewards, requirements\nspell: ranks, taught_by, used_by, talent, glyph, triggered\ngameobject: loot, spawns\narea: children, parent\nfaction: members\nachievement: criteria\nA \"reference N\" loot line points to a shared loot table; list its users with sql: SELECT * FROM loot WHERE reference = N.\n\nOffsets: va is a Windows x86 preferred-image address (image base 0x00400000); runtime address = module base + rva. Only status=verified is confirmed; read comment for every caveat. List one domain with sql: SELECT name, va, status FROM offsets WHERE domain = 'Lua'."
)]
#[expect(
    clippy::unused_async_trait_impl,
    reason = "rmcp #[tool_handler] generates async fns without .await"
)]
impl ServerHandler for WotlkServer {
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, ErrorData> {
        // The SDK's router converts parameter extraction errors into tool errors.
        // Deserialize here so malformed requests retain their JSON-RPC error code.
        let args = request.arguments.unwrap_or_default();
        let result = match request.name.as_ref() {
            "search" => {
                self.search(Parameters(rmcp::handler::server::tool::parse_json_object(args)?))
                    .await
            }
            "get" => {
                self.get(Parameters(rmcp::handler::server::tool::parse_json_object(args)?))
                    .await
            }
            "related" => {
                self.related(Parameters(rmcp::handler::server::tool::parse_json_object(args)?))
                    .await
            }
            "sql" => {
                self.sql(Parameters(rmcp::handler::server::tool::parse_json_object(args)?))
                    .await
            }
            _ => return Err(ErrorData::invalid_params("tool not found", None)),
        }?;
        Ok(result.into())
    }
}
