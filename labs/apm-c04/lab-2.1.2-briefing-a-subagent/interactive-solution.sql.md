-- Lab 2.1.2 (solution): Briefing a Subagent
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: one row per missing part of each brief. missing is 'task', 'paths' or 'check'.
  -- Each brief is paired with each part (m.missing), so you test one part on each row.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.brief_id || '/' || e.missing) = 3 FROM (
    SELECT b.id AS brief_id, m.missing AS missing
    FROM (
      SELECT 1 AS id, 'add overdue filter' AS task, 'make test' AS check_cmd
      UNION ALL SELECT 2, 'add reminder email', 'make test'
      UNION ALL SELECT 3, 'export CSV', NULL
      UNION ALL SELECT 4, '   ', 'make lint'
      UNION ALL SELECT 5, 'up' || 'date library exports', 'make test'
      UNION ALL SELECT 6, 'write the user guide', 'make docs'
      UNION ALL SELECT 7, 'retry on 503', 'make test'
    ) b
    CROSS JOIN (
      SELECT 'task' AS missing
      UNION ALL SELECT 'paths'
      UNION ALL SELECT 'check'
    ) m
    LEFT JOIN (
      SELECT 1 AS brief_id, 'src/filter.rs' AS path
      UNION ALL SELECT 1, 'tests/filter.rs'
      UNION ALL SELECT 2, 'src/mail.rs'
      UNION ALL SELECT 2, 'src/lib.rs'
      UNION ALL SELECT 3, 'src/export.rs'
      UNION ALL SELECT 4, 'src/ui.rs'
      UNION ALL SELECT 5, 'src/lib.rs'
      UNION ALL SELECT 7, 'src/http.rs'
    ) p ON p.brief_id = b.id
-- ===== YOUR WORK 1 of 5: edit only between these lines =====
    WHERE (m.missing = 'task' AND (b.task IS NULL OR TRIM(b.task) = ''))
       OR (m.missing = 'paths' AND p.brief_id IS NULL)
       OR (m.missing = 'check' AND (b.check_cmd IS NULL OR TRIM(b.check_cmd) = ''))
    GROUP BY b.id, m.missing
-- ===== END OF YOUR WORK 1 of 5 =====
  ) a LEFT JOIN (
      SELECT 3 AS brief_id, 'check' AS missing
      UNION ALL SELECT 4, 'task'
      UNION ALL SELECT 6, 'paths'
  ) e ON e.brief_id = a.brief_id AND e.missing = a.missing)
  AND
  -- Task 2: paths that two different briefs may both write. One row per pair, with a_id smaller than b_id.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.a_id || '/' || e.b_id || '/' || e.path) = 1 FROM (
    SELECT a.brief_id AS a_id, b.brief_id AS b_id, a.path AS path
    FROM (
      SELECT 1 AS brief_id, 'src/filter.rs' AS path
      UNION ALL SELECT 1, 'tests/filter.rs'
      UNION ALL SELECT 2, 'src/mail.rs'
      UNION ALL SELECT 2, 'src/lib.rs'
      UNION ALL SELECT 3, 'src/export.rs'
      UNION ALL SELECT 4, 'src/ui.rs'
      UNION ALL SELECT 5, 'src/lib.rs'
      UNION ALL SELECT 7, 'src/http.rs'
    ) a
    CROSS JOIN (
      SELECT 1 AS brief_id, 'src/filter.rs' AS path
      UNION ALL SELECT 1, 'tests/filter.rs'
      UNION ALL SELECT 2, 'src/mail.rs'
      UNION ALL SELECT 2, 'src/lib.rs'
      UNION ALL SELECT 3, 'src/export.rs'
      UNION ALL SELECT 4, 'src/ui.rs'
      UNION ALL SELECT 5, 'src/lib.rs'
      UNION ALL SELECT 7, 'src/http.rs'
    ) b
-- ===== YOUR WORK 2 of 5: edit only between these lines =====
    WHERE a.path = b.path AND a.brief_id < b.brief_id
-- ===== END OF YOUR WORK 2 of 5 =====
  ) a LEFT JOIN (
      SELECT 2 AS a_id, 5 AS b_id, 'src/lib.rs' AS path
  ) e ON e.a_id = a.a_id AND e.b_id = a.b_id AND e.path = a.path)
  AND
  -- Task 3: briefs that can run in parallel now: no missing part and no path shared with another brief.
  -- The rows in m are the right answer to task 1 and the rows in s1 and s2 are the right answer to task 2.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.brief_id) = 2 FROM (
    SELECT b.id AS brief_id
    FROM (
      SELECT 1 AS id, 'add overdue filter' AS task, 'make test' AS check_cmd
      UNION ALL SELECT 2, 'add reminder email', 'make test'
      UNION ALL SELECT 3, 'export CSV', NULL
      UNION ALL SELECT 4, '   ', 'make lint'
      UNION ALL SELECT 5, 'up' || 'date library exports', 'make test'
      UNION ALL SELECT 6, 'write the user guide', 'make docs'
      UNION ALL SELECT 7, 'retry on 503', 'make test'
    ) b
    LEFT JOIN (
      SELECT 3 AS brief_id, 'check' AS missing
      UNION ALL SELECT 4, 'task'
      UNION ALL SELECT 6, 'paths'
    ) m ON m.brief_id = b.id
    LEFT JOIN (
      SELECT 2 AS a_id, 5 AS b_id, 'src/lib.rs' AS path
    ) s1 ON s1.a_id = b.id
    LEFT JOIN (
      SELECT 2 AS a_id, 5 AS b_id, 'src/lib.rs' AS path
    ) s2 ON s2.b_id = b.id
-- ===== YOUR WORK 3 of 5: edit only between these lines =====
    WHERE m.brief_id IS NULL AND s1.a_id IS NULL AND s2.b_id IS NULL
-- ===== END OF YOUR WORK 3 of 5 =====
  ) a LEFT JOIN (
      SELECT 1 AS brief_id
      UNION ALL SELECT 7
  ) e ON e.brief_id = a.brief_id)
  AND
  -- Task 4: the brief text for each brief that can run in parallel (briefs 1 and 7, the right answer to task 3).
  -- The text has exactly this form: Task: <task>. Paths: <path>, <path>. Check: <check>.
  -- f.paths holds the paths of the brief, joined by a comma and a space, in the order of your second work area.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.brief_id) = 2 FROM (
    SELECT f.brief_id AS brief_id,
-- ===== YOUR WORK 4 of 5: edit only between these lines =====
           'Task: ' || TRIM(f.task) || '. Paths: ' || f.paths || '. Check: ' || TRIM(f.check_cmd) || '.' AS text
-- ===== END OF YOUR WORK 4 of 5 =====
    FROM (
      SELECT b.id AS brief_id, b.task AS task, b.check_cmd AS check_cmd,
             (SELECT group_concat(q.path, ', ') FROM (
                SELECT x.path AS path
                FROM (
                  SELECT 7 AS brief_id, 'src/http.rs' AS path
                  UNION ALL SELECT 5, 'src/lib.rs'
                  UNION ALL SELECT 4, 'src/ui.rs'
                  UNION ALL SELECT 3, 'src/export.rs'
                  UNION ALL SELECT 2, 'src/mail.rs'
                  UNION ALL SELECT 2, 'src/lib.rs'
                  UNION ALL SELECT 1, 'tests/filter.rs'
                  UNION ALL SELECT 1, 'src/filter.rs'
                ) x
                WHERE x.brief_id = b.id
-- ===== YOUR WORK 5 of 5: edit only between these lines =====
                ORDER BY x.path
-- ===== END OF YOUR WORK 5 of 5 =====
             ) q) AS paths
      FROM (
      SELECT 1 AS id, 'add overdue filter' AS task, 'make test' AS check_cmd
      UNION ALL SELECT 2, 'add reminder email', 'make test'
      UNION ALL SELECT 3, 'export CSV', NULL
      UNION ALL SELECT 4, '   ', 'make lint'
      UNION ALL SELECT 5, 'up' || 'date library exports', 'make test'
      UNION ALL SELECT 6, 'write the user guide', 'make docs'
      UNION ALL SELECT 7, 'retry on 503', 'make test'
      ) b
      JOIN (
        SELECT 1 AS brief_id
        UNION ALL SELECT 7
      ) k ON k.brief_id = b.id
    ) f
  ) a LEFT JOIN (
      SELECT 1 AS brief_id, 'Task: add overdue filter. Paths: src/filter.rs, tests/filter.rs. Check: make test.' AS text
      UNION ALL SELECT 7, 'Task: retry on 503. Paths: src/http.rs. Check: make test.'
  ) e ON e.brief_id = a.brief_id AND e.text = a.text)
THEN 'PASS' ELSE 'FAIL' END AS result;
