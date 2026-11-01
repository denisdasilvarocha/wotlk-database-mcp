# Rebuild the database

Build `data/wotlk.sqlite` from the pinned inputs. Use this guide after you update source data or the code that builds the search index.

## Prerequisites

Complete [Install prerequisites](../install.md#1-install-prerequisites) and run `just setup` in your checkout. Run all commands below from the repository root in Bash.

> [!WARNING]
> The first build downloads about 1.2 GB of client data and a 47 MB TDB dump. Allow network access and disk space for the archives, extracted files, and database.

The download sizes are approximate. The build uses these inputs:

| Input | Pin |
| --- | --- |
| TrinityCore world dump | TDB 335.26091. |
| Client DBC files | `wowgaming/client-data` v20.0, `Data.zip`. |
| Lua API export | Commit recorded in `builder/vendor/api/COMMIT`. |
| Windows x86 offsets | Snapshot recorded in `builder/vendor/offsets/VERSION`. |

For archive URLs, SHA-256 hashes, and member names, see the [configuration reference](../reference/configuration.md). For the roles of these inputs, see [Architecture](../architecture.md).

## 1. Select the inputs

Keep the default pins unless you need other archives. To override an archive, set its URL, SHA-256 hash, and member variables before the build. See the six build variables in the [configuration reference](../reference/configuration.md).

The builder requires enUS client DBC files for build 12340. An archive override does not change that requirement.

## 2. Prepare the vendored data

Choose the commands that match your update.

### Keep the current pins

Export the pinned Lua API data and validate the bundled offsets.

```sh
just vendor-api
just vendor-offsets
```

The API recipe uses the commit recorded in `builder/vendor/api/COMMIT`. The offsets recipe checks that each required catalog file exists and is not empty.

### Update the Lua API export

To select another API revision, pass its commit to the recipe.

```sh
just vendor-api COMMIT
```

Replace `COMMIT` with the Git commit you want from `Shard-MW/milkyway-codex`. The script exports the data to `builder/vendor/api/codex.json` and records the resolved commit in `builder/vendor/api/COMMIT`.

### Replace the offsets catalog

To import another catalog, pass its directory to the recipe.

```sh
just vendor-offsets /absolute/path/to/windows-x86
```

Replace the example path with the directory that contains all required catalog files. The script requires these nonempty files:

```text
caches.md
client-db.md
lua.md
movement-world.md
network.md
objects.md
spells.md
system-globals.md
ui-game.md
ui-widgets.md
units-players.md
warden.md
```

The script copies them into `builder/vendor/offsets/windows-x86`. It records the current date in `builder/vendor/offsets/VERSION`.

## 3. Build the database

Run the database recipe after you prepare the inputs.

```sh
just build-db
```

The script downloads archives into `target/build-db` and checks their SHA-256 hashes. Later builds reuse the verified cache. The builder writes `data/wotlk.sqlite`, including the search index.

> [!IMPORTANT]
> Rebuild the database after a search-index update. Replacing only the server binary does not update the index stored in SQLite.

If you also need a release binary, use `just build` instead. That recipe prepares the vendored data, builds the database, and compiles the server.

## 4. Verify the rebuilt database

Reconnect the MCP server to open the rebuilt file. Use the [database path configuration](../reference/configuration.md) if the harness points to another file.

Call `get` with these arguments:

```json
{"kind": "meta"}
```

Check the returned build provenance against your selected inputs. Then perform the [connection verification](../install.md#4-verify-the-connection).

## Troubleshoot the build

Use the script's error message to select the next action.

| Error or symptom | Action |
| --- | --- |
| `build-db: ... not found` | Run `just setup`, then retry `just build-db`. |
| Archive checksum mismatch | Check the URL and SHA-256 pair in the [configuration reference](../reference/configuration.md), including your overrides. Retry after you correct them. |
| `Git LFS did not fetch codex.json` | Check Git LFS access, then retry `just build-db`. |
| `vendor-api: API exporter returned incomplete data` | Check the selected API revision. Run `just vendor-api` with a compatible commit before you rebuild. |
| `vendor-offsets: missing or empty offsets file` | Supply all files listed above, then retry `just vendor-offsets` with the source directory. |

## Related pages

Use these pages for setup and interface details.

- [Install for an MCP harness](../install.md) to connect the server.
- [Configuration reference](../reference/configuration.md) for input overrides and database paths.
- [Tools reference](../reference/tools.md) for provenance and query calls.
- [Contributing](../../CONTRIBUTING.md) for development checks.
