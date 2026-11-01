//! Runs the WotLK database MCP server over stdio.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use anyhow::{Context, bail, ensure};
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{EnvFilter, filter::LevelFilter};
use wotlk_database_mcp::{SCHEMA_VERSION, WotlkServer, fetch_database};

const USAGE: &str = "usage: wotlk-database-mcp [--db <path>] | wotlk-database-mcp fetch";

#[derive(Debug, PartialEq, Eq)]
enum DatabasePath {
    Explicit(PathBuf),
    Cached(PathBuf),
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Serve(DatabasePath),
    Fetch(PathBuf),
}

fn cache_database_path(root: Option<&Path>) -> anyhow::Result<PathBuf> {
    let root = root.context("cannot determine platform cache directory; pass --db <path> or set WOTLK_DB")?;
    Ok(root
        .join("wotlk-database-mcp")
        .join(format!("schema-{SCHEMA_VERSION}"))
        .join("wotlk.sqlite"))
}

fn resolve_command(
    arguments: impl IntoIterator<Item = OsString>,
    environment_database: Option<&OsStr>,
    cache_root: Option<&Path>,
) -> anyhow::Result<Command> {
    let mut arguments = arguments.into_iter();
    match arguments.next() {
        None => match environment_database {
            Some(path) => Ok(Command::Serve(DatabasePath::Explicit(PathBuf::from(path)))),
            None => Ok(Command::Serve(DatabasePath::Cached(cache_database_path(cache_root)?))),
        },
        Some(flag) if flag == "--db" => {
            let path = arguments.next().context(USAGE)?;
            ensure!(arguments.next().is_none(), USAGE);
            Ok(Command::Serve(DatabasePath::Explicit(PathBuf::from(path))))
        }
        Some(command) if command == "fetch" => {
            ensure!(arguments.next().is_none(), USAGE);
            Ok(Command::Fetch(cache_database_path(cache_root)?))
        }
        Some(_) => bail!(USAGE),
    }
}

fn main() -> anyhow::Result<()> {
    // stdout carries the MCP protocol, so logs must go to stderr.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let environment_database = std::env::var_os("WOTLK_DB");
    let cache_root = dirs::cache_dir();
    let command = resolve_command(
        std::env::args_os().skip(1),
        environment_database.as_deref(),
        cache_root.as_deref(),
    )?;
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let result = runtime.block_on(run_command(command));
    // An installer can be blocked on another process's cache lock. Do not wait
    // for it after stdio closes; process exit releases all OS resources.
    runtime.shutdown_background();
    result
}

async fn run_command(command: Command) -> anyhow::Result<()> {
    let (server, path) = match command {
        Command::Fetch(path) => {
            tokio::task::spawn_blocking(move || {
                fetch_database(&path, |progress| {
                    tracing::info!(progress, "fetching database");
                })
                .with_context(|| format!("cannot fetch database {}", path.display()))
            })
            .await
            .context("database fetch task failed")??;
            return Ok(());
        }
        Command::Serve(DatabasePath::Explicit(path)) => {
            ensure!(
                path.is_file(),
                "database file not found: {} (pass --db <path> or set WOTLK_DB)",
                path.display()
            );
            let server =
                WotlkServer::open(&path).with_context(|| format!("cannot open database {}", path.display()))?;
            (server, path)
        }
        Command::Serve(DatabasePath::Cached(path)) => (WotlkServer::cached(&path), path),
    };
    tracing::info!(path = %path.display(), "starting MCP server on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command_selects_versioned_cache() -> anyhow::Result<()> {
        assert_eq!(
            resolve_command([], None, Some(Path::new("/cache")))?,
            Command::Serve(DatabasePath::Cached(PathBuf::from(
                "/cache/wotlk-database-mcp/schema-1/wotlk.sqlite"
            )))
        );
        Ok(())
    }

    #[test]
    fn explicit_argument_wins_over_environment_without_cache_root() -> anyhow::Result<()> {
        assert_eq!(
            resolve_command(
                ["--db", "/argument.sqlite"].map(OsString::from),
                Some(OsStr::new("/environment.sqlite")),
                None,
            )?,
            Command::Serve(DatabasePath::Explicit(PathBuf::from("/argument.sqlite")))
        );
        Ok(())
    }

    #[test]
    fn environment_path_wins_over_cache() -> anyhow::Result<()> {
        assert_eq!(
            resolve_command([], Some(OsStr::new("/environment.sqlite")), Some(Path::new("/cache")),)?,
            Command::Serve(DatabasePath::Explicit(PathBuf::from("/environment.sqlite")))
        );
        Ok(())
    }

    #[test]
    fn fetch_ignores_explicit_environment_path() -> anyhow::Result<()> {
        assert_eq!(
            resolve_command(
                [OsString::from("fetch")],
                Some(OsStr::new("/environment.sqlite")),
                Some(Path::new("/cache")),
            )?,
            Command::Fetch(PathBuf::from("/cache/wotlk-database-mcp/schema-1/wotlk.sqlite"))
        );
        Ok(())
    }

    #[test]
    fn unsupported_arguments_report_exact_usage() {
        for arguments in [
            vec!["--db"],
            vec!["--db", "a", "b"],
            vec!["fetch", "--db", "a"],
            vec!["serve"],
            vec!["--help"],
        ] {
            let result = resolve_command(
                arguments.into_iter().map(OsString::from),
                None,
                Some(Path::new("/cache")),
            );
            assert_eq!(result.err().map(|error| error.to_string()).as_deref(), Some(USAGE));
        }
    }

    #[test]
    fn missing_platform_cache_directory_is_an_error() {
        assert!(resolve_command([], None, None).is_err());
    }

    #[tokio::test]
    async fn foreground_fetch_accepts_valid_cache_without_downloading() -> anyhow::Result<()> {
        struct TemporaryCache(PathBuf);
        impl Drop for TemporaryCache {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos();
        let directory = TemporaryCache(
            std::env::temp_dir().join(format!("wotlk-foreground-fetch-{}-{timestamp}", std::process::id())),
        );
        std::fs::create_dir_all(&directory.0)?;
        let path = directory.0.join("wotlk.sqlite");
        let connection = rusqlite::Connection::open(&path)?;
        connection.execute_batch("CREATE TABLE meta(name TEXT PRIMARY KEY, value TEXT);")?;
        connection.execute(
            "INSERT INTO meta VALUES ('schema_version', ?1)",
            [SCHEMA_VERSION.to_string()],
        )?;
        connection.execute("INSERT INTO meta VALUES ('client_build', '12340')", [])?;
        drop(connection);

        run_command(Command::Fetch(path)).await?;
        Ok(())
    }
}
