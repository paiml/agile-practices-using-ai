# Lab: Set the Cut Line (Course 3, Module 2, lesson 2.2.2)

A cut line says what done means before you ship, and it also says what is out. When capacity is fixed, you do not squeeze one more item in. You rank the backlog, add up the effort from the top, and draw the line where the running total stops fitting.

In this lab the team has 10 engineer-weeks this cycle. The backlog is already ranked by priority. The running total is the sum of effort from priority 1 down to the current item. An item ships while the running total is at or below capacity. Everything below the line is cut, even a small item that would fit on its own, because the ranking is the decision.

An item that brings the total to exactly the capacity still fits. That edge is where cut lines usually go wrong.

## Tasks

1. Compute `cumulative_weeks`, the running total of `effort_weeks` in priority order.
2. Set `decision` to `ship` while `cumulative_weeks` is at or below the capacity, else `cut`.
3. Look at the result. Dark mode costs only 2 weeks. Why is it cut, and what would you have to remove to ship it?

The block checks the columns `item`, `cumulative_weeks`, `decision`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
