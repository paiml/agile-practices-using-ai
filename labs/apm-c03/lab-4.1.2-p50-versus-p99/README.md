# Lab: P50 versus P99: What the Average Hides (Course 3, Module 4, lesson 4.1.2)

An average response time is one number for many requests. It hides the shape. Two endpoints can have close averages while one of them makes a few users wait for seconds.

Percentiles show the shape. Sort the response times from fastest to slowest. The p50 (the median) is the value half of the requests beat. The p99 is the value 99 of every 100 requests beat, so it describes the slow tail: 1 request in 100 is at least that slow.

The block builds 100 requests for each of two endpoints in place. With 100 sorted values, p50 is the 50th and p99 is the 99th. The 100th value is the maximum, not the p99.

SQLite has no percentile function built in, so the block numbers the requests another way: a request's place in line is how many requests of its endpoint are at or below its time, and a percentile is the smallest `ms` whose place is at or past the place you ask for (50 for the p50, 99 for the p99).

## Tasks

1. Keep `mean_ms` as the average per endpoint (the block already has it).
2. Compute `p50_ms`: the 50th of the 100 sorted values per endpoint.
3. Compute `p99_ms`: the 99th of the 100 sorted values per endpoint.
4. By the mean, which endpoint is slower? By the p99? Which one do you put on the roadmap, and which number do you quote when you ask for the time?

The block checks the columns `endpoint`, `mean_ms`, `p50_ms` and `p99_ms`.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `requests.tsv` (the 200 rows as a plain tab-separated list, for reference; the in-page block builds its own copy), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
