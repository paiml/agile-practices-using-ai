-- Lab 1.3.4 check: prints PASS or FAIL.
WITH
exp1(incident, depth, root_cause, changeable) AS (
  VALUES ('build went red on Friday', 3, 'we do not control the registry', 0),
         ('release took 30 hours', 3, 'the workflow cancels in-progress runs on a new push', 1),
         ('servers crashed at random', 5, 'an out-of-memory error caused a kernel panic', 1)
),
exp2(incident) AS (VALUES ('build went red on Friday')),
exp3(incident) AS (VALUES ('servers crashed at random'))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 3
  AND NOT EXISTS (SELECT incident, depth, root_cause, changeable FROM answer_1 EXCEPT SELECT incident, depth, root_cause, changeable FROM exp1)
  AND NOT EXISTS (SELECT incident, depth, root_cause, changeable FROM exp1 EXCEPT SELECT incident, depth, root_cause, changeable FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 1
  AND NOT EXISTS (SELECT incident FROM answer_2 EXCEPT SELECT incident FROM exp2)
  AND (SELECT COUNT(*) FROM answer_3) = 1
  AND NOT EXISTS (SELECT incident FROM answer_3 EXCEPT SELECT incident FROM exp3)
THEN 'PASS' ELSE 'FAIL' END;
