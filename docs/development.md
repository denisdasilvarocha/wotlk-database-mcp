# Development guide

## Set up

Install [just](https://just.systems/) and clone the repository. Check the requirements for your platform:

- On macOS, install Homebrew and the Xcode Command Line Tools.
- On Windows, use the MSYS2 UCRT64 terminal and install the Microsoft C++ Build Tools with the Windows SDK.
- On Linux, use a distribution with apt, dnf, or pacman for the setup script.

Then install the remaining tools and hooks:

```bash
just setup
```

The script installs missing system and development tools, selects the Rust toolchain pinned in `rust-toolchain.toml`, and installs Git LFS and prek hooks. It may invoke a system package manager or download installers. Inspect [scripts/setup.sh](../scripts/setup.sh) for the exact steps.

## Build the database and server

```bash
just build
```

This validates or prepares the vendored offsets and Lua API data, downloads pinned TrinityCore TDB and client DBC archives, verifies their checksums, builds `data/wotlk.sqlite`, and compiles the release server. The vendored inputs are under `builder/vendor/`; input URLs, checksums, and archive members are defined in [scripts/build_db.sh](../scripts/build_db.sh).

To build only the database, run `just build-db`.

Run `just run` to start the server with the built database, or `just inspect` to open the MCP Inspector (requires Node.js). Both start the server on stdio. `just run` explicitly selects `data/wotlk.sqlite`; the default runtime cache is separate.

## Check changes

| Command | Purpose |
| --- | --- |
| `just fmt` | Format Rust and TOML files. |
| `just fmt-check` | Check formatting without writing. |
| `just lint` | Run Clippy with warnings denied. |
| `just test` | Run workspace tests and database build script tests. |
| `just doc` | Build Rust API documentation with warnings denied. |
| `just deps` | Check dependency advisories, licenses, sources, and unused dependencies. |
| `just check` | Run all hooks at the manual stage. |
| `node --test npm/*.test.mjs` | Run the npm packaging tests separately with Node.js. |

Run `just --list` for all recipes. The root [justfile](../justfile) defines their commands.

## Configure Dependabot auto-merge

Create an active branch ruleset for `main`. Enable **Require status checks to pass** and **Require branches to be up to date before merging**. Select the GitHub Actions checks `quality (ubuntu-24.04)`, `quality (macos-15)`, and `quality (windows-2022)`. Keep GitHub Actions subject to the ruleset. [GitHub branch rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets#require-status-checks-to-pass-before-merging)

Dependabot auto-merge skips merges until the strict ruleset is active.

If GitHub rejects a merge because the Dependabot branch is behind `main`, comment `@dependabot rebase` on the pull request and wait for CI on the new head. [Dependabot commands](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-pull-request-comment-commands)

If a release dispatch fails after a bot merge, rerun the **Dependabot auto-merge** workflow to retry the dispatch.
