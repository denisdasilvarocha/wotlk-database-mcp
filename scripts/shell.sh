#!/usr/bin/env bash
set -eu
# just runs recipe lines through this wrapper on all platforms.
# shellcheck source=scripts/env.sh
source "$(dirname "$0")/env.sh"
exec bash -euc "$1"
