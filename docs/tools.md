# Tool reference

The server exposes these tools through MCP. The examples show the JSON argument objects passed to each tool; your client may call the tools on your behalf. Use `search` to find an ID, then `get` or `related` to explore it.

## Search

`search` finds names case-insensitively, including prefix matches. `kind` is optional. `limit` defaults to 20 and accepts 1 to 100.

```json
{"text": "Frostbolt", "kind": "spell", "limit": 10}
```

Results contain the kind, ID, name, and available context. Exact and prefix matches rank first.

## Get a record

`get` takes a `kind` and an `id`. Numeric IDs identify game records; offsets and Lua API kinds use exact names. Omit `fields` for summary fields, pass field names to add them, or use `["all"]` for all fields. The record identifier is always included. Other fields with null, empty, or numeric zero values are omitted; text values such as a CVar default of `"0"` are retained.

```json
{"kind": "spell", "id": 116, "fields": ["all"]}
```

Use `{"kind": "meta"}` without an ID for build provenance. The other kinds are:

- Game records: `item`, `creature`, `spell`, `itemset`, `quest`, `gameobject`, `map`, `area`, `faction`, `talent`, `skill`, `glyph`, and `achievement`.
- Client and addon data: `offset`, `lua_function`, `lua_event`, `widget`, `cvar`, `lua_type`, `combatlog_event`, and `secure_template`.

## Follow relationships

`related` takes `kind`, `id`, and `relation`; `limit` defaults to 50 and accepts 1 to 1000. If the results exceed the limit, a final line reports how many rows are shown and the total.

```json
{"kind": "item", "id": 19017, "relation": "dropped_by"}
```

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

## Query SQLite

`sql` accepts one read-only SQLite statement. `limit` defaults to 200 and accepts 1 to 1000; queries have a two-second limit. Results are tab-separated. Pragma assignments and transactions are rejected.

```json
{"query": "SELECT sql FROM sqlite_schema WHERE name = 'item'"}
```

The schema includes column comments and is useful for finding tables and fields. For example, inspect offsets with `SELECT name, va, status FROM offsets WHERE domain = 'Lua'`.

## Interpret results

- Money is in copper. Vendor `price` is gold expressed in copper; `cost` lists extended costs.
- Times are milliseconds unless a schema comment says otherwise. Spawn `spawn_time` is seconds and orientation is radians.
- Spawn `world_x`, `world_y`, and `world_z` are world coordinates, not zone-map percentages. `map=0` is valid; `location_source` distinguishes zone names from map fallbacks. Missing zone or area fields mean unknown.
- Loot chance is a percentage. A zero chance inside a group means an equal share of the group's remaining chance; `via` identifies a shared loot table.
- Quest chain `step` is relative to the queried quest: zero is that quest, negative is before it, and positive is after it.
- Offset `va` is a Windows x86 preferred-image address with image base `0x00400000`; runtime address is module base plus `rva`. Only `status=verified` is confirmed; read each comment for caveats.
