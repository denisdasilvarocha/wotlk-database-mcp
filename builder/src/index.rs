//! Search index and build provenance.

use rusqlite::{Connection, params};

use crate::{BuildError, schema};

/// How a source supplies the extra full-text terms of a record.
#[derive(Clone, Copy)]
enum Terms {
    /// Append the subtext, which repeats useful detail such as a spell rank.
    Subtext,
    /// Append the separate terms column instead of the subtext.
    Separate,
}

/// One searchable kind.
struct Source {
    /// Kind name stored in `search.kind`.
    kind: &'static str,
    /// Query returning a textual key, a nonempty name, extra terms, optional
    /// subtext, and an integer priority (0 primary, 1 secondary).
    sql: &'static str,
    /// Where the extra full-text terms come from.
    terms: Terms,
}

// Every query returns a textual key, a nonempty name, extra terms, optional
// subtext, and a priority. Player-learnable spells and spawned creatures are
// primary (0); unlearnable spells and unspawned creatures are secondary (1);
// every other record is primary.
const SOURCES: &[Source] = &[
    Source {
        kind: "item",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, NULLIF(trim(coalesce(quality || ' ', '') || coalesce(subclass || ' ', '') || coalesce('ilvl ' || item_level, '')), ''), 0 FROM item WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "creature",
        terms: Terms::Separate,
        sql: "SELECT CAST(id AS TEXT), name, subname, \
              NULLIF(trim(coalesce(subname || CASE WHEN level_text IS NOT NULL OR rank IS NOT NULL THEN ', ' ELSE '' END, '') \
              || coalesce(level_text, '') \
              || CASE WHEN level_text IS NOT NULL AND rank IS NOT NULL THEN ' ' ELSE '' END \
              || coalesce(rank, '')), ''), priority \
              FROM (SELECT id, name, NULLIF(trim(subname), '') AS subname, rank, \
                    CASE WHEN min_level IS NULL AND max_level IS NULL THEN NULL \
                         WHEN min_level IS NULL THEN 'level ' || max_level \
                         WHEN max_level IS NULL OR max_level <= min_level THEN 'level ' || min_level \
                         ELSE 'level ' || min_level || '-' || max_level END AS level_text, \
                    CASE WHEN EXISTS (SELECT 1 FROM creature_spawn WHERE creature_id = creature.id) THEN 0 ELSE 1 END AS priority \
                    FROM creature WHERE name IS NOT NULL AND name <> '')",
    },
    Source {
        kind: "spell",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, NULLIF(trim(coalesce(rank || ' ', '') || coalesce(family, '')), ''), \
              CASE WHEN EXISTS (SELECT 1 FROM skill_ability WHERE spell_id = spell.id) \
                     OR EXISTS (SELECT 1 FROM trainer_spell WHERE spell_id = spell.id) \
                     OR EXISTS (SELECT 1 FROM talent_spell WHERE spell_id = spell.id) \
                     OR EXISTS (SELECT 1 FROM spell_rank r WHERE r.spell_id = spell.id AND ( \
                         EXISTS (SELECT 1 FROM skill_ability WHERE spell_id = r.first_spell_id) \
                         OR EXISTS (SELECT 1 FROM trainer_spell WHERE spell_id = r.first_spell_id) \
                         OR EXISTS (SELECT 1 FROM talent_spell WHERE spell_id = r.first_spell_id))) THEN 0 ELSE 1 END \
              FROM spell WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "itemset",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, NULL, 0 FROM itemset WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "quest",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, 'level ' || level, 0 FROM quest WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "gameobject",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, type, 0 FROM gameobject WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "map",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, type, 0 FROM map WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "area",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, map, 0 FROM area WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "faction",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, NULL, 0 FROM faction WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "talent",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, tab, 0 FROM talent WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "skill",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, category, 0 FROM skill WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "glyph",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, type, 0 FROM glyph WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "achievement",
        terms: Terms::Subtext,
        sql: "SELECT CAST(id AS TEXT), name, NULL, category, 0 FROM achievement WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "offset",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, NULLIF(trim(coalesce(va || ' ', '') || coalesce(status, '')), ''), 0 FROM offsets WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "lua_function",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, signature, 0 FROM lua_function WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "lua_event",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, category, 0 FROM lua_event WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "widget",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, category, 0 FROM widget WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "cvar",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, 'default=' || default_value, 0 FROM cvar WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "lua_type",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, NULL, 0 FROM lua_type WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "combatlog_event",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, type, 0 FROM combatlog_event WHERE name IS NOT NULL AND name <> ''",
    },
    Source {
        kind: "secure_template",
        terms: Terms::Subtext,
        sql: "SELECT name, name, NULL, NULL, 0 FROM secure_template WHERE name IS NOT NULL AND name <> ''",
    },
];
/// Creates secondary indexes, the unified search index, and provenance records.
pub(crate) fn build(conn: &Connection, meta: &[(&str, &str)]) -> Result<(), BuildError> {
    conn.execute_batch(schema::SECONDARY_INDEXES)?;
    conn.execute_batch(
        "CREATE VIRTUAL TABLE search USING fts5(\
         name, terms, kind UNINDEXED, id UNINDEXED, subtext UNINDEXED, priority UNINDEXED, \
         tokenize = 'unicode61 remove_diacritics 2', prefix = '2 3')",
    )?;

    let mut insert =
        conn.prepare("INSERT INTO search(name, terms, kind, id, subtext, priority) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
    for source in SOURCES {
        let mut select = conn.prepare(source.sql)?;
        let mut rows = select.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let extra: Option<String> = row.get(2)?;
            let subtext: Option<String> = row.get(3)?;
            let priority: i64 = row.get(4)?;
            let mut terms = split_identifier(&name);
            if terms == name {
                terms.clear();
            }
            let extra = match source.terms {
                Terms::Subtext => subtext.as_deref(),
                Terms::Separate => extra.as_deref(),
            };
            if let Some(extra) = extra.filter(|text| !text.is_empty()) {
                if !terms.is_empty() {
                    terms.push(' ');
                }
                terms.push_str(extra);
            }
            insert.execute(params![name, terms, source.kind, id, subtext, priority])?;
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
