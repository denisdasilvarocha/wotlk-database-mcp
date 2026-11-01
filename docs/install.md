# Install for an MCP harness

Use this guide to build the server and register it in the harness you are using.

## 1. Install prerequisites

Follow the [platform prerequisites](../README.md#quick-start), including installing `just` and using the specified Bash shell. Setup requires network access and may request administrator privileges. The first build downloads about 1.2 GB of client data and a 47 MB database dump.

## 2. Clone and build

In the supported Bash shell, clone the repository and run these commands from its root:

```sh
git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
just setup
just build
```

The build prepares the vendored offsets and API, database, and release binary. The outputs are `target/release/wotlk-database-mcp` and `data/wotlk.sqlite`. On native Windows, the binary has an `.exe` extension.

## 3. Register the server

Add `wotlk-database` under `mcpServers` in your harness configuration. Preserve existing server entries:

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "/absolute/path/to/wotlk-database-mcp/target/release/wotlk-database-mcp",
      "args": ["--db", "/absolute/path/to/wotlk-database-mcp/data/wotlk.sqlite"]
    }
  }
}
```

Replace both example paths with absolute paths to your checkout's binary and database. On native Windows, use the absolute path to the `.exe` binary. The harness launches the command over stdio; there is no HTTP port. Logs go to stderr.

## 4. Reconnect and verify

Reload the configuration or reconnect the server using your harness. Confirm it exposes `search`, `get`, `related`, and `sql`. Call the `sql` tool with these arguments:

```json
{"query": "SELECT 1 AS ok"}
```

The result contains the header `ok` and one row containing `1`. If the server fails to connect, inspect its stderr and check the binary and database paths.
