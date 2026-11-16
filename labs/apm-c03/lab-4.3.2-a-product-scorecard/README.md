# Lab: A Product Scorecard (Course 3, Module 4, lesson 4.3.2)

A scorecard puts the few numbers that matter on one page, each against its target, so a reader can see in seconds what is healthy and what is not. The trick is that "better" points in different directions: you want activation and retention to go up, and you want latency and cost to go down.

In this lab each metric has a value, a target and a direction. To compare them on one scale, convert each to an attainment percent. For a metric where higher is better, attainment is value / target. For a metric where lower is better, flip it: target / value. At 100 percent you are exactly on target.

The status is a colour: `green` at 100 or more, `amber` at 90 or more, otherwise `red`. A metric that sits exactly on the amber line (90) is amber.

## Tasks

1. Compute `attainment_pct` for each metric, respecting its direction.
2. Set `status` to `green` at 100 or more, `amber` at 90 or more, else `red`.
3. Look at the result. Two metrics are red. Which one would you feed back into next quarter's roadmap first, and why?

The block checks the columns `metric`, `attainment_pct`, `status`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
