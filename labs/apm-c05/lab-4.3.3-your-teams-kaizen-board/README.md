# Lab: Your Team's Kaizen Board (Course 5, Module 4, lesson 4.3.3)

**Objective (O4):** Make work flow with visible boards, work-in-progress limits and standard work. This lab chooses the team's next improvements from a kaizen board with a query.

A kaizen board lists small improvements. The team should not start with the biggest idea. It should start with the best return for the effort, and it should skip ideas that are already done. Each idea has a `status` (`open` or `done`), an `impact` from 1 to 10 and an `effort` from 1 to 10. The score of an idea is `impact * 1.0 / effort`. The `* 1.0` keeps SQL from rounding down to a whole number.

Two ideas on the board have the same score. The rule for a tie is written down, because a rule nobody wrote down is a different rule each week: the lower `id` goes first.

The block holds one list of rows, typed in: the kaizen ideas (the block calls them `k`).

## Tasks

1. **Task 1: the next three.** `id`, `idea` and `score` (rounded to 1 decimal) of the three open ideas with the highest score, ties by `id`.
2. **Task 2: the commitment.** One column, `total_effort`: the effort of the three ideas in the Task 1 rows added up.
3. **Task 3: not this round.** The `id` of every open idea that is not in the Task 1 rows.
4. **Not checked by the query:** "Rewrite the build system" has the biggest impact (9) and is not in your result. Say in one sentence why, using its effort. Then decide what goes on the board after the top idea is finished, and which number you measure first so the check step has a baseline.

## What this lab cannot teach

SQL can rank ideas by a score. It cannot tell whether an impact of 8 is honest. The score is only as good as the numbers the team agreed on, and the lab does not judge them.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
