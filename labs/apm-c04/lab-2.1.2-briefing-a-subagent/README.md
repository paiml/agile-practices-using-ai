# Lab: Briefing a Subagent (Course 4, Module 2, lesson 2.1.2)

**Objective (O2):** Write a brief for each subagent with a task, paths and a check, and dispatch parallel agents with disjoint write sets.

A subagent starts with nothing but its brief. A good brief has three parts: the task, the paths the agent may write, and the check that proves it is done. Leave one out and the agent guesses. Two agents that may write the same file will overwrite each other, so briefs that run in parallel need disjoint write sets.

In this lab the briefs are rows and the paths are a second list of rows, one row per path a brief may write. A part is missing when it is NULL, an empty string or only spaces. For paths it is missing when the brief has no row at all.

The block holds two lists of rows, typed in: the briefs (the block calls them `b`, with `id`, `task` and `check_cmd`) and the paths (`brief_id`, `path`).

## Tasks

1. **Task 1: missing parts.** One row per missing part of each brief: the `brief_id` and the word `task`, `paths` or `check`.
2. **Task 2: shared paths.** One row for each path two different briefs both may write: `a_id`, `b_id` (with `a_id` smaller) and `path`.
3. **Task 3: safe to dispatch in parallel.** Each brief with no missing part and no path shared with another brief.
4. **Task 4: the brief text.** For each brief in the Task 3 rows, the text in exactly this form: `Task: <task>. Paths: <path>, <path>. Check: <check>.` Paths are in alphabetical order.
5. **Not checked by the query:** briefs 2 and 5 both write `src/lib.rs`. The block's rows stay as they are, so decide on paper: should brief 5 change its paths so the two briefs are disjoint, or must one wait for the other? Say why. Say which row would then leave the Task 2 rows.

## What this lab cannot teach

SQL can check that a brief has its three parts and that two write sets do not overlap on an exact path. It cannot tell whether a task is small enough for one agent, whether the check really proves the task, or whether two different files depend on each other. A brief can pass every query here and still be a bad brief.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
