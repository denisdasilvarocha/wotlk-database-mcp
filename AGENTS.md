# AGENTS.md

## Commands

All commands run from the repository root in Bash (MSYS2 UCRT64 on Windows). `just` is the single entry point; Git hooks and CI call these recipes.

- `just setup`: install missing tools and configure Git LFS and the Git hooks.
- `just fmt`: format Rust and TOML.
- `just lint`: clippy with warnings denied.
- `just test`: workspace Rust tests, then the offline Bash provisioning test.
- Focused Rust test: `cargo test --workspace --all-features --locked <filter>`. This skips the Bash test.
- `just check`: every gate (file hygiene, format, lint, test, doc, dependencies). Use it as the final gate.
- `just build`: validate offsets, vendor the API, build the database, then build the release binary.
- `just run`: run the server on stdio.

## Layout

- `builder/`: separate workspace crate that validates inputs and builds the SQLite database. The server crate only reads the result.
- `src/server.rs`: tool schemas, tool descriptions, and the advertised server instructions.
- `tests/server.rs`: end-to-end MCP tests through a real client and a fixture database.
- `tests/builder.rs`: builder tests from small fixtures; no game data is required.
- `data/wotlk.sqlite`: built artifact tracked with Git LFS.

## Invariants

- stdout carries MCP JSON-RPC frames; any other byte breaks clients. The server binary logs through `tracing` to stderr.
- Runtime database access is read-only: attached databases are disabled and `meta.client_build` must exist. Do not add write paths.
- Keep error channels distinct: arguments that fail to deserialize and unknown tool names return `invalid_params`; every error raised inside a tool (bad values, invalid SQL, a missing record, a query failure) returns `CallToolResult::error`, so the model can read it and correct the call; internal failures return `internal_error`. An empty result is a success, not an error.
- Doc comments on tool parameter fields become the JSON Schema descriptions clients read; write them for the model.
- The `instructions` argument of `#[tool_handler]` lists kinds, relations, and tables so clients need no discovery call. A test pins its table list to `sqlite_schema`. Update it when the tool set, schema, or contract changes.
- Tool descriptions start with the game and build, and every tool carries read-only `annotations`. Keep both when you add a tool.
- Preserve valid `map=0` in spawn relations. Label world coordinates and distinguish zone names from map-name fallbacks; missing zone metadata does not prove spawn absence.

## Do not

- Do not edit `builder/vendor/api/codex.json` or `builder/vendor/api/COMMIT`. Run `just vendor-api [commit]` to export the bundled pin or update it.
- Do not edit `builder/vendor/offsets/`. Run `just vendor-offsets` to validate the bundled catalog, or `just vendor-offsets <dir>` to replace it.
- Do not edit `data/wotlk.sqlite`. Run `just build-db`; it replaces the output only after a successful build.
- Do not bypass a gate with `git commit --no-verify` or `git push --no-verify`. Do not lower a lint level, add an advisory ignore, or allow a new license to pass a check. Ask the user to approve a new license; report a wrong gate instead of working around it.
- Do not edit dependency versions by hand; add dependencies with `cargo add <crate>`. Prefer existing features and `rmcp` re-exports (`schemars`, `serde_json`).

## Verify

- Run `just fmt`, then `just check`, before finishing. `just check` must pass without unrequested changes to files.
- For tool changes, drive the new behavior through a real `rmcp` client with a fixture database in `tests/server.rs`; run `cargo test -p wotlk-database-mcp --test server --all-features --locked`.
- For builder changes, add or update fixture coverage in `tests/builder.rs`; run `cargo test -p wotlk-database-mcp --test builder --all-features --locked`.
- For provisioning script changes, run `bash tests/test_build_db.sh`, which uses offline fixtures.
- Update affected docs (`README.md`, `docs/install.md`, `docs/spec/wotlk-data-server.md`) and this file when behavior or the tool contract changes.

## Read when needed

- `README.md`: setup, build, database inputs, and runtime path precedence. Read it before changing setup or runtime configuration.
- `docs/spec/wotlk-data-server.md`: schema, tool contract, and testing decisions. Read it before changing tools, schema, or data semantics.
- `docs/install.md`: harness registration steps. Read it when connecting a client.
- Installed `rmcp` version in `Cargo.toml` and <https://docs.rs/rmcp>. Read the matching docs before using an API; `rmcp` changes between major versions.
