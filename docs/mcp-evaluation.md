# wotlk-database MCP server: evaluation

Date: 2026-11-01. Build: `wotlk-database-mcp 0.1.0`, TDB 335.26091, client 12340, macOS arm64.

## Status after fixes

All findings I-1 to I-11 are fixed. The sections below this one describe the state before the fixes. Results from the rebuilt database and release binary:

| Finding | Result after the fix |
|---|---|
| I-1 search ranking | `search fireball kind=spell` returns `spell 133 Fireball (Rank 1 MAGE)` first, then ranks 2–5. |
| I-2 reference loot | `related item 17076 dropped_by` → `creature 11502 Ragnaros count=1 group=2 via=34030`. Ragnaros `loot` lists `reference 34030`, then its items with `via=34030`. |
| I-3 error channels | Bad relation, unknown field, bad SQL, and writing SQL return `isError: true`. Only malformed arguments (for example `kind=meta` in `search`) return `-32602`. |
| I-4 empty results | `no dropped_by for item 19019`, `no matches for "thunderfurry"`, and the SQL header with `-- no rows` are normal results. `item 999999999 not found` stays an error. |
| I-5 related limit | Default 50, maximum 1000. `related item 4306 dropped_by limit=3` ends with `-- 3 of 1076 shown; raise limit (max 1000)`. |
| I-6 annotations | All four tools have a `title` and `readOnlyHint=true`, `destructiveHint=false`, `idempotentHint=true`, `openWorldHint=false`. |
| I-7 descriptions | Each description starts with "World of Warcraft 3.3.5a (WotLK, build 12340)". The instructions name the uses (addon and Lua API, TrinityCore/AzerothCore data, offsets). The `relation` parameter lists the relations per kind. |
| I-8 context | `search onyxia kind=creature` → `Onyxia (level 83 WORLDBOSS)`. Trainer, vendor, giver, and ender rows carry `faction=` and `zone=`. TDB 335 has a zone id for only 2,330 of 151,815 spawns, so `zone=` usually gives the map name (for example `Eastern Kingdoms`). |
| I-9 chain | `related quest 7787 chain` returns 7785 (`step=-2`), 7786 (`step=-1`), and 7787 (`step=0`). |
| I-10 schema | The `sql` description and instructions recommend `SELECT sql FROM sqlite_schema WHERE name = '<table>'`. The instructions list the 45 tables, without the FTS shadow tables. |
| I-11 smaller items | `search` has no `meta` kind; `related` accepts only the 8 kinds with relations. `PRAGMA cache_size=1` and `BEGIN` are rejected; `PRAGMA table_info(meta)` works. Spell icons are bare names (`Spell_Fire_FlameBolt`). A 1-letter search takes 2.0 ms instead of 27.6 ms. |

Latency after the fixes, 20 iterations each:

| Call | p50 ms | p95 ms | Bytes |
|---|---|---|---|
| `search fireball` | 0.36 | 0.48 | 637 |
| `search a limit=100` | 1.96 | 2.14 | 3,946 |
| `search fire limit=100` | 3.23 | 3.67 | 3,660 |
| `related item 4306 dropped_by` (50 of 1,076) | 3.25 | 3.48 | 2,765 |
| `related item 17076 dropped_by` | 0.20 | 0.24 | 49 |
| `related creature 11502 loot` (50 of 182) | 0.32 | 0.38 | 2,784 |
| `related quest 7787 chain` | 0.14 | 0.19 | 157 |
| `related spell 143 taught_by` (41 rows) | 1.29 | 1.41 | 3,469 |
| `get item 19019` | 0.06 | 0.08 | 314 |

Tradeoff: the fixed context cost grew from 5,929 B to 8,675 B (≈1.5k to ≈2.2k tokens): `instructions` 2,751 B and `tools/list` 5,924 B. The table list, the relation list per kind, and the domain words cause the growth. They remove discovery calls and make the server work in clients that drop the instructions.

## Verdict

| Question | Rating | Short answer |
|---|---|---|
| Performant | Very good | 2 ms cold start, 9.8 MiB peak RSS, 2,700 calls/s over 4 processes. |
| Fast | Very good | Typical call 0.05–0.3 ms. Worst real call: 1-letter search, 28 ms. Unindexed SQL stops at the 2 s limit. |
| Reliable | Good | No crash, hang, or lost response. Read-only protection held against every write attempt. The error channels do not follow the current MCP spec (see I-3). |
| Token efficient | Good | Fixed cost about 1.5k tokens per session. `get` and `search` are 20–200 tokens. Large outputs: `related` up to 2.8k tokens and `sql` up to 10k tokens, with no `limit` on `related`. |
| Easy for an agent | Good, with gaps | Error messages list the valid values, so the agent corrects itself in one step. Search ranking and reference loot need extra SQL calls for common questions (I-1, I-2). |
| Agent picks it for WotLK tasks | Yes, if the client loads the server instructions | The instructions name WoW 3.3.5a. The four tool descriptions do not name the game (I-7). |

## Method

1. I used the four tools through the omp harness, as an agent does, on realistic tasks (loot, quests, spells, trainers, Lua API, offsets).
2. A Python stdlib client drove the release binary over stdio JSON-RPC: cold start, `tools/list` size, 20 iterations per call, an error matrix, pipelined and multi-process load.
3. Token counts are bytes/4 (`tiktoken` was not installed), ±20%.

## Performance

| Measure | Result |
|---|---|
| Spawn → `initialize` response | 1.7 ms min, 1.9 ms median |
| Spawn → `tools/list` response | 1.9 ms min, 2.1 ms median |
| Peak RSS | 9.8 MiB (17.6 MiB after 20 × 40 KB results) |
| Binary / database | 6.6 MiB / 122.9 MiB |
| 50 pipelined requests, 1 process | 32.8 ms, 1,523 req/s, all ids matched |
| 4 processes × 100 calls | 0.15 s wall, 2,705 calls/s |

Latency, 20 iterations each, warm session:

| Call | p50 ms | p95 ms | Bytes | ≈ Tokens |
|---|---|---|---|---|
| `get item 19019` | 0.07 | 0.09 | 314 | 78 |
| `get spell 133 fields=[all]` | 0.07 | 0.08 | 816 | 204 |
| `search thunderfury` | 0.11 | 0.16 | 398 | 100 |
| `related creature 11502 loot` | 0.06 | 0.08 | 347 | 87 |
| `related spell 133 ranks` | 0.07 | 0.08 | 444 | 111 |
| `related faction 14 members` | 0.16 | 0.18 | 11,168 | 2,792 |
| `related item 4306 dropped_by` (cut at 200 of 1,019) | 2.2 | 2.3 | 11,374 | 2,844 |
| `search fire limit=100` | 2.7 | 2.9 | 3,646 | 912 |
| `search a limit=100` | 27.6 | 27.8 | 3,173 | 793 |
| `sql` schema dump | 0.14 | 0.16 | 23,557 | 5,889 |
| `sql … limit=1000` on `loot` | 0.51 | 0.57 | 40,508 | 10,127 |
| `sql` cross join (timeout) | 2000 | 2000 | 40 | 10 |

The SQLite indexes cover every lookup path that the tools use. Speed is not a limiting factor.

## Token cost

| Payload | Bytes | ≈ Tokens |
|---|---|---|
| `instructions` | 1,770 | 442 |
| `tools/list` | 4,159 | 1,039 |
| — `related` | 1,330 | 333 |
| — `get` | 1,191 | 298 |
| — `search` | 970 | 243 |
| — `sql` | 618 | 155 |
| Fixed cost per session | 5,929 | 1,482 |

- The input schemas are 73% of `tools/list`. The 22-value `kind` enum occurs three times, and the 28-value `relation` enum occurs once.
- The output format is good for tokens: one line per record, `key=value`, default fields left out, enums as names, money in copper.
- An unknown `fields` value returns the full field list (up to 701 B). This costs tokens once and removes a schema lookup, so it is a good trade.

## Reliability

- No panic, hang, or lost response in any test.
- Writes are blocked twice: the statement check (`stmt.readonly()`) and `PRAGMA query_only=1`. The test blocked `DELETE`, `DROP`, `ALTER`, `CREATE INDEX`, `WITH … DELETE`, `EXPLAIN DELETE`, `ATTACH`, and `load_extension`. The database file did not change.
- Responses to pipelined requests can arrive out of order. JSON-RPC permits this; the ids always matched.
- Every `-32602` response also writes one `WARN` line to stderr.

## Agent walkthrough

| Task | Calls | Result |
|---|---|---|
| Who drops Bindings of the Windseeker? | `search` → `related dropped_by` | 2 calls, correct: Baron Geddon, 3%. |
| Signature of Lua `UnitHealth` | `get lua_function` | 1 call. |
| Address of `FrameScript::Execute` (query "lua execute") | `search` | 1 call, multi-word match works. |
| Reward source of Thunderfury | `related item reward_of` | 1 call: quest 7787. |
| Mage Fireball rank 1 | `search fireball` | **Fails.** Spell 133 is not in the first 100 of 145 hits (I-1). |
| Who drops Bonereaver's Edge? | `related dropped_by` → `sql` | Returns only `reference 34030`. The agent needs SQL to find Ragnaros (I-2). |
| What does Ragnaros drop? | `related loot` → `sql` | 2 of 7 lines are `reference N` without content (I-2). |
| Full Thunderfury quest chain | `related chain` per quest | One step per call (I-9). |
| Where can I learn Fireball rank 2? | `related taught_by` | 41 trainers, no zone or faction (I-8). |
| Wrong input (`relation=loot` on an item, field `dps`, `id="Thunderfury"`) | 1 retry | The message lists the valid values. The agent corrects itself at once. |

## Descriptions and discoverability

What works:

- The `instructions` give the source of truth (DBC vs. TDB), units, output conventions, all kinds, all relations per kind, and the reference-loot hint. An agent needs no discovery call.
- `search` says "use it first when you only know a name". This gives the correct workflow.
- The `sql` description gives the schema query, and every column has a comment. Units ("Respawn time in seconds") are clear.
- The parameter docs are written for a model, for example the `id` doc lists which kinds take a name.

Gaps:

- No tool description names World of Warcraft or WotLK. Some clients load tools on demand and match the user request against tool descriptions, and some clients drop server `instructions`. In these clients, a request such as "WotLK addon: which event fires on health change?" may not find this server.
- `related` refers to "the server instructions" for the relations of each kind. Without the instructions, the agent can only learn them from errors.
- `kind` enums include values that cannot be used: `meta` in `search` and `related`, and kinds with no relations (`offset`, `lua_function`, …) in `related`.
- No tool sets `annotations`. Clients cannot see that all tools are read-only, so they may ask the user to approve each call.

## Improvements

Priority: P1 = wrong or missing answers for common questions; P2 = extra calls, tokens, or spec conformance; P3 = polish.

### I-1 (P1) Search ranking hides player spells

- Evidence: `search fireball` returns NPC copies first. 145 spells have the name "Fireball"; 16 are player ranks; none is in the first 100 hits. 86% of spell rows and 80% of creature rows in the `search` table have no subtext.
- Cause: `bm25` (`src/db.rs` line 110) ranks rows with an empty `terms` column above player spells, because player spells have `Rank N` in `terms`. The tie-break `ORDER BY … id` compares the FTS `UNINDEXED` id as text, so `10578` comes before `9053`.
- Fix:
  1. Store a numeric priority in the search table at build time: player-learnable spells (in `spell_rank`, `skill_ability`, or `trainer_spell`) and spawned creatures first. Order by it before `bm25`.
  2. Fill the subtext: spells `Rank 1, MAGE`; creatures `level 63 WORLDBOSS`.
  3. Order the tie-break by `CAST(id AS INTEGER)`.

### I-2 (P1) Reference loot is not resolved

- Evidence: `related item 17076 dropped_by` returns `reference 34030 count=1 group=2` and does not name Ragnaros. `related creature 11502 loot` shows `reference 34030` and `reference 34002` without their items. Most raid loot in TDB is in reference tables.
- The instructions give `SELECT * FROM loot WHERE reference = N`. This query lists the users of a reference, not its items. An agent that wants the items must find `WHERE source='reference' AND entry=N` itself.
- Inside a group, `chance=0` means an equal share of the group. No text explains this, so an agent can read it as "never drops".
- Fix: in `dropped_by`, follow references to the creatures and gameobjects that use them (`creature 11502 Ragnaros via=reference 34030`). In `loot`, expand reference items (with a depth limit). Document `chance=0` in a group in the `loot.chance` column comment and the instructions.

### I-3 (P2) Validation errors use the protocol error channel

- Evidence: a wrong relation, an unknown field, a text id for a numeric kind, invalid SQL, a write statement, and an out-of-range `limit` return JSON-RPC `-32602`. The 2 s timeout, `ATTACH`, and missing records return `isError: true`.
- [MCP spec 2025-11-25, Tools → Error Handling](https://modelcontextprotocol.io/specification/2025-11-25/server/tools) (from [SEP-1303](https://modelcontextprotocol.io/seps/1303-input-validation-errors-as-tool-execution-errors), status Final) puts input validation errors in tool results with `isError: true`. Clients SHOULD give these to the model. Clients only MAY give protocol errors to the model. The current messages are very good for self-correction, but some clients do not show them to the model.
- Fix: return `QueryError::Invalid` as `CallToolResult::error`. Keep `-32602` for errors the SDK raises before the handler runs (unknown tool, missing or wrongly typed fields). This removes the stderr `WARN` line per rejected input.
- Decision needed: the `AGENTS.md` invariant "invalid requests or malformed SQL return `invalid_params`" requires the current behavior. Change the invariant and `docs/spec/wotlk-data-server.md` together with the code.

### I-4 (P2) An empty result is an error

- Evidence: `related item 19019 dropped_by` → `isError: no dropped_by for item 19019`. A valid `SELECT` with zero rows → `isError: query returned no rows`.
- An agent reads `isError` as a failed call and can retry or change its plan. "No item drops it" is a correct answer, not a failure.
- Fix: return a normal result such as `none: item 19019 has no dropped_by`. Keep `isError` for a record id that does not exist.

### I-5 (P2) `related` has no `limit` and no total

- Evidence: `related item 4306 dropped_by` returns 200 of 1,019 rows (≈2.8k tokens). The last line does not give the total.
- Fix: add `limit` (default 50, maximum 1000), as `search` and `sql` have. End cut output with `-- 50 of 1019 shown`.

### I-6 (P2) No tool annotations

- Fix: add `annotations(read_only_hint = true, open_world_hint = false, idempotent_hint = true)` and a `title` to each `#[tool]` in `src/server.rs`. `rmcp` 3.8 supports this (`rmcp-macros` `tool.rs`). Clients can then approve calls automatically.

### I-7 (P2) Tool descriptions do not name the game

- Fix: start each description with the domain, for example `search`: "Find World of Warcraft 3.3.5a (WotLK) records by name: items, spells, creatures, quests, Lua/addon API, client offsets, …". Add trigger words to the first line of `instructions`: "Wrath of the Lich King", "addon", "TrinityCore/AzerothCore private server", "reverse engineering".
- Put the relation list per kind in the `relation` parameter doc (≈60 tokens), so the tool works without the instructions.

### I-8 (P2) Not enough context to choose between records

- Evidence: `search onyxia kind=creature` returns `Onyxia` and `Onyxia (1)` with no level or rank. `taught_by` returns 41 trainers without zone or faction.
- Fix: add creature subtext (I-1). In `taught_by`, `sells`, `sold_by`, `givers`, and `enders`, add `faction=` or the zone of the first spawn.

### I-9 (P2) `chain` returns one step

- Evidence: `related quest 7787 chain` returns only 7786. Quest 7785 needs a second call. The description says "a quest chain".
- Fix: return the full ordered chain (`prev_quest` back to the start, follow-ups forward), or change the description to "the previous and next quest".

### I-10 (P3) Schema dump is large

- The full schema is 23.6 KB (≈5.9k tokens) and also shows the FTS shadow tables (`search_config`, `search_data`, …).
- Fix: in the `sql` description, recommend `SELECT sql FROM sqlite_schema WHERE name = '<table>'`, and list the table names in the instructions.

### I-11 (P3) Smaller items

- Remove unusable `kind` values from the `search` and `related` enums (saves tokens and invalid calls).
- Reject `PRAGMA` assignments and `BEGIN`/`COMMIT`. Now they run and then report `query returned no rows`. They are harmless (`query_only=1`), but the message is wrong.
- Spell icons are full paths with escaped backslashes (`Interface\\Icons\\Spell_Fire_FlameBolt`); item icons are bare names (`INV_Sword_39`). Use one form.
- A 1-letter search costs 28 ms, because the FTS prefix index starts at 2 letters. This is acceptable; a 2-letter minimum is an option.

## No action needed

- Latency, memory, and throughput.
- Read-only protection and the 2 s SQL time limit.
- The compact output format and the error messages that list valid values.
- Out-of-order pipelined responses (JSON-RPC permits them).
