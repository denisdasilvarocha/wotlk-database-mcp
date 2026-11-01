#!/usr/bin/env bash
# Export the pinned API checkout without damaging the current export on failure.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
VENDOR="$ROOT/builder/vendor/api"
CHECKOUT="$ROOT/target/api"
URL="https://github.com/Shard-MW/milkyway-codex.git"

requested=${1:-}
if [ -z "$requested" ]; then
    requested=$(cat "$VENDOR/COMMIT")
    requested="${requested#"${requested%%[!$' \t\r\n\v\f']*}"}"
    requested="${requested%"${requested##*[!$' \t\r\n\v\f']}"}"
fi
if [ -z "$requested" ] || [[ "$requested" == -* ]]; then
    echo "vendor-api: invalid API revision" >&2
    exit 1
fi

mkdir -p "$(dirname "$CHECKOUT")"
if [ ! -e "$CHECKOUT" ]; then
    git clone --quiet --no-checkout "$URL" "$CHECKOUT"
fi
if ! git -C "$CHECKOUT" cat-file -e "${requested}^{commit}" 2>/dev/null; then
    git -C "$CHECKOUT" fetch --quiet origin "$requested"
fi
commit=$(git -C "$CHECKOUT" rev-parse --verify "${requested}^{commit}")
git -C "$CHECKOUT" checkout --quiet --detach "$commit"

stage=$(mktemp -d "$VENDOR/.codex-XXXXXXXX")
cleanup() { rm -rf "$stage"; }
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

# The exporter must write a fresh file; a failed or empty export cannot reuse old data.
(cd "$ROOT" && bun "$VENDOR/export.ts" "$CHECKOUT") > "$stage/codex.json"
if ! jq -e 'type == "object" and
    ([.functions, .events, .widgets, .cvars, .types, .combatlog_prefixes,
      .combatlog_events, .secure_templates] | all(type == "array"))' \
    "$stage/codex.json" > /dev/null; then
    echo "vendor-api: API exporter returned incomplete data" >&2
    exit 1
fi
printf '%s\n' "$commit" > "$stage/COMMIT"
mv -f "$stage/codex.json" "$VENDOR/codex.json"
mv -f "$stage/COMMIT" "$VENDOR/COMMIT"
