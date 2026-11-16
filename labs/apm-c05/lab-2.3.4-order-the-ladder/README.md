# Lab: Order the Verification Ladder (Course 5, Module 2, lesson 2.3.4)

**Objective (O2):** Order checks from cheapest to most expensive. This lab builds the verification ladder as a query: cheapest check first, with the running cost of climbing it, and leaves out the checks that earn nothing.

The verification ladder runs the cheapest check first. A formatter takes seconds and a clean-room build takes minutes, so when a change is going to fail, you want it to fail on the cheap rung, before anyone waits for the expensive one.

The rows in the block list the checks one pipeline can run (the block calls them `g`). `seconds` is how long one run takes and `catches` is how many defects the check caught per 100 changes. A check that catches nothing is not a rung: it only adds waiting time.

The block holds one list of rows, typed in: the checks, with `name`, `seconds` and `catches`.

## Tasks

1. **Task 1: the ladder.** `rung` (1 for the cheapest), `name`, `seconds` and `total_seconds` (the seconds of this rung and every rung below it, added up). Only checks that catch something are rungs. Ties in `seconds` go to the lower `name`.
2. **Task 2: the dead rungs.** The `name` of every check that catches nothing.
3. **Task 3: the time saved.** One column, `seconds_saved`: the seconds the pipeline saves per run by dropping the checks of the Task 2 rows.
4. **Not checked by the query:** the ladder orders checks by cost, not by how many defects they catch. Name one check from the rows whose place you would argue about, and say what number you would want first.

## What this lab cannot teach

SQL can order the checks and add up their time. It cannot tell whether a check that caught nothing last quarter is safe to drop. The `catches` figure is a past count, and a check that guards a rare failure looks the same as one that guards nothing.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
