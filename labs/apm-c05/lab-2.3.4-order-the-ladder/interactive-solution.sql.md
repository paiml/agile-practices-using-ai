-- Lab 2.3.4 (solution): Order the Verification Ladder
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the ladder. Cheapest check first. A check that catches nothing is not a rung.
  -- Ties in seconds go to the lower name.
  -- Return name, seconds, rung (1 for the cheapest) and total_seconds (the seconds of this rung
  -- and every rung below it, added up). The order is the rung number, not the order of the rows.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.rung) = 6 FROM (
    SELECT g.name AS name,
           g.seconds AS seconds,
-- ===== YOUR WORK 1 of 5: edit only between these lines =====
           COUNT(*) AS rung,
           SUM(h.seconds) AS total_seconds
-- ===== END OF YOUR WORK 1 of 5 =====
    FROM (
      SELECT 'format' AS name, 2 AS seconds, 12 AS catches
      UNION ALL SELECT 'lint', 8, 20
      UNION ALL SELECT 'type check', 25, 15
      UNION ALL SELECT 'unit tests', 90, 30
      UNION ALL SELECT 'integration', 420, 12
      UNION ALL SELECT 'clean-room build', 600, 6
      UNION ALL SELECT 'spell check', 5, 0
      UNION ALL SELECT 'old smoke test', 300, 0
    ) g,
    (
      SELECT 'format' AS name, 2 AS seconds, 12 AS catches
      UNION ALL SELECT 'lint', 8, 20
      UNION ALL SELECT 'type check', 25, 15
      UNION ALL SELECT 'unit tests', 90, 30
      UNION ALL SELECT 'integration', 420, 12
      UNION ALL SELECT 'clean-room build', 600, 6
      UNION ALL SELECT 'spell check', 5, 0
      UNION ALL SELECT 'old smoke test', 300, 0
    ) h
-- ===== YOUR WORK 2 of 5: edit only between these lines =====
    WHERE g.catches > 0 AND h.catches > 0
      AND (h.seconds < g.seconds OR (h.seconds = g.seconds AND h.name <= g.name))
-- ===== END OF YOUR WORK 2 of 5 =====
    GROUP BY g.name
  ) a LEFT JOIN (
      SELECT 1 AS rung, 'format' AS name, 2 AS seconds, 2 AS total_seconds
      UNION ALL SELECT 2, 'lint', 8, 10
      UNION ALL SELECT 3, 'type check', 25, 35
      UNION ALL SELECT 4, 'unit tests', 90, 125
      UNION ALL SELECT 5, 'integration', 420, 545
      UNION ALL SELECT 6, 'clean-room build', 600, 1145
  ) e ON e.rung = a.rung AND e.name = a.name AND e.seconds = a.seconds
     AND e.total_seconds = a.total_seconds)
  AND
  -- Task 2: the checks that catch nothing. Return name.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.name) = 2 FROM (
    SELECT g.name AS name
    FROM (
      SELECT 'format' AS name, 2 AS seconds, 12 AS catches
      UNION ALL SELECT 'lint', 8, 20
      UNION ALL SELECT 'type check', 25, 15
      UNION ALL SELECT 'unit tests', 90, 30
      UNION ALL SELECT 'integration', 420, 12
      UNION ALL SELECT 'clean-room build', 600, 6
      UNION ALL SELECT 'spell check', 5, 0
      UNION ALL SELECT 'old smoke test', 300, 0
    ) g
-- ===== YOUR WORK 3 of 5: edit only between these lines =====
    WHERE g.catches = 0
-- ===== END OF YOUR WORK 3 of 5 =====
  ) a LEFT JOIN (
      SELECT 'spell check' AS name
      UNION ALL SELECT 'old smoke test'
  ) e ON e.name = a.name)
  AND
  -- Task 3: the seconds the pipeline saves per run by dropping those checks. One column: seconds_saved.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.seconds_saved) = 1 FROM (
    SELECT
-- ===== YOUR WORK 4 of 5: edit only between these lines =====
    SUM(g.seconds) AS seconds_saved
-- ===== END OF YOUR WORK 4 of 5 =====
    FROM (
      SELECT 'format' AS name, 2 AS seconds, 12 AS catches
      UNION ALL SELECT 'lint', 8, 20
      UNION ALL SELECT 'type check', 25, 15
      UNION ALL SELECT 'unit tests', 90, 30
      UNION ALL SELECT 'integration', 420, 12
      UNION ALL SELECT 'clean-room build', 600, 6
      UNION ALL SELECT 'spell check', 5, 0
      UNION ALL SELECT 'old smoke test', 300, 0
    ) g
-- ===== YOUR WORK 5 of 5: edit only between these lines =====
    WHERE g.catches = 0
-- ===== END OF YOUR WORK 5 of 5 =====
  ) a LEFT JOIN (
      SELECT 305 AS seconds_saved
  ) e ON e.seconds_saved = a.seconds_saved)
THEN 'PASS' ELSE 'FAIL' END AS result;
