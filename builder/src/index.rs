//! Search index and build provenance.

use rusqlite::{Connection, params};

use crate::{BuildError, schema};

// Every query returns a textual key, a nonempty name, and optional subtext.
const SOURCES: &[(&str, &str)] = &[
    (
        "item",
        "SELECT CAST(id AS TEXT), name, NULLIF(trim(coalesce(quality || ' ', '') || coalesce(subclass || ' ', '') || coalesce('ilvl ' || item_level, '')), '') FROM item WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "creature",
        "SELECT CAST(id AS TEXT), name, subname FROM creature WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "spell",
        "SELECT CAST(id AS TEXT), name, rank FROM spell WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "itemset",
        "SELECT CAST(id AS TEXT), name, NULL FROM itemset WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "quest",
        "SELECT CAST(id AS TEXT), name, 'level ' || level FROM quest WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "gameobject",
        "SELECT CAST(id AS TEXT), name, type FROM gameobject WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "map",
        "SELECT CAST(id AS TEXT), name, type FROM map WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "area",
        "SELECT CAST(id AS TEXT), name, map FROM area WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "faction",
        "SELECT CAST(id AS TEXT), name, NULL FROM faction WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "talent",
        "SELECT CAST(id AS TEXT), name, tab FROM talent WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "skill",
        "SELECT CAST(id AS TEXT), name, category FROM skill WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "glyph",
        "SELECT CAST(id AS TEXT), name, type FROM glyph WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "achievement",
        "SELECT CAST(id AS TEXT), name, category FROM achievement WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "offset",
        "SELECT name, name, NULLIF(trim(coalesce(va || ' ', '') || coalesce(status, '')), '') FROM offsets WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "lua_function",
        "SELECT name, name, signature FROM lua_function WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "lua_event",
        "SELECT name, name, category FROM lua_event WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "widget",
        "SELECT name, name, category FROM widget WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "cvar",
        "SELECT name, name, 'default=' || default_value FROM cvar WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "lua_type",
        "SELECT name, name, NULL FROM lua_type WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "combatlog_event",
        "SELECT name, name, type FROM combatlog_event WHERE name IS NOT NULL AND name <> ''",
    ),
    (
        "secure_template",
        "SELECT name, name, NULL FROM secure_template WHERE name IS NOT NULL AND name <> ''",
    ),
];
/// Creates secondary indexes, the unified search index, and provenance records.
pub(crate) fn build(conn: &Connection, meta: &[(&str, &str)]) -> Result<(), BuildError> {
    conn.execute_batch(schema::SECONDARY_INDEXES)?;
    conn.execute_batch(
        "CREATE VIRTUAL TABLE search USING fts5(\
         name, terms, kind UNINDEXED, id UNINDEXED, subtext UNINDEXED, \
         tokenize = 'unicode61 remove_diacritics 2', prefix = '2 3')",
    )?;

    let mut insert = conn.prepare("INSERT INTO search(name, terms, kind, id, subtext) VALUES (?1, ?2, ?3, ?4, ?5)")?;
    for &(kind, sql) in SOURCES {
        let mut select = conn.prepare(sql)?;
        let mut rows = select.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let subtext: Option<String> = row.get(2)?;
            let mut terms = split_identifier(&name);
            if terms == name {
                terms.clear();
            }
            if let Some(subtext) = subtext.as_deref().filter(|s| !s.is_empty()) {
                if !terms.is_empty() {
                    terms.push(' ');
                }
                terms.push_str(subtext);
            }
            insert.execute(params![name, terms, kind, id, subtext])?;
        }
    }

    let mut insert_meta = conn.prepare("INSERT INTO meta(name, value) VALUES (?1, ?2)")?;
    for &(name, value) in meta {
        insert_meta.execute(params![name, value])?;
    }
    Ok(())
}
fn split_identifier(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut chars = name.chars().peekable();
    let mut previous: Option<char> = None;
    while let Some(ch) = chars.next() {
        if matches!(ch, ':' | '_' | '.') {
            if !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
            previous = None;
            continue;
        }
        let boundary = previous.is_some_and(|prev| {
            (prev.is_lowercase() && ch.is_uppercase())
                || (prev.is_alphabetic() && ch.is_numeric())
                || (prev.is_numeric() && ch.is_alphabetic())
                || (prev.is_uppercase() && ch.is_uppercase() && chars.peek().is_some_and(|next| next.is_lowercase()))
        });
        if boundary && !result.ends_with(' ') {
            result.push(' ');
        }
        result.push(ch);
        previous = Some(ch);
    }
    result.truncate(result.trim_end().len());
    result
}
