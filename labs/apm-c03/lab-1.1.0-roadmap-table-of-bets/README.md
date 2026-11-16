# Lab: A Roadmap Is a Table of Bets (Course 3, Module 1, lesson 1.1.0)

A roadmap item is a bet. You pay the cost for certain. You get the value only if the bet works. Confidence is your estimate of how likely that is, and it should come from evidence, not from how much the team likes the idea.

In this lab the roadmap is a list of rows typed into the block. Each bet has a value if it works, a cost to build, and the strongest evidence you hold for it. A second list of rows, the evidence ladder, turns each kind of evidence into a confidence number: an opinion is worth 0.1, a live A/B test 0.9.

The expected value of a bet is `value x confidence - cost`. The cost is not multiplied by confidence, because you pay it either way.

A bet with a negative expected value is not always a bad idea. Often it is a good idea with weak evidence. The cheap move is to climb one rung of the ladder (run five interviews, build a prototype) before paying the build cost. So the decision column has two values: `fund`, or `buy evidence`.

## Tasks

1. Join each bet to the evidence ladder and return its `confidence`.
2. Compute `expected_value_k` as value x confidence - cost.
3. Set `decision` to `fund` when the expected value is above zero, else `buy evidence`.
4. Not checked by the block: the two bets with the largest value are Mobile app rewrite and Usage-based pricing. Which one do you fund, and what would you do about the other one this month?

The block checks the columns `bet`, `confidence`, `expected_value_k` and `decision`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
