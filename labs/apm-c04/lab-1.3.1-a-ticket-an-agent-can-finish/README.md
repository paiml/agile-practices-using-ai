# Lab: A Ticket an Agent Can Finish (Course 4, Module 1, lesson 1.3.1)

**Objective (O1):** Write a ticket with a measurable done condition and a plan you approve before code. This lab checks the form of a ticket and the gate in front of the agent.

An agent can finish a ticket when it can tell, without asking, when it is done. That needs a title, a done condition it can measure, and a plan a person approved before any code is written. "Export is fast" is not a done condition: no command can say yes or no to it. "`make test` exits 0 and the new test `overdue_filter` passes" is one.

In this lab the tickets are rows with a title, a done condition and a flag for whether the plan was approved. A list of vague words stands in for the words a measurable condition avoids. A part is missing when it is NULL, an empty string or only spaces; a check that looks only for NULL lets the other two through.

The block holds two lists of rows, typed in: the tickets (the block calls them `t`) and the vague words (`w`, with a `word` column).

## Tasks

1. **Task 1: missing parts.** One row per missing part of each ticket: the `ticket_id` and the word `title` or `done_condition`.
2. **Task 2: vague done conditions.** Each ticket whose done condition is present but contains a word from the list of vague words.
3. **Task 3: ready for an agent.** Each ticket with an approved plan, no missing part and no vague done condition.
4. **Task 4: approved but not ready.** Each ticket whose plan was approved although it is not in the Task 3 rows.
5. **Not checked by the query:** rewrite the done condition of ticket 2 so that a command can answer it. Name the command and the number it must print, and say why ticket 2 would then leave the Task 2 rows.

## What this lab cannot teach

SQL can check that a done condition is present and free of vague words. It cannot tell whether a condition without those words can really be measured, and it cannot judge a plan. A list of vague words is a start, not a proof.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
