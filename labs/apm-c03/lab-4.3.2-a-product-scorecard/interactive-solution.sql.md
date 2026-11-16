-- Lab 4.3.2 (solution): A Product Scorecard
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: attainment_pct. Higher is better ('up'): value / target. Lower is better ('down'): target / value. In percent.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.metric) = 6 FROM (
    SELECT s.metric AS metric,
-- ===== YOUR WORK 1 of 2: edit only between these lines =====
           CASE s.direction
             WHEN 'up' THEN 100.0 * s.value / s.target
             ELSE 100.0 * s.target / s.value
           END AS attainment_pct
-- ===== END OF YOUR WORK 1 of 2 =====
    FROM (
        SELECT 'Activation rate (%)' AS metric, 42 AS value, 40 AS target, 'up' AS direction
        UNION ALL SELECT 'p99 latency (ms)', 900, 800, 'down'
        UNION ALL SELECT 'Cost per outcome (USD)', 1.45, 1.50, 'down'
        UNION ALL SELECT 'Week-4 retention (%)', 36, 40, 'up'
        UNION ALL SELECT 'NPS', 28, 35, 'up'
        UNION ALL SELECT 'Tickets per 100 users', 4, 5, 'down'
    ) s
  ) a LEFT JOIN (
      SELECT 'Activation rate (%)' AS metric, 105.0 AS attainment_pct
      UNION ALL SELECT 'p99 latency (ms)', 88.9
      UNION ALL SELECT 'Cost per outcome (USD)', 103.4
      UNION ALL SELECT 'Week-4 retention (%)', 90.0
      UNION ALL SELECT 'NPS', 80.0
      UNION ALL SELECT 'Tickets per 100 users', 125.0
  ) e ON e.metric = a.metric AND e.attainment_pct = ROUND(a.attainment_pct, 1))
  AND
  -- Task 2: status, 'green' at 100 or more, 'amber' at 90 or more, else 'red'.
  -- The rows below are the right answer to task 1.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.metric) = 6 FROM (
    SELECT s.metric AS metric,
-- ===== YOUR WORK 2 of 2: edit only between these lines =====
           CASE
             WHEN s.attainment_pct >= 100 THEN 'green'
             WHEN s.attainment_pct >= 90 THEN 'amber'
             ELSE 'red'
           END AS status
-- ===== END OF YOUR WORK 2 of 2 =====
    FROM (
        SELECT 'Activation rate (%)' AS metric, 105.0 AS attainment_pct
        UNION ALL SELECT 'p99 latency (ms)', 88.9
        UNION ALL SELECT 'Cost per outcome (USD)', 103.4
        UNION ALL SELECT 'Week-4 retention (%)', 90.0
        UNION ALL SELECT 'NPS', 80.0
        UNION ALL SELECT 'Tickets per 100 users', 125.0
    ) s
  ) a LEFT JOIN (
      SELECT 'Activation rate (%)' AS metric, 'green' AS status
      UNION ALL SELECT 'p99 latency (ms)', 'red'
      UNION ALL SELECT 'Cost per outcome (USD)', 'green'
      UNION ALL SELECT 'Week-4 retention (%)', 'amber'
      UNION ALL SELECT 'NPS', 'red'
      UNION ALL SELECT 'Tickets per 100 users', 'green'
  ) e ON e.metric = a.metric AND e.status = a.status)
THEN 'PASS' ELSE 'FAIL' END AS result;
