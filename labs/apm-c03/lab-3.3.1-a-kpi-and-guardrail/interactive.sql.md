-- Lab 3.3.1: A KPI and Guardrail
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: activation_pct and ticket_pct, each as a percent of the week's signups.
  (SELECT COUNT(*) = 4 AND COUNT(DISTINCT e.week) = 4 FROM (
    SELECT w.week AS week,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           -- TODO: activated as a percent of signups (this placeholder is the activated count). Keep the comma.
           w.activated AS activation_pct,
-- ===== END OF YOUR WORK 1 of 3 =====
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
           -- TODO: tickets as a percent of signups (this placeholder is the ticket count).
           w.tickets AS ticket_pct
-- ===== END OF YOUR WORK 2 of 3 =====
    FROM (
        SELECT 1 AS week, 200 AS signups, 80 AS activated, 10 AS tickets
        UNION ALL SELECT 2, 250, 125, 25
        UNION ALL SELECT 3, 220, 66, 11
        UNION ALL SELECT 4, 300, 150, 24
    ) w
  ) a LEFT JOIN (
      SELECT 1 AS week, 40.0 AS activation_pct, 5.0 AS ticket_pct
      UNION ALL SELECT 2, 50.0, 10.0
      UNION ALL SELECT 3, 30.0, 5.0
      UNION ALL SELECT 4, 50.0, 8.0
  ) e ON e.week = a.week AND e.activation_pct = ROUND(a.activation_pct, 1) AND e.ticket_pct = ROUND(a.ticket_pct, 1))
  AND
  -- Task 2: status. The limits are inclusive: the KPI target is 40 percent activation, the guardrail is 8 percent tickets.
  -- The guardrail is checked first, then the KPI, and a week inside both is healthy.
  (SELECT COUNT(*) = 4 AND COUNT(DISTINCT e.week) = 4 FROM (
    SELECT w.week AS week,
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
           -- TODO: 'guardrail breach', 'kpi miss' or 'healthy' (this placeholder says healthy for every week).
           'healthy' AS status
-- ===== END OF YOUR WORK 3 of 3 =====
    FROM (
        SELECT 1 AS week, 200 AS signups, 80 AS activated, 10 AS tickets
        UNION ALL SELECT 2, 250, 125, 25
        UNION ALL SELECT 3, 220, 66, 11
        UNION ALL SELECT 4, 300, 150, 24
    ) w, (SELECT 40 AS min_activation_pct, 8 AS max_ticket_pct) t
  ) a LEFT JOIN (
      SELECT 1 AS week, 'healthy' AS status
      UNION ALL SELECT 2, 'guardrail breach'
      UNION ALL SELECT 3, 'kpi miss'
      UNION ALL SELECT 4, 'healthy'
  ) e ON e.week = a.week AND e.status = a.status)
THEN 'PASS' ELSE 'FAIL' END AS result;
