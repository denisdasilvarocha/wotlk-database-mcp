# wotlk-database-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) server for the World of Warcraft:
Wrath of the Lich King database, built with the official Rust SDK
[`rmcp`](https://github.com/modelcontextprotocol/rust-sdk).

## Build

```sh
cargo build --release
```

`rustup` installs the pinned toolchain from `rust-toolchain.toml` on the first build.

## Use with an MCP client

The server speaks MCP over stdio. Add it to your client configuration:

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "/absolute/path/to/target/release/wotlk-database-mcp"
    }
  }
}
```

Logs go to stderr. Set the level with `RUST_LOG`, for example `RUST_LOG=debug`.

## Tools

| Tool   | Description                                                      |
| ------ | ---------------------------------------------------------------- |
| `echo` | Returns the given text unchanged. Checks that the server responds. |

## Development

Install the tools, then the git hooks:

```sh
brew install just prek taplo cargo-deny cargo-shear
just setup
```

Run `just` to list all commands. `just check` runs every quality gate that CI runs.
See [AGENTS.md](AGENTS.md) for the project rules.
