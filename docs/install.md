# Install for an MCP harness

Build the server and connect it to your MCP harness. This guide uses a source checkout and a stdio connection.

## 1. Install prerequisites

Install the base tools for your platform before you run `just setup`.

| Platform | Requirements |
| --- | --- |
| macOS | Homebrew and Xcode Command Line Tools (`xcode-select --install`). |
| Linux | `apt-get`, `dnf`, or `pacman`, and `sudo` or a root shell. WSL counts as Linux and builds a Linux binary. |
| Windows (x86_64) | MSYS2 with its UCRT64 terminal, Windows PowerShell, and Microsoft C++ Build Tools. Select the "Desktop development with C++" workload and a Windows SDK. Git Bash, cmd, and PowerShell alone are not supported. |

Install `just` with the command for your platform.

| Platform | Command |
| --- | --- |
| macOS | `brew install just` |
| Debian, Ubuntu | `sudo apt install just` |
| Fedora | `sudo dnf install just` |
| Arch Linux | `sudo pacman -S just` |
| Windows (MSYS2 UCRT64) | `pacman -S just` |

Use Bash for the commands below. On Windows, use the MSYS2 UCRT64 terminal. You need access to the private repository and a network connection. On Linux, setup can ask for `sudo`.

## 2. Clone and build

Clone the repository and run the setup and build recipes.

> [!IMPORTANT]
> The first build downloads game data. Read the [download warning](guides/rebuild-database.md) before you start.

```sh
git clone https://github.com/denisdasilvarocha/wotlk-database-mcp.git
cd wotlk-database-mcp
just setup
just build
```

The build creates these files:

| File | Purpose |
| --- | --- |
| `target/release/wotlk-database-mcp` | Server binary. Windows adds the `.exe` extension. |
| `data/wotlk.sqlite` | Database for the server. |

For setup details and development recipes, see [Contributing](../CONTRIBUTING.md).

## 3. Register the server

Add this entry under `mcpServers` in your harness configuration. Keep the existing entries.

```json
{
  "mcpServers": {
    "wotlk-database": {
      "command": "/absolute/path/to/wotlk-database-mcp/target/release/wotlk-database-mcp",
      "args": ["--db", "/absolute/path/to/wotlk-database-mcp/data/wotlk.sqlite"]
    }
  }
}
```

Replace both example paths with absolute paths into your checkout. On Windows, use the `.exe` binary. Write Windows paths with forward slashes, such as `C:/src/wotlk-database-mcp/data/wotlk.sqlite`.

If your harness uses another format, set its command and arguments to the same values. Select stdio transport. For database path settings and precedence, see the [configuration reference](reference/configuration.md).

## 4. Verify the connection

Use your harness to check that the server answers tool calls.

1. Reload the harness configuration or reconnect the server.
2. Confirm that the harness lists `search`, `get`, `related`, and `sql`.
3. Call `sql` with these arguments:

   ```json
   {"query": "SELECT 1 AS ok"}
   ```

The result must contain this header and row:

```text
ok
1
```

## Troubleshoot the connection

Read the server's stderr when the harness cannot connect.

### `database file not found`

The selected database path does not name a file. Check the [database path configuration](reference/configuration.md). If the build did not create the database, follow [Rebuild the database](guides/rebuild-database.md). Reconnect the server and repeat the verification call.

### Missing binary

Check that the harness command names the release binary in your checkout. On Windows, check the `.exe` extension. If the binary does not exist, run `just build` from the repository root. Resolve any build error before you reconnect the server. Then repeat the verification call.

## Related pages

Use these pages for the next task.

- [Rebuild the database](guides/rebuild-database.md) after an input or search-index update.
- [Tools reference](reference/tools.md) for tool arguments and output.
- [Documentation index](index.md) for all documentation routes.
