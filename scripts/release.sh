#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$ROOT"

die() { echo "release: $*" >&2; exit 1; }

[ "$#" -eq 1 ] || die "usage: just release <version>"
version=$1
semver='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-((0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)(\.(0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*))*))?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$'
[[ $version =~ $semver ]] || die "invalid semantic version: $version"
tag="v$version"
[ "$(git branch --show-current)" = main ] || die "switch to main first"
[ -z "$(git status --porcelain)" ] || die "working tree is not clean"
git fetch origin || die "could not fetch origin"
[ "$(git rev-parse HEAD)" = "$(git rev-parse refs/remotes/origin/main)" ] || die "local main must match origin/main"
if git show-ref --verify --quiet "refs/tags/$tag"; then
    die "tag $tag already exists locally"
fi
remote_tags=$(git ls-remote --tags origin "refs/tags/$tag") || die "could not check remote tags"
[ -z "$remote_tags" ] || die "tag $tag already exists on origin"

schema=$(sed -nE 's/^pub const SCHEMA_VERSION: u32 = ([0-9]+);$/\1/p' src/schema_version.rs)
[[ $schema =~ ^[0-9]+$ ]] || die "could not read SCHEMA_VERSION"
db_tag="db-schema$schema"
assets=$(gh release view "$db_tag" --json assets --jq '.assets[].name') || die "database release $db_tag is missing; run Release database from main first"
for asset in "wotlk-$db_tag.sqlite.zst" "wotlk-$db_tag.sqlite.sha256"; do
    printf '%s\n' "$assets" | grep -Fxq "$asset" || die "database asset $asset is missing; run Release database from main first"
done

prerelease=${version%%+*}
if [[ $prerelease != *-* ]]; then
    # Ignore headings and whitespace when deciding whether a release has entries.
    awk '
        /^## \[Unreleased\]$/ { section++; in_section=1; next }
        in_section && /^## \[/ { in_section=0 }
        in_section && /^\[[^]]+\]: / { in_section=0 }
        in_section && $0 !~ /^[[:space:]]*$/ && $0 !~ /^### / { content=1 }
        /^\[Unreleased\]: / { links++ }
        END { exit !(section == 1 && content && links == 1) }
    ' CHANGELOG.md || die "Unreleased section is empty or CHANGELOG.md has invalid release markers"
fi

# Prepare both edits before replacing either file.
stage=$(mktemp -d "${TMPDIR:-/tmp}/wotlk-release.XXXXXXXX")
trap 'rm -rf "$stage"' EXIT
awk -v version="$version" '
    /^\[/ { in_package=($0 == "[package]") }
    in_package && /^version[[:space:]]*=/ && !changed { print "version = \"" version "\""; changed=1; next }
    { print }
    END { if (!changed) exit 1 }
' Cargo.toml > "$stage/Cargo.toml" || die "could not find package version"

if [[ $prerelease != *-* ]]; then
    date_utc=$(date -u +%F)
    awk -v version="$version" -v date="$date_utc" '
        /^## \[Unreleased\]$/ {
            print; print ""; print "## [" version "] - " date; next
        }
        /^\[Unreleased\]: / {
            base="https://github.com/denisdasilvarocha/wotlk-database-mcp"
            print "[Unreleased]: " base "/compare/v" version "...HEAD"
            print "[" version "]: " base "/releases/tag/v" version
            next
        }
        { print }
    ' CHANGELOG.md > "$stage/CHANGELOG.md"
fi

cp "$stage/Cargo.toml" Cargo.toml
if [[ $prerelease != *-* ]]; then
    cp "$stage/CHANGELOG.md" CHANGELOG.md
fi
cargo update --workspace --offline
git add Cargo.toml Cargo.lock
if [[ $prerelease != *-* ]]; then git add CHANGELOG.md; fi
git commit -m "chore(release): $version"
git tag -a "$tag" -m "$tag"
printf '\nPush the release with:\n  git push --atomic origin main %s\n' "$tag"
