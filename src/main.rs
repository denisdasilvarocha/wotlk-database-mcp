//! Runs the WotLK database MCP server over stdio.

use std::path::PathBuf;

use anyhow::{Context, bail, ensure};
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{EnvFilter, filter::LevelFilter};
use wotlk_database_mcp::WotlkServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let mut args = std::env::args_os().skip(1);
    let path = match args.next() {
        None => std::env::var_os("WOTLK_DB").map_or_else(
            || PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/data/wotlk.sqlite")),
            PathBuf::from,
        ),
        Some(flag) if flag == "--db" => {
            let Some(path) = args.next() else {
                bail!("usage: wotlk-database-mcp [--db <path>]");
            };
            if args.next().is_some() {
                bail!("usage: wotlk-database-mcp [--db <path>]");
            }
            PathBuf::from(path)
        }
        Some(_) => bail!("usage: wotlk-database-mcp [--db <path>]"),
    };
    ensure!(
        path.is_file(),
        "database file not found: {} (pass --db <path> or set WOTLK_DB)",
        path.display()
    );
    let server = WotlkServer::open(&path).with_context(|| format!("cannot open database {}", path.display()))?;
    tracing::info!(path = %path.display(), "starting MCP server on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
