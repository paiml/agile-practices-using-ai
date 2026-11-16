#!/usr/bin/env bash
# Falsifier for interactive-check.sh: each planted defect is refused with its own message; a good lab is green.
set -uo pipefail
H=$(cd "$(dirname "$0")" && pwd); K=${INTERACTIVE_CHECK:-$H/interactive-check.sh}
T=$(mktemp -d); trap 'rm -rf "${T:?}"' EXIT; f=0
A1='-- ===== YOUR WORK 1 of 2: edit only between these lines ====='; B1='-- ===== END OF YOUR WORK 1 of 2 ====='
A2='-- ===== YOUR WORK 2 of 2: edit only between these lines ====='; B2='-- ===== END OF YOUR WORK 2 of 2 ====='
body() { printf '%s\nSELECT CASE WHEN (SELECT\n%s\n  s.x + %s\n%s\n  FROM (SELECT 41 AS x) s) = 42 AND (SELECT\n%s\n  %s\n%s\n  ) = 1 THEN '"'PASS'"' ELSE '"'FAIL'"' END AS result;\n' "-- $1" "$A1" "$2" "$B1" "$A2" "$2" "$B2"; }
good() { local d=$T/$1/c1/lab-a; mkdir -p "$d"; body starter 0 > "$d/interactive.sql.md"; body solution 1 > "$d/interactive-solution.sql.md"; echo "$d"; }
arm() { # arm NAME WANT_RC TEXT DIR [COURSE...]
  local name=$1 want=$2 text=$3 dir=$4 out rc; shift 4; out=$(bash "$K" "$dir" "$@" 2>&1); rc=$?
  if [ "$rc" = "$want" ] && grep -q -- "$text" <<<"$out"; then echo "  ok   $name (rc=$rc)"
  else echo "  FAIL $name: want rc=$want '$text', got rc=$rc: $(head -c 200 <<<"$out")"; f=1; fi
}
both() { sed -i "$1" "$d/interactive.sql.md" "$d/interactive-solution.sql.md"; }
d=$(good a); arm "a complete lab is green" 0 "interactive OK: 1 labs on SQLite 3.7.17" "$T/a" c1
d=$(good b); body starter 1 > "$d/interactive.sql.md"; arm "a starter that prints PASS is RED" 1 "must print FAIL" "$T/b" c1
d=$(good c); body solution 0 > "$d/interactive-solution.sql.md"; arm "a solution that prints FAIL is RED" 1 "must print PASS" "$T/c" c1
d=$(good e); { echo "SELECT 1;"; cat "$d/interactive-solution.sql.md"; } > "$d/x"; mv "$d/x" "$d/interactive-solution.sql.md"; arm "two statements are RED" 1 "exactly one ';'" "$T/e" c1
d=$(good g); sed -i 's/result;$/result/' "$d/interactive.sql.md"; arm "no ';' is RED" 1 "exactly one ';'" "$T/g" c1
d=$(good h); echo "-- after" >> "$d/interactive.sql.md"; echo "-- after" >> "$d/interactive-solution.sql.md"; arm "text after the ';' is RED" 1 "must end with its ';'" "$T/h" c1
d=$(good i); sed -i '2i .mode list' "$d/interactive.sql.md"; arm "a dot-command is RED" 1 "dot-command" "$T/i" c1
d=$(good j); sed -i "s/s.x + 1/s.x + 1 + ('insert' IS NULL)/" "$d/interactive-solution.sql.md"; arm "a writing word is RED" 1 "statement that writes" "$T/j" c1
d=$(good k); rm "${d:?}/interactive-solution.sql.md"; arm "a starter with no solution is RED" 1 "needs both" "$T/k" c1
d=$(good m); sed -i 's/SELECT 41 AS x/SELECT 42 AS x/' "$d/interactive.sql.md"; arm "a seed that differs between the files is RED" 1 "differ outside" "$T/m" c1
d=$(good n); sed -i "/END OF YOUR WORK 2 of 2/d" "$d/interactive.sql.md"; arm "a work area never closed is RED" 1 "numbered 1..n of n" "$T/n" c1
d=$(good n2); both 's/YOUR WORK 2 of 2/YOUR WORK 3 of 2/'; arm "a work area out of sequence is RED" 1 "numbered 1..n of n" "$T/n2" c1
d=$(good n3); both 's/ of 2/ of 3/'; arm "fewer work areas than the count says is RED" 1 "numbered 1..n of n" "$T/n3" c1
d=$(good n4); both 's/^-- ===== YOUR WORK 1 of 2: edit only between these lines =====$/-- ===== YOUR WORK: edit here =====/'; arm "a marker in another form is RED" 1 "numbered 1..n of n" "$T/n4" c1
d=$(good n5); both '/^-- =====/d'; arm "no work area at all is RED" 1 "numbered 1..n of n" "$T/n5" c1
d=$(good l1); sed -i "1s/.*/SELECT 'FAIL' AS r/" "$d/interactive.sql.md"; sed -i "1s/.*/SELECT 'PASS' AS r/" "$d/interactive-solution.sql.md"; sed -i '2,$s/^[^-].*//; s/result;$//' "$d/interactive.sql.md" "$d/interactive-solution.sql.md"; arm "SQL on line 1 is RED" 1 "line 1 must be a comment" "$T/l1" c1
d=$(good u); sed -i 's/^SELECT CASE WHEN (SELECT$/WITH seed(x) AS (SELECT 41) SELECT CASE WHEN (SELECT/' "$d/interactive.sql.md" "$d/interactive-solution.sql.md"; arm "WITH, which the page's engine lacks, is RED" 1 "must print PASS" "$T/u" c1
d=$(good v); sed -i 's/s.x + 1/s.x + row_number() OVER ()/' "$d/interactive-solution.sql.md"; arm "a window function is RED" 1 "must print PASS" "$T/v" c1
d=$(good p); mkdir -p "$T/p/c1/lab-b"; arm "a lab of a named course with no in-page file is RED" 1 "lab-b: no interactive.sql.md" "$T/p" c1
d=$(good q); mkdir -p "$T/q/c2/lab-z"; arm "a lab of a course not named may lack the file" 0 "interactive OK: 1 labs" "$T/q" c1
d=$(good r); printf 'SELECT oops;\n' > "$d/interactive.sql.md"; arm "a starter that errors is RED, not FAIL" 1 "must print FAIL" "$T/r" c1
mkdir -p "$T/s/c1/lab-a"; arm "no in-page lab at all is RED" 1 "no in-page lab" "$T/s"
arm "a missing labs directory is refused" 2 "REFUSE" "$T/nope" c1
d=$(good t); arm "a missing named course is refused" 2 "no course directory" "$T/t" c9
d=$(good w); if command -v sqlite3 >/dev/null; then SQLITE3=$(command -v sqlite3) arm "a shell that is not 3.7.17 is refused" 2 "is not SQLite 3.7.17" "$T/w" c1; else echo "  FAIL no other sqlite3 to plant"; f=1; fi
d=$(good x); SQLITE3='' arm "no shell named is refused" 2 "set SQLITE3" "$T/x" c1
[ "$f" = 0 ] || { echo "falsify-interactive-check: FAIL"; exit 1; }
echo "falsify-interactive-check: every planted defect is refused"
