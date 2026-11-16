# Lab: Required Checks and the Merge Button (Course 4, Module 4, lesson 4.3.4)

**Objective (O4):** Make CI and required checks the gate that decides a merge.

The merge button turns on only when every required check is `success` and at least one person approved. Two things surprise people. A required check that never reported is not a pass: the button stays off, and nothing on the page says which check is missing. And a check that is not required can be red, or still running, and the button still turns on. What decides a merge is the list of required checks, not the colors on the page.

In this lab the pull requests, the required checks and the check runs each pull request reported are three lists of rows. A required check with no row for a pull request has never reported.

The block holds these three lists of rows, typed in: the required checks (the block calls them `r`, with `name`), the pull requests (`p`, with `id`, `title` and `approvals`) and the check runs (`c`, with `pr_id`, `name` and `status`).

## Tasks

1. **Task 1: what blocks each pull request.** One row for each required check that is not `success` on a pull request: `pr_id`, `check_name` and `state`. `state` is the check's status, or `missing` when it never reported.
2. **Task 2: may merge.** Each pull request with at least one approval and no blocking row in Task 1.
3. **Task 3: a required check never reported.** Each pull request with at least one `missing` row in the Task 1 rows.
4. **Task 4: merges with a red or pending check.** Each pull request in the Task 2 rows that has a check run that is not `success` and is not required.
5. **Not checked by the query:** pull request 4 has all its checks green and still cannot merge. Say why, and what the author must do. Then say what would change in the Task 3 rows if pull request 3 had its missing check run.

## What this lab cannot teach

SQL can say which required checks are not green. It cannot tell whether the list of required checks is the right one, whether a check really tests anything, or whether a person approved without reading. A green gate is only as good as what is behind its names.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
