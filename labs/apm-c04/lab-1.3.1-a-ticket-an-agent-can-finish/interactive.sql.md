-- Lab 1.3.1: A Ticket an Agent Can Finish
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: one row per missing part of each ticket. missing is 'title' or 'done_condition'.
  -- Each ticket is paired with each part (p.missing), so you test one part on each row.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.ticket_id || '/' || e.missing) = 2 FROM (
    SELECT t.id AS ticket_id, p.missing AS missing
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 'make test exits 0 and the new test overdue_filter passes' AS done_condition, 1 AS plan_approved
      UNION ALL SELECT 2, 'Speed up export', 'Export is fast', 1
      UNION ALL SELECT 3, 'Fix rounding', '', 0
      UNION ALL SELECT 4, '   ', 'make lint prints 0 warnings', 1
      UNION ALL SELECT 5, 'Add CSV header', 'The CSV has a header row and make test exits 0', 0
      UNION ALL SELECT 6, 'Reminder email', 'The reminder looks nicer and works', 0
      UNION ALL SELECT 7, 'Retry on 503', 'curl against the stub returns 200 after 2 retries' || char(59) || ' make test exits 0', 1
    ) t
    CROSS JOIN (
      SELECT 'title' AS missing
      UNION ALL SELECT 'done_condition'
    ) p
-- ===== YOUR WORK 1 of 4: edit only between these lines =====
    -- TODO: keep the row when the part named in p.missing is missing.
    -- A part is missing when it is NULL, an empty string or only spaces.
    -- This starter tests only the title, and only for NULL. Add the other tests.
    WHERE p.missing = 'title' AND t.title IS NULL
-- ===== END OF YOUR WORK 1 of 4 =====
  ) a LEFT JOIN (
      SELECT 3 AS ticket_id, 'done_condition' AS missing
      UNION ALL SELECT 4, 'title'
  ) e ON e.ticket_id = a.ticket_id AND e.missing = a.missing)
  AND
  -- Task 2: tickets whose done condition is present but vague.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.ticket_id) = 2 FROM (
    SELECT t.id AS ticket_id
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 'make test exits 0 and the new test overdue_filter passes' AS done_condition, 1 AS plan_approved
      UNION ALL SELECT 2, 'Speed up export', 'Export is fast', 1
      UNION ALL SELECT 3, 'Fix rounding', '', 0
      UNION ALL SELECT 4, '   ', 'make lint prints 0 warnings', 1
      UNION ALL SELECT 5, 'Add CSV header', 'The CSV has a header row and make test exits 0', 0
      UNION ALL SELECT 6, 'Reminder email', 'The reminder looks nicer and works', 0
      UNION ALL SELECT 7, 'Retry on 503', 'curl against the stub returns 200 after 2 retries' || char(59) || ' make test exits 0', 1
    ) t
    CROSS JOIN (
      SELECT 'fast' AS word
      UNION ALL SELECT 'nicer'
      UNION ALL SELECT 'better'
      UNION ALL SELECT 'works'
      UNION ALL SELECT 'good'
    ) w
-- ===== YOUR WORK 2 of 4: edit only between these lines =====
    -- TODO: each ticket is paired with each word in w.word. Keep a ticket whose done condition is
    -- present and contains the word, and list each ticket once (the GROUP BY does that).
    -- LIKE ignores upper and lower case for plain letters. This starter keeps every pair.
    WHERE 1
-- ===== END OF YOUR WORK 2 of 4 =====
  ) a LEFT JOIN (
      SELECT 2 AS ticket_id
      UNION ALL SELECT 6
  ) e ON e.ticket_id = a.ticket_id)
  AND
  -- Task 3: tickets an agent can take now: plan approved, no missing part, no vague done condition.
  -- The rows in m are the right answer to task 1 and the rows in v are the right answer to task 2.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.ticket_id) = 2 FROM (
    SELECT t.id AS ticket_id
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 'make test exits 0 and the new test overdue_filter passes' AS done_condition, 1 AS plan_approved
      UNION ALL SELECT 2, 'Speed up export', 'Export is fast', 1
      UNION ALL SELECT 3, 'Fix rounding', '', 0
      UNION ALL SELECT 4, '   ', 'make lint prints 0 warnings', 1
      UNION ALL SELECT 5, 'Add CSV header', 'The CSV has a header row and make test exits 0', 0
      UNION ALL SELECT 6, 'Reminder email', 'The reminder looks nicer and works', 0
      UNION ALL SELECT 7, 'Retry on 503', 'curl against the stub returns 200 after 2 retries' || char(59) || ' make test exits 0', 1
    ) t
    LEFT JOIN (
      SELECT 3 AS ticket_id, 'done_condition' AS missing
      UNION ALL SELECT 4, 'title'
    ) m ON m.ticket_id = t.id
    LEFT JOIN (
      SELECT 2 AS ticket_id
      UNION ALL SELECT 6
    ) v ON v.ticket_id = t.id
-- ===== YOUR WORK 3 of 4: edit only between these lines =====
    -- TODO: m has a row for each ticket with a missing part, v has a row for each vague ticket.
    -- A ticket with no match gets NULL in those columns. Keep only the tickets with an approved plan
    -- that are in neither. This starter keeps every ticket.
    WHERE 1
-- ===== END OF YOUR WORK 3 of 4 =====
  ) a LEFT JOIN (
      SELECT 1 AS ticket_id
      UNION ALL SELECT 7
  ) e ON e.ticket_id = a.ticket_id)
  AND
  -- Task 4: tickets whose plan was approved but that are not ready.
  -- The rows in r are the right answer to task 3 (the ready tickets).
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.ticket_id) = 2 FROM (
    SELECT t.id AS ticket_id
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 'make test exits 0 and the new test overdue_filter passes' AS done_condition, 1 AS plan_approved
      UNION ALL SELECT 2, 'Speed up export', 'Export is fast', 1
      UNION ALL SELECT 3, 'Fix rounding', '', 0
      UNION ALL SELECT 4, '   ', 'make lint prints 0 warnings', 1
      UNION ALL SELECT 5, 'Add CSV header', 'The CSV has a header row and make test exits 0', 0
      UNION ALL SELECT 6, 'Reminder email', 'The reminder looks nicer and works', 0
      UNION ALL SELECT 7, 'Retry on 503', 'curl against the stub returns 200 after 2 retries' || char(59) || ' make test exits 0', 1
    ) t
    LEFT JOIN (
      SELECT 1 AS ticket_id
      UNION ALL SELECT 7
    ) r ON r.ticket_id = t.id
-- ===== YOUR WORK 4 of 4: edit only between these lines =====
    -- TODO: r has a row for each ticket that is ready. Keep the tickets with an approved plan
    -- that are not ready. This starter keeps every ticket.
    WHERE 1
-- ===== END OF YOUR WORK 4 of 4 =====
  ) a LEFT JOIN (
      SELECT 2 AS ticket_id
      UNION ALL SELECT 4
  ) e ON e.ticket_id = a.ticket_id)
THEN 'PASS' ELSE 'FAIL' END AS result;
