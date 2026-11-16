#!/usr/bin/env bash
# Usage: ./check.sh starter.sql.md | solution.sql.md   (needs sqlite3; same engine as the in-browser SQLite WASM)
set -u
here="$(cd "$(dirname "$0")" && pwd)"; q="${1:?usage: check.sh QUERY.sql.md}"
got="$(sqlite3 :memory: ".read $here/data.sql.md" ".read $q" 2>&1)"
if [[ "$got" == "$(cat "$here/expected.txt")" ]]; then echo "PASS"; exit 0; fi
echo "FAIL: expected"; cat "$here/expected.txt"; echo "got"; echo "$got"; exit 1
