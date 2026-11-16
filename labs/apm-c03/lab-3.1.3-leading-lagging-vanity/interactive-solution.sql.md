-- Lab 3.1.3 (solution): Leading, Lagging and Vanity
-- Press Run. It prints FAIL until your three answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: total_signups, the running total of sign-ups since week 1.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.week) = 6 FROM (
    SELECT w.week AS week,
-- ===== YOUR WORK 1 of 4: edit only between these lines =====
           SUM(s.signups) AS total_signups
-- ===== END OF YOUR WORK 1 of 4 =====
    FROM (
      SELECT DISTINCT week FROM (
        SELECT '2026-W01' AS week, 'organic' AS channel, 100 AS signups, 40 AS activated, 12 AS paid_w4
        UNION ALL SELECT '2026-W01', 'paid_ads', 100, 20, 4
        UNION ALL SELECT '2026-W02', 'organic', 110, 44, 13
        UNION ALL SELECT '2026-W02', 'paid_ads', 90, 18, 3
        UNION ALL SELECT '2026-W03', 'organic', 120, 48, 14
        UNION ALL SELECT '2026-W03', 'paid_ads', 130, 27, 6
        UNION ALL SELECT '2026-W04', 'organic', 120, 48, NULL
        UNION ALL SELECT '2026-W04', 'paid_ads', 380, 27, NULL
        UNION ALL SELECT '2026-W05', 'organic', 125, 50, NULL
        UNION ALL SELECT '2026-W05', 'paid_ads', 475, 40, NULL
        UNION ALL SELECT '2026-W06', 'organic', 130, 52, NULL
        UNION ALL SELECT '2026-W06', 'paid_ads', 670, 44, NULL
      )
    ) w
    JOIN (
        SELECT '2026-W01' AS week, 'organic' AS channel, 100 AS signups, 40 AS activated, 12 AS paid_w4
        UNION ALL SELECT '2026-W01', 'paid_ads', 100, 20, 4
        UNION ALL SELECT '2026-W02', 'organic', 110, 44, 13
        UNION ALL SELECT '2026-W02', 'paid_ads', 90, 18, 3
        UNION ALL SELECT '2026-W03', 'organic', 120, 48, 14
        UNION ALL SELECT '2026-W03', 'paid_ads', 130, 27, 6
        UNION ALL SELECT '2026-W04', 'organic', 120, 48, NULL
        UNION ALL SELECT '2026-W04', 'paid_ads', 380, 27, NULL
        UNION ALL SELECT '2026-W05', 'organic', 125, 50, NULL
        UNION ALL SELECT '2026-W05', 'paid_ads', 475, 40, NULL
        UNION ALL SELECT '2026-W06', 'organic', 130, 52, NULL
        UNION ALL SELECT '2026-W06', 'paid_ads', 670, 44, NULL
    ) s
-- ===== YOUR WORK 2 of 4: edit only between these lines =====
    ON s.week <= w.week
-- ===== END OF YOUR WORK 2 of 4 =====
    GROUP BY w.week
  ) a LEFT JOIN (
      SELECT '2026-W01' AS week, 200 AS total_signups
      UNION ALL SELECT '2026-W02', 400
      UNION ALL SELECT '2026-W03', 650
      UNION ALL SELECT '2026-W04', 1150
      UNION ALL SELECT '2026-W05', 1750
      UNION ALL SELECT '2026-W06', 2550
  ) e ON e.week = a.week AND e.total_signups = a.total_signups)
  AND
  -- Task 2: activation_pct, the week's activated accounts over the week's sign-ups, in percent.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.week) = 6 FROM (
    SELECT s.week AS week,
-- ===== YOUR WORK 3 of 4: edit only between these lines =====
           100.0 * SUM(s.activated) / SUM(s.signups) AS activation_pct
-- ===== END OF YOUR WORK 3 of 4 =====
    FROM (
        SELECT '2026-W01' AS week, 'organic' AS channel, 100 AS signups, 40 AS activated, 12 AS paid_w4
        UNION ALL SELECT '2026-W01', 'paid_ads', 100, 20, 4
        UNION ALL SELECT '2026-W02', 'organic', 110, 44, 13
        UNION ALL SELECT '2026-W02', 'paid_ads', 90, 18, 3
        UNION ALL SELECT '2026-W03', 'organic', 120, 48, 14
        UNION ALL SELECT '2026-W03', 'paid_ads', 130, 27, 6
        UNION ALL SELECT '2026-W04', 'organic', 120, 48, NULL
        UNION ALL SELECT '2026-W04', 'paid_ads', 380, 27, NULL
        UNION ALL SELECT '2026-W05', 'organic', 125, 50, NULL
        UNION ALL SELECT '2026-W05', 'paid_ads', 475, 40, NULL
        UNION ALL SELECT '2026-W06', 'organic', 130, 52, NULL
        UNION ALL SELECT '2026-W06', 'paid_ads', 670, 44, NULL
    ) s
    GROUP BY s.week
  ) a LEFT JOIN (
      SELECT '2026-W01' AS week, 30.0 AS activation_pct
      UNION ALL SELECT '2026-W02', 31.0
      UNION ALL SELECT '2026-W03', 30.0
      UNION ALL SELECT '2026-W04', 15.0
      UNION ALL SELECT '2026-W05', 15.0
      UNION ALL SELECT '2026-W06', 12.0
  ) e ON e.week = a.week AND e.activation_pct = ROUND(a.activation_pct, 1))
  AND
  -- Task 3: paid_w4_pct, the week's paying accounts over the week's sign-ups, in percent.
  -- It stays empty (NULL) where the data is not in yet.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.week) = 6 FROM (
    SELECT s.week AS week,
-- ===== YOUR WORK 4 of 4: edit only between these lines =====
           100.0 * SUM(s.paid_w4) / SUM(s.signups) AS paid_w4_pct
-- ===== END OF YOUR WORK 4 of 4 =====
    FROM (
        SELECT '2026-W01' AS week, 'organic' AS channel, 100 AS signups, 40 AS activated, 12 AS paid_w4
        UNION ALL SELECT '2026-W01', 'paid_ads', 100, 20, 4
        UNION ALL SELECT '2026-W02', 'organic', 110, 44, 13
        UNION ALL SELECT '2026-W02', 'paid_ads', 90, 18, 3
        UNION ALL SELECT '2026-W03', 'organic', 120, 48, 14
        UNION ALL SELECT '2026-W03', 'paid_ads', 130, 27, 6
        UNION ALL SELECT '2026-W04', 'organic', 120, 48, NULL
        UNION ALL SELECT '2026-W04', 'paid_ads', 380, 27, NULL
        UNION ALL SELECT '2026-W05', 'organic', 125, 50, NULL
        UNION ALL SELECT '2026-W05', 'paid_ads', 475, 40, NULL
        UNION ALL SELECT '2026-W06', 'organic', 130, 52, NULL
        UNION ALL SELECT '2026-W06', 'paid_ads', 670, 44, NULL
    ) s
    GROUP BY s.week
  ) a LEFT JOIN (
      SELECT '2026-W01' AS week, 8.0 AS paid_w4_pct
      UNION ALL SELECT '2026-W02', 8.0
      UNION ALL SELECT '2026-W03', 8.0
      UNION ALL SELECT '2026-W04', NULL
      UNION ALL SELECT '2026-W05', NULL
      UNION ALL SELECT '2026-W06', NULL
  ) e ON e.week = a.week AND e.paid_w4_pct IS ROUND(a.paid_w4_pct, 1))
THEN 'PASS' ELSE 'FAIL' END AS result;
