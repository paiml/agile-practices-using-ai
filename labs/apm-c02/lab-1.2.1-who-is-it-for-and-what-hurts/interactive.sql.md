-- Lab 1.2.1: Who Is It For, and What Hurts
-- Press Run. It prints FAIL until your three answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: tickets per segment.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.segment) = 3 FROM (
    SELECT u.segment AS segment,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           -- TODO: count the tickets (the 0 is a placeholder).
           0 AS tickets
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
      SELECT 1 AS id, 1 AS user_id, '2026-02-02' AS opened_on, 'chasing late payments' AS complaint, 180 AS minutes_lost
      UNION ALL SELECT 2, 9, '2026-02-02', 'login', 20
      UNION ALL SELECT 3, 2, '2026-02-03', 'chasing late payments', 180
      UNION ALL SELECT 4, 9, '2026-02-03', 'login', 20
      UNION ALL SELECT 5, 13, '2026-02-04', 'tax export', 120
      UNION ALL SELECT 6, 3, '2026-02-05', 'chasing late payments', 180
      UNION ALL SELECT 7, 9, '2026-02-05', 'login', 20
      UNION ALL SELECT 8, 7, '2026-02-06', 'tax export', 120
      UNION ALL SELECT 9, 9, '2026-02-09', 'login', 20
      UNION ALL SELECT 10, 4, '2026-02-09', 'chasing late payments', 180
      UNION ALL SELECT 11, 11, '2026-02-10', 'recurring invoices', 60
      UNION ALL SELECT 12, 9, '2026-02-10', 'login', 20
      UNION ALL SELECT 13, 5, '2026-02-11', 'chasing late payments', 180
      UNION ALL SELECT 14, 14, '2026-02-11', 'tax export', 120
      UNION ALL SELECT 15, 9, '2026-02-12', 'login', 20
      UNION ALL SELECT 16, 10, '2026-02-12', 'login', 20
      UNION ALL SELECT 17, 6, '2026-02-13', 'chasing late payments', 180
      UNION ALL SELECT 18, 9, '2026-02-16', 'login', 20
      UNION ALL SELECT 19, 8, '2026-02-16', 'tax export', 120
      UNION ALL SELECT 20, 12, '2026-02-17', 'recurring invoices', 60
      UNION ALL SELECT 21, 9, '2026-02-17', 'login', 20
      UNION ALL SELECT 22, 15, '2026-02-18', 'tax export', 120
      UNION ALL SELECT 23, 1, '2026-02-19', 'chasing late payments', 180
      UNION ALL SELECT 24, 9, '2026-02-19', 'login', 20
      UNION ALL SELECT 25, 16, '2026-02-20', 'chasing late payments', 180
    ) t
    JOIN (
      SELECT 1 AS id, 'freelancer' AS segment, 'paid' AS plan
      UNION ALL SELECT 2, 'freelancer', 'free'
      UNION ALL SELECT 3, 'freelancer', 'paid'
      UNION ALL SELECT 4, 'freelancer', 'free'
      UNION ALL SELECT 5, 'freelancer', 'paid'
      UNION ALL SELECT 6, 'freelancer', 'free'
      UNION ALL SELECT 7, 'freelancer', 'paid'
      UNION ALL SELECT 8, 'freelancer', 'free'
      UNION ALL SELECT 9, 'agency', 'paid'
      UNION ALL SELECT 10, 'agency', 'paid'
      UNION ALL SELECT 11, 'agency', 'paid'
      UNION ALL SELECT 12, 'agency', 'free'
      UNION ALL SELECT 13, 'shop', 'paid'
      UNION ALL SELECT 14, 'shop', 'free'
      UNION ALL SELECT 15, 'shop', 'free'
      UNION ALL SELECT 16, 'shop', 'paid'
    ) u ON u.id = t.user_id
    GROUP BY u.segment
  ) a LEFT JOIN (
      SELECT 'agency' AS segment, 12 AS tickets
      UNION ALL SELECT 'freelancer', 9
      UNION ALL SELECT 'shop', 4
  ) e ON e.segment = a.segment AND e.tickets = a.tickets)
  AND
  -- Task 2: one row per segment and complaint.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.segment || '/' || e.complaint) = 6 FROM (
    SELECT u.segment   AS segment,
           t.complaint AS complaint,
           COUNT(*)    AS tickets,
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
           -- TODO: users = how many different people complained,
           -- minutes_lost = the total minutes lost.
           0 AS users,
           0 AS minutes_lost
-- ===== END OF YOUR WORK 2 of 3 =====
    FROM (
      SELECT 1 AS id, 1 AS user_id, '2026-02-02' AS opened_on, 'chasing late payments' AS complaint, 180 AS minutes_lost
      UNION ALL SELECT 2, 9, '2026-02-02', 'login', 20
      UNION ALL SELECT 3, 2, '2026-02-03', 'chasing late payments', 180
      UNION ALL SELECT 4, 9, '2026-02-03', 'login', 20
      UNION ALL SELECT 5, 13, '2026-02-04', 'tax export', 120
      UNION ALL SELECT 6, 3, '2026-02-05', 'chasing late payments', 180
      UNION ALL SELECT 7, 9, '2026-02-05', 'login', 20
      UNION ALL SELECT 8, 7, '2026-02-06', 'tax export', 120
      UNION ALL SELECT 9, 9, '2026-02-09', 'login', 20
      UNION ALL SELECT 10, 4, '2026-02-09', 'chasing late payments', 180
      UNION ALL SELECT 11, 11, '2026-02-10', 'recurring invoices', 60
      UNION ALL SELECT 12, 9, '2026-02-10', 'login', 20
      UNION ALL SELECT 13, 5, '2026-02-11', 'chasing late payments', 180
      UNION ALL SELECT 14, 14, '2026-02-11', 'tax export', 120
      UNION ALL SELECT 15, 9, '2026-02-12', 'login', 20
      UNION ALL SELECT 16, 10, '2026-02-12', 'login', 20
      UNION ALL SELECT 17, 6, '2026-02-13', 'chasing late payments', 180
      UNION ALL SELECT 18, 9, '2026-02-16', 'login', 20
      UNION ALL SELECT 19, 8, '2026-02-16', 'tax export', 120
      UNION ALL SELECT 20, 12, '2026-02-17', 'recurring invoices', 60
      UNION ALL SELECT 21, 9, '2026-02-17', 'login', 20
      UNION ALL SELECT 22, 15, '2026-02-18', 'tax export', 120
      UNION ALL SELECT 23, 1, '2026-02-19', 'chasing late payments', 180
      UNION ALL SELECT 24, 9, '2026-02-19', 'login', 20
      UNION ALL SELECT 25, 16, '2026-02-20', 'chasing late payments', 180
    ) t
    JOIN (
      SELECT 1 AS id, 'freelancer' AS segment, 'paid' AS plan
      UNION ALL SELECT 2, 'freelancer', 'free'
      UNION ALL SELECT 3, 'freelancer', 'paid'
      UNION ALL SELECT 4, 'freelancer', 'free'
      UNION ALL SELECT 5, 'freelancer', 'paid'
      UNION ALL SELECT 6, 'freelancer', 'free'
      UNION ALL SELECT 7, 'freelancer', 'paid'
      UNION ALL SELECT 8, 'freelancer', 'free'
      UNION ALL SELECT 9, 'agency', 'paid'
      UNION ALL SELECT 10, 'agency', 'paid'
      UNION ALL SELECT 11, 'agency', 'paid'
      UNION ALL SELECT 12, 'agency', 'free'
      UNION ALL SELECT 13, 'shop', 'paid'
      UNION ALL SELECT 14, 'shop', 'free'
      UNION ALL SELECT 15, 'shop', 'free'
      UNION ALL SELECT 16, 'shop', 'paid'
    ) u ON u.id = t.user_id
    GROUP BY u.segment, t.complaint
  ) a LEFT JOIN (
      SELECT 'freelancer' AS segment, 'chasing late payments' AS complaint, 7 AS tickets, 6 AS users, 1260 AS minutes_lost
      UNION ALL SELECT 'freelancer', 'tax export', 2, 2, 240
      UNION ALL SELECT 'agency', 'login', 10, 2, 200
      UNION ALL SELECT 'agency', 'recurring invoices', 2, 2, 120
      UNION ALL SELECT 'shop', 'tax export', 3, 3, 360
      UNION ALL SELECT 'shop', 'chasing late payments', 1, 1, 180
  ) e ON e.segment = a.segment AND e.complaint = a.complaint AND e.tickets = a.tickets
     AND e.users = a.users AND e.minutes_lost = a.minutes_lost)
  AND
  -- Task 3: the one segment and complaint the product is for.
  -- The rows below are the right answer to task 2. Pick the top row.
  (SELECT COUNT(*) = 1 AND SUM(a.segment = 'freelancer' AND a.complaint = 'chasing late payments') = 1 FROM (
    SELECT s.segment AS segment, s.complaint AS complaint
    FROM (
      SELECT 'shop' AS segment, 'tax export' AS complaint, 3 AS tickets, 3 AS users, 360 AS minutes_lost
      UNION ALL SELECT 'freelancer', 'tax export', 2, 2, 240
      UNION ALL SELECT 'agency', 'login', 10, 2, 200
      UNION ALL SELECT 'agency', 'recurring invoices', 2, 2, 120
      UNION ALL SELECT 'freelancer', 'chasing late payments', 7, 6, 1260
      UNION ALL SELECT 'shop', 'chasing late payments', 1, 1, 180
    ) s
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
    -- TODO: decide which column ranks the rows, and say why.
    ORDER BY s.tickets DESC
-- ===== END OF YOUR WORK 3 of 3 =====
    LIMIT 1
  ) a)
THEN 'PASS' ELSE 'FAIL' END AS result;
