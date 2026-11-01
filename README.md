# wotlk-database-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) server for the World of Warcraft:
Wrath of the Lich King database, built with the official Rust SDK
[`rmcp`](https://github.com/modelcontextprotocol/rust-sdk).

## Quick start

Install `just` first and run these commands from a Bash shell. Setup needs
network access and may request administrator privileges to install missing tools.
It installs the remaining tools, the pinned Rust toolchain, Git LFS, and hooks.

- **macOS:** Install Homebrew and Xcode Command Line Tools.
- **Linux:** Use a system with apt, dnf, or pacman. Install a C toolchain if
  your system package manager does not provide one during setup.
- **Native Windows x86_64:** Use an MSYS2 UCRT64 Bash shell with pacman, not
  cmd, PowerShell, or Git Bash alone. Install the Microsoft C++ Build Tools
  with MSVC and a Windows SDK, and use the native MSVC Rust toolchain. Install
  `just` in the UCRT64 shell with `pacman -S mingw-w64-ucrt-x86_64-just`.

WSL follows the Linux instructions and produces a Linux binary, not a native
Windows executable.

The scripts use Bash, curl, `bsdtar`, `unzip`, `jq`, and a SHA-256 utility;
setup installs missing dependencies through the supported package manager.

```sh
just setup
just build
```

The build writes `target/release/wotlk-database-mcp` (`target/release/wotlk-database-mcp.exe`
on Windows) and `data/wotlk.sqlite`.
It uses the checked-in 12-domain offsets catalog without requiring a local agent
skill. It also exports the API at the commit in
`builder/vendor/api/COMMIT`, reusing the checkout in `target/api` when possible.

## Build the database

To rebuild the database without building the server or refreshing the API export:

```sh
just build-db
```

The first build downloads the pinned TrinityCore TDB 335.26091 world dump
(about 47 MB) and [wowgaming/client-data v20.0 `Data.zip`](https://github.com/wowgaming/client-data/releases/tag/v20.0)
(about 1.2 GB). The community package also contains maps, but the build extracts
only its DBC files. Archives and extracted inputs are cached under
`target/build-db`; corrupt cached inputs are repaired. SHA-256 checks pin the
downloaded artifacts and detect corruption, but do not independently establish
Blizzard provenance. The builder checks DBC layouts and the enUS Fireball name,
not record counts.

The database bootstrap fetches the API export through Git LFS if needed. It does
not install packages. It writes `data/wotlk.sqlite`, which is tracked with Git LFS.
With cached inputs and the API export present, subsequent database builds need no
input downloads.

For existing extracted inputs, run the offline builder directly:

```sh
cargo run --release --locked -p wotlk-db-builder -- --tdb <world-dump.sql> --dbc <dbc-folder>
```

Replace the placeholders with the TDB SQL file and enUS 3.3.5a build 12340 DBC
folder paths.

## Use with an MCP client

The server speaks MCP over stdio. Add it to your client configuration:

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "/absolute/path/to/target/release/wotlk-database-mcp"
    }
  }
}
```

Logs go to stderr. Set the level with `RUST_LOG`, for example `RUST_LOG=debug`.

The database path is selected in this order: `--db <path>`, the `WOTLK_DB`
environment variable, then `data/wotlk.sqlite` relative to the project directory
compiled into the binary. Pass `--db` or set `WOTLK_DB` when relocating it.

## Tools

| Tool      | Description |
| --------- | ----------- |
| `search`  | Find records by name. |
| `get`     | Read one record or build provenance. |
| `related` | List records linked to a record. |
| `sql`     | Run one read-only SQLite statement. |

`sql` returns a tab-separated header and rows. Other tools return compact text.
Read the schema and its column comments with
`SELECT sql FROM sqlite_schema WHERE type = 'table'`.

## Development

`just setup` installs the development tools and preserves Git LFS and prek hooks.
Git LFS tracks the database and the vendored API export.

Rust tests use fixtures. `just test` also runs offline Bash provisioning tests,
which require `zip` to create archive fixtures. Neither test suite requires real
TDB or DBC inputs.
Run `just` to list commands. CI runs `just check` and additional Bash and release checks.
The vendor recipes use the checked-in inputs by default:

```sh
just vendor-offsets
just vendor-api
```

`vendor-offsets` validates the bundled catalog. To replace it, pass a folder
containing the 12 catalog Markdown files: `just vendor-offsets <src>`.
`vendor-api` uses the pin in `builder/vendor/api/COMMIT`. To change the pin,
run `just vendor-api <commit>` with a commit from `Shard-MW/milkyway-codex`.
See [AGENTS.md](AGENTS.md) for the project rules.
