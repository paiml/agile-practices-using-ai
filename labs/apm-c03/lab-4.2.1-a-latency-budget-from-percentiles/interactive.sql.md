-- Lab 4.2.1: A Latency Budget from Percentiles
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
-- The per-step rows below hold each step's time at p50 and at p99, one row per percentile and step.
SELECT CASE WHEN
  -- Task 1: total_ms, the sum of the step times at each percentile.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.percentile) = 2 FROM (
    SELECT s.percentile AS percentile,
-- ===== YOUR WORK 1 of 4: edit only between these lines =====
           -- TODO: add up the step times (this placeholder counts the steps).
           COUNT(*) AS total_ms
-- ===== END OF YOUR WORK 1 of 4 =====
    FROM (
        SELECT 'p50' AS percentile, step, p50_ms AS ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
          UNION ALL SELECT 'p99', step, p99_ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
    ) s
    GROUP BY s.percentile
  ) a LEFT JOIN (
      SELECT 'p50' AS percentile, 280 AS total_ms
      UNION ALL SELECT 'p99', 660
  ) e ON e.percentile = a.percentile AND e.total_ms = a.total_ms)
  AND
  -- Task 2: headroom_ms (the budget minus the total) and status ('ok' at or below the budget, else 'over').
  -- The rows below are the right answer to task 1. The budgets are 280 ms at p50 and 600 ms at p99.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.percentile) = 2 FROM (
    SELECT t.percentile AS percentile,
-- ===== YOUR WORK 2 of 4: edit only between these lines =====
           -- TODO: the budget minus the total (this placeholder is zero). Keep the comma.
           0 AS headroom_ms,
-- ===== END OF YOUR WORK 2 of 4 =====
-- ===== YOUR WORK 3 of 4: edit only between these lines =====
           -- TODO: 'ok' when the total is at or below the budget, else 'over' (this placeholder says ok).
           'ok' AS status
-- ===== END OF YOUR WORK 3 of 4 =====
    FROM (
        SELECT 'p50' AS percentile, 280 AS total_ms
        UNION ALL SELECT 'p99', 660
    ) t
    JOIN (
        SELECT 'p50' AS percentile, 280 AS budget_ms UNION ALL SELECT 'p99', 600
    ) b ON b.percentile = t.percentile
  ) a LEFT JOIN (
      SELECT 'p50' AS percentile, 0 AS headroom_ms, 'ok' AS status
      UNION ALL SELECT 'p99', -60, 'over'
  ) e ON e.percentile = a.percentile AND e.headroom_ms = a.headroom_ms AND e.status = a.status)
  AND
  -- Task 3: biggest_step, the step that takes the most time at each percentile.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.percentile) = 2 FROM (
    SELECT m.percentile AS percentile,
      (SELECT s.step FROM (
        SELECT 'p50' AS percentile, step, p50_ms AS ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
          UNION ALL SELECT 'p99', step, p99_ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
      ) s
      WHERE s.percentile = m.percentile
-- ===== YOUR WORK 4 of 4: edit only between these lines =====
        -- TODO: keep only the row whose time is the biggest of its percentile (this placeholder keeps every row).
        AND 1 = 1
-- ===== END OF YOUR WORK 4 of 4 =====
      ) AS biggest_step
    FROM (
      SELECT x.percentile AS percentile, MAX(x.ms) AS biggest_ms FROM (
        SELECT 'p50' AS percentile, step, p50_ms AS ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
          UNION ALL SELECT 'p99', step, p99_ms FROM (SELECT 'auth' AS step, 15 AS p50_ms, 40 AS p99_ms UNION ALL SELECT 'search', 100, 380 UNION ALL SELECT 'ranking', 120, 150 UNION ALL SELECT 'render', 45, 90)
      ) x
      GROUP BY x.percentile
    ) m
  ) a LEFT JOIN (
      SELECT 'p50' AS percentile, 'ranking' AS biggest_step
      UNION ALL SELECT 'p99', 'search'
  ) e ON e.percentile = a.percentile AND e.biggest_step = a.biggest_step)
THEN 'PASS' ELSE 'FAIL' END AS result;
