# Lab: A Latency Budget from Percentiles (Course 3, Module 4, lesson 4.2.1)

A latency budget says how long the whole request may take, and the steps must share it. Because the typical case and the slow tail behave differently, you hold a budget at two percentiles: a p50 budget for the usual request and a p99 budget for the unlucky one.

In this lab a request passes through four steps. Each step has its own p50 and p99 time. Adding the p99 times of the steps gives a safe upper bound for the p99 of the whole request (the real figure is usually a little lower, because the slowest case rarely hits every step at once). You compare each total with its budget.

A request that lands exactly on its budget is still inside it. The headroom is the budget minus the total, so a negative number means you are over.

## Tasks

1. For each percentile (`p50` and `p99`), sum the step times into `total_ms`.
2. Compute `headroom_ms` as the budget minus the total, and set `status` to `ok` when the total is at or below the budget, else `over`.
3. Name the `biggest_step`: the step that takes the most time at that percentile.
4. Look at the result. The p99 path is over. Which step would you attack first, and does the answer change if you look at p50?

The block checks the columns `percentile`, `total_ms`, `headroom_ms`, `status`, `biggest_step`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
