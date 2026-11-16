-- Lab 2.2.2: Set the Cut Line
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: cumulative_weeks, the running total of effort_weeks in priority order.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.item) = 6 FROM (
    SELECT b.item AS item,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           -- TODO: add up the effort of the items you joined (this placeholder is the effort of one item).
           b.effort_weeks AS cumulative_weeks
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
        SELECT 1 AS priority, 'Fix signup bug' AS item, 3 AS effort_weeks
        UNION ALL SELECT 2, 'AI ticket triage', 4
        UNION ALL SELECT 3, 'Bulk export', 3
        UNION ALL SELECT 4, 'Usage-based pricing', 5
        UNION ALL SELECT 5, 'Dark mode', 2
        UNION ALL SELECT 6, 'Smart search', 3
    ) b
    JOIN (
        SELECT 1 AS priority, 'Fix signup bug' AS item, 3 AS effort_weeks
        UNION ALL SELECT 2, 'AI ticket triage', 4
        UNION ALL SELECT 3, 'Bulk export', 3
        UNION ALL SELECT 4, 'Usage-based pricing', 5
        UNION ALL SELECT 5, 'Dark mode', 2
        UNION ALL SELECT 6, 'Smart search', 3
    ) c
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
    -- TODO: join each item to itself and to every item ranked above it.
    ON c.priority = b.priority
-- ===== END OF YOUR WORK 2 of 3 =====
    GROUP BY b.item
  ) a LEFT JOIN (
      SELECT 'Fix signup bug' AS item, 3 AS cumulative_weeks
      UNION ALL SELECT 'AI ticket triage', 7
      UNION ALL SELECT 'Bulk export', 10
      UNION ALL SELECT 'Usage-based pricing', 15
      UNION ALL SELECT 'Dark mode', 17
      UNION ALL SELECT 'Smart search', 20
  ) e ON e.item = a.item AND e.cumulative_weeks = a.cumulative_weeks)
  AND
  -- Task 2: decision, 'ship' while the running total fits the capacity of 10 engineer-weeks, else 'cut'.
  -- The rows below are the right answer to task 1.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.item) = 6 FROM (
    SELECT s.item AS item,
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
           -- TODO: 'ship' while the running total is at or below the capacity, else 'cut'.
           'ship' AS decision
-- ===== END OF YOUR WORK 3 of 3 =====
    FROM (
        SELECT 'Fix signup bug' AS item, 3 AS cumulative_weeks
        UNION ALL SELECT 'AI ticket triage', 7
        UNION ALL SELECT 'Bulk export', 10
        UNION ALL SELECT 'Usage-based pricing', 15
        UNION ALL SELECT 'Dark mode', 17
        UNION ALL SELECT 'Smart search', 20
    ) s, (SELECT 10 AS weeks) k
  ) a LEFT JOIN (
      SELECT 'Fix signup bug' AS item, 'ship' AS decision
      UNION ALL SELECT 'AI ticket triage', 'ship'
      UNION ALL SELECT 'Bulk export', 'ship'
      UNION ALL SELECT 'Usage-based pricing', 'cut'
      UNION ALL SELECT 'Dark mode', 'cut'
      UNION ALL SELECT 'Smart search', 'cut'
  ) e ON e.item = a.item AND e.decision = a.decision)
THEN 'PASS' ELSE 'FAIL' END AS result;
