# WotLK Database MCP

[![npm version](https://img.shields.io/npm/v/wotlk-database-mcp)](https://registry.npmjs.org/wotlk-database-mcp/latest)
[![Monthly npm downloads](https://img.shields.io/npm/dm/wotlk-database-mcp)](https://api.npmjs.org/downloads/point/last-month/wotlk-database-mcp)

WotLK Database MCP is a Model Context Protocol server that gives AI clients read-only access to World of Warcraft: Wrath of the Lich King 3.3.5a game data, Lua and addon APIs, and Windows x86 client offsets.

## Requirements

You need Node.js 22 or later and an MCP client that supports stdio servers. Native packages support macOS and Linux on x64 or ARM64, and Windows on x64. Linux requires glibc. Keep npm optional dependencies enabled; they contain the native executables.

## Connect a client

For example, register the server with Codex:

```bash
codex mcp add wotlk-database -- npx -y wotlk-database-mcp
```

For other clients, configure `npx` with the arguments `-y` and `wotlk-database-mcp`. See the [client setup instructions](https://github.com/denisdasilvarocha/wotlk-database-mcp/blob/main/README.md#install) for client-specific commands and configuration.

Restart or reconnect your client if needed, then ask: “What is the spell ID for Frostbolt rank 3?” The snapshot identifies it as `837`.

If the default database is not cached, the server downloads and verifies it in the background. If a tool reports download progress, retry after the download finishes. To prepare the cache before connecting, run:

```bash
npx -y wotlk-database-mcp fetch
```

## Use the tools

The server provides `search`, `get`, `related`, and `sql` for finding records, following relationships, and querying SQLite. See the [tool reference](https://github.com/denisdasilvarocha/wotlk-database-mcp/blob/main/docs/tools.md) for parameters, examples, and data conventions.

The database is a snapshot; data on a customized server may differ. Use `get` with `{"kind": "meta"}` to inspect its provenance. Only offsets marked `verified` are confirmed; read their comments for caveats.

## License

MIT. See the [license](https://github.com/denisdasilvarocha/wotlk-database-mcp/blob/main/LICENSE).
