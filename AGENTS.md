# AGENTS.md

Instructions for AI coding agents in this repository. Read this file before you change code.

## Project

`wotlk-database-mcp` is a [Model Context Protocol](https://modelcontextprotocol.io) server for the
World of Warcraft: Wrath of the Lich King database. It uses the official Rust SDK
[`rmcp`](https://github.com/modelcontextprotocol/rust-sdk) and serves MCP over stdio.

## Layout

| Path                      | Purpose                                                                 |
| ------------------------- | ----------------------------------------------------------------------- |
| `src/main.rs`             | Binary entry point: logging setup, stdio transport. Keep it thin.       |
| `src/lib.rs`              | Library root: module declarations and public re-exports.                |
| `src/server.rs`           | `WotlkServer`: MCP tools (`#[tool_router]`) and `ServerHandler`.        |
| `tests/server.rs`         | End-to-end tests: real MCP client against the server, in memory.        |
| `justfile`                | All development commands. Hooks and CI call these recipes.             |
| `.pre-commit-config.yaml` | Git hooks (run by `prek`).                                              |
| `Cargo.toml` `[lints]`    | Compiler and clippy lint levels.                                        |
| `clippy.toml`             | Clippy settings (test exemptions, accepted proper nouns).               |
| `rustfmt.toml`            | Formatter settings (stable options only).                               |
| `.taplo.toml`             | TOML formatter settings.                                                |
| `deny.toml`               | Allowed licenses, advisories, and dependency sources.                   |
| `rust-toolchain.toml`     | Pinned Rust toolchain. `rustup` installs it automatically.              |

## Commands

Run every command from the repository root.

| Command          | Use                                                                          |
| ---------------- | ---------------------------------------------------------------------------- |
| `just fmt`       | Format Rust and TOML files. Run after every edit.                            |
| `just lint`      | Clippy with `-D warnings`.                                                   |
| `just fix`       | Apply automatic clippy fixes, then format.                                   |
| `just test`      | All tests. Filter one test: `cargo test --all-features <name>`.              |
| `just doc`       | Build docs with `-D warnings`.                                               |
| `just deps`      | `cargo deny` (advisories, licenses, sources) and `cargo shear` (unused deps). |
| `just check`     | Every gate above plus file hygiene. CI runs exactly this.                    |
| `just run`       | Run the server on stdio.                                                     |
| `just inspect`   | Open the server in the MCP Inspector (needs Node.js).                        |

Required tools: `rustup`, `just`, `prek`, `taplo`, `cargo-deny`, `cargo-shear`.
On macOS: `brew install just prek taplo cargo-deny cargo-shear`. Then run `just setup` once.

## Definition of done

1. `just check` passes with no changes to the working tree.
2. New behavior has a test in `tests/` that drives it through an MCP client.
3. `README.md` and this file still describe the project correctly.

Never bypass a gate. Do not use `git commit --no-verify` or `git push --no-verify`.
Do not lower a lint level, add an advisory ignore, or allow a new license to make a check pass.
If a gate is wrong, stop and report it to the user.

## Rules

### Protocol safety

- stdout carries MCP JSON-RPC frames. Any other byte on stdout breaks the client.
- Never use `println!`, `print!`, `eprintln!`, or `dbg!`. Clippy rejects them.
- Log with `tracing` (`tracing::info!`, `tracing::debug!`, …). Logs go to stderr.
- Set the log level with `RUST_LOG`, for example `RUST_LOG=debug just run`.

### Errors

- Never use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` outside tests. Clippy rejects them.
- Return `Result` and propagate with `?`. `anyhow` is for `main.rs` and tests only.
- In a tool, choose the failure type by who must act:
  - The tool ran but failed (no rows, bad lookup): return `Ok(CallToolResult::error(...))`. The model sees the message.
  - The request is invalid or cannot be routed: return `Err(rmcp::ErrorData::invalid_params(...))`.

### Lint exceptions

- Use `#[expect(lint, reason = "...")]`. Never use `#[allow(...)]`: clippy rejects it.
- Put the exception on the smallest item possible. The reason must state why the lint does not apply.
- `unsafe` code is forbidden.

### Code style

- `rustfmt` and `taplo` decide formatting. Do not format by hand.
- Every public item needs a `///` doc comment (`missing_docs`). Doc comments on tool parameter fields
  become the JSON Schema descriptions that the model reads, so write them for the model.
- Keep `pub` items reachable from `src/lib.rs` (`unreachable_pub`). Use `pub(crate)` for internals.
- Comments explain why, not what.

### Dependencies

- Add dependencies with `cargo add <crate>`. Do not edit versions by hand.
- Prefer features of existing dependencies. `rmcp` re-exports `schemars` and `serde_json`.
- `just deps` must pass: new licenses need user approval, and unused dependencies fail the build.

## Add a tool

1. Define a parameter struct in `src/server.rs`. Derive `Debug`, `serde::Deserialize`, and
   `schemars::JsonSchema`. Document every field.
2. Add a method in the `#[tool_router] impl WotlkServer` block:

   ```rust
   #[tool(description = "One sentence that tells the model when to call this tool.")]
   async fn find_item(&self, Parameters(args): Parameters<FindItemParams>) -> Result<CallToolResult, ErrorData> {
       // ...
   }
   ```

   Return `String` for plain text, `Json<T>` for structured output, or `Result<CallToolResult, ErrorData>`
   when the tool can fail.
3. Add a test in `tests/server.rs` that calls the tool with `client.call_tool(...)`.
4. Run `just fmt` and `just check`.

The `#[tool_handler]` attribute on `impl ServerHandler for WotlkServer` sets the server name and the
`instructions` text that clients show to the model. Update `instructions` when the tool set changes.

## rmcp reference

- Version: see `Cargo.toml`. Docs: <https://docs.rs/rmcp>. Examples:
  <https://github.com/modelcontextprotocol/rust-sdk/tree/main/examples/servers>.
- Read the docs for the installed version before you use an API. Do not guess from memory: `rmcp`
  changes between major versions.
