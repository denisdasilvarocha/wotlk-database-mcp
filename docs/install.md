# Install for an MCP harness

Follow these steps to build the server and register it in the harness you run in.

## 1. Install prerequisites

Install the platform requirements and `just` from the [README prerequisites](../README.md#prerequisites). On Windows, use the MSYS2 UCRT64 terminal for every command. The build needs network access. On Linux, setup can ask for `sudo`.

## 2. Clone and build

```sh
git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
just setup
just build
```

The first build downloads about 1.2 GB of client data and a 47 MB TDB dump. It produces two files:

- `target/release/wotlk-database-mcp` (`wotlk-database-mcp.exe` on Windows)
- `data/wotlk.sqlite`

## 3. Register the server

Add a `wotlk-database` entry under `mcpServers` in the harness configuration. Keep the existing entries:

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

- Replace both paths with absolute paths into the checkout.
- On Windows, point `command` at the `.exe` file and write paths with forward slashes, for example `C:/src/wotlk-database-mcp/data/wotlk.sqlite`.
- If the harness uses another configuration format, put the binary path in its command field and `--db <path>` in its arguments.

The server uses stdio transport. It has no HTTP port and needs no environment variables.

## 4. Verify

1. Reload the harness configuration or reconnect the server.
2. Confirm that the server shows the tools `search`, `get`, `related`, and `sql`.
3. Call `sql` with `{"query": "SELECT 1 AS ok"}`. The result is the line `ok` followed by the line `1`.

If the server does not connect, read its stderr. `database file not found` means the `--db` path is wrong. A missing binary means `just build` did not finish.

After updating the server, rebuild with `just build` and reconnect the harness. Spell-search priority is stored in the generated database, so updating only the binary does not update its search index.
