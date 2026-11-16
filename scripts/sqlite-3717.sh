#!/usr/bin/env bash
# sqlite-3717.sh OUT_DIR -> builds OUT_DIR/sqlite3, the SQLite 3.7.17 shell (the version the course page runs).
# Source: the sqlite.org release tarball, refused unless its sha256 is the pinned one. Exit 2 when it cannot be built.
set -uo pipefail
O=${1:?usage: sqlite-3717.sh OUT_DIR}
U=https://www.sqlite.org/2013/sqlite-autoconf-3071700.tar.gz
H=8ff46d0baa9e64c0815544e829e985f1161c096aa6344c8f430791dbeadc2baf
ok() { [ -x "$O/sqlite3" ] && [ "$("$O/sqlite3" -version 2>/dev/null | cut -d' ' -f1)" = 3.7.17 ]; }
ok && exit 0
mkdir -p "$O" || exit 2
curl -fsSL --retry 3 -o "$O/src.tar.gz" "$U" || { echo "REFUSE: cannot fetch $U"; exit 2; }
[ "$(sha256sum "$O/src.tar.gz" | cut -d' ' -f1)" = "$H" ] || { echo "REFUSE: $U is not the pinned tarball"; exit 2; }
tar -xzf "$O/src.tar.gz" -C "$O" || exit 2
S=$O/sqlite-autoconf-3071700
${CC:-cc} -O1 -w -DSQLITE_THREADSAFE=0 -DSQLITE_OMIT_LOAD_EXTENSION -o "$O/sqlite3" "$S/shell.c" "$S/sqlite3.c" || { echo "REFUSE: build failed"; exit 2; }
ok || { echo "REFUSE: the built shell is not 3.7.17"; exit 2; }
