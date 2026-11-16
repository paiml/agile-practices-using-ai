-- Lab 2.3.4 planted wrong answer: the ladder keeps the checks that catch nothing, so the spell
-- check and the old smoke test take a rung and the rungs and running totals all shift.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT ROW_NUMBER() OVER (ORDER BY seconds, name) AS rung,
       name,
       seconds,
       SUM(seconds) OVER (ORDER BY seconds, name) AS total_seconds
FROM gate;

CREATE VIEW answer_2 AS
SELECT name
FROM gate
WHERE catches = 0;

CREATE VIEW answer_3 AS
SELECT SUM(seconds) AS seconds_saved
FROM gate
WHERE catches = 0;
