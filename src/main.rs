//! Runs the WotLK database MCP server over stdio.

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

    tracing::info!("starting MCP server on stdio");
    let service = WotlkServer.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
