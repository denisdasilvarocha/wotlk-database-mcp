//! Acceptance tests for invalid builder inputs and output preservation.

mod common;

use std::{
    fs,
    path::{Path, PathBuf},
};
use wotlk_db_builder::{Inputs, build};

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
