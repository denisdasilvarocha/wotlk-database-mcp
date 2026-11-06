# WotLK Database MCP

![WotLK Database MCP: Give AI agents access to WotLK 3.3.5 game data, WoW Lua API, and client offsets.](docs/assets/banner.png)

<p align="center">
  <a href="https://registry.npmjs.org/wotlk-database-mcp/latest"><img src="https://img.shields.io/npm/v/wotlk-database-mcp" alt="npm version"></a>
  <a href="https://api.npmjs.org/downloads/point/last-month/wotlk-database-mcp"><img src="https://img.shields.io/npm/dm/wotlk-database-mcp" alt="Monthly npm downloads"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/built%20with-Rust-orange?logo=rust" alt="Built with Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/npm/l/wotlk-database-mcp" alt="License: MIT"></a>
</p>

WotLK Database MCP is a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server for World of Warcraft: Wrath of the Lich King 3.3.5a (client build 12340). It lets an MCP client search game data, Lua and addon APIs, and client offsets in a read-only SQLite database.

The snapshot combines client DBC records, TrinityCore world data, and vendored Lua API and offset catalogs. Data on a customized server may differ. Use `get` with `{"kind": "meta"}` to see the snapshot's provenance.

## Install

You need Node.js 22 or later and an MCP client that supports [stdio transport](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#stdio). Native packages support macOS and Linux on x64 or ARM64, and Windows on x64. Linux requires glibc.

Run the command for your client:

<details>
<summary>Codex</summary>

```bash
codex mcp add wotlk-database -- npx -y wotlk-database-mcp
```

See the [Codex MCP documentation](https://learn.chatgpt.com/docs/extend/mcp?surface=cli).

</details>

<details>
<summary>Claude Code</summary>

```bash
claude mcp add --transport stdio --scope user wotlk-database -- npx -y wotlk-database-mcp
```

See the [Claude Code MCP documentation](https://code.claude.com/docs/en/mcp).

</details>

<details>
<summary>Cursor</summary>

The Cursor CLI shares its MCP configuration with the editor:

```bash
agent mcp add wotlk-database -- npx -y wotlk-database-mcp
```

See the [Cursor MCP documentation](https://cursor.com/docs/mcp).

</details>

<details>
<summary>Hermes Agent</summary>

```bash
hermes mcp add wotlk-database --command npx --args -y wotlk-database-mcp
```

Follow the prompts to select tools and save the server. See the [Hermes MCP CLI reference](https://hermes-agent.nousresearch.com/docs/reference/cli-commands#hermes-mcp).

</details>

<details>
<summary>OpenClaw</summary>

Stop the local gateway before registering the server, then start it again afterward:

```bash
openclaw mcp add wotlk-database --command npx --arg -y --arg wotlk-database-mcp
```

See the [OpenClaw MCP registry reference](https://docs.openclaw.ai/cli/mcp/registry).

</details>

<details>
<summary>OpenCode</summary>

In OpenCode v2, run this from your project:

```bash
opencode mcp add wotlk-database -- npx -y wotlk-database-mcp
```

Add `--global` before `--` to make the server available in every project. In OpenCode v1, run `opencode mcp add` and follow the prompts for a local server named `wotlk-database` with the command `npx -y wotlk-database-mcp`. See the [OpenCode v1 CLI reference](https://opencode.ai/docs/cli/#mcp) and [v2 MCP documentation](https://dev.opencode.ai/v2/docs/mcp-servers/).

</details>

<details>
<summary>Pi</summary>

Pi [0.99.0](https://pi.dev/changelog/releases/0.99.0) introduced built-in MCP support. With that version or later, run:

```bash
pi mcp add wotlk-database -- npx -y wotlk-database-mcp
```

See the [Pi MCP documentation](https://pi.dev/docs/latest/mcp).

</details>

<details>
<summary>Oh My Pi (OMP)</summary>

Start `omp` and run `/mcp add` in the session. In the setup wizard, name the server `wotlk-database`, choose stdio, enter `npx` as the command and `-y wotlk-database-mcp` as the arguments, and select your preferred scope. See the [OMP MCP configuration guide](https://github.com/c1357/oh-my-pi/blob/main/docs/mcp-config.md).

</details>

<details>
<summary>GitHub Copilot CLI</summary>

```bash
copilot mcp add wotlk-database -- npx -y wotlk-database-mcp
```

See the [GitHub Copilot CLI MCP guide](https://docs.github.com/en/copilot/how-tos/copilot-cli/customize-copilot/add-mcp-servers).

</details>

<details>
<summary>Other clients</summary>

Configure the client to start `npx` with the arguments `-y` and `wotlk-database-mcp`. Some clients accept a JSON configuration like this:

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "npx",
      "args": ["-y", "wotlk-database-mcp"]
    }
  }
}
```

Add the server to any existing configuration without replacing your other servers. Configuration formats and file locations differ by client. See the [npm `npx` reference](https://docs.npmjs.com/cli/v11/commands/npx/).

</details>

See [installation and configuration](docs/install.md) for local databases, source builds, and troubleshooting.

Restart or reconnect your client if needed, then ask: “What is the spell ID for Frostbolt rank 3?” The snapshot identifies it as `837`. If the default database is not cached, the server downloads and verifies it in the background; if a tool reports download progress, retry when the download finishes.

To prepare the database cache before connecting, run:

```bash
npx -y wotlk-database-mcp fetch
```

## Usage

The server exposes four tools:

| Tool | Purpose |
| --- | --- |
| `search` | Find records by name or related search terms. |
| `get` | Retrieve a record and optionally select additional fields. |
| `related` | Follow a supported relationship from a record. |
| `sql` | Run one read-only SQLite statement. |

For example, call `search` with this argument object to find Frostbolt spells:

```json
{"text": "Frostbolt", "kind": "spell"}
```

Call `related` with a spell ID from the results to list its ranks:

```json
{"kind": "spell", "id": 116, "relation": "ranks"}
```

You can also ask your client about item stats, loot, vendors, spawns, quest chains, Lua function signatures, or Windows x86 offsets. Only offsets marked `verified` are confirmed; read their comments for caveats. The [tool reference](docs/tools.md) documents parameters, relationships, output, and data conventions.

## Development

The Rust workspace contains the server and database builder. Install [just](https://just.systems/) and review the platform prerequisites in the [development guide](docs/development.md). Setup may install system packages and Git hooks. Then run:

```bash
just setup
just test
```

The development guide also covers building the database, running the server, and the separate npm packaging tests.

## Documentation

- [Installation and configuration](docs/install.md)
- [Tool reference](docs/tools.md)
- [Development guide](docs/development.md)

## License

MIT. See [LICENSE](LICENSE).
