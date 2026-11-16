-- Lab 2.3.4 check: prints PASS or FAIL.
WITH
exp1(rung, name, seconds, total_seconds) AS (
  VALUES (1, 'format', 2, 2), (2, 'lint', 8, 10), (3, 'type check', 25, 35),
         (4, 'unit tests', 90, 125), (5, 'integration', 420, 545), (6, 'clean-room build', 600, 1145)
),
exp2(name) AS (VALUES ('spell check'), ('old smoke test'))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 6
  AND NOT EXISTS (SELECT rung, name, seconds, total_seconds FROM answer_1 EXCEPT SELECT rung, name, seconds, total_seconds FROM exp1)
  AND NOT EXISTS (SELECT rung, name, seconds, total_seconds FROM exp1 EXCEPT SELECT rung, name, seconds, total_seconds FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 2
  AND NOT EXISTS (SELECT name FROM answer_2 EXCEPT SELECT name FROM exp2)
  AND NOT EXISTS (SELECT name FROM exp2 EXCEPT SELECT name FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 1
  AND (SELECT seconds_saved FROM answer_3) = 305
THEN 'PASS' ELSE 'FAIL' END;
