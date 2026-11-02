# Single entry point for every development command. Git hooks and CI call these recipes.

set shell := ["bash", "scripts/shell.sh"]

# List all recipes.
_default:
    @just --list

# Install missing tools and the git hooks.
setup:
    bash scripts/setup.sh

# Format Rust and TOML files in place.
fmt:
    cargo fmt --all
    RUST_LOG=warn taplo fmt

# Check formatting without changing files.
fmt-check:
    cargo fmt --all -- --check
    RUST_LOG=warn taplo fmt --check

# Run clippy. Any warning fails.
lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# Apply automatic clippy fixes, then format.
fix:
    cargo clippy --workspace --fix --allow-dirty --allow-staged --all-targets --all-features
    just fmt

# Run unit, integration, and doc tests.
test:
    cargo test --workspace --all-features --locked
    bash tests/test_build_db.sh

# Build the API docs. Any rustdoc warning fails.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked

# Check advisories, licenses, sources, and unused dependencies.
deps:
    cargo deny --locked check
    cargo shear --deny-warnings --locked

# Run every gate (file hygiene, fmt, lint, test, doc, deps). CI runs this recipe.
check:
    prek run --all-files --hook-stage manual

# Run the server on stdio.
run:
    cargo run --locked -- --db data/wotlk.sqlite

# Open the server in the MCP Inspector (needs Node.js).
inspect:
    npx @modelcontextprotocol/inspector cargo run --quiet --locked -- --db data/wotlk.sqlite

# Prepare vendored inputs, build the database, and compile the release server.
build: vendor-offsets vendor-api build-db
    cargo build --release --locked

# Download pinned inputs and build data/wotlk.sqlite.
build-db:
    bash scripts/build_db.sh

# Check release prerequisites and preview the release. Add --execute to commit, tag, and push.
release level *flags:
    bash scripts/release-preflight.sh {{quote(level)}}
    cargo release {{quote(level)}} {{flags}}

# Validate the bundled offsets, or refresh them from a directory.
vendor-offsets src="":
    bash scripts/vendor_offsets.sh {{quote(src)}}

# Export the pinned Lua API data, optionally updating the commit.
vendor-api commit="":
    bash scripts/vendor_api.sh {{quote(commit)}}
