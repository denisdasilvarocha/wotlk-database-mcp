#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$ROOT"

die() { echo "release: $*" >&2; exit 1; }

[ "$#" -eq 1 ] || die "usage: just release <level|version> [--execute]"
level=$1

case "$level" in
    alpha|beta|rc|*-*) ;;
    *)
        # Ignore headings and whitespace when deciding whether a release has entries.
        awk '
            /^## \[Unreleased\]$/ { section++; in_section=1; next }
            in_section && /^## \[/ { in_section=0 }
            in_section && /^\[[^]]+\]: / { in_section=0 }
            in_section && $0 !~ /^[[:space:]]*$/ && $0 !~ /^### / { content=1 }
            /^\[Unreleased\]: / { links++ }
            END { exit !(section == 1 && content && links == 1) }
        ' CHANGELOG.md || die "Unreleased section in CHANGELOG.md is empty or has invalid release markers"
        ;;
esac

schema=$(sed -nE 's/^pub const SCHEMA_VERSION: u32 = ([0-9]+);$/\1/p' src/schema_version.rs)
[[ $schema =~ ^[0-9]+$ ]] || die "could not read SCHEMA_VERSION"
db_tag="db-schema$schema"
db_error="database release $db_tag is incomplete; push the database to main or run: gh workflow run release-database.yml --ref main"
assets=$(gh release view "$db_tag" --json assets --jq '.assets[].name') || die "$db_error"
for asset in "wotlk-$db_tag.sqlite.zst" "wotlk-$db_tag.sqlite.sha256"; do
    printf '%s\n' "$assets" | grep -Fxq "$asset" || die "$db_error"
done

for name in wotlk-database-mcp wotlk-database-mcp-darwin-arm64 wotlk-database-mcp-darwin-x64 wotlk-database-mcp-linux-arm64 wotlk-database-mcp-linux-x64 wotlk-database-mcp-win32-x64; do
    npm view "$name" name >/dev/null 2>&1 || die "npm package $name does not exist; do the one-time npm setup in docs/guides/release.md"
done

sha=$(git rev-parse HEAD)
ci=$(gh run list --workflow ci.yml --commit "$sha" --json conclusion --jq 'any(.[]; .conclusion == "success")') || die "CI has not passed on $sha; push it and wait for the CI workflow"
[ "$ci" = true ] || die "CI has not passed on $sha; push it and wait for the CI workflow"
