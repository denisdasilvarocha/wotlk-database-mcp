# Install for an MCP harness

Connect the server to your MCP harness over stdio. Use Node.js 22 or later with npm and `npx` on your PATH.
Keep npm optional dependencies enabled so it installs the native package for your platform. You do not need Rust for the npm package.

The project code and packaged binaries use the MIT License. Third-party game data retains its own rights. See the [MIT License](../LICENSE).

## 1. Register the server

Add this entry to your harness configuration. Keep the existing MCP servers.

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "npx",
      "args": ["-y", "wotlk-database-mcp"]
    }
  }
}
```

If your harness uses another format, use the same command and arguments. Select stdio transport.

## 2. Download the database

To download the database before you connect, run:

```sh
npx -y wotlk-database-mcp fetch
```

The command shows progress on stderr. It verifies the download and exits with code 0 on success. If a valid database is cached, it exits without a download. A failure produces a non-zero exit code.

You can also connect without this step. With an empty cache, the server answers the MCP handshake and lists its tools at once. It then downloads the database in the background. Until the database is ready, tools return a downloading error with a percentage and an instruction to retry. After the download, tools work without a restart.

The server stores schema version 1 at these locations:

| Platform | Cache file |
| --- | --- |
| macOS | `~/Library/Caches/wotlk-database-mcp/schema-1/wotlk.sqlite` |
| Linux | `$XDG_CACHE_HOME/wotlk-database-mcp/schema-1/wotlk.sqlite`, or `~/.cache/wotlk-database-mcp/schema-1/wotlk.sqlite` when unset |
| Windows | `%LOCALAPPDATA%/wotlk-database-mcp/schema-1/wotlk.sqlite` |

## 3. Select a local database

To use an existing database instead of the cache, pass its path:

```sh
npx -y wotlk-database-mcp --db /absolute/path/to/wotlk.sqlite
```

In the client configuration, append `"--db", "/absolute/path/to/wotlk.sqlite"` to `args`. Replace the example path with your database path.

You can also set `WOTLK_DB` in the server process environment. `--db` takes precedence over `WOTLK_DB`. Both bypass downloads. A missing explicit file causes the server to exit with an error that names the path.

> [!IMPORTANT]
> The server requires `meta.schema_version` to equal `1`. It refuses databases with a missing or different version. Rebuild with `just build-db` or download the matching database.

### Download manually

If the automatic download fails, download and decompress the asset. These commands use a POSIX shell, `curl`, and `zstd`:

```sh
curl -fL -O https://github.com/denisdasilvarocha/wotlk-database-mcp/releases/download/db-schema1/wotlk-db-schema1.sqlite.zst
curl -fL -O https://github.com/denisdasilvarocha/wotlk-database-mcp/releases/download/db-schema1/wotlk-db-schema1.sqlite.sha256
zstd -d wotlk-db-schema1.sqlite.zst
sha256sum -c wotlk-db-schema1.sqlite.sha256
export WOTLK_DB="$PWD/wotlk-db-schema1.sqlite"
npx -y wotlk-database-mcp
```

On macOS, use `shasum -a 256 -c wotlk-db-schema1.sqlite.sha256` instead of `sha256sum`. The checksum covers the decompressed SQLite file. Set `WOTLK_DB` in your harness environment as well.

The asset URL pattern is `https://github.com/denisdasilvarocha/wotlk-database-mcp/releases/download/db-schema<N>/wotlk-db-schema<N>.sqlite.zst`. Replace `<N>` with the binary's schema version, currently `1`.

### Use a proxy

Set `HTTPS_PROXY` or `https_proxy` in the server environment before you start it. Database downloads use that proxy.

## 4. Check platform support

The npm package supplies these native binaries. It needs no install script and supports installation with scripts disabled.

| Platform | Architectures |
| --- | --- |
| macOS | ARM64 and x86_64 |
| Linux | x86_64 and ARM64 (GNU targets) |
| Windows | x86_64 |

## 5. Build from source

Use source builds for development or a local database build. They require Rust 1.99 or later.

### Install prerequisites

Install the base tools for your platform before you run `just setup`.

| Platform | Requirements |
| --- | --- |
| macOS | Homebrew and Xcode Command Line Tools (`xcode-select --install`). |
| Linux | `apt-get`, `dnf`, or `pacman`, and `sudo` or a root shell. WSL builds a Linux binary. |
| Windows x86_64 | MSYS2 UCRT64 Bash, Windows PowerShell, and Microsoft C++ Build Tools with the C++ desktop workload and a Windows SDK. |

Install `just` with the command for your platform.

| Platform | Command |
| --- | --- |
| macOS | `brew install just` |
| Debian, Ubuntu | `sudo apt install just` |
| Fedora | `sudo dnf install just` |
| Arch Linux | `sudo pacman -S just` |
| Windows (MSYS2 UCRT64) | `pacman -S just` |

Use Bash for the following commands. On Windows, use MSYS2 UCRT64, not Git Bash, cmd, or PowerShell alone. You need repository access and a network connection.

### Clone and build

The build downloads inputs and creates the database and release binary.

> [!IMPORTANT]
> Read the [download warning](guides/rebuild-database.md) before the first build.

```sh
git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
just setup
just build
just run
```

`just run` passes `--db data/wotlk.sqlite`. The equivalent Cargo command is:

```sh
cargo run -- --db data/wotlk.sqlite
```

To register a source build, set the client command to the absolute path of `target/release/wotlk-database-mcp`. On Windows, add `.exe`. Set `args` to `["--db", "/absolute/path/to/wotlk-database-mcp/data/wotlk.sqlite"]`.

A build without an embedded database checksum cannot download a database. It still answers the handshake and lists tools. Use `--db` or `WOTLK_DB` when no valid cache exists. Its `fetch` command exits with an error.

For development recipes, see [Contributing](../CONTRIBUTING.md).

## 6. Verify the connection

Use your harness to check that the server answers tool calls.

1. Reload the configuration or reconnect the server.
2. Confirm that the harness lists `search`, `get`, `related`, and `sql`.
3. Call `sql` with these arguments:

   ```json
   {"query": "SELECT 1 AS ok"}
   ```

If the tool reports a download, retry after it finishes. The ready database returns:

```text
ok
1
```

## Troubleshoot the connection

Read the server's stderr when the harness cannot connect. For download failures, check the cause and asset URL in the tool error. Run `npx -y wotlk-database-mcp fetch` to retry, or use the manual download steps above.

For a missing local file, check the [database path configuration](reference/configuration.md). For a schema mismatch, rebuild or download schema version 1. For a missing source binary, run `just build` and check the client command path.

## Related pages

Use these pages for the next task.

- [Rebuild the database](guides/rebuild-database.md) after an input or search-index update.
- [Tools reference](reference/tools.md) for tool arguments and output.
- [Documentation index](index.md) for all documentation routes.
