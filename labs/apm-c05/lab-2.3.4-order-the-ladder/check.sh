#!/usr/bin/env bash
# Usage: ./check.sh starter.sql.md | solution.sql.md   (needs sqlite3; same engine as the in-browser SQLite WASM)
# Runs the seed, then the given file, then the check, in a database held in memory. Exit 0 only when the check prints PASS.
set -u
here="$(cd "$(dirname "$0")" && pwd)"; q="${1:?usage: check.sh ANSWER.sql.md}"
got="$(sqlite3 :memory: ".read $here/seed.sql.md" ".read $q" ".read $here/check.sql.md" 2>&1)"
echo "$got"
[[ "$got" == "PASS" ]]
