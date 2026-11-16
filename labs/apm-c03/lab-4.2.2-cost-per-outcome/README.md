# Lab: Cost per Outcome (Course 3, Module 4, lesson 4.2.2)

A cost chart shows what you spent. It does not show whether the money bought anything. A feature whose bill doubled may be a bargain if it delivers four times the results, and a feature whose bill is flat may be quietly getting worse. The number to decide on is the cost per outcome: cost divided by the useful results delivered.

In this lab three features report their monthly cost and their outcomes (tickets resolved, searches that ended in a click, exports completed). You compare the cost per outcome in month 1 with month 3. If it fell, the feature earns more budget (`scale`). If it rose, someone should look at it (`review`). If it did not move, `hold`.

Month 2 is in the block's rows too. You do not need it for the decision, so make sure the query looks only at the first and the last month.

## Tasks

1. Compute the cost per outcome for each feature in month 1 (`first_cpo`) and month 3 (`last_cpo`).
2. Set `decision` to `scale` when it fell, `review` when it rose, else `hold`.
3. Look at the result. AI triage has the largest bill. What do you tell the person who sees only the cost chart?

The block checks the columns `feature`, `first_cpo`, `last_cpo`, `decision`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
