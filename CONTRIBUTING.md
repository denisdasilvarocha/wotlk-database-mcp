# Contributing

Use this workflow to change the server, the builder, or the documentation. The same `just` recipes run locally, in Git hooks, and in CI.

## Set up development

Install the [platform prerequisites and `just`](docs/install.md#install-prerequisites) before setup. Use Node.js 24 and pnpm 10.34.9. From the repository root, run:

```sh
just setup
pnpm install --frozen-lockfile --ignore-scripts
```

Setup installs missing tools and the selected Rust toolchain. It configures Git LFS and installs the `prek` commit and push hooks.
The pnpm command installs workspace dependencies and the Changesets CLI without running package scripts.

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
| `just run` | Run the server over stdio with Cargo and `--db data/wotlk.sqlite`. |
| `just inspect` | Start MCP Inspector through `npx` with `--db data/wotlk.sqlite`. This command requires Node.js. |
| `just build` | Validate offsets, export the pinned API data, build the database, and compile the release server. |
| `just build-db` | Download pinned inputs and build `data/wotlk.sqlite`. |
| `just vendor-offsets` | Validate the bundled offsets catalog. |
| `just vendor-offsets <src>` | Refresh the catalog from the directory `<src>`. |
| `just vendor-api` | Export the API data at the recorded commit. |
| `just vendor-api <commit>` | Export the API data at `<commit>` and update the recorded commit. |

Replace `<src>` with a catalog directory. Replace `<commit>` with an upstream Git revision. See the [database rebuild guide](docs/guides/rebuild-database.md) for input updates and the [release guide](docs/guides/release.md) for Changesets releases.

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

The `CI` workflow runs directly on pull requests and manual dispatches. On pushes to `main`, the `Release` workflow calls the same CI checks once.

| Runner | Shell | Checks |
| --- | --- | --- |
| `ubuntu-24.04` | Bash. | `just check`, workflow lint, npm packaging tests, and npm smoke. |
| `macos-15` | Bash. | `just test`, npm packaging tests, and npm smoke. |
| `windows-2022` | MSYS2 UCRT64. | `just test`, npm packaging tests, and npm smoke. |

Each npm smoke check stages the debug binary built by tests. It packs the launcher and host platform package with pnpm, then installs both without scripts.
The check starts the launcher through `npx` in an isolated cache. It sends MCP `initialize` and `tools/list` requests and checks that the response includes `search`.

Only Linux runs clippy, so CI does not lint Windows-only code. New pushes cancel older CI runs for the same pull request.

The Linux job runs the pinned `actionlint` container. CI skips Git LFS payload downloads. Tests use fixtures instead of the full database.

With GitHub CLI access to the repository, inspect the failed run:

```sh
gh run view --log-failed
```

Use the failed job and step names to select the corresponding local recipe.

## Update the documentation

Keep documentation with the behavior it describes. Update the applicable guide or reference page when you change a command, setting, tool, or data rule.

Add a Changesets file for each user-visible npm package change. Run `pnpm exec changeset` from the repository root.
Choose the release level and write a user-facing summary. The version workflow generates changelogs for all six packages.
Keep the root [CHANGELOG.md](CHANGELOG.md) as project history. Do not replace it with generated package changelogs.

In your pull request, describe the behavior change and report the checks you ran. State any checks that you did not run.
