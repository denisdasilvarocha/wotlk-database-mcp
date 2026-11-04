# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

> [!WARNING]
> **BREAKING**: Source builds no longer use the checkout database by default. Pass `--db <path>` or set `WOTLK_DB`. The server requires schema version 2. Rebuild or download a matching database; a downgrade requires a database matching the older server. See [local database setup](docs/install.md#3-select-a-local-database).

### Added

- Query local WotLK data over stdio with the read-only MCP tools `search`, `get`, `related`, and `sql`. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Build the database from enUS client build 12340 DBC files, TrinityCore TDB 335.26091, bundled offsets, and a pinned Lua API export. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Read spell descriptions with signed dice ranges. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Search puts exact and prefix name matches first, then player-learnable spells and creatures that spawn.
- Loot relations follow shared loot tables and identify the source table with `via=N`.
- Quest chains report each quest's distance from the queried quest with `step=n`.
- Vendor relations separate required gold from extended costs. Spawn relations label world coordinates and separate zone names from map names.
- Start the server through npm and `npx` on macOS ARM64 and x86_64, Linux ARM64 and x86_64, and Windows x86_64.
- Use `fetch` to download and verify the database before you connect.
- Download the database into a schema-version cache when no explicit path is set. The handshake and tool list remain available during the download.
- Check `meta.schema_version` when the server opens a database.
- Install the server from npm without an install script. npm installs the matching native binary as an optional platform package.

### Changed

- **BREAKING**: Databases without `meta.schema_version` equal to `2` are refused. Rebuild with `just build-db` or download the matching database. Older servers require a matching older database.
- **BREAKING**: Database assets now use `wotlk-db-schema<N>.sqlite.zst` and `wotlk-db-schema<N>.sqlite.sha256`. Servers built before this change cannot download the database.
- Database releases now use the title `Database schema <N>`.
- Start database downloads in the background at startup. The handshake and tool list do not wait for the download.

### Removed

- **BREAKING**: Removed the compile-time fallback to the checkout's `data/wotlk.sqlite`. Pass `--db <path>` or set `WOTLK_DB`.

### Fixed

- The Lua API export handles MSYS file URLs on Windows. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))

[Unreleased]: https://github.com/denisdasilvarocha/wotlk-database-mcp/commits/main
