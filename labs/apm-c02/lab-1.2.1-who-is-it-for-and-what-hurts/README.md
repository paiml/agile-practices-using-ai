# Lab: Who Is It For, and What Hurts (Course 2, Module 1, lesson 1.2.1)

**Objective (O1):** Describe what you are building and for whom, and scope an MVP with a not-now list. This lab serves the first half (M1a): name the user and the pain in one sentence.

You have an invoicing app and one month of support tickets. Before you decide what to build next, you need one sentence: who the product is for, and what hurts them most. The tickets can give you that sentence, if you count the right thing.

The trap is volume. One user who writes in nine times looks like nine problems. A product is for a group of people who share a pain, so you count people. In this data, counting tickets and counting people give two different answers, and only one of them is a market.

There are two lists of rows, typed into the block. The customers (the block calls them `u`) have one row each, with a `segment` (freelancer, agency or shop). The support tickets (`t`) have one row each, with the `complaint` that support assigned and the `minutes_lost` the user reported.

The block prints `FAIL` until all three parts are right. Its comments mark them `Task 1`, `Task 2` and `Task 3`.

## Tasks

1. **Task 1: tickets per segment.** The block joins the tickets to the customers. In YOUR WORK 1, replace the placeholder `0 AS tickets` with the count of tickets in each segment. Note which segment writes in most.
2. **Task 2: people per complaint.** For each segment and complaint the block already gives the number of tickets. In YOUR WORK 2, replace the two placeholders with the number of different users who filed them (`users`) and the total minutes lost (`minutes_lost`).
3. **Task 3: the user and the pain.** The block lists the six right rows of Task 2 and keeps the first one. In YOUR WORK 3, choose the column that ranks the rows so the row the product should be built for comes first. Ranking by tickets and ranking by users give different rows. Decide which one you would defend to your team.
4. **Not checked by the query:** write the sentence. "We are building for ___, who lose ___ to ___." Fill it in from the row Task 3 keeps, and put its minutes lost into hours.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
