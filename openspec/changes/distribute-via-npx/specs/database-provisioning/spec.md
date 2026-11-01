# Spec Delta

## Purpose

Defines how the server finds, downloads, verifies, caches, and version-checks its SQLite database, and how tools behave until the database is ready.

## ADDED Requirements

### Requirement: Database path precedence
The server SHALL choose the database in this order: the `--db <path>` argument, then the `WOTLK_DB` environment variable, then the cached database for its schema version. An explicit path SHALL never trigger a download. The server SHALL NOT use a path that was fixed at compile time.

#### Scenario: Explicit path wins
- **WHEN** the server starts with `--db /x/wotlk.sqlite` and `WOTLK_DB` is also set
- **THEN** the server uses `/x/wotlk.sqlite` and does not download anything

#### Scenario: Explicit path missing
- **WHEN** the server starts with `WOTLK_DB` set to a file that does not exist
- **THEN** the server exits with an error that names the path

#### Scenario: Valid cache used
- **WHEN** no explicit path is given and a verified database for the current schema version is in the cache
- **THEN** the server uses it and does not download anything

### Requirement: Cache location
The cached database SHALL be stored in a directory for its schema version under the platform user cache directory: `$XDG_CACHE_HOME` or `~/.cache` on Linux, `~/Library/Caches` on macOS, and `%LOCALAPPDATA%` on Windows.

#### Scenario: Version folders
- **WHEN** a binary with schema version 2 downloads the database
- **THEN** the file is stored in a version-2 folder, and a version-1 folder is not used

### Requirement: Handshake before download
When the server needs a download, it SHALL finish the MCP handshake without waiting for the download, and download the database in the background.

#### Scenario: First run
- **WHEN** the server starts with an empty cache and the client sends `initialize`
- **THEN** the server answers `initialize` and lists its tools before the download ends

### Requirement: Tools during download
While the database is not ready, every tool call SHALL return a tool error that says the database is downloading, gives progress as a percentage, and tells the agent to retry. After the database is ready, tool calls SHALL work without a server restart.

#### Scenario: Call during download
- **WHEN** a tool is called while the download is at 40%
- **THEN** the call returns an error that contains "downloading" and "40%"

#### Scenario: Call after download
- **WHEN** a tool is called after the download is verified
- **THEN** the call returns normal results

### Requirement: Download failure reporting
If the download or the verification fails, every later tool call SHALL return an error that gives the cause, the asset URL, and the `fetch` command to retry by hand.

#### Scenario: Network failure
- **WHEN** the download fails because the host cannot be reached
- **THEN** a tool call returns an error with the cause, the URL, and `npx wotlk-database-mcp fetch`

### Requirement: Integrity and atomic install
The server SHALL check the downloaded data against the published sha256 checksum and SHALL put the database in the cache only after the check passes. A partial or corrupt file SHALL never appear at the final cache path.

#### Scenario: Checksum mismatch
- **WHEN** the downloaded data does not match the checksum
- **THEN** the cache stays without a database and tool calls return a checksum error

#### Scenario: Interrupted download
- **WHEN** the process stops during the download
- **THEN** the next start does not find a database at the final path and downloads again

### Requirement: Concurrent starts
When several server processes need the same cached database at the same time, only one process SHALL download it. The other processes SHALL wait for that download and then use the result.

#### Scenario: Two servers on first run
- **WHEN** two servers start at the same time with an empty cache
- **THEN** the asset is downloaded once, and both servers serve tool calls after it is installed

### Requirement: Schema version check
The database SHALL store its schema version as `meta.schema_version`. The server SHALL refuse a database whose schema version does not match its own, and the error SHALL give both versions.

#### Scenario: Old explicit database
- **WHEN** `--db` points to a database without the binary's schema version
- **THEN** the server exits with an error that gives the expected and found versions

### Requirement: Proxy support
Downloads SHALL use the proxy set in `HTTPS_PROXY` or `https_proxy` when one is set.

#### Scenario: Proxy configured
- **WHEN** `HTTPS_PROXY` is set and the server downloads the database
- **THEN** the request goes through that proxy

### Requirement: Fetch subcommand
`wotlk-database-mcp fetch` SHALL download and verify the database into the cache, show progress on stderr, and exit with code 0 on success and non-zero on failure. If a valid database is already cached, it SHALL exit with code 0 and not download anything.

#### Scenario: Prefetch in CI
- **WHEN** a user runs `npx -y wotlk-database-mcp fetch` with an empty cache
- **THEN** the database is in the cache and the command exits with code 0

#### Scenario: Already cached
- **WHEN** `fetch` runs and a valid database is cached
- **THEN** the command exits with code 0 and does not download anything
