//! Schema compatibility checks through the explicit database-open boundary.

mod common;

use std::{fs, path::PathBuf};

use rusqlite::Connection;
use wotlk_database_mcp::WotlkServer;

#[expect(clippy::expect_used, reason = "fixture setup must fail loudly on invalid input")]
fn database(name: &str, version: Option<&str>) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    fs::create_dir_all(&dir).expect("create fixture directory");
    let path = dir.join("wotlk.sqlite");
    fs::remove_file(&path)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(error)
            }
        })
        .expect("remove previous fixture database");
    let connection = Connection::open(&path).expect("open fixture database");
    connection
        .execute_batch(
            "CREATE TABLE meta(name TEXT PRIMARY KEY, value TEXT);
             INSERT INTO meta VALUES ('client_build', '12340');",
        )
        .expect("create fixture metadata");
    if let Some(version) = version {
        connection
            .execute("INSERT INTO meta VALUES ('schema_version', ?1)", [version])
            .expect("insert fixture schema version");
    }
    path
}

#[test]
fn open_rejects_missing_schema_version() {
    let path = database("schema-version-missing", None);
    WotlkServer::open(&path).expect_err("unversioned database must be rejected");
}

#[test]
fn open_schema_version_2_rejects_version_1() {
    let path = database("schema-version-wrong", Some("1"));
    WotlkServer::open(&path).expect_err("schema version 1 must be rejected");
}

#[test]
fn open_rejects_malformed_schema_version() {
    let path = database("schema-version-malformed", Some("unknown"));
    WotlkServer::open(&path).expect_err("malformed version must be rejected");
}

#[test]
fn open_rejects_database_without_metadata() {
    let path = database("schema-version-no-meta", None);
    Connection::open(&path)
        .unwrap()
        .execute_batch("DROP TABLE meta")
        .unwrap();
    WotlkServer::open(&path).expect_err("missing metadata must be rejected");
}

#[test]
fn open_schema_version_2_accepts_version_2() {
    let path = database("schema-version-matching", Some("2"));
    WotlkServer::open(&path).expect("schema version 2 must open");
}

#[test]
fn builder_database_opens_with_current_schema_version() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("schema-version-builder");
    let inputs = common::write_inputs(&dir, |_| {});
    let path = dir.join("wotlk.sqlite");
    wotlk_db_builder::build(&inputs, &path).expect("build fixture database");
    WotlkServer::open(&path).expect("builder must write a compatible schema version");
}

#[test]
fn open_preserves_sqlite_failures() {
    let path = database("schema-version-corrupt", Some("2"));
    fs::write(&path, b"not a sqlite database").unwrap();
    let error = WotlkServer::open(&path).expect_err("corrupt database must be rejected");
    assert!(
        error.downcast_ref::<rusqlite::Error>().is_some(),
        "unexpected error: {error}"
    );
}
