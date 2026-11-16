-- Lab 4.3.4: Required Checks and the Merge Button
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: for each pull request, each required check that is not success.
  -- state is the status of the check run, or 'missing' when the check never reported.
  -- Every pull request is paired with every required check (p and r), and c is the run that check reported.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.pr_id || '/' || e.check_name) = 3 FROM (
    SELECT p.id AS pr_id, r.name AS check_name,
-- ===== YOUR WORK 1 of 5: edit only between these lines =====
           -- TODO: state is the status of the check run, or 'missing' when the check never reported.
           c.status AS state
-- ===== END OF YOUR WORK 1 of 5 =====
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 1 AS approvals
      UNION ALL SELECT 2, 'Add reminder email', 1
      UNION ALL SELECT 3, 'Export as CSV', 1
      UNION ALL SELECT 4, 'Retry on 503', 0
      UNION ALL SELECT 5, 'Up' || 'date library docs', 2
      UNION ALL SELECT 6, 'Rename a variable', 1
    ) p
    CROSS JOIN (
      SELECT 'build' AS name
      UNION ALL SELECT 'test'
      UNION ALL SELECT 'lint'
    ) r
    LEFT JOIN (
      SELECT 1 AS pr_id, 'build' AS name, 'success' AS status
      UNION ALL SELECT 1, 'test', 'success'
      UNION ALL SELECT 1, 'lint', 'success'
      UNION ALL SELECT 1, 'bench', 'failure'
      UNION ALL SELECT 2, 'build', 'success'
      UNION ALL SELECT 2, 'test', 'failure'
      UNION ALL SELECT 2, 'lint', 'success'
      UNION ALL SELECT 3, 'build', 'success'
      UNION ALL SELECT 3, 'test', 'success'
      UNION ALL SELECT 4, 'build', 'success'
      UNION ALL SELECT 4, 'test', 'success'
      UNION ALL SELECT 4, 'lint', 'success'
      UNION ALL SELECT 5, 'build', 'success'
      UNION ALL SELECT 5, 'test', 'pending'
      UNION ALL SELECT 5, 'lint', 'success'
      UNION ALL SELECT 6, 'build', 'success'
      UNION ALL SELECT 6, 'test', 'success'
      UNION ALL SELECT 6, 'lint', 'success'
      UNION ALL SELECT 6, 'bench', 'pending'
    ) c ON c.pr_id = p.id AND c.name = r.name
-- ===== YOUR WORK 2 of 5: edit only between these lines =====
    -- TODO: every pull request is paired with every required check, and c is the run (NULL when the
    -- check never reported). Keep the pairs that are not success. This starter keeps only failures.
    WHERE c.status = 'failure'
-- ===== END OF YOUR WORK 2 of 5 =====
  ) a LEFT JOIN (
      SELECT 2 AS pr_id, 'test' AS check_name, 'failure' AS state
      UNION ALL SELECT 3, 'lint', 'missing'
      UNION ALL SELECT 5, 'test', 'pending'
  ) e ON e.pr_id = a.pr_id AND e.check_name = a.check_name AND e.state = a.state)
  AND
  -- Task 2: pull requests that may merge: at least one approval and no row in a.
  -- The rows in a are the right answer to task 1.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.pr_id) = 2 FROM (
    SELECT p.id AS pr_id
    FROM (
      SELECT 1 AS id, 'Add overdue filter' AS title, 1 AS approvals
      UNION ALL SELECT 2, 'Add reminder email', 1
      UNION ALL SELECT 3, 'Export as CSV', 1
      UNION ALL SELECT 4, 'Retry on 503', 0
      UNION ALL SELECT 5, 'Up' || 'date library docs', 2
      UNION ALL SELECT 6, 'Rename a variable', 1
    ) p
    LEFT JOIN (
      SELECT 2 AS pr_id, 'test' AS check_name, 'failure' AS state
      UNION ALL SELECT 3, 'lint', 'missing'
      UNION ALL SELECT 5, 'test', 'pending'
    ) a ON a.pr_id = p.id
-- ===== YOUR WORK 3 of 5: edit only between these lines =====
    -- TODO: keep the pull requests with at least one approval that have no row in a (the join gives NULL).
    -- This starter keeps every pull request.
    WHERE 1
-- ===== END OF YOUR WORK 3 of 5 =====
  ) a LEFT JOIN (
      SELECT 1 AS pr_id
      UNION ALL SELECT 6
  ) e ON e.pr_id = a.pr_id)
  AND
  -- Task 3: pull requests with at least one required check that never reported.
  -- The rows in a are the right answer to task 1.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.pr_id) = 1 FROM (
    SELECT a.pr_id AS pr_id
    FROM (
      SELECT 2 AS pr_id, 'test' AS check_name, 'failure' AS state
      UNION ALL SELECT 3, 'lint', 'missing'
      UNION ALL SELECT 5, 'test', 'pending'
    ) a
-- ===== YOUR WORK 4 of 5: edit only between these lines =====
    -- TODO: keep the rows whose state is 'missing', one row per pull request.
    -- This starter keeps every row.
    WHERE 1
-- ===== END OF YOUR WORK 4 of 5 =====
  ) a LEFT JOIN (
      SELECT 3 AS pr_id
  ) e ON e.pr_id = a.pr_id)
  AND
  -- Task 4: pull requests that may merge although they have a check run that is not success and not required.
  -- Those checks do not block the merge button. The rows in k are the right answer to task 2.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.pr_id) = 2 FROM (
    SELECT c.pr_id AS pr_id
    FROM (
      SELECT 1 AS pr_id
      UNION ALL SELECT 6
    ) k
    JOIN (
      SELECT 1 AS pr_id, 'build' AS name, 'success' AS status
      UNION ALL SELECT 1, 'test', 'success'
      UNION ALL SELECT 1, 'lint', 'success'
      UNION ALL SELECT 1, 'bench', 'failure'
      UNION ALL SELECT 2, 'build', 'success'
      UNION ALL SELECT 2, 'test', 'failure'
      UNION ALL SELECT 2, 'lint', 'success'
      UNION ALL SELECT 3, 'build', 'success'
      UNION ALL SELECT 3, 'test', 'success'
      UNION ALL SELECT 4, 'build', 'success'
      UNION ALL SELECT 4, 'test', 'success'
      UNION ALL SELECT 4, 'lint', 'success'
      UNION ALL SELECT 5, 'build', 'success'
      UNION ALL SELECT 5, 'test', 'pending'
      UNION ALL SELECT 5, 'lint', 'success'
      UNION ALL SELECT 6, 'build', 'success'
      UNION ALL SELECT 6, 'test', 'success'
      UNION ALL SELECT 6, 'lint', 'success'
      UNION ALL SELECT 6, 'bench', 'pending'
    ) c ON c.pr_id = k.pr_id
    LEFT JOIN (
      SELECT 'build' AS name
      UNION ALL SELECT 'test'
      UNION ALL SELECT 'lint'
    ) r ON r.name = c.name
-- ===== YOUR WORK 5 of 5: edit only between these lines =====
    -- TODO: c is a check run, k lists the pull requests that may merge, r has a row when the check is required.
    -- Keep the runs that are not success and not required, one row per pull request.
    -- This starter keeps every run.
    WHERE 1
-- ===== END OF YOUR WORK 5 of 5 =====
  ) a LEFT JOIN (
      SELECT 1 AS pr_id
      UNION ALL SELECT 6
  ) e ON e.pr_id = a.pr_id)
THEN 'PASS' ELSE 'FAIL' END AS result;
