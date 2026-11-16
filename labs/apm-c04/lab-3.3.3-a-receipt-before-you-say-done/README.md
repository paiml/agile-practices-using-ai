# Lab: A Receipt Before You Say Done (Course 4, Module 3, lesson 3.3.3)

**Objective (O3):** Require a receipt of evidence before accepting that an agent is done.

An agent may say "done" only with a receipt: the command it ran, an exit code of 0, and output that is not empty. A claim with no receipt is not done, however confident the sentence is. A receipt with a failing exit code is a receipt that says "not done". And a receipt with an exit code but no command, or a command with no output, proves nothing: you cannot tell what was run or what it showed.

In this lab each claim is a row: the agent, whether it said "done", and the command, exit code and output it attached. A command or output is missing when it is NULL, an empty string or only spaces. An exit code is missing when it is NULL, and NULL is not 0 and not "different from 0" either: a comparison with NULL is never true, so a check that only asks `exit_code <> 0` lets it through.

The block holds one list of rows, typed in: the claims (the block calls them `c`, with `id`, `agent`, `said_done`, `command`, `exit_code` and `output`).

## Tasks

1. **Task 1: claims with no valid receipt.** For each claim with `said_done = 1` that fails, the `claim_id` and the first `reason` that applies, in this order: `no command`, `no exit code`, `failed`, `no output`.
2. **Task 2: accepted claims.** Each claim that said "done" and is not in the Task 1 rows.
3. **Task 3: a record per agent.** For each agent that claimed "done" at least once: `agent`, `claimed` (how many "done" claims) and `accepted` (how many of them were accepted).
4. **Task 4: agents with nothing accepted.** Each agent that claimed "done" and had no claim accepted.
5. **Not checked by the query:** claim 1 is accepted. Say what else you would want in the receipt before you merged on it (which test, how many ran, which commit). Say which column the rows would need for one of those.

## What this lab cannot teach

SQL can check that a receipt has its three parts and that the exit code is 0. It cannot tell whether the command was the right one, whether the output is real, or whether the receipt was copied from another run. A receipt the agent typed in itself looks exactly like one the machine produced. That is why the course tells you to run the command yourself or read it from CI.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
