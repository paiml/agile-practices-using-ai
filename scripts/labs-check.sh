#!/usr/bin/env bash
# labs-check.sh [LABS_DIR]  -> exit 0 every lab proves itself, 1 a lab is RED, 2 a tool or the directory is missing.
# A lab is labs/<course>/<lab>/. It must carry check.sh, exactly one solution.*, one starter.* and at least one wrong*.
# Proof: check.sh on the solution exits 0; on the starter and on EVERY wrong file it exits non-zero. A lab with no
# wrong file is RED: a check that was never shown to refuse an answer proves nothing.
set -uo pipefail
R=${1:-$(cd "$(dirname "$0")/.." && pwd)/labs}; bad=0; n=0
fail() { echo "RED $*"; bad=1; }
command -v sqlite3 >/dev/null || { echo "REFUSE: sqlite3 not found"; exit 2; }
[ -d "$R" ] || { echo "REFUSE: no directory $R"; exit 2; }
for d in "$R"/*/*/; do
  d=${d%/}; [ -d "$d" ] || continue; n=$((n+1)); l=${d#"$R"/}
  [ -f "$d/check.sh" ] || { fail "$l: no check.sh"; continue; }
  sol=("$d"/solution.*); st=("$d"/starter.*)
  { [ "${#sol[@]}" -eq 1 ] && [ -f "${sol[0]}" ]; } || { fail "$l: needs exactly one solution.*"; continue; }
  { [ "${#st[@]}" -eq 1 ] && [ -f "${st[0]}" ]; } || { fail "$l: needs exactly one starter.*"; continue; }
  bash "$d/check.sh" "${sol[0]}" >/dev/null 2>&1 || fail "$l: the solution does not PASS"
  if bash "$d/check.sh" "${st[0]}" >/dev/null 2>&1; then fail "$l: the starter PASSES, it must FAIL"; fi
  w=0
  for x in "$d"/wrong*; do
    [ -f "$x" ] || continue; w=$((w+1))
    if bash "$d/check.sh" "$x" >/dev/null 2>&1; then fail "$l: $(basename "$x") PASSES, it must FAIL"; fi
  done
  [ "$w" -ge 1 ] || fail "$l: no wrong file"
done
[ "$n" -ge 1 ] || { echo "RED no lab under $R"; exit 1; }
[ "$bad" = 0 ] || exit 1
echo "labs OK: $n labs, each solution PASSES, each starter and wrong file FAILS"
