-- Lab 4.1.1 (solution): Acceptance Criteria in Given-When-Then
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- The data below holds two worlds. Leave the data rows alone.
--   'given'    is the Given of each criterion.
--   'reversed' flips each Given: 101 is paid, 102 is unpaid, 103 is 8 days late.
-- A real test says 'pass' in the given world and 'fail' in the reversed world.
-- A test that is the word 'pass' typed in can never fail, so it is not a test.
-- Each test looks at r, the list of invoices that get a reminder: how many times is the invoice in it?
SELECT CASE WHEN
  (SELECT COUNT(*) = 2
      AND SUM(a.world = 'given' AND a.ac1 = 'pass' AND a.ac2 = 'pass' AND a.ac3 = 'pass'
              AND a.listed = 2 AND a.listed_ok = 2) = 1
      AND SUM(a.world = 'reversed' AND a.ac1 = 'fail' AND a.ac2 = 'fail' AND a.ac3 = 'fail') = 1
   FROM (
    SELECT w.world AS world,
           -- AC1  Given invoice 101 is unpaid and 8 days past due,
           --      When  the reminders are listed on 10 March,
           --      Then  invoice 101 is in the list.
           CASE WHEN COUNT(CASE WHEN r.invoice_id = 101 THEN 1 END) = 1
                THEN 'pass' ELSE 'fail' END AS ac1,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           CASE WHEN COUNT(CASE WHEN r.invoice_id = 102 THEN 1 END) = 0
                THEN 'pass' ELSE 'fail' END AS ac2,
-- ===== END OF YOUR WORK 1 of 3 =====
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
           CASE WHEN COUNT(CASE WHEN r.invoice_id = 103 THEN 1 END) = 0
                THEN 'pass' ELSE 'fail' END AS ac3,
-- ===== END OF YOUR WORK 2 of 3 =====
           -- The rule must list exactly invoices 101 and 105 in the given world.
           COUNT(r.invoice_id) AS listed,
           COUNT(CASE WHEN r.invoice_id IN (101, 105) THEN 1 END) AS listed_ok
    FROM (
      SELECT 'given' AS world
      UNION ALL SELECT 'reversed'
    ) w
    LEFT JOIN (
      SELECT i.world AS world, i.id AS invoice_id
      FROM (
        SELECT 'given' AS world, 101 AS id, 'Harbor Dental' AS client, 48000 AS amount_cents, '2026-03-02' AS due_on, NULL AS paid_on
        UNION ALL SELECT 'given', 102, 'Pine Street Bakery', 12500, '2026-02-20', '2026-03-05'
        UNION ALL SELECT 'given', 103, 'Lumen Studio', 90000, '2026-03-03', NULL
        UNION ALL SELECT 'given', 104, 'Orchard Vets', 30000, '2026-03-20', NULL
        UNION ALL SELECT 'given', 105, 'Kite Legal', 15000, '2026-02-01', NULL
        UNION ALL SELECT 'reversed', 101, 'Harbor Dental', 48000, '2026-03-02', '2026-03-09'
        UNION ALL SELECT 'reversed', 102, 'Pine Street Bakery', 12500, '2026-02-20', NULL
        UNION ALL SELECT 'reversed', 103, 'Lumen Studio', 90000, '2026-03-02', NULL
        UNION ALL SELECT 'reversed', 104, 'Orchard Vets', 30000, '2026-03-20', NULL
        UNION ALL SELECT 'reversed', 105, 'Kite Legal', 15000, '2026-02-01', NULL
      ) i
      CROSS JOIN (SELECT '2026-03-10' AS today) c
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
      -- Task 3: "more than 7 days" is > 7, not >= 7.
      WHERE i.paid_on IS NULL
        AND julianday(c.today) - julianday(i.due_on) > 7
-- ===== END OF YOUR WORK 3 of 3 =====
    ) r ON r.world = w.world
    GROUP BY w.world
  ) a)
THEN 'PASS' ELSE 'FAIL' END AS result;
