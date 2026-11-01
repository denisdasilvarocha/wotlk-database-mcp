# Proposal

## Why

Today a user must clone the repo, build the Rust binary, and put two absolute paths into the MCP client config. The binary also falls back to a compile-time path (`CARGO_MANIFEST_DIR/data/wotlk.sqlite`), so a built binary only works inside its own checkout. A `npx -y wotlk-database-mcp` entry removes the clone and build steps on macOS, Linux, and Windows.

## What Changes

- Publish the server to npm as the unscoped package `wotlk-database-mcp`, with one native binary package per platform: darwin-arm64, darwin-x64, linux-x64, linux-arm64, win32-x64.
- Publish the SQLite database as a compressed GitHub Release asset with a sha256 checksum, separate from the npm packages.
- On start without `--db` or `WOTLK_DB`, the server uses a cached database in the user cache directory. If no valid cached database exists, the server finishes the MCP handshake first and downloads the database in the background. Until the download ends, tools return a "database downloading" error with progress.
- Add a `fetch` subcommand that downloads and verifies the database, then exits.
- Add a `schema_version` key to the `meta` table. The binary accepts only a database with the matching schema version.
- **BREAKING**: Remove the compile-time fallback to `data/wotlk.sqlite` in the checkout. Local development passes `--db` or sets `WOTLK_DB`.
- Add a release workflow that builds all platform binaries, publishes the npm packages, and uploads the database asset from one tag.
- Change the install docs to show the `npx` config first and building from source second.

## Capabilities

### New Capabilities
- `database-provisioning`: How the server finds, downloads, verifies, caches, and version-checks its database, and how tools behave while the database is not ready.
- `npm-distribution`: How users install and start the server through npm and `npx` on the supported platforms.

### Modified Capabilities

None. The project has no specs yet.

## Impact

- `src/main.rs`: argument parsing (`fetch` subcommand) and database path resolution.
- `src/server.rs`: tool calls must handle a database that is not ready yet.
- New Rust dependencies for HTTPS download, zstd decompression, sha256, cache directory lookup, and file locking.
- `builder`: writes `meta.schema_version`.
- New `.github/workflows` release workflow and dist configuration. New npm packages on the registry.
- `README.md`, `docs/install.md`, `CHANGELOG.md`, `justfile`.
