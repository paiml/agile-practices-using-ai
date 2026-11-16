#!/usr/bin/env bash
# interactive-check.sh LABS_DIR COURSE...  -> exit 0 every in-page lab proves itself, 1 a lab is RED, 2 a tool or directory is missing.
# The course page runs ONE SQL statement per code block on SQLite 3.7.17 (no WITH, no window functions, nothing kept
# between runs). So each lab carries interactive.sql.md (what the learner starts from) and interactive-solution.sql.md.
# Proof per lab, on a SQLite 3.7.17 shell ($SQLITE3; any other version is refused): each file holds exactly one ';'
# and it is the last character, no dot-command, no statement that writes; the shell prints exactly FAIL for the
# starter and exactly PASS for the solution; the work areas are numbered 1..n of n, opened and closed in order; the
# two files differ only inside the work areas (and on line 1, which must be a comment).
# Every lab of a COURSE named on the command line must carry both files; a lab elsewhere that carries one is checked too.
set -uo pipefail
R=${1:?usage: interactive-check.sh LABS_DIR COURSE...}; shift; bad=0; n=0
Q=${SQLITE3:-}
fail() { echo "RED $*"; bad=1; }
{ [ -n "$Q" ] && [ -x "$Q" ]; } || { echo "REFUSE: set SQLITE3 to a SQLite 3.7.17 shell (scripts/sqlite-3717.sh builds one)"; exit 2; }
[ "$("$Q" -version 2>/dev/null | cut -d' ' -f1)" = 3.7.17 ] || { echo "REFUSE: $Q is not SQLite 3.7.17"; exit 2; }
[ -d "$R" ] || { echo "REFUSE: no directory $R"; exit 2; }
for c in "$@"; do [ -d "$R/$c" ] || { echo "REFUSE: no course directory $R/$c"; exit 2; }; done
one() { # one LAB FILE WANT
  local f=$2 l=$1 got
  [ "$(tr -cd ';' < "$f" | wc -c)" -eq 1 ] || { fail "$l: $(basename "$f") must hold exactly one ';'"; return; }
  [ "$(tr -d ' \t\n\r' < "$f" | tail -c 1)" = ';' ] || { fail "$l: $(basename "$f") must end with its ';'"; return; }
  if grep -qE '^[[:space:]]*\.' "$f"; then fail "$l: $(basename "$f") has a dot-command"; return; fi
  if grep -v '^[[:space:]]*--' "$f" | grep -qiwE 'create|drop|insert|update|delete|attach|pragma|replace|alter|vacuum'; then fail "$l: $(basename "$f") has a statement that writes"; return; fi
  got=$("$Q" :memory: < "$f" 2>&1)
  [ "$got" = "$3" ] || fail "$l: $(basename "$f") prints '$(head -c 60 <<<"$got")', it must print $3"
}
# markers FILE -> exit 0 when the work areas are 1..n of n, each opened then closed, in order, and n >= 1.
markers() { awk '
  /^-- =====/ {
    if (match($0, /^-- ===== YOUR WORK [0-9]+ of [0-9]+: edit only between these lines =====$/)) t = "S"
    else if (match($0, /^-- ===== END OF YOUR WORK [0-9]+ of [0-9]+ =====$/)) t = "E"
    else exit 1
    split($0, w, " "); k = (t == "S") ? w[5] : w[7]; m = (t == "S") ? w[7] : w[9]; sub(/:$/, "", m)
    if (tot == "") tot = m; if (m != tot) exit 1
    if (t == "S") { if (open || k != done + 1) exit 1; open = 1 }
    else { if (!open || k != done + 1) exit 1; open = 0; done++ }
  }
  END { if (open || tot == "" || done != tot || tot < 1) exit 1 }' "$1"; }
outside() { awk 'NR==1{next} /^-- ===== YOUR WORK /{print; s=1; next} /^-- ===== END OF YOUR WORK /{s=0} !s' "$1"; }
for d in "$R"/*/*/; do
  d=${d%/}; [ -d "$d" ] || continue; l=${d#"$R"/}; s=$d/interactive.sql.md; o=$d/interactive-solution.sql.md; need=0
  for c in "$@"; do [ "${l%%/*}" = "$c" ] && need=1; done
  if [ ! -f "$s" ] && [ ! -f "$o" ]; then [ "$need" = 0 ] || fail "$l: no interactive.sql.md"; continue; fi
  n=$((n+1))
  { [ -f "$s" ] && [ -f "$o" ]; } || { fail "$l: needs both interactive.sql.md and interactive-solution.sql.md"; continue; }
  for f in "$s" "$o"; do markers "$f" || fail "$l: $(basename "$f") needs its YOUR WORK areas numbered 1..n of n, each opened then closed"; done
  for f in "$s" "$o"; do head -n 1 "$f" | grep -q "^-- " || fail "$l: $(basename "$f") line 1 must be a comment"; done
  one "$l" "$s" FAIL; one "$l" "$o" PASS
  [ "$(outside "$s")" = "$(outside "$o")" ] || fail "$l: the two files differ outside the YOUR WORK areas"
done
[ "$n" -ge 1 ] || { echo "RED no in-page lab under $R"; exit 1; }
[ "$bad" = 0 ] || exit 1
echo "interactive OK: $n labs on SQLite 3.7.17, one statement each, starter prints FAIL, solution prints PASS"
