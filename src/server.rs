use std::{
    path::Path,
    sync::{Arc, Once},
};

use rmcp::{
    ErrorData, ServerHandler,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use tokio::sync::watch;

use crate::{
    db::{self, Database, DatabaseError, QueryError},
    kinds::{Kind, RecordId, RelatedKind, Relation, SearchKind},
    provision::Provisioner,
};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchParams {
    /// Name or words to find (case-insensitive; prefixes match).
    pub text: String,
    /// Only return records of this kind.
    pub kind: Option<SearchKind>,
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
    pub kind: RelatedKind,
    /// Id of the starting record.
    pub id: RecordId,
    /// Relation to follow. item: `dropped_by`, `sold_by`, `reward_of`, `set`,
    /// `created_by`, `disenchant`; creature: `loot`, `sells`, `trains`, `starts_quests`,
    /// `ends_quests`, `spawns`, `spells`; quest: `givers`, `enders`, `chain`, `rewards`,
    /// `requirements`; spell: `ranks`, `taught_by`, `used_by`, `talent`, `glyph`,
    /// `triggered`; gameobject: `loot`, `spawns`; area: `children`, `parent`; faction:
    /// `members`; achievement: `criteria`.
    /// Vendor `price` is gold in copper; `cost` lists extended costs.
    /// Spawn `world_x/world_y/world_z` are world coordinates, not map percentages.
    /// Spawn `location_source` distinguishes zone names from map-name fallbacks.
    pub relation: Relation,
    /// Maximum rows, 1 to 1000. Default 50; the last line says how many rows it cut.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct SqlParams {
    /// One read-only SQLite statement. Pragma assignments and transactions are rejected.
    pub query: String,
    /// Maximum rows, 1 to 1000. Default 200.
    pub limit: Option<u32>,
}

/// MCP server that exposes the WotLK database tools.
#[derive(Debug, Clone)]
pub struct WotlkServer {
    state: watch::Sender<DatabaseState>,
    provisioner: Option<Arc<Provisioner>>,
    start: Arc<Once>,
}

#[derive(Debug)]
enum DatabaseState {
    Loading { percent: u8 },
    Ready(Arc<Database>),
    Failed(String),
}

impl WotlkServer {
    /// Opens an existing database in read-only mode.
    ///
    /// # Errors
    /// Returns an error if the database cannot be opened or has a different schema version.
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        Ok(Self::with_state(
            DatabaseState::Ready(Arc::new(Database::open(path)?)),
            None,
        ))
    }

    /// Uses a valid database cache, or downloads it after the MCP handshake.
    ///
    /// `path` is the final SQLite cache path. Download failures are returned as tool
    /// errors, so initialization and tool discovery do not require network access.
    #[must_use]
    pub fn cached(path: &Path) -> Self {
        Self::with_provisioner(Provisioner::new(path))
    }

    fn with_provisioner(provisioner: Provisioner) -> Self {
        if let Ok(database) = Database::open(&provisioner.path) {
            return Self::with_state(DatabaseState::Ready(Arc::new(database)), None);
        }
        Self::with_state(DatabaseState::Loading { percent: 0 }, Some(Arc::new(provisioner)))
    }

    fn with_state(state: DatabaseState, provisioner: Option<Arc<Provisioner>>) -> Self {
        let (state, _) = watch::channel(state);
        Self {
            state,
            provisioner,
            start: Arc::new(Once::new()),
        }
    }

    async fn query<F>(&self, f: F) -> Result<CallToolResult, ErrorData>
    where
        F: FnOnce(&mut rusqlite::Connection) -> Result<String, QueryError> + Send + 'static,
    {
        let db = match &*self.state.borrow() {
            DatabaseState::Ready(db) => Arc::clone(db),
            DatabaseState::Loading { percent } => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(format!(
                    "Database downloading: {percent}%. Please retry when the download finishes."
                ))]));
            }
            DatabaseState::Failed(message) => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(message.clone())]));
            }
        };
        let result = tokio::task::spawn_blocking(move || db.run(f))
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        match result {
            Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
            Err(QueryError::Tool(message)) => Ok(CallToolResult::error(vec![ContentBlock::text(message)])),
            Err(QueryError::Internal(message)) => Err(ErrorData::internal_error(message, None)),
        }
    }
}

#[tool_router]
impl WotlkServer {
    #[tool(
        title = "Search records",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        description = "World of Warcraft 3.3.5a (WotLK, build 12340): find records by name across all kinds (items, spells, creatures, quests, Lua/addon API, client offsets). Use it first when you only know a name. Returns one line per hit: kind id name (subtext); exact and prefix name matches come first, then player-learnable spells and creatures that spawn, then the rest. A spell's subtext is its rank and family, a creature's its title, level, and rank."
    )]
    async fn search(&self, Parameters(args): Parameters<SearchParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::search(conn, &args.text, args.kind.map(SearchKind::kind), args.limit))
            .await
    }

    #[tool(
        title = "Get record",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        description = "World of Warcraft 3.3.5a (WotLK, build 12340): return one record as key=value lines. Without fields you get the summary fields; pass field names to add them, or [\"all\"] for every field. Zero, empty, and default fields are always left out. kind=meta returns build provenance and needs no id."
    )]
    async fn get(&self, Parameters(args): Parameters<GetParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::get(conn, args.kind, args.id, args.fields.as_deref()))
            .await
    }

    #[tool(
        title = "Related records",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        description = "World of Warcraft 3.3.5a (WotLK, build 12340): list records linked to one record: where an item drops or is sold, what a creature drops, sells, or trains, the whole quest chain, spell ranks, and more. Returns one line per row: kind id name, then extra col=value fields; via=N names the shared loot table a row came from, step=n its distance along the quest chain. Vendor price is the required gold in copper; cost lists extended costs. Spawn world_x/world_y/world_z are world coordinates, not zone-map percentages; orientation is radians and spawn_time is seconds. Spawn map=0 is valid; location_source=zone or map identifies the location name, and missing zone/area fields mean unknown. Default 50 rows, maximum 1000; a last line reports rows cut by the limit."
    )]
    async fn related(&self, Parameters(args): Parameters<RelatedParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::related(conn, args.kind.kind(), args.id, args.relation, args.limit))
            .await
    }

    #[tool(
        title = "SQL query",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        description = "World of Warcraft 3.3.5a (WotLK, build 12340): run one read-only SQLite statement. Returns a tab-separated header line and rows. Default 200 rows, maximum 1000, 2 second limit; a last line says when rows were cut. Read one table's schema (with column comments) by running: SELECT sql FROM sqlite_schema WHERE name = '<table>'"
    )]
    async fn sql(&self, Parameters(args): Parameters<SqlParams>) -> Result<CallToolResult, ErrorData> {
        self.query(move |conn| db::sql(conn, &args.query, args.limit)).await
    }
}

#[tool_handler(
    name = "wotlk-database-mcp",
    instructions = "World of Warcraft: Wrath of the Lich King 3.3.5a (WotLK, build 12340) reference data for addon and Lua API work, TrinityCore/AzerothCore private-server data, and client reverse engineering with offsets. The client DBC files are the truth; TrinityCore TDB 335.26091 supplies server data: loot, spawns, vendors, trainers, quests. Output is compact key=value text; zero, empty, and default fields are left out, except the valid map=0 in spawn output; flags and enums are names. Money is in copper; times are in milliseconds unless the schema comment says seconds. Vendor price is the required gold in copper; cost lists extended costs. Spawn world_x/world_y/world_z are world coordinates, not zone-map percentages; orientation is radians and spawn_time is seconds. Spawn location_source=zone or map identifies the location name; missing zone/area fields mean unknown, not no spawns. Loot chance is a percentage, and chance=0 inside a group means an equal share of the group's remaining chance. A row with via=N came from shared loot table N; a quest chain row's step=n counts quests from the queried quest (0 = it, negative before, positive after).\n\nTools: search(text, kind?, limit?) finds records by name, 20 by default (max 100). get(kind, id, fields?) returns one record. related(kind, id, relation, limit?) lists linked records, 50 by default (max 1000); a last line reports rows cut by the limit. sql(query, limit?) runs one read-only SQLite statement, 200 rows by default (max 1000, 2 second limit); read one table's schema with SELECT sql FROM sqlite_schema WHERE name = '<table>'.\n\nTables: item, creature, spell, itemset, quest, gameobject, map, area, faction, talent, skill, glyph, achievement, offsets, lua_function, lua_event, widget, cvar, lua_type, combatlog_event, secure_template, meta, item_spell, creature_spell, creature_spawn, gameobject_spawn, loot, vendor, trainer, trainer_spell, creature_trainer, quest_relation, quest_item, quest_objective, quest_reward_faction, spell_rank, spell_effect, spell_icon, item_display, itemset_item, itemset_bonus, talent_tab, talent_spell, skill_ability, achievement_criteria.\n\nKinds with numeric ids: item, creature, spell, itemset, quest, gameobject, map, area, faction, talent, skill, glyph, achievement.\nKinds with name ids: offset (qualified name, for example WoWOffsets::Lua::FrameScript::Execute), lua_function, lua_event, widget, cvar, lua_type, combatlog_event, secure_template.\nmeta: build provenance; get(kind=\"meta\") needs no id.\n\nRelations:\nitem: dropped_by, sold_by, reward_of, set, created_by, disenchant\ncreature: loot, sells, trains, starts_quests, ends_quests, spawns, spells\nquest: givers, enders, chain, rewards, requirements\nspell: ranks, taught_by, used_by, talent, glyph, triggered\ngameobject: loot, spawns\narea: children, parent\nfaction: members\nachievement: criteria\n\nOffsets: va is a Windows x86 preferred-image address (image base 0x00400000); runtime address = module base + rva. Only status=verified is confirmed; read comment for every caveat. List one domain with sql: SELECT name, va, status FROM offsets WHERE domain = 'Lua'."
)]
#[expect(
    clippy::unused_async_trait_impl,
    reason = "rmcp #[tool_handler] generates async fns without .await"
)]
impl ServerHandler for WotlkServer {
    async fn on_initialized(&self, _context: rmcp::service::NotificationContext<rmcp::RoleServer>) {
        let Some(provisioner) = &self.provisioner else {
            return;
        };
        self.start.call_once(|| {
            let provisioner = Arc::clone(provisioner);
            let state = self.state.clone();
            tokio::spawn(async move {
                let progress_state = state.clone();
                let download = Arc::clone(&provisioner);
                let result = tokio::task::spawn_blocking(move || {
                    download
                        .install(|percent| {
                            progress_state.send_replace(DatabaseState::Loading { percent });
                        })
                        .map_err(|error| download.failure(&error))
                })
                .await;
                let next = match result {
                    Ok(Ok(database)) => DatabaseState::Ready(Arc::new(database)),
                    Ok(Err(message)) => DatabaseState::Failed(message),
                    Err(error) => DatabaseState::Failed(provisioner.failure(&error)),
                };
                state.send_replace(next);
            });
        });
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, ErrorData> {
        // Handler failures (bad values, unknown records) are tool errors so clients show
        // them to the model, as the MCP spec requires. Only arguments that do not
        // deserialize keep a JSON-RPC error code, which is why they are parsed here.
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

#[cfg(test)]
mod tests;
