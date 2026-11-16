# Lab: A KPI and Guardrail (Course 3, Module 3, lesson 3.3.1)

A KPI tells you whether the thing you are pushing is moving. A guardrail tells you whether you are breaking something else to move it. You need both, because a team that is paid for one number will find a way to raise it at the expense of another.

In this lab the KPI is activation: the percent of weekly signups that reach the first useful action. The target is at least 40 percent. The guardrail is support load: tickets opened per signup, which must stay at or below 8 percent. Both limits are written in the block: 40 and 8.

A guardrail outranks the KPI. A week that hits the target while the guardrail is breached is not healthy, it is a problem that looks like a win. So the status has three values: `guardrail breach` (checked first), `kpi miss`, and `healthy`.

The limits are inclusive. A week that lands exactly on a limit is inside it.

## Tasks

1. Compute `activation_pct` and `ticket_pct` as percents of signups.
2. Set `status` to `guardrail breach` when the ticket rate is above its limit, else `kpi miss` when activation is below its target, else `healthy`.
3. Look at the result. Week 2 has the best activation of the four weeks. What do you tell the team, and what would you check before the next release?

The block checks the columns `week`, `activation_pct`, `ticket_pct`, `status`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
