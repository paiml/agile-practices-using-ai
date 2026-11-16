# Lab: Walk the Five Whys (Course 5, Module 1, lesson 1.3.4)

**Objective (O1):** Apply plan, do, check, act and the five whys to a real team problem. This lab walks a chain of "why?" answers with self-joins and reads off the root cause of each incident and whether the team can change it.

The five whys asks "why did this happen?" again and again until you reach a cause you can change. Each answer explains the one before it. In a table, that is a row that points at its parent row. A recursive query can follow such a chain without knowing in advance how long it is; here no chain is more than five answers deep, so the block joins the rows to themselves five times instead (it calls the copies `w1` to `w5`).

Not every chain reaches a cause you can change. The `changeable` column records whether the team can change that answer. A chain that ends on "we do not control the registry" ended too early: the useful question is what the team does about it.

The block holds one list of rows, typed in: the answers, with `id`, `incident`, `parent_id`, `answer` and `changeable`.

## Tasks

1. **Task 1: the end of each chain.** For each incident, `incident`, `depth` (the symptom is depth 1), `root_cause` (the deepest answer) and `changeable`.
2. **Task 2: stuck chains.** The `incident` of every chain whose last answer the team cannot change.
3. **Task 3: full chains.** The `incident` of every chain that is five answers deep or more.
4. **Not checked by the query:** take the incident in the Task 2 rows and write the next "why?" that leads to something the team can change (for example, what the team does when a dependency breaks).

## What this lab cannot teach

SQL can follow the chain and read its flags. It cannot tell whether the chain is true or whether the person stopped asking too early. Depth five is a habit, not a law.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
