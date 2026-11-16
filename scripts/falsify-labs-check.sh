#!/usr/bin/env bash
# Falsifier for labs-check.sh: each planted defect is refused with its own message; the real labs/ tree is green.
set -uo pipefail
H=$(cd "$(dirname "$0")" && pwd); K=${LABS_CHECK:-$H/labs-check.sh}
T=$(mktemp -d); trap 'rm -rf "${T:?}"' EXIT; f=0
good() { # good NAME -> a lab whose check passes only the answer "42"
  local d=$T/$1/c/lab-a; mkdir -p "$d"
  printf '%s\n' '#!/usr/bin/env bash' '[ "$(sqlite3 :memory: ".read $1")" = 42 ]' > "$d/check.sh"
  echo 'SELECT 42;' > "$d/solution.sql.md"; echo 'SELECT 1;' > "$d/starter.sql.md"; echo 'SELECT 41;' > "$d/wrong.sql.md"
  echo "$d"
}
arm() { # arm NAME WANT_RC TEXT DIR
  local out rc; out=$(bash "$K" "$4" 2>&1); rc=$?
  if [ "$rc" = "$2" ] && grep -q -- "$3" <<<"$out"; then echo "  ok   $1 (rc=$rc)"
  else echo "  FAIL $1: want rc=$2 '$3', got rc=$rc: $(head -c 200 <<<"$out")"; f=1; fi
}
d=$(good a); arm "a complete lab is green" 0 "labs OK: 1 labs" "$T/a"
d=$(good b); rm "${d:?}/wrong.sql.md"; arm "no wrong file is RED" 1 "no wrong file" "$T/b"
d=$(good c); echo 'SELECT 42;' > "$d/wrong-2.sql.md"; arm "a wrong file that passes is RED" 1 "wrong-2.sql.md PASSES" "$T/c"
d=$(good d); echo 'SELECT 7;' > "$d/solution.sql.md"; arm "a solution that fails is RED" 1 "solution does not PASS" "$T/d"
d=$(good e); echo 'SELECT 42;' > "$d/starter.sql.md"; arm "a starter that passes is RED" 1 "starter PASSES" "$T/e"
d=$(good g); rm "${d:?}/check.sh"; arm "no check.sh is RED" 1 "no check.sh" "$T/g"
d=$(good h); rm "${d:?}/solution.sql.md"; arm "no solution is RED" 1 "exactly one solution" "$T/h"
mkdir -p "$T/i"; arm "an empty labs directory is RED" 1 "no lab under" "$T/i"
arm "a missing labs directory is refused" 2 "REFUSE" "$T/nope"
arm "the real labs/ tree is green" 0 "labs OK" "$H/../labs"
[ "$f" = 0 ] && echo "falsify-labs-check: all rows held" || { echo "falsify-labs-check: FAILED"; exit 1; }
