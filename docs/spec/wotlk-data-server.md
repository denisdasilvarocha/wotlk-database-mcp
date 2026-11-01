# Spec: WotLK 3.3.5a (12340) data server

Status: ready-for-agent

## Problem Statement

AI agents that write client-side tooling for World of Warcraft 3.3.5a (build 12340) need three kinds of
reference data: native client offsets, the Lua API, and game data (items, creatures, spells, item sets,
quests, and related records). Today this data is in many places: a skill catalog in Markdown, a
third-party Lua API dataset, a TrinityCore world database dump, client DBC files, and web databases such
as Rising Gods. An agent must search the web, parse HTML, or guess. The answers are slow, use many
tokens, change between calls, and are often for the wrong client build. Unverified offsets look the same
as verified ones, so agents can treat a wrong address as confirmed.

## Solution

A private, offline MCP server that answers from one read-only SQLite database. A separate offline builder
makes that database from a TrinityCore TDB 335 dump, an enUS 12340 DBC folder, a vendored copy of the
offsets catalog, and a vendored, pinned copy of the milkyway-codex Lua API data. A bootstrap command
downloads and caches pinned TDB and community DBC inputs when needed. The database is tracked with Git LFS.

Agents use four generic tools: `search`, `get`, `related`, and `sql`. Every tool returns compact
`key=value` text. Fields that are zero, empty, or default are left out. Flags and enums are decoded to
names. Spell descriptions come with simple tokens resolved. Each offset carries an explicit verification
status. The data is "Blizzlike": the client DBC is the truth, and the TDB supplies only server-side data.

## User Stories

1. As an agent that writes a client DLL, I want to find an offset by name, so that I can call or hook a native function without reverse engineering it again.
2. As an agent that writes a client DLL, I want each offset to show its VA and RVA, so that I can compute the runtime address from the module base.
3. As an agent that writes a client DLL, I want each offset to carry a verification status, so that I never treat an unverified or mislabeled address as confirmed.
4. As an agent that writes a client DLL, I want the raw catalog comment with each offset, so that I can read the full caveat in its original words.
5. As an agent that writes a client DLL, I want each offset to name its platform and domain, so that I do not apply a Windows x86 address to another binary.
6. As an agent that writes a client DLL, I want to list all offsets of one domain (for example Lua or Spell), so that I can see the related functions together.
7. As an agent that writes an addon, I want to look up a Lua function by name, so that I get its signature, parameters, returns, and description for 3.3.5a.
8. As an agent that writes an addon, I want to search Lua functions by partial name or keyword, so that I can find an API when I do not know its exact name.
9. As an agent that writes an addon, I want to look up an event and its payload, so that I can register for it and read its arguments correctly.
10. As an agent that writes an addon, I want to look up a widget type with its methods, inheritance, and script handlers, so that I can build frames correctly.
11. As an agent that writes an addon, I want to look up a CVar with its default value, so that I can read or set client settings.
12. As an agent that writes an addon, I want to look up combat-log events and secure templates, so that I can parse combat logs and build secure frames.
13. As an agent, I want to find a spell by name and get every rank with its ID, so that I can call `CastSpellByID` with the correct rank.
14. As an agent, I want a spell's school, cost, cast time, range, cooldown, duration, and effects in one compact record, so that I can reason about it in a bot.
15. As an agent, I want spell attributes decoded to flag names, so that I do not have to look up bit masks.
16. As an agent, I want a spell description with its `$s1`, `$d`, `$o1` style tokens resolved to values, so that I can read what the spell does.
17. As an agent, I want the raw spell description too, so that I can see the tokens the resolver left unchanged (formulas and conditionals).
18. As an agent, I want to know which trainers teach a spell and which items teach or use it, so that I can plan how a character gets it.
19. As an agent, I want to see the spells that a spell triggers, so that I can follow proc and trigger chains.
20. As an agent, I want to know if a spell is a talent or a glyph, so that I can link it to the talent tree or glyph system.
21. As an agent, I want to find an item by name and get its ID, quality, item level, class, subclass, and slot, so that I can identify it in bags or on the character.
22. As an agent, I want an item's stats, sockets, spells, and requirements in a compact record, so that I can compare items in code.
23. As an agent, I want item flags decoded to names, so that I can see properties such as unique-equipped or bind-on-pickup.
24. As an agent, I want to know which creatures, gameobjects, items, and reference loot tables drop an item, with their chances, so that I can plan where to farm it.
25. As an agent, I want to know which vendors sell an item and at what cost, so that I can automate purchases.
26. As an agent, I want to know which quests reward an item, so that I can plan how to get it.
27. As an agent, I want an item's item set with the set bonuses, so that I can evaluate set gear.
28. As an agent, I want to know which spells create an item and what it disenchants into, so that I can automate professions.
29. As an agent, I want to find a creature by name and get its entry, level range, rank, faction, and NPC flags, so that I can identify it in the object manager.
30. As an agent, I want a creature's spawn points with map and coordinates, so that I can navigate to it.
31. As an agent, I want a creature's loot table, so that I can decide if it is worth killing.
32. As an agent, I want a creature's vendor list and trainer list, so that I can automate buying and training.
33. As an agent, I want to know which quests a creature starts and ends, so that I can automate questing.
34. As an agent, I want the spells a creature casts, so that I can react to them in combat logic.
35. As an agent, I want to find a quest by name and get its level, required level, zone, and objectives, so that I can plan a quest route.
36. As an agent, I want a quest's givers, enders, and chain (previous, next, exclusive group), so that I can do quests in the correct order.
37. As an agent, I want a quest's rewards and requirements (items, kills, objects), so that I can track its progress.
38. As an agent, I want to find a gameobject by name and get its spawn points and loot, so that I can automate gathering.
39. As an agent, I want maps, areas, and zones with their parent and child areas, so that I can convert area IDs into names and regions.
40. As an agent, I want factions with their member creatures, so that I can reason about reputation and hostility.
41. As an agent, I want talents, talent tabs, skill lines, and skill-line abilities, so that I can automate talent and profession logic.
42. As an agent, I want achievements with their criteria, so that I can track progress.
43. As an agent that writes an addon, I want spell icon and item display data, so that I can show correct textures in UI code.
44. As an agent, I want one search over all kinds of data, so that I can find a name when I do not know if it is an item, a spell, or a creature.
45. As an agent, I want to limit a search to one kind, so that I get only relevant results.
46. As an agent, I want exact and prefix name matches ranked first, so that the correct record is at the top.
47. As an agent, I want search results as one short line each (kind, ID, name, subtext), so that a search uses few tokens.
48. As an agent, I want `get` to return a short summary by default, so that I do not spend tokens on fields I do not need.
49. As an agent, I want to request more fields, or all fields, with a `fields` parameter, so that I can get detail only when I need it.
50. As an agent, I want a read-only `sql` tool, so that I can run joins and filters that the other tools do not support.
51. As an agent, I want the database schema available to me, so that I can write correct SQL without guessing table and column names.
52. As an agent, I want `sql` results to say clearly when they are truncated, so that I do not take a partial answer as complete.
53. As an agent, I want a clear error when I call an unknown kind or relation, so that I can correct my call.
54. As an agent, I want a clear tool error when an ID does not exist, so that I can tell the difference between "no data" and "bad request".
55. As an agent, I want the server instructions to list every kind and relation, so that I can use the tools without a discovery call.
56. As an agent, I want to see which inputs built the database (TDB release, dump hash, client build, codex commit, offsets copy date, builder version), so that I know where the data came from.
57. As an agent, I want the same answer every time for the same question, so that I can trust and cache results.
58. As an agent, I want answers in milliseconds without network access, so that queries do not slow down my work.
59. As the maintainer, I want to rebuild the database with one command that provisions its inputs, so that updating the data is easy.
60. As the maintainer, I want the builder to check that the DBC folder is for build 12340 and enUS, so that I do not build from the wrong client by mistake.
61. As the maintainer, I want the builder to read the TDB dump directly without a MySQL server, so that the build has no service dependencies.
62. As the maintainer, I want the build to fail with a clear message when an input is missing or malformed, so that I never commit a partial database.
63. As the maintainer, I want the server to find the database through a flag, an environment variable, or a compiled-in default, so that it works from any working directory.
64. As the maintainer, I want the server to fail fast when the database file is missing, so that the MCP client shows the real problem.
65. As the maintainer, I want the large database stored with Git LFS, so that the repository stays within GitHub limits.
66. As the maintainer, I want CI to run without downloading the LFS database, so that CI stays fast and uses no LFS bandwidth.
67. As the maintainer, I want the offsets and the Lua API data vendored and pinned, so that a build is reproducible on any machine.
68. As the maintainer, I want flag and enum names in reviewable source tables, so that I can fix a wrong name in one place.

## Implementation Decisions

### Workspace and modules

- The repository becomes a Cargo workspace with two crates: the existing server crate and a new builder crate.
- The server crate keeps its MCP role: the tool router, the server handler, the database access layer, and the compact output formatter. It must not depend on dump parsing or DBC parsing code.
- The builder crate has a library and a thin binary. The library has one public entry point: build a database file from a set of input paths. Both the binary and the tests call this entry point. This is the only new interface that tests use.
- The builder library has these internal parts:
  - a streaming mysqldump reader that uses `CREATE TABLE` for column order and `INSERT ... VALUES` for rows, and reads only the tables in scope. It handles MySQL string escapes.
  - a DBC importer that uses the `wow_dbc` crate with the `wrath` feature and reads from the extracted DBC folder.
  - hand-ported flag and enum name tables (ItemFlags, SpellAttr0 to SpellAttr7, NPCFlags, spell effects, aura types, and similar), taken from TrinityCore `SharedDefines.h` and related headers at the commit that matches the pinned TDB release.
  - a spell text resolver.
  - an offsets catalog parser.
  - a codex JSON importer.
  - a search index writer and a meta writer.
- The zero-argument `just build` validates the bundled offsets, vendors the API at the commit in `builder/vendor/api/COMMIT`, builds the database, and builds the release server with Cargo's lockfile.
- The zero-argument `just build-db` provisions pinned inputs and invokes the offline builder with `--tdb <dump file>` and `--dbc <folder>`. The vendored offsets and codex data have fixed locations in the repository.
- Database bootstrap requires Bash, curl, `jq`, Cargo, `bsdtar`, `unzip`, and a SHA-256 utility; it fetches the codex through Git LFS if it is a pointer. It does not install packages. The cache under `target/build-db` verifies archives and extracted files and repairs corrupted entries.
- `just setup` installs missing tools on macOS with Homebrew, Linux with apt, dnf, or pacman, and native x86_64 Windows from MSYS2 Bash with pacman. macOS requires Xcode Command Line Tools. Native Windows requires Microsoft C++ Build Tools with MSVC and a Windows SDK; cmd, PowerShell, and Git Bash alone are not the native Windows environment. WSL follows the Linux instructions and produces a Linux binary. Setup installs the pinned Rust toolchain and configures Git LFS and prek hooks.

### Inputs and source precedence

- The world data comes from TrinityCore TDB 335, pinned to release 335.26091. The bootstrap downloads its release archive; the dump stays outside git.
- The client data comes from the pinned community [wowgaming/client-data v20.0 `Data.zip`](https://github.com/wowgaming/client-data/releases/tag/v20.0). The bootstrap extracts its DBC files, not its maps; the folder stays outside git.
- SHA-256 checks pin the downloaded artifacts and detect corruption. They do not independently establish Blizzard provenance. Source URLs and hashes are recorded in `scripts/build_db.sh`.
- Only enUS text is stored. `wow_dbc` names the first English string field `en_gb`. The builder reads that field and checks that it holds enUS text.
- The builder checks DBC layouts and verifies that Spell record 133 has the enUS name `Fireball`. It does not check record counts.
- Precedence ("client truth, TDB for server-only data"):
  - Spells come only from `Spell.dbc` and its helper DBCs (durations, radii, ranges, cast times, icons). The server-only rows in the TDB `spell_dbc` table are ignored.
  - Items take their stats from `item_template`. `Item.dbc` fills only class and display ID, and only for items that `item_template` does not have.
  - Creatures, quests, loot, spawns, vendors, and trainers come only from the TDB. Trainers use the current TrinityCore trainer tables (`trainer`, `trainer_spell`, `creature_default_trainer`). There is no `npc_trainer` table.
  - Negative `npc_vendor.item` values refer to another vendor's stock. Expand these references, including nested references, before storing vendor items.
  - Item spell IDs at or below zero mean no spell. Do not store these slots.
  - Maps, areas, factions, talents, talent tabs, skill lines, glyphs, achievements, item sets, item display info, and spell icons come only from the DBCs.
- The offsets come from the checked-in wow-335-offsets catalog: 787 Windows x86 entries in 12 domain files. The default deliberately uses this bundled catalog and does not require a local agent skill. `just vendor-offsets` validates it; `just vendor-offsets <src>` refreshes it from a reference folder.
- The Lua API comes from the upstream `Shard-MW/milkyway-codex`, pinned to a commit and vendored. `just vendor-api` uses `builder/vendor/api/COMMIT`; an optional commit changes the pin. The checkout is cached under `target/api`. All categories are in scope: functions, events, widgets, CVars, types, combat-log events, and secure templates.

### Data scope

All five domain groups are in scope for the first build:

- Core: items, creatures, spells, item sets, quests.
- Relations: creature, gameobject, item, and reference loot; vendors; trainers; quest givers and enders; quest chains.
- World: creature and gameobject spawns with coordinates, gameobject templates, maps, areas and zones, factions.
- Character systems: talents and talent tabs, skill lines and abilities, spell ranks, glyphs, achievements and criteria.
- Client UI data: spell icons, item display info.

Measured TDB 335.26091 volumes, for sizing: about 317k creature loot rows, 152k creature spawns, 95k gameobject spawns, 38.6k items, 29.9k creatures, 21.6k gameobject templates, 9.5k quests, and 37.5k vendor rows. `Spell.dbc` has about 49.8k records. The curated database is estimated at 100 to 300 MB.

### Schema

- Curated tables, one per kind. Each table holds only the useful columns, with stable column names. Flags and enums are stored as decoded names, or the formatter decodes them; either way the agent sees names, not bit masks. There is no raw mirror of the source tables.
- Offsets are stored with qualified name, domain, platform, VA, RVA (VA minus image base 0x00400000), status, and raw comment. Status is a closed enum: `verified`, `unverified`, `uncertain`, `label_unverified`, `not_present`. Explicit rules map the catalog's inconsistent caveat markers to these values. An entry with no caveat is `verified`. The raw comment is always kept.
- Spells store both the raw description and the resolved description. The resolver handles `$s`, `$m`, `$d`, `$o`, `$t`, `$a`, `$x`, `$r`, and references to other spells (for example `$12345s1`). Formulas (`${...}`) and `$l`/`$g` conditionals are left unchanged.
- One unified FTS5 search table holds every searchable entity as (kind, id, name, subtext). It uses the `unicode61 remove_diacritics` tokenizer and a prefix index.
- A `meta` table records the TDB release, the dump's sha256, the client build, the codex commit, the offsets copy date, and the builder version.

### Server runtime

- `rusqlite` with the `bundled` feature (version 0.41; FTS5 is included). The database is opened read-only with `query_only` on. Queries run on `spawn_blocking`.
- The server takes the database path from `--db`, then the `WOTLK_DB` environment variable, then a default of `data/wotlk.sqlite` under the crate manifest directory, compiled in. If the file is missing, the server stops at startup with a clear error.
- No network access at runtime.

### Tool contract

- `search(text, kind?, limit?)`: queries the unified index. Exact and prefix name matches rank first, then FTS5 `bm25`. Default limit 20, maximum 100. One line per hit: `kind id name [subtext]`.
- `get(kind, id, fields?)`: returns one record. Each kind has a curated summary field set. `fields` adds named fields, or `all` returns every curated field. Fields that are zero, empty, or default are always left out.
- `related(kind, id, relation)`: returns linked records as compact lines. Supported relations:
  - item: `dropped_by` (with chance), `sold_by` (with cost), `reward_of`, `set` (with bonuses), `created_by`, `disenchant`.
  - creature: `loot`, `sells`, `trains`, `starts_quests`, `ends_quests`, `spawns`, `spells`.
  - quest: `givers`, `enders`, `chain`, `rewards`, `requirements`.
  - spell: `ranks`, `taught_by`, `used_by`, `talent`, `glyph`, `triggered`.
  - gameobject: `loot`, `spawns`. area: `children`, `parent`. faction: `members`. achievement: `criteria`.
- `sql(query, limit?)`: one read-only statement only. Default 200 rows, maximum 1000. A progress handler stops the query after 2 seconds. When rows are cut, the output says so.
- Kinds: item, creature, spell, itemset, quest, gameobject, map, area, faction, talent, skill, glyph, achievement, offset, lua_function, lua_event, widget, cvar, lua_type, combatlog_event, secure_template, and `meta`.
- The builder includes a comment for every column in each `CREATE TABLE` statement. Agents read the schema with `SELECT sql FROM sqlite_schema WHERE type = 'table'`.
- The `sql` tool returns a tab-separated header and rows, with `NULL` for null values. Other tools return compact `key=value` text, never JSON structured output.
- Errors follow the project rule. An unknown kind, an unknown relation, or a malformed SQL statement is `invalid_params`. A missing ID, an empty result, or an SQL runtime error is a tool error result that the model can read.
- The server `instructions` text lists every kind and relation, so agents need no discovery call.

### Storage in git

- The built database is tracked with Git LFS. On the free plan this gives 10 GiB of storage and 10 GiB of bandwidth per month. Each rebuild stores a full new copy.
- CI checks out without LFS. CI does not need the real database, because the tests use fixtures.

## Testing Decisions

- Fixture sources go through the builder library entry point into a temporary SQLite file. A real `rmcp` client, connected over an in-memory duplex transport, calls the tools and checks their text output.
- Offline Bash tests cover input provisioning separately and use `zip` to create archive fixtures. CI uses fixtures and does not download the large source artifacts.
- The fixtures are small and committed: a few TDB `CREATE TABLE` and `INSERT` statements, including hard MySQL escapes (quotes, backslashes, `NULL`, empty strings, multi-row inserts); a few small DBC files; an offsets Markdown snippet with each caveat marker; and a codex JSON snippet.
- These parts are covered only through the seam:
  - the dump tokenizer: names with escaped quotes come back correctly through `search` and `get`.
  - flag decoding: `get` shows flag names, not numbers.
  - the spell resolver: `get` on a spell shows resolved values, and formulas stay unchanged.
  - offset status: each caveat marker shows the correct status enum.
  - precedence: a server-only `spell_dbc` row is not found; `Item.dbc` fills only missing items.
  - ranking: an exact match ranks above prefix and partial matches.
  - every relation listed in the tool contract.
  - `sql`: read-only refusal, row cap, and truncation notice.
  - errors: `invalid_params` and tool-error cases.
  - `meta`: build provenance.
  - default omission: zero and default fields are not in the output.
- The builder's input checks have tests too. A wrong build or a missing input makes the entry point return an error, and no database file is left behind.
- Prior art: the existing end-to-end test drives the `echo` tool through a real `rmcp` client over a Tokio in-memory duplex. The new tests follow the same pattern, with a fixture database in place of no data.

## Out of Scope

- Offsets for any platform other than Windows x86 12340, for example macOS. The schema has a platform column so this can be added later.
- Locales other than enUS.
- Data for a specific server, for example Rising Gods custom fixes. Scraping db.rising-gods.de is out of scope.
- The TDB `spell_dbc` server-only spells and other server overrides of client data.
- Network access at server runtime and an online fallback.
- Reading MPQ archives directly. The offline builder needs an extracted DBC folder.
- A full spell formula evaluator (`${...}` expressions and `$l`/`$g` conditionals).
- JSON structured output.
- Public release of the data. The repository is private, and the committed database contains Blizzard-derived and GPL data.
- Runtime write access to the database.

## Further Notes

- The `check-added-large-files` hook runs on every commit. Configure Git LFS tracking before the first database commit, so that the hook and GitHub's 100 MiB limit do not reject the file.
- `deny.toml` allows only `Apache-2.0`, `MIT`, and `Unicode-3.0`. The planned dependencies (`rusqlite` and `libsqlite3-sys`: MIT; `wow_dbc`: MIT/Apache-2.0) fit. Do not widen the allow list.
- The milkyway-codex upstream says it is MIT, but some of its text comes from the *WoW Programming* book and archived wowprogramming.com pages. This is acceptable only because the repository is private.
- The Lua API data is described as 3.3.5a, but its build provenance and per-function verification are as good as the upstream only. The offset statuses come from the catalog's own caveats, not from new verification.
- `wow_dbc` 0.3.0 was last published in 2024, but the repository is active (2026 commits) and has no open issues.
- `README.md` and `AGENTS.md` must describe the workspace, the builder, the `build-db` recipe, the LFS setup, and the new tools when the work lands.
