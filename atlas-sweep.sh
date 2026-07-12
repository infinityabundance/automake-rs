#!/bin/sh
# GNU-free-vs-GNU-oracle atlas differential sweep launcher.
# Runs the PREBUILT xtask binary directly (cargo is intentionally NOT on PATH so every child
# build sees only the GNU oracle at /usr/bin, never a stray toolchain). One repo at a time.
#
# Usage: ./atlas-sweep.sh <corpus-list> [out-dir]
# Env you may override: ATLAS_MEM_LIMIT_MB (default 8192, per-repo RLIMIT_AS memory catch),
#                       ATLAS_SCAN_ONLY (stop after generating configure).
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
LIST="${1:?usage: atlas-sweep.sh <corpus-list> [out-dir]}"
OUT="${2:-atlas/recipes}"

exec env -i \
  HOME="$HOME" \
  PATH=/usr/bin:/bin \
  ATLAS_ORACLE=1 \
  ATLAS_MEM_LIMIT_MB="${ATLAS_MEM_LIMIT_MB:-8192}" \
  ${ATLAS_SCAN_ONLY:+ATLAS_SCAN_ONLY="$ATLAS_SCAN_ONLY"} \
  AUTOCONF_RS=/home/one/autoconf-rs/target/release/autoconf \
  AUTOHEADER_RS=/home/one/autoconf-rs/target/release/autoheader \
  ACLOCAL_RS=/home/one/automake-rs/target/release/aclocal \
  AUTOMAKE_RS=/home/one/automake-rs/target/release/automake \
  AUTORECONF_RS=/home/one/automake-rs/target/release/autoreconf-rs \
  "$HERE/target/release/xtask" atlas "$LIST" "$OUT"
