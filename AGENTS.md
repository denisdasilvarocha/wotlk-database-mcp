# AGENTS.md

## Project

- Preserve the read-only WotLK 3.3.5a snapshot and the MCP stdio protocol when changing server behavior.
- Use the Rust toolchain pinned in `rust-toolchain.toml`; the npm launcher requires Node.js 22 or later.

## Commands

Run from the repository root. Review platform prerequisites in `docs/development.md` before `just setup`, which can install system tools and Git hooks.

- Run Rust workspace tests and database build-script tests: `just test`.
- Run one integration-test file: `cargo test --locked --test schema_version`.
- Run one test by name: `cargo test --locked --test schema_version open_rejects_missing_schema_version`.
- Run the npm packaging tests separately: `node --test npm/*.test.mjs`.
- Run the combined project gates used by Linux CI: `just check`.
- Check formatting or Clippy separately: `just fmt-check` or `just lint`.
- Rebuild the database from pinned inputs: `just build-db`.

## Architecture

- Keep database construction and imports in `builder/src/`, and server queries and read-only protections in `src/db.rs`. Both share `src/schema_version.rs`.
- Preserve the read-only SQLite connection, authorizer, and limits when changing the SQL tool.
- When changing a tool's public behavior, keep its definitions and server instructions in `src/server.rs` consistent with `docs/tools.md`.
- For integration tests that need game data, use the generated fixtures in `tests/common/`; CI runs without fetching the Git LFS payloads.
- Treat client DBC data as the source for client records; TrinityCore TDB supplies server data such as loot and spawns.

## Gotchas

- `data/wotlk.sqlite` and `builder/vendor/api/codex.json` are Git LFS artifacts. Regenerate the database with `just build-db` and the API export with `just vendor-api` instead of editing them by hand.
- `just build` also refreshes the vendored API and replaces `data/wotlk.sqlite`; use `just test` for routine validation.
- On Windows, run the shell recipes in an MSYS2 UCRT64 environment, as in CI.

## Workflow

- Keep npm platform packages versioned together through Changesets; the release workflow synchronizes the server's Cargo version.
- For the full CI definition, including the launcher smoke test, inspect `.github/workflows/ci.yml`.

## Boundaries

- Always: reserve server stdout for MCP frames and send logs through `tracing` to stderr.
- Ask first: publish a database or npm release unless publication was explicitly requested.
- Never: replace published database assets. If the new database differs from the current schema release, increment `SCHEMA_VERSION` in `src/schema_version.rs` and rebuild; release binaries embed the original checksum.

## Read when needed

| When | Read |
| --- | --- |
| Setting up tools or building the database | `docs/development.md` |
| Changing tool parameters, output, or data conventions | `docs/tools.md` |
| Changing database publication or checksum handling | `.github/workflows/release-database.yml` |
| Changing npm packaging or release versioning | `.github/workflows/release.yml` |
