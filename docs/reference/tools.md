# Tool reference

This page describes the four MCP tools in server 0.1.0. The data targets World of Warcraft 3.3.5a, client build 12340.

## Transport and common output

The server uses MCP over standard input and standard output. It has no authentication setting. The tools read the local database and do not change it. Each tool is marked read-only, non-destructive, idempotent, and closed-world.

A successful call returns one MCP text content block. A query error returns a text block with `isError=true`. Arguments that fail JSON deserialization return a JSON-RPC error. Internal database failures return a JSON-RPC internal error.

Text values escape backslashes, newlines, carriage returns, and tabs as `\\`, `\n`, `\r`, and `\t`. Blob values appear as `<N bytes>`. Compact record output omits null, empty, and numeric zero fields, except record keys and spawn `map=0`. SQL output retains these values and writes null as `NULL`.

Money values use copper. Time values use milliseconds unless a schema comment specifies seconds. Flags and enums use names.

## search

`search` finds records by name. It searches all supported kinds except `meta`.

| Name | Type | Required | Default | Meaning |
| --- | --- | --- | --- | --- |
| `text` | String | Yes | None | Name or words to find. Must contain a letter or digit. |
| `kind` | String enum | No | All searchable kinds | Restricts results to one kind from [Kinds](#kinds). |
| `limit` | Unsigned 32-bit integer | No | `20` | Maximum hits, from `1` to `100`. |

Search is case-insensitive. Non-alphanumeric characters separate tokens. Tokens with two or more characters match prefixes. One-character tokens match exactly. All tokens must match.

Exact names appear first, then name prefixes. Within these groups, player-learnable spells and creatures with spawns have priority. The remaining order uses search relevance, name length, kind, and ID.

The output has one line per hit, in the form `kind id name (subtext)`. When the ID equals the name, the output includes the name once. Spell subtext contains rank and family. Creature subtext contains title, level, and rank. No matches returns `no matches for "text"`, with `of kind kind` when a kind filter is present.

Example request:

```json
{"text":"Fireball","kind":"spell","limit":1}
```

Expected text:

```text
spell 133 Fireball (Rank 1 MAGE)
```

The example uses the search query and data in `data/wotlk.sqlite`. Search has no cursor or offset. It does not report the number of omitted hits.

| Error | Caller action |
| --- | --- |
| `search text needs at least one letter or digit` | Supply a name or alphanumeric token. |
| `limit must be between 1 and 100` | Set `limit` within that range. |
| Argument deserialization error | Use the parameter types and kind names in this reference. |

## get

`get` returns one record, or all build provenance entries for `meta`.

| Name | Type | Required | Default | Meaning |
| --- | --- | --- | --- | --- |
| `kind` | String enum | Yes | None | Record kind from [Kinds](#kinds). |
| `id` | Signed 64-bit integer or string | Except for `meta` | None | Numeric ID or exact name, as defined by the kind. |
| `fields` | Array of strings | No | Summary fields | Adds columns to the summary. Exactly `["all"]` selects every column. |

Numeric kinds also accept a string that parses as a signed 64-bit integer. Named kinds use the exact name. A numeric argument for a named kind becomes its decimal string. For `meta`, the server ignores `id` and `fields`.

The output has one `key=value` pair per line. The record key is always present. Requested fields still follow the default-value omission rules. Field order follows the table columns, not the request order. `meta` returns `name=value` lines in database row order.

Example request:

```json
{"kind":"map","id":0}
```

Expected text:

```text
id=0
name=Eastern Kingdoms
type=COMMON
```

The example record comes from `data/wotlk.sqlite`. This tool has no row limit or pagination.

| Error | Caller action |
| --- | --- |
| `kind KIND needs an id` | Supply the record ID. |
| `kind KIND needs a numeric id` | Supply an integer or an integer string. |
| `unknown field 'FIELD' for KIND; fields: ...` | Select a column from the returned field list. |
| `KIND ID not found` | Use `search` to find an existing record. |
| Argument deserialization error | Use a listed kind and the documented parameter types. |

### Kinds

`get` supports every kind below. `search` supports every row except `meta`. Only the kinds in [Relations](#relations) support `related`.

| ID form | Kinds |
| --- | --- |
| Numeric ID | `item`, `creature`, `spell`, `itemset`, `quest`, `gameobject`, `map`, `area`, `faction`, `talent`, `skill`, `glyph`, `achievement` |
| Exact name | `offset`, `lua_function`, `lua_event`, `widget`, `cvar`, `lua_type`, `combatlog_event`, `secure_template` |
| No ID | `meta` |

### Summary fields

The record key is `id` for numeric kinds and `name` for named kinds. The server adds these summary fields when their columns exist and their values are not defaults.

| Kind | Summary fields |
| --- | --- |
| `item` | `name`, `quality`, `item_level`, `required_level`, `class`, `subclass`, `slot`, `bonding`, `stats`, `armor`, `damage`, `speed`, `spells`, `sockets`, `itemset`, `icon` |
| `creature` | `name`, `subname`, `min_level`, `max_level`, `rank`, `type`, `faction`, `npc_flags` |
| `spell` | `name`, `rank`, `school`, `power_type`, `mana_cost`, `mana_cost_pct`, `cast_time`, `range_min`, `range_max`, `cooldown`, `duration`, `description`, `icon` |
| `itemset` | `name`, `items`, `bonuses`, `required_skill`, `required_skill_rank` |
| `quest` | `name`, `level`, `min_level`, `zone`, `sort`, `type`, `objectives`, `prev_quest`, `next_quest`, `exclusive_group`, `reward_money`, `reward_spell` |
| `gameobject` | `name`, `type`, `display_id`, `loot_id` |
| `map` | `name`, `type`, `expansion`, `max_players` |
| `area` | `name`, `map`, `parent`, `level`, `team` |
| `faction` | `name`, `description`, `parent` |
| `talent` | `name`, `tab_id`, `tab`, `class`, `tier`, `column_index`, `max_rank`, `ranks`, `prereq_talent`, `prereq_rank`, `required_spell` |
| `skill` | `name`, `category`, `description` |
| `glyph` | `name`, `spell_id`, `type`, `icon` |
| `achievement` | `name`, `description`, `category`, `points`, `faction`, `reward` |
| `offset` | `domain`, `platform`, `va`, `rva`, `status`, `comment` |
| `lua_function` | `signature`, `description`, `parameters`, `returns`, `protected`, `hw_event` |
| `lua_event` | `description`, `parameters`, `category` |
| `widget` | `description`, `inherits`, `methods`, `script_handlers` |
| `cvar` | `default_value`, `description`, `category` |
| `lua_type` | `description`, `enum_values` |
| `combatlog_event` | `type`, `prefixes`, `description`, `arguments` |
| `secure_template` | `description`, `attributes`, `examples` |
| `meta` | All provenance entries, as `name=value` lines. |

### Offsets

An offset ID is a qualified name, such as `WoWOffsets::Lua::FrameScript::Execute`. The `offset` kind uses the SQL table `offsets`.

> [!WARNING]
> Only `status=verified` identifies a confirmed offset. Read the `comment` field for its constraints.

`va` is a preferred-image address for Windows x86 with image base `0x00400000`. The runtime address is the loaded module base plus `rva`.

## related

`related` lists records linked to a starting record.

| Name | Type | Required | Default | Meaning |
| --- | --- | --- | --- | --- |
| `kind` | String enum | Yes | None | Starting kind from [Relations](#relations). |
| `id` | Signed 64-bit integer or string | Yes | None | Starting record's numeric ID. Integer strings are accepted. |
| `relation` | String enum | Yes | None | Relation valid for the starting kind. |
| `limit` | Unsigned 32-bit integer | No | `50` | Maximum rows, from `1` to `1000`. |

The output has one `kind id name` line per row. Extra columns appear as `column=value`. Empty names and default extra fields are omitted. No linked rows returns `no RELATION for KIND ID`.

When rows exceed the limit, the last line is `-- LIMIT of TOTAL shown; raise limit (max 1000)`. This tool has no cursor or offset. Use `sql` for narrower queries or explicit pagination.

Example request:

```json
{"kind":"spell","id":133,"relation":"ranks","limit":1}
```

Expected text:

```text
spell 133 Fireball rank=1
-- 1 of 16 shown; raise limit (max 1000)
```

The rank rows and count come from `data/wotlk.sqlite`.

| Error | Caller action |
| --- | --- |
| `relation RELATION is not valid for KIND; valid: ...` | Select a relation from the returned list. |
| `kind KIND needs a numeric id` | Supply an integer or an integer string. |
| `KIND ID not found` | Use `search` to find the starting record. |
| `limit must be between 1 and 1000` | Set `limit` within that range. |
| Argument deserialization error | Use a supported related kind, relation name, and parameter types. |

### Relations

Each starting kind supports exactly the relations in this table.

| Starting kind | Relations |
| --- | --- |
| `item` | `dropped_by`, `sold_by`, `reward_of`, `set`, `created_by`, `disenchant` |
| `creature` | `loot`, `sells`, `trains`, `starts_quests`, `ends_quests`, `spawns`, `spells` |
| `quest` | `givers`, `enders`, `chain`, `rewards`, `requirements` |
| `spell` | `ranks`, `taught_by`, `used_by`, `talent`, `glyph`, `triggered` |
| `gameobject` | `loot`, `spawns` |
| `area` | `children`, `parent` |
| `faction` | `members` |
| `achievement` | `criteria` |

### Relation field rules

These fields have meanings that are not visible in their names.

| Field or relation | Meaning |
| --- | --- |
| Loot `chance` | Percentage. A grouped row with `chance=0` shares the group's remaining chance equally. Compact output omits this zero. |
| Loot `via` | Shared loot table that supplies the row. The row's chance applies inside that table. |
| Quest chain `step` | Distance from the queried quest. Negative values precede it. Positive values follow it. Zero identifies the queried quest and is omitted. |
| Vendor `price` | Required gold, in copper. Extended-cost purchases omit gold unless the item requires the buy price. |
| Vendor `cost` | Extended costs, such as currencies or items. |
| Spawn `world_x`, `world_y`, `world_z` | World coordinates, not zone-map percentages. |
| Spawn `orientation` | Radians. |
| Spawn `spawn_time` | Seconds. |
| Spawn `map` | Map ID. `map=0` is valid and remains in the output. |
| Spawn `location_source` | `zone` for a zone name, or `map` for a map-name fallback. |
| Missing spawn `zone` or `area` | Unknown location detail, not an absence of spawns. |

## sql

`sql` runs one read-only SQLite statement.

| Name | Type | Required | Default | Meaning |
| --- | --- | --- | --- | --- |
| `query` | String | Yes | None | One non-empty, read-only SQLite statement. |
| `limit` | Unsigned 32-bit integer | No | `200` | Maximum result rows, from `1` to `1000`. |

The server rejects writes, pragma assignments, transaction control, savepoints, `ATTACH`, and `DETACH`. Schema inspection pragmas remain available. The SQLite progress handler checks elapsed time every 1000 virtual machine operations and interrupts after two seconds.

The output starts with a tab-separated header. Each result row is tab-separated. No rows adds `-- no rows` after the header. Extra rows add `-- truncated: first LIMIT rows shown; raise limit (max 1000) or narrow the query`.

Example request:

```json
{"query":"SELECT value FROM meta WHERE name = 'client_build'"}
```

Expected text:

```text
value
12340
```

The query ran against `data/wotlk.sqlite`. For table definitions and column comments, query `sqlite_schema`, such as `SELECT sql FROM sqlite_schema WHERE name = 'item'`. SQL has no separate pagination parameter. A query can use `ORDER BY`, `LIMIT`, and `OFFSET`.

| Error | Caller action |
| --- | --- |
| `query is empty` | Supply a statement. |
| `limit must be between 1 and 1000` | Set `limit` within that range. |
| `only one statement is allowed` | Send one statement per call. |
| `only read-only statements are allowed` | Remove writes, assignments, transaction control, or file attachment. |
| `invalid SQL: ...` | Correct the SQL using the reported SQLite error and table schema. |
| `query stopped after 2 seconds; narrow it` | Add filters or reduce joins and recursive work. |
| `SQL error: ...` | Correct the query using the reported SQLite error. |
| Argument deserialization error | Supply a string query and an integer limit. |

## Related pages

[Configuration reference](configuration.md) defines database selection and logging. [Rebuild the database](../guides/rebuild-database.md) describes data updates. [Architecture](../architecture.md) describes the data pipeline.
