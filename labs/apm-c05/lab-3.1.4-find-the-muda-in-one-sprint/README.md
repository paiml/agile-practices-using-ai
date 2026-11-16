# Lab: Find the Muda in One Sprint (Course 5, Module 3, lesson 3.1.4)

**Objective (O3):** Measure before you remove waste. This lab groups one sprint's hours into the wastes of lean and ranks them, so the team fixes the biggest waste first and not the one it noticed first.

Muda is waste: work that adds no value for the customer. An AI team makes waste faster, because an agent can write an unused helper in a minute or open a duplicate ticket. The rule of the Toyota Way is to measure before you remove waste.

One list of rows, the sprint's work log (the block calls it `l`), holds one row per piece of work with its `kind` and its `hours`. A second list (`w`) says which kinds are a waste and what the waste is called. A `feature` row is value work. It is not in the waste list, so a join with that list drops the row on its own. That is what you want for the share of wasted hours, and it is not what you want for the hours of value work.

The block holds these two lists of rows, typed in.

## Tasks

1. **Task 1: the wastes.** `waste_name`, `hours` (total hours of that waste) and `share_pct` (its share of ALL wasted hours, rounded to a whole percent).
2. **Task 2: the value work.** One column, `value_hours`: the hours of every row whose kind is not a waste.
3. **Task 3: the biggest waste.** One row, `waste_name`: the waste with the most hours. If two tie, the one that comes first in the alphabet.
4. **Not checked by the query:** the biggest waste is the one to attack first, but the list says nothing about how. Pick the top waste and write the one measurement you would take next sprint to see whether your change worked.

## What this lab cannot teach

SQL can add up hours that someone labelled. It cannot tell whether the label is right: "rework" and "defects" depend on a person reading the work. The lab takes the `kind` column as given.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
