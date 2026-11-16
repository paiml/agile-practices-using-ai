# Lab: Reading an A/B Test (Course 3, Module 4, lesson 4.1.3)

An A/B test result has two parts: how big the gap is, and whether the gap could be noise. A big lift on a small sample is the classic trap. Read the size and the evidence together, then make the call.

In this lab each experiment has a control and a test variant, with the users who saw it and the conversions that followed. The conversion rate of a variant is conversions / users. The lift is the test rate minus the control rate, divided by the control rate.

To test whether the gap is real, use a two-proportion z-test. Take the pooled rate `p = (ca + cb) / (na + nb)`. Then `z^2 = (rb - ra)^2 / (p * (1 - p) * (1/na + 1/nb))`. At the usual 95 percent level a gap is real when `z^2` is above 3.8416 (that is 1.96 squared, which avoids needing a square root in plain SQLite).

The verdict has three values: `ship` (a real gain), `harm` (a real loss), and `no call` (the data cannot tell the two variants apart yet).

## Tasks

1. Compute `lift_pct` for each experiment.
2. Compute `z_squared` with the pooled rate.
3. Set `verdict` to `no call` when `z_squared` is at or below 3.8416, else `ship` if the test rate is higher, else `harm`.
4. Look at the result. The short signup form shows the biggest lift. What do you tell the team, and what would you do next?

The block checks the columns `experiment`, `lift_pct`, `z_squared`, `verdict`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
