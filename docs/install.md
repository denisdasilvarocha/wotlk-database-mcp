# Installation and configuration

## Run with npm

Install Node.js 22 or later and use an MCP client that launches stdio servers. The npm launcher selects a native package for macOS or Linux on x64 or ARM64, or Windows on x64. Linux requires glibc.

Keep optional dependencies enabled when installing the package; they contain the native executables.

Configure your client to run `npx` with arguments `-y` and `wotlk-database-mcp`. Client configuration locations and formats differ; open Other in the [README](../README.md#ai-agents) for a common JSON format. See the [npm documentation](https://docs.npmjs.com/cli/v11/commands/npx/) for how `npx` runs packages.

## Select a database

The server chooses its database in this order:

1. `--db <path>`, if supplied.
2. The `WOTLK_DB` environment variable.
3. The versioned platform cache, under `wotlk-database-mcp/schema-2/wotlk.sqlite`.

An explicitly selected file must exist and have the expected schema version. For example, set the server's `args` to include `--db` and the absolute path:

```json
["-y", "wotlk-database-mcp", "--db", "/absolute/path/wotlk.sqlite"]
```

Without an explicit path, the server uses a valid cached database or downloads it in the background from the project's GitHub release. The server accepts connections during the download; tool calls report progress or failure until the database is ready. The server verifies the download against a checksum embedded in the binary.

To populate the cache before starting the client, run `npx -y wotlk-database-mcp fetch`. This command always targets the cache, even when `WOTLK_DB` is set.

A locally compiled binary without an embedded `WOTLK_DB_SHA256` cannot download a missing cache. Use `--db` or `WOTLK_DB` with a compatible database instead. The [development guide](development.md) explains how to build one.

## Run from source

With the Rust toolchain installed, build the server from the repository root:

```bash
cargo build --release --locked
```

Configure the client to execute the resulting `target/release/wotlk-database-mcp` binary (with `.exe` on Windows) and supply `--db` and an absolute path to a compatible database. The server uses stdout for MCP messages and stderr for logs; `RUST_LOG` controls the log filter.
