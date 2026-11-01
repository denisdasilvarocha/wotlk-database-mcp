#!/usr/bin/env bash
# Pinned inputs for the offline database builder. Source this file to use prepare_inputs.
TDB_URL=${TDB_URL:-https://github.com/TrinityCore/TrinityCore/releases/download/TDB335.26091/TDB_full_world_335.26091_2026_09_09.7z}
TDB_SHA256=${TDB_SHA256:-ed0c50177da912b50c9f386ba8a26254cecfb997ca7fd2e992c25b344e11274f}
TDB_MEMBER=${TDB_MEMBER:-TDB_full_world_335.26091_2026_09_09.sql}
DBC_URL=${DBC_URL:-https://github.com/wowgaming/client-data/releases/download/v20.0/Data.zip}
DBC_SHA256=${DBC_SHA256:-a3d4df635ae6c2c8f08052c32a79e0f806955150ad36b014a823dd08a32a4610}
DBC_MEMBER=${DBC_MEMBER:-dbc}

build_db_checksum() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d ' ' -f 1
    else
        shasum -a 256 "$1" | cut -d ' ' -f 1
    fi
}

build_db_archive() {
    local cache=$1 name=$2 url=$3 digest=$4 archive partial actual
    archive="$cache/$name-$digest"; partial="$archive.partial"
    if [ -f "$archive" ]; then
        echo "Verify cached $name archive" >&2
        actual=$(build_db_checksum "$archive") || return 1
        if [ "$actual" = "$digest" ]; then
            echo "Reuse cached $name archive" >&2
            return 0
        fi
        echo "Cached $name archive is corrupt; downloading again" >&2
        rm -f "$archive" || return 1
    fi
    echo "Download $name: $url" >&2
    if ! curl --fail --location --silent --show-error --output "$partial" "$url"; then
        rm -f "$partial"; return 1
    fi
    actual=$(build_db_checksum "$partial") || { rm -f "$partial"; return 1; }
    if [ "$actual" != "$digest" ]; then
        echo "build-db: $name SHA-256 mismatch; refusing to extract $url" >&2
        rm -f "$partial"; return 1
    fi
    mv -f "$partial" "$archive" || { rm -f "$partial"; return 1; }
}

build_db_valid_extraction() {
    local dir=$1 digest=$2 member=$3 name=$4 entries path hash actual count expected
    [ -f "$dir/manifest.json" ] && [ ! -L "$dir/manifest.json" ] || return 1
    jq -e --arg sha "$digest" --arg member "$member" '.sha256 == $sha and .member == $member and (.files | type == "object" and length > 0) and all(.files[]; type == "string" and test("^[0-9a-f]{64}$"))' "$dir/manifest.json" >/dev/null 2>&1 || return 1
    entries=$(jq -r '.files | to_entries[] | [.key, .value] | @tsv' "$dir/manifest.json") || return 1
    expected=0
    while IFS="$(printf '\t')" read -r path hash; do
        [ -n "$path" ] || return 1
        case "$name:$path" in
            TDB:world.sql) ;;
            DBC:dbc/*) case ${path#dbc/} in ''|*/*|*\\*) return 1;; esac ;;
            *) return 1 ;;
        esac
        [ -f "$dir/$path" ] && [ ! -L "$dir/$path" ] || return 1
        actual=$(build_db_checksum "$dir/$path") || return 1
        [ "$actual" = "$hash" ] || return 1
        expected=$((expected + 1))
    done <<EOF
$entries
EOF
    # Reject unexpected files and symlinks, including dangling links.
    [ -z "$(find "$dir" -type l -print)" ] || return 1
    count=$(find "$dir" -type f -print | wc -l | tr -d ' ') || return 1
    [ "$count" -eq $((expected + 1)) ]
}

build_db_extract_stage() {
    local stage=$1 name=$2 archive=$3 member=$4 digest=$5 listing verbose line info base lower member_lower pattern count files rel hash keys
    if [ "$name" = TDB ]; then
        bsdtar -xOf "$archive" -- "$member" > "$stage/world.sql" || return 1
        [ -s "$stage/world.sql" ] || { echo 'build-db: TDB archive contained an empty SQL dump' >&2; return 1; }
    else
        mkdir "$stage/dbc" || return 1
        listing="$stage/listing"; verbose="$stage/verbose"
        unzip -Z1 "$archive" > "$listing" || return 1
        unzip -Z -l "$archive" > "$verbose" || return 1
        member_lower=$(printf '%s' "$member" | tr '[:upper:]' '[:lower:]') || return 1
        keys="$stage/keys"
        : > "$keys" || return 1
        # unzip -p matches all entries of a given name, including symlinks.
        # Reject collisions before inspecting types or streaming any payload.
        while IFS= read -r line; do
            case "$line" in *\\*) continue ;; */*) base=${line#*/} ;; *) continue ;; esac
            case "$base" in ''|*/*|.|..) continue ;; *.[Dd][Bb][Cc]) ;; *) continue ;; esac
            lower=$(printf '%s' "${line%%/*}" | tr '[:upper:]' '[:lower:]') || return 1
            [ "$lower" = "$member_lower" ] || continue
            case "$base" in *"$(printf '\t')"*|*"$(printf '\r')"*) echo 'build-db: invalid DBC member name' >&2; return 1 ;; esac
            printf '%s\n' "$base" | tr '[:upper:]' '[:lower:]' >> "$keys" || return 1
        done < "$listing"
        if [ -n "$(LC_ALL=C sort "$keys" | uniq -d)" ]; then
            echo 'build-db: duplicate DBC archive member' >&2
            return 1
        fi
        rm -f "$keys" || return 1
        count=0
        exec 3< "$verbose"
        while IFS= read -r line; do
            # ZipInfo lists names and metadata in the same central-directory order.
            # Skip its archive header and summary, not entry records.
            while :; do
                IFS= read -r info <&3 || { exec 3<&-; return 1; }
                case "$info" in [-ld?bcps]*) break ;; esac
            done
            # Type '?' denotes an entry without Unix file-type attributes.
            case "$info" in -*|\?*) ;; *) continue ;; esac
            case "$line" in *\\*) continue ;; esac
            case "$line" in */*) base=${line#*/} ;; *) continue ;; esac
            case "$base" in ''|*/*|.|..) continue ;; *.[Dd][Bb][Cc]) ;; *) continue ;; esac
            lower=$(printf '%s' "${line%%/*}" | tr '[:upper:]' '[:lower:]') || { exec 3<&-; return 1; }
            [ "$lower" = "$member_lower" ] || continue
            [ ! -e "$stage/dbc/$base" ] || { echo "build-db: duplicate DBC archive member: $line" >&2; exec 3<&-; return 1; }
            # Escape unzip's member-pattern metacharacters to select exactly this raw name.
            pattern=${line//\[/\\[}; pattern=${pattern//\*/\\*}; pattern=${pattern//\?/\\?}
            unzip -p "$archive" "$pattern" > "$stage/dbc/$base" || { exec 3<&-; return 1; }
            count=$((count + 1))
        done < "$listing"
        exec 3<&-
        rm -f "$listing" "$verbose" || return 1
        [ "$count" -gt 0 ] || { echo "build-db: DBC archive contains no $member/*.dbc files" >&2; return 1; }
    fi
    files="$stage/files.tsv"
    : > "$files" || return 1
    if [ "$name" = TDB ]; then
        hash=$(build_db_checksum "$stage/world.sql") || return 1
        printf 'world.sql\t%s\n' "$hash" >> "$files"
    else
        for path in "$stage"/dbc/* "$stage"/dbc/.[!.]* "$stage"/dbc/..?*; do
            [ -f "$path" ] || continue
            rel=dbc/${path##*/}
            hash=$(build_db_checksum "$path") || return 1
            printf '%s\t%s\n' "$rel" "$hash" >> "$files" || return 1
        done
    fi
    jq -Rn --arg sha "$digest" --arg member "$member" '{sha256:$sha,member:$member,files:([inputs | split("\t") | {(.[0]): .[1]}] | add)}' < "$files" > "$stage/manifest.json" || return 1
    rm -f "$files"
}

build_db_extract() {
    local cache=$1 name=$2 digest=$3 member=$4 archive=$5 dir stage
    dir="$cache/$name-inputs-$digest"
    if build_db_valid_extraction "$dir" "$digest" "$member" "$name"; then
        echo "Reuse extracted $name inputs" >&2; return 0
    fi
    echo "Extract $name inputs" >&2
    stage=$(mktemp -d "$cache/$name-partial-XXXXXX") || return 1
    if ! build_db_extract_stage "$stage" "$name" "$archive" "$member" "$digest"; then
        rm -rf "$stage"; return 1
    fi
    rm -rf "$dir" || { rm -rf "$stage"; return 1; }
    mv "$stage" "$dir" || { rm -rf "$stage"; return 1; }
}

# Sets SQL_PATH and DBC_PATH; callers may override the source variables before invoking.
prepare_inputs() {
    local cache=$1
    mkdir -p "$cache" || return 1
    build_db_archive "$cache" TDB "$TDB_URL" "$TDB_SHA256" || return 1
    build_db_archive "$cache" DBC "$DBC_URL" "$DBC_SHA256" || return 1
    build_db_extract "$cache" TDB "$TDB_SHA256" "$TDB_MEMBER" "$cache/TDB-$TDB_SHA256" || return 1
    build_db_extract "$cache" DBC "$DBC_SHA256" "$DBC_MEMBER" "$cache/DBC-$DBC_SHA256" || return 1
    SQL_PATH="$cache/TDB-inputs-$TDB_SHA256/world.sql"
    DBC_PATH="$cache/DBC-inputs-$DBC_SHA256/dbc"
}

build_db_main() {
    local root codex program first sql_arg dbc_arg
    root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd) || return 1
    for program in cargo bsdtar curl jq unzip; do
        command -v "$program" >/dev/null 2>&1 || { echo "build-db: $program not found; install it and run just build-db again" >&2; return 1; }
    done
    codex="$root/builder/vendor/api/codex.json"
    first=$(dd if="$codex" bs=43 count=1 2>/dev/null) || return 1
    if [ "$first" = 'version https://git-lfs.github.com/spec/v1' ]; then
        for program in git git-lfs; do
            command -v "$program" >/dev/null 2>&1 || { echo "build-db: $program not found; install Git LFS and run just build-db again" >&2; return 1; }
        done
        echo 'Download vendored API data with Git LFS' >&2
        (cd "$root" && git lfs pull --include=builder/vendor/api/codex.json) || return 1
        first=$(dd if="$codex" bs=43 count=1 2>/dev/null) || return 1
        [ "$first" != 'version https://git-lfs.github.com/spec/v1' ] || { echo 'build-db: Git LFS did not fetch codex.json; check Git LFS access' >&2; return 1; }
    fi
    prepare_inputs "$root/target/build-db" || return 1
    sql_arg=$SQL_PATH; dbc_arg=$DBC_PATH
    if command -v cygpath >/dev/null 2>&1; then
        sql_arg=$(cygpath -w "$sql_arg") || return 1
        dbc_arg=$(cygpath -w "$dbc_arg") || return 1
    fi
    echo 'Build data/wotlk.sqlite' >&2
    (cd "$root" && cargo run --release --locked -p wotlk-db-builder -- --tdb "$sql_arg" --dbc "$dbc_arg")
}

if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    build_db_main "$@"
fi
