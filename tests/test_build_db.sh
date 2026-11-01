#!/usr/bin/env bash
set -eu
ROOT=$(cd "$(dirname "$0")/.." && pwd)
. "$ROOT/scripts/build_db.sh"
work=$(mktemp -d "${TMPDIR:-/tmp}/build-db-test.XXXXXX")
trap 'rm -rf "$work"' EXIT
sha() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d ' ' -f 1; else shasum -a 256 "$1" | cut -d ' ' -f 1; fi; }
fail() { echo "FAIL: $*" >&2; exit 1; }
fixture() {
    rm -rf "$work/fixture" "$work/cache"
    mkdir -p "$work/fixture/dbc"
    printf 'fixture SQL' > "$work/fixture/world.sql"
    printf 'fixture DBC' > "$work/fixture/dbc/Spell.dbc"
    bsdtar --format=7zip -cf "$work/world.7z" -C "$work/fixture" world.sql
    bsdtar --format=zip -cf "$work/data.zip" -C "$work/fixture" dbc/Spell.dbc
    TDB_URL="file://$work/world.7z"; TDB_SHA256=$(sha "$work/world.7z"); TDB_MEMBER=world.sql
    DBC_URL="file://$work/data.zip"; DBC_SHA256=$(sha "$work/data.zip"); DBC_MEMBER=dbc
}
fixture
prepare_inputs "$work/cache"
[ "$(cat "$SQL_PATH")" = 'fixture SQL' ] || fail 'cold SQL'
[ "$(cat "$DBC_PATH/Spell.dbc")" = 'fixture DBC' ] || fail 'cold DBC'
rm "$work/world.7z" "$work/data.zip"
prepare_inputs "$work/cache"
[ "$(cat "$SQL_PATH")" = 'fixture SQL' ] || fail 'offline SQL'
[ "$(cat "$DBC_PATH/Spell.dbc")" = 'fixture DBC' ] || fail 'offline DBC'
printf corrupt > "$SQL_PATH"
rm "$DBC_PATH/Spell.dbc"
printf extra > "$DBC_PATH/extra.dbc"
prepare_inputs "$work/cache"
[ "$(cat "$SQL_PATH")" = 'fixture SQL' ] || fail 'repair SQL'
[ "$(cat "$DBC_PATH/Spell.dbc")" = 'fixture DBC' ] || fail 'repair DBC'
[ ! -e "$DBC_PATH/extra.dbc" ] || fail 'remove extra file'
fixture
bad=0000000000000000000000000000000000000000000000000000000000000000
TDB_SHA256=$bad
if (prepare_inputs "$work/cache"); then fail 'accepted bad checksum'; fi
[ ! -e "$work/cache/TDB-$bad.partial" ] || fail 'partial left behind'
[ ! -d "$work/cache/TDB-inputs-$bad" ] || fail 'extracted bad archive'
fixture
prepare_inputs "$work/cache"
printf corrupt > "$work/cache/TDB-$TDB_SHA256"
rm "$SQL_PATH"
prepare_inputs "$work/cache"
[ "$(cat "$SQL_PATH")" = 'fixture SQL' ] || fail 'repair cached archive'
[ "$(sha "$work/cache/TDB-$TDB_SHA256")" = "$TDB_SHA256" ] || fail 'cached archive hash'
fixture
mkdir -p "$work/fixture/maps" "$work/fixture/dbc/nested"
printf map > "$work/fixture/maps/Map.dbc"
printf nested > "$work/fixture/dbc/nested/Other.dbc"
ln -s target "$work/fixture/dbc/Link.dbc"
printf escape > "$work/fixture/dbc\\escape.dbc"
printf injected > "$work/fixture/dbc\\Spell.dbc"
printf traversal > "$work/escape.dbc"
rm "$work/data.zip"
(cd "$work/fixture" && zip -q -y "$work/data.zip" dbc/Spell.dbc dbc/Link.dbc dbc/nested/Other.dbc maps/Map.dbc 'dbc\escape.dbc' 'dbc\Spell.dbc' dbc/../../escape.dbc)
rm "$work/escape.dbc"
DBC_SHA256=$(sha "$work/data.zip")
prepare_inputs "$work/cache"
[ "$(cat "$DBC_PATH/Spell.dbc")" = 'fixture DBC' ] || fail 'selected DBC'
[ "$(find "$DBC_PATH" -type f | wc -l | tr -d ' ')" = 1 ] || fail 'unsafe DBC selected'
[ ! -e "$work/escape.dbc" ] || fail 'escaped extraction'
# ZIP central-directory order differs from local-header order: symlink first in
# the directory, regular file first on disk. Their metadata must not be mixed.
fixture
printf '%b' '\120\113\003\004\024\000\000\000\000\000\000\000\041\120\176\040\046\316\013\000\000\000\013\000\000\000\015\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\146\151\170\164\165\162\145\040\104\102\103\120\113\003\004\024\000\000\000\000\000\000\000\041\120\374\057\157\106\006\000\000\000\006\000\000\000\014\000\000\000\144\142\143\057\114\151\156\153\056\144\142\143\164\141\162\147\145\164\120\113\001\002\024\003\024\000\000\000\000\000\000\000\041\120\374\057\157\106\006\000\000\000\006\000\000\000\014\000\000\000\000\000\000\000\000\000\000\000\377\241\066\000\000\000\144\142\143\057\114\151\156\153\056\144\142\143\120\113\001\002\024\003\024\000\000\000\000\000\000\000\041\120\176\040\046\316\013\000\000\000\013\000\000\000\015\000\000\000\000\000\000\000\000\000\000\000\244\201\000\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\120\113\005\006\000\000\000\000\002\000\002\000\165\000\000\000\146\000\000\000\000\000' > "$work/data.zip"
DBC_SHA256=$(sha "$work/data.zip")
prepare_inputs "$work/cache"
[ "$(cat "$DBC_PATH/Spell.dbc")" = 'fixture DBC' ] || fail 'reordered ZIP regular member'
[ ! -e "$DBC_PATH/Link.dbc" ] || fail 'reordered ZIP symlink selected'
[ "$(find "$DBC_PATH" -type f | wc -l | tr -d ' ')" = 1 ] || fail 'reordered ZIP unexpected file'
# A regular member and a symlink share the same raw ZIP name; streaming by
# name would otherwise concatenate both payloads.
fixture
printf '%b' '\120\113\003\004\024\000\000\000\000\000\000\000\041\120\176\040\046\316\013\000\000\000\013\000\000\000\015\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\146\151\170\164\165\162\145\040\104\102\103\120\113\003\004\024\000\000\000\000\000\000\000\041\120\374\057\157\106\006\000\000\000\006\000\000\000\015\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\164\141\162\147\145\164\120\113\001\002\024\003\024\000\000\000\000\000\000\000\041\120\176\040\046\316\013\000\000\000\013\000\000\000\015\000\000\000\000\000\000\000\000\000\000\000\244\201\000\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\120\113\001\002\024\003\024\000\000\000\000\000\000\000\041\120\374\057\157\106\006\000\000\000\006\000\000\000\015\000\000\000\000\000\000\000\000\000\000\000\377\241\066\000\000\000\144\142\143\057\123\160\145\154\154\056\144\142\143\120\113\005\006\000\000\000\000\002\000\002\000\166\000\000\000\147\000\000\000\000\000' > "$work/data.zip"
DBC_SHA256=$(sha "$work/data.zip")
if (prepare_inputs "$work/cache"); then fail 'accepted duplicate DBC member'; fi
[ ! -e "$work/cache/DBC-inputs-$DBC_SHA256" ] || fail 'published duplicate DBC extraction'
fixture
rm "$work/data.zip"
(cd "$work/fixture" && zip -q "$work/data.zip" dbc/Spell.dbc)
rm "$work/fixture/dbc/Spell.dbc"
printf collision > "$work/fixture/dbc/spell.DBC"
(cd "$work/fixture" && zip -q "$work/data.zip" dbc/spell.DBC)
DBC_SHA256=$(sha "$work/data.zip")
if (prepare_inputs "$work/cache"); then fail 'accepted case-insensitive DBC collision'; fi
[ ! -e "$work/cache/DBC-inputs-$DBC_SHA256" ] || fail 'published colliding DBC extraction'
echo 'Provisioning tests passed'
