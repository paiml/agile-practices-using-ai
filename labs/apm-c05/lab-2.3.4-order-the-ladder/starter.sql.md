-- Lab 2.3.4 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: the ladder. Cheapest check first. rung is 1 for the cheapest. total_seconds is the
-- time to climb from the first rung to this one, added up. A check that catches nothing is not a rung.
-- Ties in seconds go to the lower name.
-- TODO: this lists every check, in table order, and the running total is just its own seconds.
CREATE VIEW answer_1 AS
SELECT ROW_NUMBER() OVER (ORDER BY name) AS rung,
       name,
       seconds,
       seconds AS total_seconds
FROM gate;

-- Task 2: the checks that catch nothing. Return name.
-- TODO: this lists every check.
CREATE VIEW answer_2 AS
SELECT name
FROM gate;

-- Task 3: the seconds the pipeline saves by dropping them. One column: seconds_saved.
-- TODO: this adds up every check.
CREATE VIEW answer_3 AS
SELECT SUM(seconds) AS seconds_saved
FROM gate;
