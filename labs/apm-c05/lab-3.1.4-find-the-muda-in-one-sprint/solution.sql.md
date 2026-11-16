-- Lab 3.1.4 solution.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT w.waste_name AS waste_name,
       SUM(l.hours) AS hours,
       ROUND(SUM(l.hours) * 100.0 / (SELECT SUM(l2.hours) FROM work_log AS l2 JOIN waste AS w2 ON w2.kind = l2.kind), 0) AS share_pct
FROM work_log AS l
JOIN waste AS w ON w.kind = l.kind
GROUP BY w.waste_name;

CREATE VIEW answer_2 AS
SELECT SUM(hours) AS value_hours
FROM work_log
WHERE kind NOT IN (SELECT kind FROM waste);

CREATE VIEW answer_3 AS
SELECT waste_name
FROM answer_1
ORDER BY hours DESC, waste_name
LIMIT 1;
