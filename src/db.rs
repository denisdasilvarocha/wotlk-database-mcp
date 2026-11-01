//! Read-only database access and tool queries.

use std::{
    fmt::Write as _,
    path::Path,
    sync::Mutex,
    time::{Duration, Instant},
};

use rusqlite::{Connection, Error, ErrorCode, OpenFlags, limits::Limit, params, types::Value};

use crate::{
    format,
    kinds::{Kind, Relation},
    server::RecordId,
};

#[derive(Debug)]
pub(crate) struct Database {
    conn: Mutex<Connection>,
}

#[derive(Debug)]
pub(crate) enum QueryError {
    Invalid(String),
    Tool(String),
    Internal(String),
}

impl From<Error> for QueryError {
    fn from(error: Error) -> Self {
        Self::Internal(error.to_string())
    }
}

impl Database {
    pub(crate) fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)?;
        conn.pragma_update(None, "query_only", true)?;
        conn.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;
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

fn record_id(kind: Kind, id: RecordId) -> Result<Value, QueryError> {
    if kind.key() == "id" {
        let number = match id {
            RecordId::Number(n) => n,
            RecordId::Text(text) => text
                .parse()
                .map_err(|_| QueryError::Invalid(format!("kind {} needs a numeric id", kind.as_str())))?,
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

fn bounded_limit(limit: Option<u32>, default: u32, max: u32) -> Result<usize, QueryError> {
    let limit = limit.unwrap_or(default);
    if limit == 0 || limit > max {
        return Err(QueryError::Invalid(format!("limit must be between 1 and {max}")));
    }
    Ok(limit as usize)
}

pub(crate) fn search(
    conn: &mut Connection,
    text: &str,
    kind: Option<Kind>,
    limit: Option<u32>,
) -> Result<String, QueryError> {
    let text = text.trim();
    let tokens: Vec<_> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| format!("\"{s}\"*"))
        .collect();
    if tokens.is_empty() {
        return Err(QueryError::Invalid(
            "search text needs at least one letter or digit".to_owned(),
        ));
    }
    let limit = bounded_limit(limit, 20, 100)?;
    let mut stmt = conn.prepare("SELECT kind, id, name, subtext FROM search WHERE search MATCH ?1 AND (?3 IS NULL OR kind = ?3) ORDER BY CASE WHEN lower(name) = lower(?2) THEN 0 WHEN substr(lower(name), 1, length(?2)) = lower(?2) THEN 1 ELSE 2 END, bm25(search, 10.0, 1.0), length(name), kind, id LIMIT ?4")?;
    let limit = i64::try_from(limit).map_err(|_| QueryError::Invalid("limit is too large".to_owned()))?;
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
        return Err(QueryError::Tool(format!("no matches for \"{text}\"{suffix}")));
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
        id.ok_or_else(|| QueryError::Invalid(format!("kind {} needs an id", kind.as_str())))?,
    )?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} WHERE {} = ?1", kind.table(), kind.key()))?;
    let columns: Vec<String> = stmt.column_names().map(str::to_owned).collect();
    let all = fields.is_some_and(|fields| matches!(fields, [field] if field == "all"));
    if !all {
        for field in fields.into_iter().flatten() {
            if !columns.contains(field) {
                return Err(QueryError::Invalid(format!(
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
        return Err(QueryError::Invalid(message));
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
    let mut stmt = conn.prepare(query)?;
    let columns: Vec<String> = stmt.column_names().map(str::to_owned).collect();
    let mut rows = stmt.query([&id])?;
    let mut lines = Vec::new();
    while let Some(row) = rows.next()? {
        if lines.len() == 200 {
            lines.push("-- truncated: first 200 lines shown; use the sql tool for the full list".to_owned());
            break;
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
    if lines.is_empty() {
        return Err(QueryError::Tool(format!(
            "no {} for {} {}",
            relation.as_str(),
            kind.as_str(),
            id_text(&id)
        )));
    }
    Ok(lines.join("\n"))
}

pub(crate) fn sql(conn: &mut Connection, query: &str, limit: Option<u32>) -> Result<String, QueryError> {
    if query.trim().is_empty() {
        return Err(QueryError::Invalid("query is empty".to_owned()));
    }
    let limit = bounded_limit(limit, 200, 1000)?;
    let start = Instant::now();
    conn.progress_handler(1000, Some(move || start.elapsed() > Duration::from_secs(2)))?;
    let result = execute_sql(conn, query, limit);
    conn.progress_handler(0, None::<fn() -> bool>)?;
    result
}

fn execute_sql(conn: &Connection, query: &str, limit: usize) -> Result<String, QueryError> {
    let mut stmt = conn.prepare(query).map_err(|error| match error {
        Error::MultipleStatement => QueryError::Invalid("only one statement is allowed".to_owned()),
        other => QueryError::Invalid(format!("invalid SQL: {other}")),
    })?;
    if !stmt.readonly() {
        return Err(QueryError::Invalid("only read-only statements are allowed".to_owned()));
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
        return Err(QueryError::Tool("query returned no rows".to_owned()));
    }
    Ok(lines.join("\n"))
}

fn sql_error(error: &Error) -> QueryError {
    if error.sqlite_error_code() == Some(ErrorCode::OperationInterrupted) {
        QueryError::Tool("query stopped after 2 seconds; narrow it".to_owned())
    } else {
        QueryError::Tool(format!("SQL error: {error}"))
    }
}
