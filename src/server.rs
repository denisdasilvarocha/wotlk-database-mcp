use rmcp::{ServerHandler, handler::server::wrapper::Parameters, schemars, tool, tool_handler, tool_router};
use serde::Deserialize;

/// Arguments of the `echo` tool.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EchoParams {
    /// Text to return unchanged.
    pub text: String,
}

/// MCP server that exposes the WotLK database tools.
#[derive(Debug, Clone, Default)]
pub struct WotlkServer;

#[tool_router]
impl WotlkServer {
    #[tool(description = "Return the given text unchanged. Use it to check that the server responds.")]
    #[expect(clippy::unused_self, reason = "rmcp #[tool] methods must take &self")]
    fn echo(&self, Parameters(EchoParams { text }): Parameters<EchoParams>) -> String {
        text
    }
}

#[tool_handler(
    name = "wotlk-database-mcp",
    instructions = "Tools to query the World of Warcraft: Wrath of the Lich King database."
)]
#[expect(
    clippy::unused_async_trait_impl,
    reason = "rmcp #[tool_handler] generates async fns without .await"
)]
impl ServerHandler for WotlkServer {}
