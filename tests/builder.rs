//! Acceptance tests for invalid builder inputs and output preservation.

mod common;

use rusqlite::{Connection, params, types::FromSql};
use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use wotlk_db_builder::{Inputs, build};
use wow_dbc::wrath_tables::spell_icon::SpellIconKey;

fn inputs(name: &str) -> (PathBuf, Inputs) {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let inputs = common::write_inputs(&dir, |_| {});
    (dir, inputs)
}

#[expect(
    clippy::expect_used,
    reason = "an unexpectedly successful build must fail the acceptance test"
)]
fn assert_failure(inputs: &Inputs, dir: &Path, expected: &str) {
    let out = dir.join("out.sqlite");
    let error = build(inputs, &out).expect_err("invalid input must fail");
    assert!(error.to_string().contains(expected), "unexpected error: {error}");
    assert!(!out.exists(), "failed build left output");
    assert!(
        !dir.join("out.sqlite.partial").exists(),
        "failed build left partial output"
    );
}

#[test]
fn rejects_wrong_dbc_layout() {
    let (dir, inputs) = inputs("builder-wrong-layout");
    fs::copy(inputs.dbc.join("SpellIcon.dbc"), inputs.dbc.join("Spell.dbc")).unwrap();
    assert_failure(&inputs, &dir, "Spell.dbc");
}

#[test]
fn rejects_non_english_client() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("builder-not-enus");
    let inputs = common::write_inputs(&dir, |fixtures| {
        let fireball = fixtures.spell.rows.iter_mut().find(|row| row.id.id == 133).unwrap();
        fireball.name_lang.en_gb.clear();
        fireball.name_lang.de_de = "Feuerball".into();
    });
    assert_failure(&inputs, &dir, "enUS");
}

#[test]
fn rejects_missing_dbc() {
    let (dir, inputs) = inputs("builder-missing-dbc");
    fs::remove_file(inputs.dbc.join("Map.dbc")).unwrap();
    assert_failure(&inputs, &dir, "Map.dbc");
}

#[test]
fn rejects_missing_dump() {
    let (dir, mut inputs) = inputs("builder-missing-dump");
    inputs.tdb = dir.join("none.sql");
    assert_failure(&inputs, &dir, "none.sql");
}

#[test]
fn rejects_malformed_dump() {
    let (dir, mut inputs) = inputs("builder-malformed-dump");
    let mut dump = fs::read_to_string(&inputs.tdb).unwrap();
    let last_quote = dump.rfind('\'').unwrap();
    dump.remove(last_quote);
    inputs.tdb = dir.join("bad.sql");
    fs::write(&inputs.tdb, dump).unwrap();
    assert_failure(&inputs, &dir, "TDB dump line");
}

#[test]
fn rejects_wrong_tdb_release() {
    let (dir, mut inputs) = inputs("builder-wrong-release");
    let dump = fs::read_to_string(&inputs.tdb)
        .unwrap()
        .replace("TDB 335.26091", "TDB 335.24081");
    inputs.tdb = dir.join("bad-version.sql");
    fs::write(&inputs.tdb, dump).unwrap();
    assert_failure(&inputs, &dir, "335.26091");
}

#[test]
fn preserves_existing_output_on_failure() {
    let (dir, inputs) = inputs("builder-preserve-output");
    let out = dir.join("out.sqlite");
    fs::write(&out, b"old").unwrap();
    fs::copy(inputs.dbc.join("SpellIcon.dbc"), inputs.dbc.join("Spell.dbc")).unwrap();
    let error = build(&inputs, &out).expect_err("invalid layout must fail");
    assert!(error.to_string().contains("Spell.dbc"), "unexpected error: {error}");
    assert_eq!(fs::read(&out).unwrap(), b"old");
    assert!(!dir.join("out.sqlite.partial").exists());
}

#[test]
fn rejects_cyclic_vendor_references_without_leaving_output() {
    let (dir, mut inputs) = inputs("builder-vendor-cycle");
    let original = fs::read_to_string(&inputs.tdb).unwrap();
    assert!(original.contains("(5001,0,-5000,0,0,0),"));
    let dump = original.replace("(5001,0,-5000,0,0,0),", "(5001,0,-5002,0,0,0),");
    inputs.tdb = dir.join("vendor-cycle.sql");
    fs::write(&inputs.tdb, dump).unwrap();
    assert_failure(&inputs, &dir, "cyclic vendor reference");
}

#[test]
fn rejects_branching_vendor_references_and_preserves_existing_output() {
    let (dir, mut inputs) = inputs("builder-vendor-branching");
    let mut dump = fs::read_to_string(&inputs.tdb).unwrap();
    // Only 70 rows, but more than 1.5 million recursive visits across all roots.
    for layer in 0..18 {
        for vendor in [10000 + layer * 2, 10001 + layer * 2] {
            if layer == 17 {
                write!(&mut dump, "\nINSERT INTO `npc_vendor` VALUES ({vendor},0,4002,0,0,0);").unwrap();
            } else {
                let next = 10000 + (layer + 1) * 2;
                write!(
                    &mut dump,
                    "\nINSERT INTO `npc_vendor` VALUES ({vendor},0,-{next},0,0,0),({vendor},1,-{},0,0,0);",
                    next + 1
                )
                .unwrap();
            }
        }
    }
    inputs.tdb = dir.join("vendor-branching.sql");
    fs::write(&inputs.tdb, dump).unwrap();
    let out = dir.join("out.sqlite");
    fs::write(&out, b"existing database").unwrap();

    let error = build(&inputs, &out).expect_err("excessive vendor expansion must fail");

    assert!(
        error.to_string().contains("vendor reference expansion exceeds"),
        "{error}"
    );
    assert_eq!(fs::read(&out).unwrap(), b"existing database");
    assert!(!dir.join("out.sqlite.partial").exists());
}

#[test]
fn rejects_excessively_deep_vendor_references() {
    let (dir, mut inputs) = inputs("builder-vendor-deep");
    let mut dump = fs::read_to_string(&inputs.tdb).unwrap();
    for vendor in 10000..11500 {
        write!(
            &mut dump,
            "\nINSERT INTO `npc_vendor` VALUES ({vendor},0,-{},0,0,0);",
            vendor + 1
        )
        .unwrap();
    }
    dump.push_str("\nINSERT INTO `npc_vendor` VALUES (11500,0,4002,0,0,0);");
    inputs.tdb = dir.join("vendor-deep.sql");
    fs::write(&inputs.tdb, dump).unwrap();

    assert_failure(&inputs, &dir, "vendor reference expansion exceeds");
}

#[test]
fn shared_vendor_references_preserve_slots_and_stock() {
    let (dir, mut inputs) = inputs("builder-vendor-shared");
    let mut dump = fs::read_to_string(&inputs.tdb).unwrap();
    dump.push_str(
        "\nINSERT INTO `npc_vendor` VALUES
         (6000,7,-6001,0,0,0),(6000,9,-6002,0,0,0),
         (6001,5,-6003,0,0,0),(6002,5,-6003,0,0,0),
         (6003,2,4002,3,60,1),
         (6004,1,-6003,0,0,0),(6004,2,-6003,0,0,0),
         (6005,0,-999999,0,0,0),(6005,1,0,0,0,0);",
    );
    inputs.tdb = dir.join("vendor-shared.sql");
    fs::write(&inputs.tdb, dump).unwrap();
    let out = dir.join("out.sqlite");

    build(&inputs, &out).unwrap();

    let conn = Connection::open(out).unwrap();
    let mut statement = conn
        .prepare("SELECT creature_id, slot, item_id, max_count, restock_time, extended_cost, cost FROM vendor WHERE creature_id IN (6000,6004,6005) ORDER BY creature_id, slot")
        .unwrap();
    let stock = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, String>(6)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        stock,
        vec![
            (6000, 7, 4002, 3, 60, 1, "honor:100,4002x2".into()),
            (6000, 9, 4002, 3, 60, 1, "honor:100,4002x2".into()),
            (6004, 1, 4002, 3, 60, 1, "honor:100,4002x2".into()),
            (6004, 2, 4002, 3, 60, 1, "honor:100,4002x2".into()),
        ]
    );
}

/// Builds one database from the fixtures with a spell family and icon rows that
/// exercise the search subtext and icon normalization.
#[expect(clippy::expect_used, reason = "fixture setup must fail loudly on invalid input")]
fn fixture_db() -> &'static Path {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("builder-fixture");
        let inputs = common::write_inputs(&dir, |fixtures| {
            let fireball = fixtures
                .spell
                .rows
                .iter_mut()
                .find(|row| row.id.id == 133)
                .expect("fixture spell 133");
            fireball.spell_class_set = 3;
            for row in &mut fixtures.skill_line.rows {
                row.spell_icon_id = SpellIconKey::new(1);
            }
            fixtures
                .talent_tab
                .rows
                .first_mut()
                .expect("fixture talent tab")
                .spell_icon_id = SpellIconKey::new(1);
            fixtures
                .achievement
                .rows
                .first_mut()
                .expect("fixture achievement")
                .icon_id = SpellIconKey::new(1);
            fixtures
                .spell_icon
                .rows
                .first_mut()
                .expect("fixture spell icon")
                .texture_filename = r"interface\ICONS\Spell_Fire_FlameBolt".into();
        });
        let db = dir.join("wotlk.sqlite");
        build(&inputs, &db).expect("build fixture database");
        db
    })
}

#[expect(clippy::expect_used, reason = "a missing fixture row must fail the acceptance test")]
fn read<T: FromSql>(sql: &str, parameters: impl rusqlite::Params) -> T {
    Connection::open(fixture_db())
        .expect("open fixture database")
        .query_row(sql, parameters, |row| row.get(0))
        .expect("fixture row")
}

fn search_value<T: FromSql>(kind: &str, id: &str, column: &str) -> T {
    read(
        &format!("SELECT {column} FROM search WHERE kind = ?1 AND id = ?2"),
        params![kind, id],
    )
}

fn icon(table: &str, id: i64) -> Option<String> {
    read(&format!("SELECT icon FROM {table} WHERE id = ?1"), params![id])
}

#[test]
fn search_priority_puts_player_spells_and_spawned_creatures_first() {
    assert_eq!(search_value::<i64>("spell", "133", "priority"), 0, "player spell");
    assert_eq!(search_value::<i64>("spell", "14147", "priority"), 1, "NPC spell copy");
    assert_eq!(
        search_value::<i64>("creature", "12056", "priority"),
        0,
        "spawned creature"
    );
    assert_eq!(
        search_value::<i64>("creature", "5004", "priority"),
        1,
        "unspawned creature"
    );
    assert_eq!(search_value::<i64>("item", "19019", "priority"), 0, "other kinds");
}

#[test]
fn triggered_chain_ranks_below_player_chain() {
    for (id, priority) in [
        ("133", 0),
        ("143", 0),
        ("50001", 1),
        ("50002", 1),
        ("9053", 1),
        ("11366", 0),
    ] {
        assert_eq!(search_value::<i64>("spell", id, "priority"), priority, "spell {id}");
    }
}

#[test]
fn search_spell_subtext_holds_rank_and_family() {
    assert_eq!(
        search_value::<Option<String>>("spell", "133", "subtext").as_deref(),
        Some("Rank 1 MAGE")
    );
    assert_eq!(search_value::<Option<String>>("spell", "14147", "subtext"), None);
    assert_eq!(search_value::<String>("spell", "133", "terms"), "Rank 1 MAGE");
}

#[test]
fn search_creature_subtext_holds_level_and_rank_but_terms_do_not() {
    assert_eq!(
        search_value::<Option<String>>("creature", "5003", "subtext").as_deref(),
        Some("Broodmother, level 60-63 WORLDBOSS")
    );
    assert_eq!(
        search_value::<Option<String>>("creature", "5004", "subtext").as_deref(),
        Some("level 63 WORLDBOSS")
    );
    assert_eq!(
        search_value::<Option<String>>("creature", "5001", "subtext").as_deref(),
        Some("Mage Trainer")
    );
    let terms = search_value::<String>("creature", "5003", "terms");
    assert_eq!(terms, "Broodmother");
    assert!(!terms.contains("level"), "{terms}");
    assert!(!terms.contains("WORLDBOSS"), "{terms}");
    assert_eq!(
        search_value::<Option<String>>("creature", "12056", "subtext").as_deref(),
        Some("WORLDBOSS"),
        "a creature without a subname still shows its rank"
    );
    assert_eq!(
        search_value::<String>("creature", "12056", "terms"),
        "",
        "rank text stays out of terms"
    );
}

#[test]
fn import_strips_the_interface_icons_prefix_from_client_icons() {
    for (table, id) in [
        ("spell", 133),
        ("skill", 8),
        ("glyph", 1),
        ("achievement", 6),
        ("talent_tab", 41),
    ] {
        assert_eq!(icon(table, id).as_deref(), Some("Spell_Fire_FlameBolt"), "{table} {id}");
    }
    assert_eq!(
        icon("item", 19019).as_deref(),
        Some("INV_Sword_39"),
        "bare icon names stay unchanged"
    );
}

#[test]
fn schema_documents_loot_groups_and_icon_names() {
    let conn = Connection::open(fixture_db()).expect("open fixture database");
    let loot: String = conn
        .query_row("SELECT sql FROM sqlite_schema WHERE name = 'loot'", params![], |row| {
            row.get(0)
        })
        .expect("loot schema");
    assert!(
        loot.contains("Drop chance percentage; 0 inside a group means an equal share of the group's remaining chance."),
        "{loot}"
    );
    for table in ["spell", "skill", "glyph", "achievement", "talent_tab"] {
        let sql: String = conn
            .query_row("SELECT sql FROM sqlite_schema WHERE name = ?1", params![table], |row| {
                row.get(0)
            })
            .expect("table schema");
        assert!(sql.contains("-- Icon name."), "{table}: {sql}");
    }
}
