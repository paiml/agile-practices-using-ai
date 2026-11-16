-- Lab 3.1.4 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: the hours of each waste, and its share of ALL wasted hours (whole percent).
-- A 'feature' row is value work: it is not in the waste table, so it is not wasted hours.
-- TODO: this groups by kind and gives the share of ALL hours, features included.
CREATE VIEW answer_1 AS
SELECT w.waste_name AS waste_name,
       SUM(l.hours) AS hours,
       ROUND(SUM(l.hours) * 100.0 / (SELECT SUM(hours) FROM work_log), 0) AS share_pct
FROM work_log AS l
LEFT JOIN waste AS w ON w.kind = l.kind
GROUP BY l.kind;

-- Task 2: the hours of value work (kinds that are not a waste). One column: value_hours.
-- TODO: this adds up every hour.
CREATE VIEW answer_2 AS
SELECT SUM(hours) AS value_hours
FROM work_log;

-- Task 3: the biggest waste. One row: waste_name. If two wastes tie, the lower name in the alphabet.
-- TODO: this returns the smallest waste.
CREATE VIEW answer_3 AS
SELECT waste_name
FROM answer_1
ORDER BY hours, waste_name
LIMIT 1;
