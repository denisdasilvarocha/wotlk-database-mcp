# Spec Delta

## Purpose

Defines how users install and start the MCP server through npm and `npx` on macOS, Linux, and Windows without building it from source.

## ADDED Requirements

### Requirement: Start through npx
The npm package `wotlk-database-mcp` SHALL start the MCP server over stdio when a client runs `npx -y wotlk-database-mcp`. All arguments SHALL go to the server unchanged, and the exit code of the server SHALL become the exit code of the command.

#### Scenario: Client config
- **WHEN** an MCP client uses `"command": "npx", "args": ["-y", "wotlk-database-mcp"]`
- **THEN** the client connects and lists the server tools

#### Scenario: Arguments forwarded
- **WHEN** a user runs `npx -y wotlk-database-mcp --db /x/wotlk.sqlite`
- **THEN** the server uses `/x/wotlk.sqlite`

### Requirement: Supported platforms
The package SHALL provide a native binary for darwin-arm64, darwin-x64, linux-x64, linux-arm64, and win32-x64. Installation SHALL NOT need a Rust toolchain or an install script.

#### Scenario: Install with scripts disabled
- **WHEN** a user installs the package with `--ignore-scripts` on a supported platform
- **THEN** the server starts

#### Scenario: Unsupported platform
- **WHEN** the package runs on a platform without a binary
- **THEN** the command exits non-zero with a message that names the platform and the supported platforms

### Requirement: Small package
The npm packages SHALL NOT contain the database file.

#### Scenario: Package contents
- **WHEN** a release is packed for npm
- **THEN** no package contains a `.sqlite` file

### Requirement: Version alignment
The npm package version SHALL equal the server version. Each release SHALL publish the database asset that matches the schema version of its binary.

#### Scenario: Release from a tag
- **WHEN** tag `v0.2.0` is released
- **THEN** npm has `wotlk-database-mcp@0.2.0`, and its binary downloads a database asset for its own schema version
