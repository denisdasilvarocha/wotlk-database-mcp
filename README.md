![WOTLK Database MCP: Give AI agents access to WotLK 3.3.5 game data, WoW Lua API, and client offsets.](docs/assets/banner.png)

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#prerequisites">Prerequisites</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#usage">Usage</a> ·
  <a href="#development">Development</a> ·
  <a href="#documentation">Documentation</a> ·
  <a href="#license">License</a>
</p>

<p align="center">
  <a href="https://registry.npmjs.org/wotlk-database-mcp"><img src="https://img.shields.io/npm/v/wotlk-database-mcp" alt="npm version"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/built%20with-Rust-orange?logo=rust" alt="Built with Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/npm/l/wotlk-database-mcp" alt="License: MIT"></a>
</p>

WotLK Database MCP is an [MCP](https://modelcontextprotocol.io/) server for World of Warcraft: Wrath of the Lich King 3.3.5a (client build 12340). It gives MCP clients access to game records, their relationships, Lua and addon API data, and client offsets through a read-only SQLite database.

## Features

- Search and retrieve items, item sets, creatures, spells, quests, game objects, maps, areas, factions, talents, skills, glyphs, and achievements.
- Inspect record details such as item stats, spell effects, and quest objectives.
- Find loot and drop chances, vendors and prices, trainers, creature and game-object spawns, quest chains and rewards, spell ranks, and achievement criteria.
- Look up Lua functions and signatures, events and their parameters, widgets, CVars, Lua types, combat-log events, and secure templates.
- Inspect Windows x86 addresses, verification status, and comments for build 12340. Only offsets marked `verified` are confirmed; check the comments for caveats.
- Run read-only SQL queries, including queries against tables not exposed as record types.

The database combines client DBC data, TrinityCore world data, and vendored Lua API and offset catalogs. It is a snapshot, so data on a customized server may differ. Call `get` with `{"kind": "meta"}` to inspect the database's provenance.

## Prerequisites

- An MCP client that supports [stdio transport](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#stdio).
- Node.js 22 or later for the npm launcher. Native packages support macOS and Linux on x64 and ARM64, and Windows on x64. Linux requires glibc.
- Internet access for the first database download, unless you supply a compatible local database.

## Quick start

Add this server to your MCP client's configuration, using the format required by your client:

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

Restart or reconnect the client, then ask it to search the WotLK database for an item or spell. On the first run, the server downloads and verifies the database in the background and stores it in a versioned platform cache. If a tool reports download progress, retry after the download finishes.

To prepare the cache before connecting, run:

```bash
npx -y wotlk-database-mcp fetch
```

For a local database, pass `--db /path/to/wotlk.sqlite` after the package name, or set `WOTLK_DB` in the server's environment. See [installation and configuration](docs/install.md) for details and source builds.

## Usage

| Tool      | Purpose                                                    |
| --------- | ---------------------------------------------------------- |
| `search`  | Find records by name, optionally filtered by kind.         |
| `get`     | Retrieve a record and optionally select additional fields. |
| `related` | Follow a supported relationship from a record.             |
| `sql`     | Run one read-only SQLite statement.                        |

To find Frostbolt, call `search` with:

```json
{ "text": "Frostbolt", "kind": "spell" }
```

Then call `related` with a spell ID from the results to list its ranks:

```json
{ "kind": "spell", "id": 116, "relation": "ranks" }
```

Your MCP client can use these tools to answer questions such as:

- What are the stats for Thunderfury, Blessed Blade of the Windseeker?
- Where does Ragnaros spawn, and what loot can it drop?
- What are the rewards and follow-up quests for The Missing Diplomat?
- What is the spell ID for Frostbolt rank 3?
- What does `UnitHealth` return?
- What is the offset for `FrameScript::Execute`?

See the [tool reference](docs/tools.md) for parameters, examples, relationships, and data conventions.

## Development

The Rust workspace contains the server and the database builder. Install [just](https://just.systems/), then run `just setup` to install missing development tools and hooks. The setup script supports macOS, Linux with apt, dnf or pacman, and Windows through MSYS2 UCRT64. It may install system packages and Rust tools.

```bash
just build
just run
```

`just build` prepares the pinned inputs, builds `data/wotlk.sqlite`, and compiles the release server. `just run` starts the server with that database on stdio. Run `just test` for tests or `just check` for all project gates. See the [development guide](docs/development.md) for prerequisites and individual commands.

### Documentation

- [Installation and configuration](docs/install.md)
- [Tool reference](docs/tools.md)
- [Development guide](docs/development.md)

## License

MIT. See [LICENSE](LICENSE).
