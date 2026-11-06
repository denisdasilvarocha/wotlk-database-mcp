# Installation and configuration

Connect an MCP client to the server, or run a locally compiled binary with your own compatible database.

## Requirements

- Node.js 22 or later for the npm launcher.
- An MCP client that can launch a stdio server.
- macOS or Linux on x64 or ARM64, or Windows on x64. Linux requires glibc.
- Network access to npm and GitHub to download the package and default database, unless they are already cached.

Keep npm optional dependencies enabled: they contain the platform-specific executables. See the [npm package documentation](https://docs.npmjs.com/cli/v11/configuring-npm/package-json#optionaldependencies).

## Configure your client

Use the client-specific commands or generic configuration in the [README](../README.md#install).

## Verify the connection

Restart or reconnect your client if needed, then ask it to search for Frostbolt or list the server's tools: `search`, `get`, `related`, and `sql`. You can also ask for Frostbolt rank 3; its spell ID in this snapshot is `837`.

The first database download runs in the background. The client can connect and list tools while it is running, but tool calls report progress until the database is ready. Retry the call after the download finishes.

## Select a database

The server chooses its database in this order:

1. `--db <path>`, if supplied.
2. The `WOTLK_DB` environment variable, if set for the server process.
3. The versioned platform cache.

An explicitly selected file must exist and have the expected schema version. For example, set the server's `args` to include `--db` and an absolute path:

```json
["-y", "wotlk-database-mcp", "--db", "/absolute/path/wotlk.sqlite"]
```

Adapt the path to your system and configure `WOTLK_DB` in the client if you prefer the environment variable. Restart or reconnect the server after changing its configuration.

Without an explicit path, the server uses a compatible cached database or downloads the schema release from GitHub. It checks the downloaded, decompressed database against a SHA-256 checksum embedded in the binary before installing it in the cache.

The cache contains `wotlk-database-mcp/schema-2/wotlk.sqlite` under the platform cache directory:

| Platform | Cache directory |
| --- | --- |
| macOS | `~/Library/Caches` |
| Linux | An absolute `$XDG_CACHE_HOME`, or `~/.cache` if unset or not absolute |
| Windows | The user's Local AppData folder, commonly `%LOCALAPPDATA%` |

To populate the cache before starting the client, run:

```bash
npx -y wotlk-database-mcp fetch
```

`fetch` uses an existing valid cache or downloads it if needed. It always targets the cache, even when `WOTLK_DB` is set. It exits successfully when the cache is ready.

## Run from source

Install the Rust toolchain selected by `rust-toolchain.toml`, then build the server from the repository root:

```bash
cargo build --release --locked
```

Configure the client to execute `target/release/wotlk-database-mcp` (`.exe` on Windows), with `--db` and the absolute path to a compatible database. The [development guide](development.md) explains how to build the database.

A locally compiled binary without an embedded `WOTLK_DB_SHA256` can use an existing valid cache but cannot download a missing one. Supply a compatible database with `--db` or `WOTLK_DB` instead. The checksum is embedded at compile time; setting the variable only when starting an already-built binary does not add it.

The server uses stdout for MCP messages and stderr for logs. Set `RUST_LOG` in the server's environment to change the log filter.

## Troubleshooting

### The native binary cannot be found

Check the platform and Node.js requirements above. If the launcher reports that it cannot find the native binary, reinstall with npm optional dependencies enabled; on Linux, also check that the distribution uses glibc.

### A tool reports that the database download failed

Check the error and that the machine can reach the GitHub release, then run `npx -y wotlk-database-mcp fetch`. Once it succeeds, restart or reconnect the client and retry the tool. Alternatively, select a compatible local database with `--db` or `WOTLK_DB`.

### The database file is missing or has the wrong schema

Check the path configured in the client, including the server process's `WOTLK_DB` value. Explicit paths do not fall back to the cache. Use a database built for the server's expected schema or remove the explicit path to use the versioned cache.
