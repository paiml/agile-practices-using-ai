# Lab: Acceptance Criteria in Given-When-Then (Course 2, Module 4, lesson 4.1.1)

**Objective (O4):** Write acceptance criteria in Given-When-Then form, turn them into tests before code and check AI output for errors, hallucinations and data leaks. This lab serves M4a: criteria in Given-When-Then form, as tests that run.

An acceptance criterion says when a requirement is met. Given-When-Then gives it three parts. **Given** is the state of the world before. **When** is the one thing that happens. **Then** is what must be true after. Written this way, a criterion is already most of a test.

The requirement here is one sentence: "An invoice gets a reminder when it is unpaid and more than 7 days past its due date." A developer has written it as a rule: the `WHERE` in the last YOUR WORK area of the block, which lists the invoices that get a reminder. You will write three criteria as tests and run them against that rule.

In SQL the three parts are easy to see. The Given is rows of the invoices (the block calls them `i`): invoice 101 is unpaid and 8 days late, 102 was paid, 103 is unpaid and exactly 7 days late. The When is the query that lists the reminders on 10 March. The Then is a test that prints `pass` or `fail`.

AC1 is written for you. You write AC2 and AC3. One of them will fail, and the test is right. "More than 7 days" and "7 days or more" differ by one day, and criteria written at the edge of a rule are the ones that find this. When a test fails, fix the rule. Do not change the test to agree with the code.

The check for this lab asks one more thing than the others: can each of your tests fail? It reverses the Given of each criterion and looks for three `fail` results. A test that says `pass` whatever the data is not a test. For this to work, each test must name its invoice by id, as AC1 does.

The block holds the invoice rows for two worlds, `given` and `reversed`, and today's date (10 March), typed in as lists. The rule is the `WHERE` in YOUR WORK 3; your tests are the columns `ac1` (written for you), `ac2` and `ac3`. The block prints only `PASS` or `FAIL`, not the individual `pass` and `fail` results.

## Tasks

1. **AC2.** In YOUR WORK 1, replace `'todo' AS ac2` with a test that says `pass` when invoice 102 is not in the list of reminders, written like AC1.
2. **AC3.** In YOUR WORK 2, do the same for invoice 103, which is exactly 7 days past due and must not get a reminder.
3. **Run the tests and fix what fails.** With AC2 and AC3 written, the block still prints `FAIL`, because one of your tests says `fail` in the given world. Read the requirement again, find the difference in the rule in YOUR WORK 3, and fix the rule.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong-a.sql.md` and `wrong-b.sql.md` (planted wrong answers: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
