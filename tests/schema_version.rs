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
    let error = WotlkServer::open(&path).expect_err("unversioned database must be rejected");
    assert_eq!(error.to_string(), "expected schema version 1, found none");
}

#[test]
fn open_rejects_wrong_schema_version() {
    let path = database("schema-version-wrong", Some("2"));
    let error = WotlkServer::open(&path).expect_err("incompatible database must be rejected");
    assert_eq!(error.to_string(), "expected schema version 1, found 2");
}

#[test]
fn open_rejects_malformed_schema_version() {
    let path = database("schema-version-malformed", Some("unknown"));
    let error = WotlkServer::open(&path).expect_err("malformed version must be rejected");
    assert_eq!(error.to_string(), "expected schema version 1, found unknown");
}

#[test]
fn open_rejects_database_without_metadata() {
    let path = database("schema-version-no-meta", None);
    Connection::open(&path)
        .unwrap()
        .execute_batch("DROP TABLE meta")
        .unwrap();
    let error = WotlkServer::open(&path).expect_err("missing metadata must be rejected");
    assert_eq!(error.to_string(), "expected schema version 1, found none");
}

#[test]
fn open_accepts_matching_schema_version() {
    let path = database("schema-version-matching", Some("1"));
    WotlkServer::open(&path).expect("matching database must open");
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
    let path = database("schema-version-corrupt", Some("1"));
    fs::write(&path, b"not a sqlite database").unwrap();
    let error = WotlkServer::open(&path).expect_err("corrupt database must be rejected");
    assert!(
        matches!(error, wotlk_database_mcp::DatabaseError::Sql(_)),
        "unexpected error: {error}"
    );
}
