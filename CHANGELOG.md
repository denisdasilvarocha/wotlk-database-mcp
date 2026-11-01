# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). The project has no releases yet.

## [Unreleased]

### Added

- Query local WotLK data over stdio with the read-only MCP tools `search`, `get`, `related`, and `sql`. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Build the database from enUS client build 12340 DBC files, TrinityCore TDB 335.26091, bundled offsets, and a pinned Lua API export. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Read spell descriptions with signed dice ranges. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))
- Search puts exact and prefix name matches first, then player-learnable spells and creatures that spawn.
- Loot relations follow shared loot tables and identify the source table with `via=N`.
- Quest chains report each quest's distance from the queried quest with `step=n`.
- Vendor relations separate required gold from extended costs. Spawn relations label world coordinates and separate zone names from map names.

### Fixed

- The Lua API export handles MSYS file URLs on Windows. ([#1](https://github.com/denisdasilvarocha/wotlk-database-mcp/pull/1))

[Unreleased]: https://github.com/denisdasilvarocha/wotlk-database-mcp/commits/main
