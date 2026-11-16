# Runs: lab-4.3.2-a-product-scorecard

Measured 2026-11-15 with sqlite3 3.37.2 on the authoring host, database in memory. Each block is the command, its whole output and its exit code. The in-browser SQLite WASM shell was not run.

```
$ ./check.sh solution.sql.md
PASS
exit 0

$ ./check.sh starter.sql.md
FAIL
exit 1

$ ./check.sh wrong.sql.md
FAIL
exit 1

```
