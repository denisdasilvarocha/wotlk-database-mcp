# Tasks

## 1. Schema version

- [x] 1.1 Add a shared `SCHEMA_VERSION` constant (value 1), make the builder write `meta.schema_version`, and verify that a rebuilt `data/wotlk.sqlite` returns `1` for `SELECT value FROM meta WHERE name='schema_version'`
- [x] 1.2 Check the schema version when the server opens a database, and verify with a test that a database without the expected version fails with both versions in the error

## 2. Path resolution and CLI

- [x] 2.1 Remove the `CARGO_MANIFEST_DIR` fallback, resolve `--db`, then `WOTLK_DB`, then the cache path from the `dirs` cache directory, and verify with tests for precedence and for a missing explicit path
- [x] 2.2 Add the `fetch` subcommand to the argument parser, and verify that usage errors still print the usage text
- [x] 2.3 Change the `justfile` recipes to pass `--db data/wotlk.sqlite`, and verify that `just` runs the server locally

## 3. Download pipeline

- [x] 3.1 Add `reqwest` (rustls), `zstd`, `sha2`, and `dirs`, and verify that `cargo deny check` passes
- [x] 3.2 Implement the lock, the cache check after the lock, the streaming download, zstd decoding to a temp file, the sha256 check, fsync, and the rename. Verify with tests against a local HTTP server: a valid file is installed, a wrong hash leaves no final file, and two concurrent calls download once
- [x] 3.3 Report progress as a percentage from `Content-Length`, and verify it in the local-server test
- [x] 3.4 Make `fetch` run the pipeline in the foreground and exit 0 or non-zero, and verify that it exits 0 without a request when a valid file is cached

## 4. Server state during download

- [x] 4.1 Hold the `Loading`, `Ready`, or `Failed` state in `WotlkServer`, and map `Loading` and `Failed` to tool errors with the progress, the cause, the URL, and the `fetch` hint. Verify with rmcp client tests: `initialize` and the tool list succeed while loading, a call while loading returns "downloading" with the percentage, and a call after ready returns data
- [x] 4.2 On a cache miss, start the download task after the handshake, and verify with a client test against the local HTTP server that tools work after the download without a restart

## 5. Release pipeline

- [x] 5.1 Initialize dist for the five targets with GitHub Release archives only (npm installer disabled), and verify that `dist plan` lists the five targets
- [ ] 5.2 Add a `db-release` workflow that compresses `data/wotlk.sqlite` with zstd, uploads it with its sha256 to the `db-schema<N>` release, and verify that the asset downloads and the hash matches
- [ ] 5.3 Embed the database URL and the sha256 in release builds, and verify that a release binary downloads `db-schema1` successfully with `fetch`
- [ ] 5.4 Add the npm launcher package (bin script, `optionalDependencies` at the exact version, unsupported-platform message) and the platform package template, and verify on each OS in CI with `npm install --ignore-scripts` from packed tarballs, then run the server and list its tools
- [x] 5.5 Add the publish job to the release workflow: publish platform packages first and the launcher last. Verify that `npm pack --dry-run` shows no `.sqlite` file in any package
- [ ] 5.6 Publish placeholder versions of all six package names, and verify that each name shows on npmjs.com under the owner account

## 6. Documentation

- [ ] 6.1 Change `README.md` and `docs/install.md` to show the npx `mcpServers` config first, then `fetch`, the cache locations, `WOTLK_DB`, and building from source. Verify that the documented config connects from an MCP client
- [x] 6.2 Add a CHANGELOG entry that marks the removed compile-time fallback as **BREAKING**, and verify that it names `--db` and `WOTLK_DB` as the migration

## 7. Integration

- [ ] 7.1 Tag a release candidate and verify on macOS, Linux, and Windows that `npx -y wotlk-database-mcp` with an empty cache answers the handshake, reports download progress, and then serves `search` results
