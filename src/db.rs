//! Read-only database access and tool queries.

use std::{
    fmt::Write as _,
    path::Path,
    sync::Mutex,
    time::{Duration, Instant},
};

use rusqlite::{
    Connection, Error, ErrorCode, OpenFlags, OptionalExtension,
    hooks::{AuthAction, AuthContext, Authorization},
    limits::Limit,
    params,
    types::Value,
};

use crate::{
    format,
    kinds::{Kind, RecordId, Relation},
    schema_version::SCHEMA_VERSION,
};

#[derive(Debug)]
pub(crate) struct Database {
    conn: Mutex<Connection>,
}

/// A database cannot be opened or does not match the server's schema.
#[derive(Debug)]
pub enum DatabaseError {
    /// SQLite could not open or inspect the database.
    Sql(Error),
    /// The database has a different schema version or no schema version.
    SchemaMismatch {
        /// The schema version required by this server.
        expected: u32,
        /// The stored version, or `None` when the metadata is absent.
        found: Option<String>,
    },
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sql(error) => std::fmt::Display::fmt(error, f),
            Self::SchemaMismatch { expected, found } => {
                write!(
                    f,
                    "expected schema version {expected}, found {}",
                    found.as_deref().unwrap_or("none")
                )
            }
        }
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sql(error) => Some(error),
            Self::SchemaMismatch { .. } => None,
        }
    }
}

impl From<Error> for DatabaseError {
    fn from(error: Error) -> Self {
        Self::Sql(error)
    }
}

#[derive(Debug)]
pub(crate) enum QueryError {
    /// Returned to the model as a failed tool call (`isError`).
    Tool(String),
    /// Returned as a JSON-RPC internal error.
    Internal(String),
}

impl From<Error> for QueryError {
    fn from(error: Error) -> Self {
        Self::Internal(error.to_string())
    }
}

impl Database {
    pub(crate) fn open(path: &Path) -> Result<Self, DatabaseError> {
        let mut conn =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)?;
        conn.pragma_update(None, "query_only", true)?;
        // Installed after the pragma above, which is itself an assignment.
        conn.authorizer(Some(read_only_authorizer))?;
        conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;
        let has_meta = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type = 'table' AND name = 'meta')",
            [],
            |row| row.get::<_, bool>(0),
        )?;
        let found = if has_meta {
            conn.query_row("SELECT value FROM meta WHERE name = 'schema_version'", [], |row| {
                row.get::<_, Option<String>>(0)
            })
            .optional()?
            .flatten()
        } else {
            None
        };
        if found.as_deref().and_then(|value| value.parse::<u32>().ok()) != Some(SCHEMA_VERSION) {
            return Err(DatabaseError::SchemaMismatch {
                expected: SCHEMA_VERSION,
                found,
            });
        }
        conn.query_row("SELECT value FROM meta WHERE name = 'client_build'", [], |row| {
            row.get::<_, String>(0)
        })?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    pub(crate) fn run(
        &self,
        f: impl FnOnce(&mut Connection) -> Result<String, QueryError>,
    ) -> Result<String, QueryError> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|_| QueryError::Internal("database mutex poisoned".to_owned()))?;
        f(&mut conn)
    }
}

/// Pragmas that inspect the schema or the database; their argument is not an assignment.
const READ_ONLY_PRAGMAS: &[&str] = &[
    "table_info",
    "table_xinfo",
    "table_list",
    "index_info",
    "index_xinfo",
    "index_list",
    "foreign_key_list",
    "foreign_key_check",
    "integrity_check",
    "quick_check",
    "pragma_list",
];

/// Denies the statements that the read-only statement check cannot see.
///
/// `PRAGMA name = value` and transaction control are not writes to a table, so
/// `Statement::readonly` accepts them; `ATTACH` and `DETACH` would open another file.
fn read_only_authorizer(context: AuthContext<'_>) -> Authorization {
    match context.action {
        AuthAction::Pragma {
            pragma_name,
            pragma_value: Some(_),
        } if !READ_ONLY_PRAGMAS.contains(&pragma_name) => Authorization::Deny,
        AuthAction::Transaction { .. }
        | AuthAction::Savepoint { .. }
        | AuthAction::Attach { .. }
        | AuthAction::Detach { .. } => Authorization::Deny,
        _ => Authorization::Allow,
    }
}

fn record_id(kind: Kind, id: RecordId) -> Result<Value, QueryError> {
    if kind.key() == "id" {
        let number = match id {
            RecordId::Number(n) => n,
            RecordId::Text(text) => text
                .parse()
                .map_err(|_| QueryError::Tool(format!("kind {} needs a numeric id", kind.as_str())))?,
        };
        Ok(Value::Integer(number))
    } else {
        Ok(Value::Text(match id {
            RecordId::Number(n) => n.to_string(),
            RecordId::Text(text) => text,
        }))
    }
}

fn id_text(id: &Value) -> String {
    match id {
        Value::Integer(n) => n.to_string(),
        Value::Text(text) => text.clone(),
        _ => String::new(),
    }
}

fn bounded_limit(limit: Option<u32>, default: u32, max: u32) -> Result<u32, QueryError> {
    let limit = limit.unwrap_or(default);
    if limit == 0 || limit > max {
        return Err(QueryError::Tool(format!("limit must be between 1 and {max}")));
    }
    Ok(limit)
}

pub(crate) fn search(
    conn: &mut Connection,
    text: &str,
    kind: Option<Kind>,
    limit: Option<u32>,
) -> Result<String, QueryError> {
    let text = text.trim();
    // The FTS prefix index starts at two characters, so one-character tokens match exactly.
    let tokens: Vec<_> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            if s.chars().count() > 1 {
                format!("\"{s}\"*")
            } else {
                format!("\"{s}\"")
            }
        })
        .collect();
    if tokens.is_empty() {
        return Err(QueryError::Tool(
            "search text needs at least one letter or digit".to_owned(),
        ));
    }
    let limit = bounded_limit(limit, 20, 100)?;
    let mut stmt = conn.prepare(
        "SELECT kind, id, name, subtext FROM search \
         WHERE search MATCH ?1 AND (?3 IS NULL OR kind = ?3) \
         ORDER BY CASE WHEN lower(name) = lower(?2) THEN 0 \
                       WHEN substr(lower(name), 1, length(?2)) = lower(?2) THEN 1 ELSE 2 END, \
                  priority, bm25(search, 10.0, 1.0), length(name), kind, CAST(id AS INTEGER), id \
         LIMIT ?4",
    )?;
    let mut rows = stmt.query(params![tokens.join(" "), text, kind.map(Kind::as_str), limit])?;
    let mut lines = Vec::new();
    while let Some(row) = rows.next()? {
        let k = format::value(row.get_ref(0)?);
        let id = format::value(row.get_ref(1)?);
        let name = format::value(row.get_ref(2)?);
        let mut line = if id == name {
            format!("{k} {name}")
        } else {
            format!("{k} {id} {name}")
        };
        let subtext = row.get_ref(3)?;
        if !format::is_default(subtext) {
            let _ = write!(line, " ({})", format::value(subtext));
        }
        lines.push(line);
    }
    if lines.is_empty() {
        let suffix = kind.map_or_else(String::new, |k| format!(" of kind {}", k.as_str()));
        return Ok(format!("no matches for \"{text}\"{suffix}"));
    }
    Ok(lines.join("\n"))
}

pub(crate) fn get(
    conn: &mut Connection,
    kind: Kind,
    id: Option<RecordId>,
    fields: Option<&[String]>,
) -> Result<String, QueryError> {
    if kind == Kind::Meta {
        let mut stmt = conn.prepare("SELECT name, value FROM meta ORDER BY rowid")?;
        let mut rows = stmt.query([])?;
        let mut lines = Vec::new();
        while let Some(row) = rows.next()? {
            lines.push(format!(
                "{}={}",
                format::value(row.get_ref(0)?),
                format::value(row.get_ref(1)?)
            ));
        }
        return Ok(lines.join("\n"));
    }
    let id = record_id(
        kind,
        id.ok_or_else(|| QueryError::Tool(format!("kind {} needs an id", kind.as_str())))?,
    )?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} WHERE {} = ?1", kind.table(), kind.key()))?;
    let columns: Vec<String> = stmt.column_names().map(str::to_owned).collect();
    let all = fields.is_some_and(|fields| matches!(fields, [field] if field == "all"));
    if !all {
        for field in fields.into_iter().flatten() {
            if !columns.contains(field) {
                return Err(QueryError::Tool(format!(
                    "unknown field '{field}' for {}; fields: {}",
                    kind.as_str(),
                    columns.join(", ")
                )));
            }
        }
    }
    let mut rows = stmt.query([&id])?;
    let Some(row) = rows.next()? else {
        return Err(QueryError::Tool(format!(
            "{} {} not found",
            kind.as_str(),
            id_text(&id)
        )));
    };
    let mut lines = Vec::new();
    for (index, column) in columns.iter().enumerate() {
        if all
            || column == kind.key()
            || kind.summary().contains(&column.as_str())
            || fields.is_some_and(|f| f.contains(column))
        {
            let value = row.get_ref(index)?;
            if column == kind.key() || !format::is_default(value) {
                lines.push(format!("{column}={}", format::value(value)));
            }
        }
    }
    Ok(lines.join("\n"))
}

pub(crate) fn related(
    conn: &mut Connection,
    kind: Kind,
    id: RecordId,
    relation: Relation,
    limit: Option<u32>,
) -> Result<String, QueryError> {
    let relations = kind.relations();
    let Some((_, query)) = relations.iter().find(|(r, _)| *r == relation) else {
        let message = if relations.is_empty() {
            format!("{} has no relations", kind.as_str())
        } else {
            let valid = relations.iter().map(|(r, _)| r.as_str()).collect::<Vec<_>>().join(", ");
            format!(
                "relation {} is not valid for {}; valid: {valid}",
                relation.as_str(),
                kind.as_str()
            )
        };
        return Err(QueryError::Tool(message));
    };
    let id = record_id(kind, id)?;
    let exists = conn
        .prepare(&format!("SELECT 1 FROM {} WHERE {} = ?1", kind.table(), kind.key()))?
        .exists([&id])?;
    if !exists {
        return Err(QueryError::Tool(format!(
            "{} {} not found",
            kind.as_str(),
            id_text(&id)
        )));
    }
    let limit = bounded_limit(limit, 50, 1000)? as usize;
    let mut stmt = conn.prepare(query)?;
    let columns: Vec<String> = stmt.column_names().map(str::to_owned).collect();
    let mut rows = stmt.query([&id])?;
    let mut lines = Vec::new();
    let mut total = 0_usize;
    while let Some(row) = rows.next()? {
        total += 1;
        if lines.len() == limit {
            continue;
        }
        let mut line = format!("{} {}", format::value(row.get_ref(0)?), format::value(row.get_ref(1)?));
        let name = row.get_ref(2)?;
        if !format::is_default(name) {
            line.push(' ');
            line.push_str(&format::value(name));
        }
        for (index, column) in columns.iter().enumerate().skip(3) {
            let value = row.get_ref(index)?;
            if !format::is_default(value) {
                let _ = write!(line, " {column}={}", format::value(value));
            }
        }
        lines.push(line);
    }
    if total == 0 {
        return Ok(format!(
            "no {} for {} {}",
            relation.as_str(),
            kind.as_str(),
            id_text(&id)
        ));
    }
    if total > limit {
        lines.push(format!("-- {limit} of {total} shown; raise limit (max 1000)"));
    }
    Ok(lines.join("\n"))
}

pub(crate) fn sql(conn: &mut Connection, query: &str, limit: Option<u32>) -> Result<String, QueryError> {
    if query.trim().is_empty() {
        return Err(QueryError::Tool("query is empty".to_owned()));
    }
    let limit = bounded_limit(limit, 200, 1000)? as usize;
    let start = Instant::now();
    conn.progress_handler(1000, Some(move || start.elapsed() > Duration::from_secs(2)))?;
    let result = execute_sql(conn, query, limit);
    conn.progress_handler(0, None::<fn() -> bool>)?;
    result
}

fn execute_sql(conn: &Connection, query: &str, limit: usize) -> Result<String, QueryError> {
    let mut stmt = conn.prepare(query).map_err(|error| match error {
        Error::MultipleStatement => QueryError::Tool("only one statement is allowed".to_owned()),
        other => QueryError::Tool(match other.sqlite_error_code() {
            // The authorizer in `Database::open` rejects pragma assignments and transactions.
            Some(ErrorCode::AuthorizationForStatementDenied) => "only read-only statements are allowed".to_owned(),
            _ => format!("invalid SQL: {other}"),
        }),
    })?;
    if !stmt.readonly() {
        return Err(QueryError::Tool("only read-only statements are allowed".to_owned()));
    }
    let columns: Vec<String> = stmt.column_names().map(format::escape).collect();
    let count = columns.len();
    let mut rows = stmt.query([]).map_err(|error| sql_error(&error))?;
    let mut lines = vec![columns.join("\t")];
    while let Some(row) = rows.next().map_err(|error| sql_error(&error))? {
        if lines.len() > limit {
            lines.push(format!(
                "-- truncated: first {limit} rows shown; raise limit (max 1000) or narrow the query"
            ));
            break;
        }
        let mut values = Vec::with_capacity(count);
        for index in 0..count {
            values.push(format::value(row.get_ref(index).map_err(|error| sql_error(&error))?));
        }
        lines.push(values.join("\t"));
    }
    if lines.len() == 1 {
        lines.push("-- no rows".to_owned());
    }
    Ok(lines.join("\n"))
}

fn sql_error(error: &Error) -> QueryError {
    match error.sqlite_error_code() {
        Some(ErrorCode::OperationInterrupted) => {
            QueryError::Tool("query stopped after 2 seconds; narrow it".to_owned())
        }
        Some(ErrorCode::AuthorizationForStatementDenied) => {
            QueryError::Tool("only read-only statements are allowed".to_owned())
        }
        _ => QueryError::Tool(format!("SQL error: {error}")),
    }
}
