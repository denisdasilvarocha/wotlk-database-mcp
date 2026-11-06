# Tool reference

The server exposes four read-only tools through MCP. The examples are JSON argument objects; your client may call the tools on your behalf. Search for a record first, then use its ID with `get` or `related`.

## `search`

Find records by name and indexed context, such as a spell's rank or a creature's title.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | string | Yes | Search terms; must contain at least one letter or digit. |
| `kind` | string | No | Restrict results to one [record kind](#record-kinds), except `meta`. |
| `limit` | integer | No | Maximum results, from 1 to 100; defaults to 20. |

Search is case-insensitive. Punctuation separates terms, all terms must match, and terms longer than one character match word prefixes. Exact names rank first, followed by names that start with the search text; player-learnable spells and creatures with spawns are prioritized among other matches.

For example:

```json
{"text": "Frostbolt", "kind": "spell", "limit": 3}
```

The first results in the current snapshot are:

```text
spell 116 Frostbolt (Rank 1 MAGE)
spell 205 Frostbolt (Rank 2 MAGE)
spell 837 Frostbolt (Rank 3 MAGE)
```

Each line contains the kind, ID, name, and any available context. Name-based records may show the name only once. If the result reaches the limit, raise it or narrow the search; `search` does not report how many additional matches exist.

## `get`

Retrieve a record as `key=value` lines.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | string | Yes | A [record kind](#record-kinds), including `meta`. |
| `id` | integer or string | Except for `meta` | Numeric ID for game records, or exact name for offsets and Lua API records. Numeric IDs can also be strings. |
| `fields` | array of strings | No | Extra fields to add to the summary, or exactly `["all"]` for every field. |

For example, request all fields for Frostbolt rank 1:

```json
{"kind": "spell", "id": 116, "fields": ["all"]}
```

The identifier is always included. Other null, empty, and numeric zero values are omitted even with `["all"]`; text values such as a CVar default of `"0"` are retained. Unknown field names produce an error listing the available fields. Use the [SQL schema query](#sql) to inspect their descriptions.

For build provenance, omit the ID:

```json
{"kind": "meta"}
```

### Record kinds

- Game records use numeric IDs: `item`, `creature`, `spell`, `itemset`, `quest`, `gameobject`, `map`, `area`, `faction`, `talent`, `skill`, `glyph`, and `achievement`.
- Client and addon records use exact names: `offset`, `lua_function`, `lua_event`, `widget`, `cvar`, `lua_type`, `combatlog_event`, and `secure_template`. For example, an offset ID is `WoWOffsets::Lua::FrameScript::Execute`.
- `meta` returns build provenance and does not require an ID. It is available through `get`, not `search` or `related`.

## `related`

Follow a relationship from an existing record. Each result line contains a kind, ID, optional name, and any additional `key=value` fields.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | string | Yes | Starting record kind from the table below. |
| `id` | integer or string | Yes | ID of the starting record. |
| `relation` | string | Yes | Relationship supported for that kind. |
| `limit` | integer | No | Maximum rows, from 1 to 1000; defaults to 50. |

For example:

```json
{"kind": "spell", "id": 116, "relation": "ranks", "limit": 3}
```

```text
spell 116 Frostbolt rank=1
spell 205 Frostbolt rank=2
spell 837 Frostbolt rank=3
-- 3 of 16 shown; raise limit (max 1000)
```

When more rows exist than the limit, the final line shows the number returned and the total. A record with no results returns a `no <relation> for ...` message.

| Kind | Relations |
| --- | --- |
| `item` | `dropped_by`, `sold_by`, `reward_of`, `set`, `created_by`, `disenchant` |
| `creature` | `loot`, `sells`, `trains`, `starts_quests`, `ends_quests`, `spawns`, `spells` |
| `quest` | `givers`, `enders`, `chain`, `rewards`, `requirements` |
| `spell` | `ranks`, `taught_by`, `used_by`, `talent`, `glyph`, `triggered` |
| `gameobject` | `loot`, `spawns` |
| `area` | `children`, `parent` |
| `faction` | `members` |
| `achievement` | `criteria` |

## `sql`

Run one read-only SQLite statement, including queries against tables that are not record kinds.

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `query` | string | Yes | One nonempty read-only SQLite statement. |
| `limit` | integer | No | Maximum rows, from 1 to 1000; defaults to 200. |

For example, list the `spell` table:

```json
{"query": "SELECT name FROM sqlite_schema WHERE type = 'table' AND name = 'spell'"}
```

The response has a tab-separated header and rows:

```text
name
spell
```

To discover columns and their comments, query `SELECT sql FROM sqlite_schema WHERE name = 'spell'`. For offsets, try `SELECT name, va, status FROM offsets WHERE domain = 'Lua'`.

SQL results include `NULL` values; an empty result has a `-- no rows` line. When rows are cut, a final line reports truncation. Backslashes, tabs, carriage returns, and newlines in text are escaped; blobs are represented by their byte count. Queries have a two-second execution limit. Writes, pragma assignments, transactions, and attaching or detaching databases are rejected; read-only schema inspection such as `PRAGMA table_info(spell)` is supported.

## Interpret data

- Money is in copper. Vendor `price` is the required gold expressed in copper; `cost` lists extended costs.
- Times are milliseconds unless a schema comment says otherwise. Spawn `spawn_time` is seconds and orientation is radians.
- Spawn `world_x`, `world_y`, and `world_z` are world coordinates, not zone-map percentages. `map=0` is valid; `location_source` distinguishes zone names from map fallbacks. Missing zone or area fields mean unknown.
- Loot chance is a percentage. A zero chance inside a group means an equal share of the group's remaining chance. `via` identifies the shared loot table containing the row; its chance is the chance inside that table, not a cumulative drop chance. Shared tables are followed up to five levels.
- Quest chain `step` is relative to the queried quest: zero is that quest, negative is before it, and positive is after it.
- Offset `va` is a Windows x86 preferred-image address with image base `0x00400000`; the runtime address is the module base plus `rva`. Only `status=verified` is confirmed; read each comment for caveats.

The database is a snapshot. Client DBC data supplies client records, while TrinityCore TDB supplies server data such as loot and spawns. A customized server may differ.
