# Single entry point for every development command. Git hooks and CI call these recipes.

# List all recipes.
_default:
    @just --list

# Install the git hooks (pre-commit and pre-push).
setup:
    prek install

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
    cargo clippy --all-targets --all-features --locked -- -D warnings

# Apply automatic clippy fixes, then format.
fix:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features
    just fmt

# Run unit, integration, and doc tests.
test:
    cargo test --all-features --locked

# Build the API docs. Any rustdoc warning fails.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked

# Check advisories, licenses, sources, and unused dependencies.
deps:
    cargo deny --locked check
    cargo shear --deny-warnings --locked

# Run every gate (file hygiene, fmt, lint, test, doc, deps). CI runs this recipe.
check:
    prek run --all-files --hook-stage manual

# Run the server on stdio.
run:
    cargo run --locked

# Open the server in the MCP Inspector (needs Node.js).
inspect:
    npx @modelcontextprotocol/inspector cargo run --quiet --locked
