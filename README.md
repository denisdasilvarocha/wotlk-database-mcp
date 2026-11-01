# wotlk-database-mcp

An offline [Model Context Protocol](https://modelcontextprotocol.io) server with reference data for World of Warcraft 3.3.5a (build 12340). AI agents use it to look up game data, native client offsets, and the Lua API without web searches. The server answers from one read-only SQLite database over stdio.

|Data|Source|
|---|---|
|Items, spells, creatures, quests, gameobjects, maps, areas, factions, talents, skills, glyphs, achievements|Client DBC files and TrinityCore TDB 335.26091|
|Native client offsets (Windows x86) with a verification status|Bundled offsets catalog|
|Lua functions, events, widgets, CVars, types, combat-log events, secure templates|Pinned [milkyway-codex](https://github.com/Shard-MW/milkyway-codex) export|

The client DBC files are the source of truth. The TDB supplies only server-side data: loot, spawns, vendors, trainers, and quests.

## Quick start

### Prerequisites

`just setup` installs the remaining tools, but it needs this base for your platform:

|Platform|Requirements|
|---|---|
|macOS|[Homebrew](https://brew.sh) and Xcode Command Line Tools (`xcode-select --install`).|
|Linux|`apt-get`, `dnf`, or `pacman`, and `sudo` or a root shell. WSL counts as Linux and builds a Linux binary.|
|Windows (x86_64)|[MSYS2](https://www.msys2.org) with its UCRT64 terminal, Windows PowerShell, and [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload and a Windows SDK. Git Bash, cmd, and PowerShell alone are not supported.|

Install [`just`](https://just.systems/man/en/packages.html):

|Platform|Command|
|---|---|
|macOS|`brew install just`|
|Debian, Ubuntu|`sudo apt install just`|
|Fedora|`sudo dnf install just`|
|Arch Linux|`sudo pacman -S just`|
|Windows (MSYS2 UCRT64)|`pacman -S just`|

All commands below run in Bash (the MSYS2 UCRT64 terminal on Windows) from the repository root.

### Build

```sh
git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
just setup
just build
```

- `just setup` installs Git LFS, `bsdtar`, `curl`, `jq`, `zip`, `unzip`, a C compiler, the pinned Rust toolchain, Bun, prek, taplo, cargo-deny, and cargo-shear when they are missing. It then configures Git LFS and the Git hooks.
- `just build` validates the bundled offsets, exports the pinned Lua API, builds `data/wotlk.sqlite`, and builds `target/release/wotlk-database-mcp` (`.exe` on Windows).

The first build downloads about 1.2 GB of client data and a 47 MB TDB dump into `target/build-db`. Later builds reuse the verified cache.

### Connect a client

Register the binary as a stdio server in your MCP client. Use absolute paths:

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

Step-by-step instructions for agents are in [docs/install.md](docs/install.md).

## Tools

|Tool|Use|
|---|---|
|`search(text, kind?, limit?)`|Find records by name across all kinds. Exact and prefix matches rank first, then player-learnable spells and spawned creatures. Spells show rank and class; creatures show title, level, and rank. Default 20 hits, maximum 100.|
|`get(kind, id, fields?)`|Return one record. Pass field names to add fields, or `["all"]` for every field. `get(kind="meta")` returns build provenance.|
|`related(kind, id, relation, limit?)`|List linked records, for example where an item drops (through shared loot tables), what a creature drops, sells, or trains, a whole quest chain, or the ranks of a spell. Default 50 rows, maximum 1000; a last line gives the total when rows are cut.|
|`sql(query, limit?)`|Run one read-only SQLite statement. Default 200 rows, maximum 1000, 2-second time limit.|

Output is compact `key=value` text (`sql` returns tab-separated rows). Zero, empty, and default fields are left out, except that spawn output preserves the valid `map=0` identifier. Flags and enums are names. An empty answer is a normal result; a bad value or a missing record is a tool error with a message that tells the model how to fix the call. Every tool is marked read-only. The server instructions list every kind, relation, and table, so clients need no discovery call. To read one table's schema with column comments, run `SELECT sql FROM sqlite_schema WHERE name = 'loot'`.

Vendor `price` is the required gold in copper; `cost` lists extended costs. An extended-cost exchange does not require gold unless the item has `DONT_IGNORE_BUY_PRICE`. The item detail's `buy_price` remains the generic template price.

Spawn output labels world coordinates as `world_x`, `world_y`, and `world_z`, not zone-map percentages. `orientation` is in radians and `spawn_time` is in seconds. `location_source=zone` identifies a zone name; `location_source=map` identifies a map-name fallback, which can be a continent. Missing zone or area fields mean unknown. Most TDB spawns lack this metadata, so an empty zone-filtered query does not prove there are no spawns.

Example: `related(kind="spell", id=133, relation="ranks")` returns:

```text
spell 133 Fireball rank=1
spell 143 Fireball rank=2
...
spell 42833 Fireball rank=16
```

## Configuration

The server finds the database in this order:

1. The `--db <path>` argument.
2. The `WOTLK_DB` environment variable.
3. `data/wotlk.sqlite` in the checkout that built the binary (compiled in).

The server stops at startup if the file does not exist. It has no network access at runtime. Logs go to stderr; set the level with `RUST_LOG`, for example `RUST_LOG=debug`.

## Database inputs

`just build-db` builds only the database. It downloads pinned inputs, verifies their SHA-256 hashes, and caches them in `target/build-db`:

|Input|Pin|Location|
|---|---|---|
|TrinityCore world dump|TDB 335.26091|`scripts/build_db.sh`|
|Client DBC files (enUS 12340)|[wowgaming/client-data](https://github.com/wowgaming/client-data/releases/tag/v20.0) v20.0 `Data.zip`|`scripts/build_db.sh`|
|Offsets catalog|Copy date in `builder/vendor/offsets/VERSION`|`builder/vendor/offsets/windows-x86`|
|Lua API export|Commit in `builder/vendor/api/COMMIT`|`builder/vendor/api/codex.json` (Git LFS)|

To build from other archives, set `TDB_URL`, `TDB_SHA256`, `TDB_MEMBER`, `DBC_URL`, `DBC_SHA256`, and `DBC_MEMBER` before you run `just build-db`. The builder rejects DBC files that are not enUS build 12340. `data/wotlk.sqlite` is tracked with Git LFS.

## Development

Run `just` to list all recipes. The main ones:

|Command|Use|
|---|---|
|`just run`|Run the server on stdio from source.|
|`just inspect`|Open the server in the MCP Inspector (needs Node.js).|
|`just fmt`|Format Rust and TOML files.|
|`just lint`|Run clippy; any warning fails.|
|`just test`|Run the workspace tests and the offline Bash provisioning tests. They use fixtures, not real game data.|
|`just doc`|Build the API docs; any rustdoc warning fails.|
|`just deps`|Check advisories, licenses, sources, and unused dependencies.|
|`just check`|Run every gate. CI runs this recipe.|
|`just vendor-api [commit]`|Export the Lua API at the pinned commit, or move the pin to `commit`.|
|`just vendor-offsets [dir]`|Validate the bundled offsets, or replace them from `dir`.|

The Git hooks run formatting and clippy on commit, and tests, docs, and dependency checks on push.

### Continuous integration

CI runs on pull requests, pushes to `main`, and manual dispatches, on Linux, macOS, and Windows. Each run executes `just check` and builds the workspace with `cargo build --workspace --release --locked`. Linux also runs `actionlint` on the workflow. CI does not build the database, so it downloads no game data.

To inspect a failed run, use `gh run view --log-failed` and select the run. To reproduce the gates locally, run `just check`.

Required checks, one approving review, and force-push prevention need a `main` branch protection rule. The GitHub API returns 403 for that rule on this private repository because GitHub requires Pro or public visibility. The repository does not enforce these protections yet.

The repository is a Cargo workspace:

|Path|Contents|
|---|---|
|`src/`|The MCP server: tools, database access, output formatting.|
|`builder/`|The `wotlk-db-builder` crate that builds the database from TDB, DBC, and vendored inputs.|
|`tests/`|End-to-end MCP tests and builder tests with fixture inputs.|
|`scripts/`|Setup, input download, and vendoring scripts called by `just`.|
|`docs/spec/`|Feature specs.|

The streaming TDB parser is in `builder/src/dump.rs`. Column lists share case-insensitive duplicate checks and separator handling. Numeric fields share conversion rules: `NULL` becomes zero, and invalid values report the field type, name, and source line. Text fields share one normalization rule in `Row::text`: SQL `NULL` and empty strings become absent values; other scalar text stays unchanged.

Read [AGENTS.md](AGENTS.md) before you change code. It has the coding rules, the definition of done, and how to add a tool.
