-- Lab 4.3.3 check: prints PASS or FAIL.
WITH
exp1(id, idea, score) AS (
  VALUES (5, 'Weekly metrics review', 5.0), (1, 'Add andon check to CI', 4.0), (7, 'Auto-label stale tickets', 4.0)
),
exp3(id) AS (VALUES (2), (3), (6))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 3
  AND NOT EXISTS (SELECT id, idea, score FROM answer_1 EXCEPT SELECT id, idea, score FROM exp1)
  AND NOT EXISTS (SELECT id, idea, score FROM exp1 EXCEPT SELECT id, idea, score FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 1
  AND (SELECT total_effort FROM answer_2) = 4
  AND (SELECT COUNT(*) FROM answer_3) = 3
  AND NOT EXISTS (SELECT id FROM answer_3 EXCEPT SELECT id FROM exp3)
  AND NOT EXISTS (SELECT id FROM exp3 EXCEPT SELECT id FROM answer_3)
THEN 'PASS' ELSE 'FAIL' END;
