#!/usr/bin/env bash
# Validate or refresh the bundled Windows x86 offset catalog.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
VENDOR="$ROOT/builder/vendor/offsets"
destination="$VENDOR/windows-x86"
files=(caches.md client-db.md lua.md movement-world.md network.md objects.md
       spells.md system-globals.md ui-game.md ui-widgets.md units-players.md warden.md)

validate() {
    local directory=$1 name
    for name in "${files[@]}"; do
        if [ ! -f "$directory/$name" ] || [ ! -s "$directory/$name" ]; then
            echo "vendor-offsets: missing or empty offsets file: $directory/$name" >&2
            return 1
        fi
    done
}

source_dir=${1:-}
if [ -z "$source_dir" ]; then
    validate "$destination"
    echo "Using bundled Windows x86 offsets"
    exit 0
fi

validate "$source_dir"
case $source_dir in
    -*) source_dir="./$source_dir" ;;
esac
stage=$(mktemp -d "$VENDOR/.offsets-XXXXXXXX")
cleanup() { rm -rf "$stage"; }
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

# Complete every copy before replacing any catalog file.
for name in "${files[@]}"; do
    cp "$source_dir/$name" "$stage/$name"
done
date +%Y-%m-%d > "$stage/VERSION"
mkdir -p "$destination"
for name in "${files[@]}"; do
    mv -f "$stage/$name" "$destination/$name"
done
mv -f "$stage/VERSION" "$VENDOR/VERSION"
