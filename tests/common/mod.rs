use std::{fs, path::Path};

/// Editable client DBC fixtures for integration tests.
pub(crate) mod dbc;

/// Writes all inputs for a database build, allowing DBC rows to be customized.
#[expect(
    clippy::expect_used,
    reason = "fixture setup must fail the test when its directory is unavailable"
)]
pub(crate) fn write_inputs(dir: &Path, edit: impl FnOnce(&mut dbc::Fixtures)) -> wotlk_db_builder::Inputs {
    if dir.exists() {
        fs::remove_dir_all(dir).expect("remove previous fixture directory");
    }
    fs::create_dir_all(dir).expect("create fixture directory");
    let mut fixtures = dbc::Fixtures::new();
    edit(&mut fixtures);
    let dbc = dir.join("dbc");
    fixtures.write(&dbc);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    wotlk_db_builder::Inputs {
        tdb: root.join("tdb.sql"),
        dbc,
        offsets: root.join("offsets"),
        codex: root.join("codex"),
    }
}
