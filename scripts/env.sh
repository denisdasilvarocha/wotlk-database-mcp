#!/usr/bin/env bash
# Source this file to make per-user tools available without changing shell profiles.
case $(uname -s) in
  MSYS*|MINGW*|CYGWIN*)
    if [[ -z ${USERPROFILE:-} ]] || ! command -v cygpath >/dev/null; then
      echo 'Windows requires MSYS2 Bash with USERPROFILE and cygpath available.' >&2
      return 1
    fi
    _cargo_bin=$(cygpath -u "${CARGO_HOME:-$USERPROFILE/.cargo}")/bin
    _bun_bin=$(cygpath -u "${BUN_INSTALL:-$USERPROFILE/.bun}")/bin
    PATH="$PATH:/ucrt64/bin"
    ;;
  *)
    _cargo_bin=${CARGO_HOME:-$HOME/.cargo}/bin
    _bun_bin=${BUN_INSTALL:-$HOME/.bun}/bin
    ;;
esac
export PATH="$_cargo_bin:$_bun_bin:$PATH"
unset _cargo_bin _bun_bin
