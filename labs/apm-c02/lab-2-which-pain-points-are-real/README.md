# Lab: Which pain points are real? (Course 2, Module 2)

You interviewed six customers and tagged every note. Some people repeated themselves, so counting rows overstates a theme. Finish one SQL query: keep a tag only if at least 3 different interviewees raised it.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs, against `expected.txt`.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `data.sql.md` (table and rows), `starter.sql.md` (fails on purpose: counts rows, no threshold), `solution.sql.md`, `expected.txt`, `wrong.sql.md` (a planted wrong answer), `check.sh`.

## Contract (LAB-STD-001)
- L5 starter exits non-zero and shows the diff; the solution exits 0 against the same `expected.txt` (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
