# Lab: Score the Bets: Impact, Effort, Confidence (Course 3, Module 2, lesson 2.1.0)

A list of bets needs an order. Ordering by impact alone puts the biggest idea first, even when it takes half a year and nobody has tested it.

A score fixes that with three numbers per bet. Impact is how much it moves the goal, from 1 to 10. Effort is the engineer-weeks to ship it. Confidence is how sure you are, from 0 to 1. The score is `impact x confidence / effort`: value you expect per week of work.

The score does not decide for you. It makes the argument visible. If someone wants Mobile app rewrite at the top, they now have to say which of its three numbers is wrong.

One SQL trap: `impact` and `effort_weeks` are whole numbers. If you divide one by the other first, the database drops the fraction, so 8 / 6 becomes 1. Multiply by `confidence` before you divide.

## Tasks

1. Compute `score` as impact x confidence / effort for every bet.
2. Add `rank`, where 1 is the highest score.
3. Not checked by the block: compare with a ranking by impact alone, which is what the `score` column holds in the starting statement (press Reset to get it back). Which bet falls from first place to last, and which of its numbers causes that?

The block checks the columns `bet`, `score` and `rank`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
