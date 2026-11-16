-- Lab 4.2.2: Cost per Outcome
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: first_cpo and last_cpo, the cost per outcome (cost / outcomes) in month 1 and in month 3. Month 2 is not used.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.feature) = 3 FROM (
    SELECT f.feature AS feature,
-- ===== YOUR WORK 1 of 4: edit only between these lines =====
           -- TODO: cost divided by outcomes in month 1 (this placeholder is the cost). Keep the comma.
           f.cost_usd AS first_cpo,
-- ===== END OF YOUR WORK 1 of 4 =====
-- ===== YOUR WORK 2 of 4: edit only between these lines =====
           -- TODO: cost divided by outcomes in month 3 (this placeholder is the cost).
           l.cost_usd AS last_cpo
-- ===== END OF YOUR WORK 2 of 4 =====
    FROM (
        SELECT 'AI triage' AS feature, 1 AS month, 1000 AS cost_usd, 500 AS outcomes
        UNION ALL SELECT 'AI triage', 2, 1400, 800
        UNION ALL SELECT 'AI triage', 3, 1800, 1200
        UNION ALL SELECT 'Smart search', 1, 800, 800
        UNION ALL SELECT 'Smart search', 2, 850, 700
        UNION ALL SELECT 'Smart search', 3, 900, 600
        UNION ALL SELECT 'Bulk export', 1, 300, 100
        UNION ALL SELECT 'Bulk export', 2, 320, 110
        UNION ALL SELECT 'Bulk export', 3, 300, 100
    ) f
    JOIN (
        SELECT 'AI triage' AS feature, 1 AS month, 1000 AS cost_usd, 500 AS outcomes
        UNION ALL SELECT 'AI triage', 2, 1400, 800
        UNION ALL SELECT 'AI triage', 3, 1800, 1200
        UNION ALL SELECT 'Smart search', 1, 800, 800
        UNION ALL SELECT 'Smart search', 2, 850, 700
        UNION ALL SELECT 'Smart search', 3, 900, 600
        UNION ALL SELECT 'Bulk export', 1, 300, 100
        UNION ALL SELECT 'Bulk export', 2, 320, 110
        UNION ALL SELECT 'Bulk export', 3, 300, 100
    ) l ON l.feature = f.feature
-- ===== YOUR WORK 3 of 4: edit only between these lines =====
    -- TODO: look only at month 1 for f and month 3 for l (this placeholder looks at month 1 twice).
    WHERE f.month = 1 AND l.month = 1
-- ===== END OF YOUR WORK 3 of 4 =====
  ) a LEFT JOIN (
      SELECT 'AI triage' AS feature, 2.0 AS first_cpo, 1.5 AS last_cpo
      UNION ALL SELECT 'Smart search', 1.0, 1.5
      UNION ALL SELECT 'Bulk export', 3.0, 3.0
  ) e ON e.feature = a.feature AND e.first_cpo = ROUND(a.first_cpo, 2) AND e.last_cpo = ROUND(a.last_cpo, 2))
  AND
  -- Task 2: decision, 'scale' when the cost per outcome fell, 'review' when it rose, else 'hold'.
  -- The rows below are the right answer to task 1.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.feature) = 3 FROM (
    SELECT s.feature AS feature,
-- ===== YOUR WORK 4 of 4: edit only between these lines =====
           -- TODO: 'scale' if it fell, 'review' if it rose, else 'hold' (this placeholder says hold).
           'hold' AS decision
-- ===== END OF YOUR WORK 4 of 4 =====
    FROM (
        SELECT 'AI triage' AS feature, 2.0 AS first_cpo, 1.5 AS last_cpo
        UNION ALL SELECT 'Smart search', 1.0, 1.5
        UNION ALL SELECT 'Bulk export', 3.0, 3.0
    ) s
  ) a LEFT JOIN (
      SELECT 'AI triage' AS feature, 'scale' AS decision
      UNION ALL SELECT 'Smart search', 'review'
      UNION ALL SELECT 'Bulk export', 'hold'
  ) e ON e.feature = a.feature AND e.decision = a.decision)
THEN 'PASS' ELSE 'FAIL' END AS result;
