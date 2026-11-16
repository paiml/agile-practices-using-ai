# Lab: Leading, Lagging and Vanity (Course 3, Module 3, lesson 3.1.3)

Three numbers can be drawn from the same weekly sign-up rows, and they tell three different stories.

Total sign-ups is a vanity metric. It is a running total, so it can only go up. No value of it would make you change the plan.

Paid at week 4 is a lagging indicator. It is the result you care about, but it arrives four weeks late. For the last three weeks the column is still empty.

Activation rate is a leading indicator. It is the share of a week's sign-ups who finish setup within seven days. You know it a week after sign-up, and in this data it moves the same way the paid rate does.

In week 4 the team started a paid ad campaign. Sign-ups jumped. Build all three numbers and decide which one tells you about the campaign in time to act.

Compute a weekly rate from the week's totals: all activated divided by all sign-ups. Do not average the two channel rates. That gives a channel with 130 sign-ups the same weight as one with 670.

## Tasks

1. Return one row per week with `total_signups` as the running total since week 1.
2. Add `activation_pct`: the week's activated accounts over the week's sign-ups, in percent.
3. Add `paid_w4_pct`: the week's paying accounts over the week's sign-ups, in percent. Leave it empty (NULL) where the data is not in yet.
4. Which column would you put on the growth team's dashboard, and what decision does week 4 call for?

The block checks the columns `week`, `total_signups`, `activation_pct` and `paid_w4_pct`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
