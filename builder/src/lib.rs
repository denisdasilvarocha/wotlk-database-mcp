//! Builds the offline WotLK reference database from pinned local inputs.

mod codex;
mod dbc;
mod dump;
mod index;
mod names;
mod offsets;
mod schema;
#[expect(unreachable_pub, reason = "Shared schema constant is public for the server crate")]
#[path = "../../src/schema_version.rs"]
mod schema_version;
mod tdb;

use rusqlite::Connection;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Input locations for one database build.
#[derive(Debug, Clone)]
pub struct Inputs {
    /// Extracted TDB 335.26091 world dump (`.sql`).
    pub tdb: PathBuf,
    /// Folder with the enUS 12340 `.dbc` files.
    pub dbc: PathBuf,
    /// Offsets folder with `VERSION` and platform subfolders.
    pub offsets: PathBuf,
    /// Codex folder with `codex.json` and `COMMIT`.
    pub codex: PathBuf,
}

/// An input, parsing, or output error during a database build.
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    /// A required input is absent.
    #[error("missing {what}: {path}")]
    MissingInput {
        /// The input category.
        what: &'static str,
        /// The required location.
        path: PathBuf,
    },
    /// A file operation failed.
    #[error("{path}: {source}")]
    Io {
        /// The affected location.
        path: PathBuf,
        /// The underlying file error.
        source: io::Error,
    },
    /// The TDB dump is malformed or incompatible.
    #[error("TDB dump line {line}: {message}")]
    Dump {
        /// One-based source line.
        line: u64,
        /// The invalid input detail.
        message: String,
    },
    /// A DBC file is unreadable or incompatible.
    #[error("{file}: {message}")]
    Dbc {
        /// The DBC filename.
        file: String,
        /// The invalid input detail.
        message: String,
    },
    /// An offsets catalog is malformed.
    #[error("{path} line {line}: {message}")]
    Offsets {
        /// The catalog location.
        path: PathBuf,
        /// One-based source line, or zero for folder errors.
        line: usize,
        /// The invalid input detail.
        message: String,
    },
    /// A codex input is malformed.
    #[error("{path}: {message}")]
    Codex {
        /// The affected input location.
        path: PathBuf,
        /// The invalid input detail.
        message: String,
    },
    /// SQLite could not create or populate the database.
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

/// Builds the database at `out`, replacing it only after a successful build.
///
/// # Errors
/// Returns an error for missing or malformed inputs, file failures, or SQLite
/// failures. A failed build preserves any existing output and removes its partial file.
pub fn build(inputs: &Inputs, out: &Path) -> Result<(), BuildError> {
    tracing::info!("checking build inputs");
    require(&inputs.tdb, "TDB dump", false)?;
    require(&inputs.dbc, "DBC folder", true)?;
    for file in dbc::FILES {
        require(&inputs.dbc.join(file), "DBC file", false)?;
    }
    require(&inputs.offsets.join("VERSION"), "offsets version", false)?;
    require(&inputs.offsets.join("windows-x86"), "offsets platform", true)?;
    require(&inputs.codex.join("codex.json"), "codex JSON", false)?;
    require(&inputs.codex.join("COMMIT"), "codex commit", false)?;
    let mut partial_name = out.file_name().unwrap_or_default().to_os_string();
    partial_name.push(".partial");
    let partial = out.with_file_name(partial_name);
    if let Some(parent) = out.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|source| BuildError::Io {
            path: parent.into(),
            source,
        })?;
    }
    remove_partial(&partial)?;
    let result = populate(inputs, &partial).and_then(|()| {
        fs::rename(&partial, out).map_err(|source| BuildError::Io {
            path: out.into(),
            source,
        })
    });
    if result.is_err() {
        remove_partial(&partial)?;
    }
    result
}

fn populate(inputs: &Inputs, partial: &Path) -> Result<(), BuildError> {
    tracing::info!("creating curated schema");
    let mut conn = Connection::open(partial)?;
    conn.execute_batch("PRAGMA journal_mode=OFF; PRAGMA synchronous=OFF;")?;
    let transaction = conn.transaction()?;
    transaction.execute_batch(schema::CREATE)?;
    tracing::info!("importing client DBC data");
    let client = dbc::import(&transaction, &inputs.dbc)?;
    tracing::info!("importing TDB world data");
    let world = tdb::import(&transaction, &inputs.tdb, &client)?;
    tracing::info!("importing offsets catalog");
    let copied = offsets::import(&transaction, &inputs.offsets)?;
    tracing::info!("importing Lua API codex");
    let commit = codex::import(&transaction, &inputs.codex)?;
    tracing::info!("building search index and provenance");
    index::build(
        &transaction,
        &[
            ("tdb_release", &world.release),
            ("tdb_sha256", &world.sha256),
            ("client_build", "12340"),
            ("schema_version", &schema_version::SCHEMA_VERSION.to_string()),
            ("codex_commit", &commit),
            ("offsets_copied", &copied),
            ("builder_version", env!("CARGO_PKG_VERSION")),
        ],
    )?;
    transaction.commit()?;
    tracing::info!("compacting database");
    conn.execute_batch("VACUUM")?;
    conn.close().map_err(|(_, error)| BuildError::Sqlite(error))
}

fn require(path: &Path, what: &'static str, directory: bool) -> Result<(), BuildError> {
    if if directory { path.is_dir() } else { path.is_file() } {
        Ok(())
    } else {
        Err(BuildError::MissingInput {
            what,
            path: path.into(),
        })
    }
}

fn remove_partial(path: &Path) -> Result<(), BuildError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(BuildError::Io {
            path: path.into(),
            source,
        }),
    }
}
