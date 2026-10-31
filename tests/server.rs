//! End-to-end tests: a real MCP client talks to the server over an in-memory pipe.

use rmcp::{ServiceExt, model::CallToolRequestParams, object, serde_json};
use wotlk_database_mcp::WotlkServer;

#[tokio::test]
async fn echo_returns_input_text() -> anyhow::Result<()> {
    let (server_io, client_io) = tokio::io::duplex(4096);
    let server = tokio::spawn(async move { anyhow::Ok(WotlkServer.serve(server_io).await?.waiting().await?) });
    let client = ().serve(client_io).await?;

    let result = client
        .call_tool(CallToolRequestParams::new("echo").with_arguments(object!({ "text": "Arthas" })))
        .await?;
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|text| text.text.as_str());

    assert_eq!(text, Some("Arthas"));
    client.cancel().await?;
    server.await??;
    Ok(())
}
