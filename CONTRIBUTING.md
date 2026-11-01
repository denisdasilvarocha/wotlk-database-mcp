# Contributing

Use this workflow to change the server, the builder, or the documentation. The same `just` recipes run locally, in Git hooks, and in CI.

## Set up development

Install the [platform prerequisites and `just`](docs/install.md#1-install-prerequisites) before setup. From the repository root, run:

```sh
just setup
```

Setup installs missing tools and the selected Rust toolchain. It configures Git LFS and installs the `prek` commit and push hooks.

For a local database and release binary, follow the [installation guide](docs/install.md). Tests build fixtures and do not require the full game database.

## Use the recipes

The `justfile` defines these development commands. Run `just` to list them.

| Command | Action |
| --- | --- |
| `just` | List recipes through `_default`. |
| `just setup` | Install missing tools and Git hooks. |
| `just fmt` | Format Rust and TOML files in place. |
| `just fmt-check` | Check Rust and TOML formatting without changes. |
| `just lint` | Run Clippy across the workspace, targets, and features. Treat warnings as errors. |
| `just fix` | Apply Clippy fixes, including with dirty or staged files, then format. |
| `just test` | Run workspace unit, integration, and doc tests, then `tests/test_build_db.sh`. |
| `just doc` | Build workspace API docs without dependencies. Treat rustdoc warnings as errors. |
| `just deps` | Run `cargo deny` checks and reject unused dependencies with `cargo shear`. |
| `just check` | Run all `prek` hooks at the manual stage. |
| `just run` | Run the server over stdio with Cargo. |
| `just inspect` | Start MCP Inspector through `npx`. This command requires Node.js. |
| `just build` | Validate offsets, export the pinned API data, build the database, and compile the release server. |
| `just build-db` | Download pinned inputs and build `data/wotlk.sqlite`. |
| `just vendor-offsets` | Validate the bundled offsets catalog. |
| `just vendor-offsets <src>` | Refresh the catalog from the directory `<src>`. |
| `just vendor-api` | Export the API data at the recorded commit. |
| `just vendor-api <commit>` | Export the API data at `<commit>` and update the recorded commit. |

Replace `<src>` with a catalog directory. Replace `<commit>` with an upstream Git revision. See the [database rebuild guide](docs/guides/rebuild-database.md) for input updates.

## Follow the code conventions

The workspace uses Rust 2024 and requires Rust 1.99 or later. Cargo forbids unsafe code and enables these Rust warnings:

- Missing `Debug` implementations.
- Missing documentation.
- Unreachable public items.
- Unused qualifications.

Clippy enables the `pedantic` group. It also warns on these patterns:

| Pattern | Clippy lint |
| --- | --- |
| Allow attributes | `allow_attributes` and `allow_attributes_without_reason` |
| Debug macros | `dbg_macro` |
| Unchecked success assumptions | `expect_used` and `unwrap_used` |
| Explicit panics | `panic` |
| Direct output | `print_stderr` and `print_stdout` |
| Incomplete code | `todo` and `unimplemented` |

> [!IMPORTANT]
> Standard output carries MCP frames. Use `tracing` for diagnostics, not print macros. The server sends tracing output to standard error.

Use the existing separation between server queries and builder imports. The [architecture](docs/architecture.md) explains their responsibilities and source rules.

## Run the checks

Before you submit a change, run all local checks:

```sh
just check
```

This command runs every hook at the manual stage. A successful run reports passed hooks and exits with status zero.

The `.pre-commit-config.yaml` file assigns hooks to these stages:

| Stage | Hooks |
| --- | --- |
| Pre-commit and manual | `check-added-large-files`, `check-merge-conflict`, `check-toml`, `check-yaml`, `detect-private-key`, `end-of-file-fixer`, `mixed-line-ending`, and `trailing-whitespace`. |
| Pre-commit and manual | `fmt` runs `just fmt-check`. `lint` runs `just lint`. |
| Pre-push and manual | `test` runs `just test`. `doc` runs `just doc`. `deps` runs `just deps`. |

The line-ending hook uses `--fix=lf`. Local recipe hooks always run across the workspace and do not receive individual filenames. Setup preserves the Git LFS pre-push hook when it installs `prek`.

## Inspect CI failures

The `CI` workflow runs on pull requests, pushes to `main`, and manual dispatches. It runs `just check` and a locked workspace release build on these runners:

| Runner | Shell |
| --- | --- |
| `ubuntu-latest` | Bash. |
| `macos-latest` | Bash. |
| `windows-latest` | MSYS2 UCRT64. |

The Linux job also runs the pinned `actionlint` container. CI skips Git LFS payload downloads. Tests use fixtures instead of the full database.

With GitHub CLI access to this private repository, inspect the failed run:

```sh
gh run view --log-failed
```

Use the failed job and step names to select the corresponding local recipe.

## Update the documentation

Keep documentation with the behavior it describes. Update the applicable guide or reference page when you change a command, setting, tool, or data rule.

Add a user-visible change to the `Unreleased` section of [CHANGELOG.md](CHANGELOG.md). Keep facts in one page and link to that page from other documentation. Use the [documentation index](docs/index.md) to find each page’s purpose.

In your pull request, describe the behavior change and report the checks you ran. State any checks that you did not run.
