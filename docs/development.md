# Development guide

Set up the Rust workspace, run the tests, and build the database used by the MCP server. Run the commands from the repository root.

## Set up

Install Git and [just](https://just.systems/), then clone the repository. This command skips the Git LFS payloads, which are not needed to run the tests:

```bash
GIT_LFS_SKIP_SMUDGE=1 git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
```

The checkout contains LFS pointer files until you run `git lfs pull` or regenerate the artifacts. See the [Git LFS FAQ](https://github.com/git-lfs/git-lfs/blob/main/docs/man/git-lfs-faq.adoc#working-tree-contents).

The setup script has these platform requirements:

- **macOS:** Homebrew and Xcode Command Line Tools.
- **Linux:** a distribution with apt, dnf, or pacman for automatic system package installation.
- **Windows:** an x64 MSYS2 UCRT64 terminal and Microsoft C++ Build Tools with the Windows SDK. Use this terminal for the shell recipes. If Rust is already installed, the active toolchain must use `x86_64-pc-windows-msvc`.

Setup installs missing system and development tools, selects the Rust toolchain pinned in `rust-toolchain.toml`, and configures Git LFS and prek hooks. It may invoke a system package manager or download installers. Review [scripts/setup.sh](../scripts/setup.sh), then run:

```bash
just setup
```

The npm packaging tests and MCP Inspector also require Node.js.

## Run tests

```bash
just test
```

This runs Rust workspace tests and database build-script tests. Integration tests generate fixtures and do not need the Git LFS game database or vendored API payload.

To run one integration test or one test within it:

```bash
cargo test --locked --test schema_version
cargo test --locked --test schema_version open_rejects_missing_schema_version
```

Run the npm packaging tests separately:

```bash
node --test npm/*.test.mjs
```

## Build the database and server

> **Warning:** `just build` refreshes the vendored Lua API export and replaces `data/wotlk.sqlite`. Use `just test` for routine validation, and preserve any local changes to those generated files before building.

```bash
just build
```

The build validates the bundled offsets, refreshes the pinned Lua API export, downloads pinned TrinityCore TDB and client DBC archives, verifies their checksums, builds `data/wotlk.sqlite`, and compiles the release server. Inputs and checksums are defined in [scripts/build_db.sh](../scripts/build_db.sh). Downloaded build inputs are cached under `target/build-db`.

To rebuild only the database, run `just build-db`. This also replaces `data/wotlk.sqlite`, but uses the existing vendored API and offsets. Both the database and `builder/vendor/api/codex.json` are Git LFS artifacts; do not edit them by hand.

Start the server with the database you built:

```bash
just run
```

The server communicates over stdio, so connect it through an MCP client. `just run` explicitly selects `data/wotlk.sqlite`; the normal runtime cache is separate. To launch it through the MCP Inspector, run `just inspect` with Node.js installed.

## Update database inputs

- Run `just vendor-offsets` to validate the bundled offsets. To replace them from a complete catalog, run `just vendor-offsets /path/to/offsets`.
- Run `just vendor-api` to regenerate the Lua API export at the pinned commit, or `just vendor-api <commit>` to update the pinned commit and export. The recipe uses Bun and may fetch the upstream repository.
- Run `just build-db` after changing inputs to regenerate the database. The [tool reference](tools.md#interpret-data) describes the data sources and conventions.

Published database assets are immutable because release binaries embed their checksum. If the regenerated database differs from the current schema release, increment `SCHEMA_VERSION` in [src/schema_version.rs](../src/schema_version.rs) and rebuild. Do not replace existing release assets. The [database release workflow](../.github/workflows/release-database.yml) contains the publication checks.

## Check changes

| Command | Purpose |
| --- | --- |
| `just fmt` | Format Rust and TOML files. |
| `just fmt-check` | Check formatting without writing. |
| `just lint` | Run Clippy with warnings denied. |
| `just test` | Run workspace tests and database build-script tests. |
| `just doc` | Build Rust API documentation with warnings denied. |
| `just deps` | Check dependency advisories, licenses, sources, and unused dependencies. |
| `just check` | Run all hooks at the manual stage. |
| `node --test npm/*.test.mjs` | Run the npm packaging tests separately. |

Run `just --list` for all recipes; the [justfile](../justfile) defines their commands. Linux CI runs `just check`; macOS and Windows run `just test`. CI also runs the npm tests and a launcher smoke test on each platform. See the [CI workflow](../.github/workflows/ci.yml) for the full gates.

## Configure Dependabot auto-merge

Create an active branch ruleset for `main`. Enable **Require status checks to pass** and **Require branches to be up to date before merging**. Select the GitHub Actions checks `quality (ubuntu-24.04)`, `quality (macos-15)`, and `quality (windows-2022)`. Keep GitHub Actions subject to the ruleset. See [GitHub's branch rules documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets#require-status-checks-to-pass-before-merging).

Dependabot auto-merge skips merges until a ruleset with strict required status checks is active.

If GitHub rejects a merge because the Dependabot branch is behind `main`, comment `@dependabot rebase` on the pull request and wait for CI on the new head. See [Dependabot comment commands](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-pull-request-comment-commands).

If a release dispatch fails after a bot merge, rerun the **Dependabot auto-merge** workflow to retry the dispatch.
