-- Lab 1.3.4 (solution): Walk the Five Whys
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the end of each chain of whys.
  -- Each incident is a chain of rows: every row points at the row it explains (its parent_id).
  -- No chain here is more than five answers deep, so the query joins the why rows to themselves
  -- five times. w1 is the symptom, w2 is the answer that explains w1, w3 explains w2, and so on.
  -- A level the chain does not reach is empty (NULL).
  -- Return incident, depth (the symptom is depth 1), root_cause (the deepest answer) and changeable.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.incident) = 3 FROM (
    SELECT w1.incident AS incident,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           1 + (w2.id IS NOT NULL) + (w3.id IS NOT NULL) + (w4.id IS NOT NULL) + (w5.id IS NOT NULL) AS depth,
           COALESCE(w5.answer, w4.answer, w3.answer, w2.answer, w1.answer) AS root_cause,
           COALESCE(w5.changeable, w4.changeable, w3.changeable, w2.changeable, w1.changeable) AS changeable
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
      SELECT 1 AS id, 'servers crashed at random' AS incident, NULL AS parent_id, 'servers crashed at random' AS answer, 0 AS changeable
      UNION ALL SELECT 2, 'servers crashed at random', 1, 'there were no log files to read', 0
      UNION ALL SELECT 3, 'servers crashed at random', 2, 'logs lived on ephemeral storage that vanishes on reboot', 1
      UNION ALL SELECT 4, 'servers crashed at random', 3, 'the machines had no swap and ran out of memory', 1
      UNION ALL SELECT 5, 'servers crashed at random', 4, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 11, 'release took 30 hours', NULL, 'release took 30 hours', 0
      UNION ALL SELECT 12, 'release took 30 hours', 11, 'every new push cancelled the run in flight', 1
      UNION ALL SELECT 13, 'release took 30 hours', 12, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 21, 'build went red on Friday', NULL, 'build went red on Friday', 0
      UNION ALL SELECT 22, 'build went red on Friday', 21, 'a dependency published a broken version', 0
      UNION ALL SELECT 23, 'build went red on Friday', 22, 'we do not control the registry', 0
    ) w1
    LEFT JOIN (
      SELECT 1 AS id, 'servers crashed at random' AS incident, NULL AS parent_id, 'servers crashed at random' AS answer, 0 AS changeable
      UNION ALL SELECT 2, 'servers crashed at random', 1, 'there were no log files to read', 0
      UNION ALL SELECT 3, 'servers crashed at random', 2, 'logs lived on ephemeral storage that vanishes on reboot', 1
      UNION ALL SELECT 4, 'servers crashed at random', 3, 'the machines had no swap and ran out of memory', 1
      UNION ALL SELECT 5, 'servers crashed at random', 4, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 11, 'release took 30 hours', NULL, 'release took 30 hours', 0
      UNION ALL SELECT 12, 'release took 30 hours', 11, 'every new push cancelled the run in flight', 1
      UNION ALL SELECT 13, 'release took 30 hours', 12, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 21, 'build went red on Friday', NULL, 'build went red on Friday', 0
      UNION ALL SELECT 22, 'build went red on Friday', 21, 'a dependency published a broken version', 0
      UNION ALL SELECT 23, 'build went red on Friday', 22, 'we do not control the registry', 0
    ) w2 ON w2.parent_id = w1.id
    LEFT JOIN (
      SELECT 1 AS id, 'servers crashed at random' AS incident, NULL AS parent_id, 'servers crashed at random' AS answer, 0 AS changeable
      UNION ALL SELECT 2, 'servers crashed at random', 1, 'there were no log files to read', 0
      UNION ALL SELECT 3, 'servers crashed at random', 2, 'logs lived on ephemeral storage that vanishes on reboot', 1
      UNION ALL SELECT 4, 'servers crashed at random', 3, 'the machines had no swap and ran out of memory', 1
      UNION ALL SELECT 5, 'servers crashed at random', 4, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 11, 'release took 30 hours', NULL, 'release took 30 hours', 0
      UNION ALL SELECT 12, 'release took 30 hours', 11, 'every new push cancelled the run in flight', 1
      UNION ALL SELECT 13, 'release took 30 hours', 12, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 21, 'build went red on Friday', NULL, 'build went red on Friday', 0
      UNION ALL SELECT 22, 'build went red on Friday', 21, 'a dependency published a broken version', 0
      UNION ALL SELECT 23, 'build went red on Friday', 22, 'we do not control the registry', 0
    ) w3 ON w3.parent_id = w2.id
    LEFT JOIN (
      SELECT 1 AS id, 'servers crashed at random' AS incident, NULL AS parent_id, 'servers crashed at random' AS answer, 0 AS changeable
      UNION ALL SELECT 2, 'servers crashed at random', 1, 'there were no log files to read', 0
      UNION ALL SELECT 3, 'servers crashed at random', 2, 'logs lived on ephemeral storage that vanishes on reboot', 1
      UNION ALL SELECT 4, 'servers crashed at random', 3, 'the machines had no swap and ran out of memory', 1
      UNION ALL SELECT 5, 'servers crashed at random', 4, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 11, 'release took 30 hours', NULL, 'release took 30 hours', 0
      UNION ALL SELECT 12, 'release took 30 hours', 11, 'every new push cancelled the run in flight', 1
      UNION ALL SELECT 13, 'release took 30 hours', 12, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 21, 'build went red on Friday', NULL, 'build went red on Friday', 0
      UNION ALL SELECT 22, 'build went red on Friday', 21, 'a dependency published a broken version', 0
      UNION ALL SELECT 23, 'build went red on Friday', 22, 'we do not control the registry', 0
    ) w4 ON w4.parent_id = w3.id
    LEFT JOIN (
      SELECT 1 AS id, 'servers crashed at random' AS incident, NULL AS parent_id, 'servers crashed at random' AS answer, 0 AS changeable
      UNION ALL SELECT 2, 'servers crashed at random', 1, 'there were no log files to read', 0
      UNION ALL SELECT 3, 'servers crashed at random', 2, 'logs lived on ephemeral storage that vanishes on reboot', 1
      UNION ALL SELECT 4, 'servers crashed at random', 3, 'the machines had no swap and ran out of memory', 1
      UNION ALL SELECT 5, 'servers crashed at random', 4, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 11, 'release took 30 hours', NULL, 'release took 30 hours', 0
      UNION ALL SELECT 12, 'release took 30 hours', 11, 'every new push cancelled the run in flight', 1
      UNION ALL SELECT 13, 'release took 30 hours', 12, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 21, 'build went red on Friday', NULL, 'build went red on Friday', 0
      UNION ALL SELECT 22, 'build went red on Friday', 21, 'a dependency published a broken version', 0
      UNION ALL SELECT 23, 'build went red on Friday', 22, 'we do not control the registry', 0
    ) w5 ON w5.parent_id = w4.id
    WHERE w1.parent_id IS NULL
  ) a LEFT JOIN (
      SELECT 'servers crashed at random' AS incident, 5 AS depth, 'an out-of-memory error caused a kernel panic' AS root_cause, 1 AS changeable
      UNION ALL SELECT 'release took 30 hours', 3, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 'build went red on Friday', 3, 'we do not control the registry', 0
  ) e ON e.incident = a.incident AND e.depth = a.depth AND e.root_cause = a.root_cause
     AND e.changeable = a.changeable)
  AND
  -- Task 2: the incidents whose end of chain the team cannot change. Return incident.
  -- The rows below are the right answer to task 1.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.incident) = 1 FROM (
    SELECT a.incident AS incident
    FROM (
      SELECT 'servers crashed at random' AS incident, 5 AS depth, 'an out-of-memory error caused a kernel panic' AS root_cause, 1 AS changeable
      UNION ALL SELECT 'release took 30 hours', 3, 'the workflow cancels in-progress runs on a new push', 1
      UNION ALL SELECT 'build went red on Friday', 3, 'we do not control the registry', 0
    ) a
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
    WHERE a.changeable = 0
-- ===== END OF YOUR WORK 2 of 3 =====
  ) a LEFT JOIN (
      SELECT 'build went red on Friday' AS incident
  ) e ON e.incident = a.incident)
  AND
  -- Task 3: the incidents whose chain is five answers deep or more. Return incident.
  -- The rows below are the right answer to task 1.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.incident) = 1 FROM (
    SELECT a.incident AS incident
    FROM (
      SELECT 'build went red on Friday' AS incident, 3 AS depth, 'we do not control the registry' AS root_cause, 0 AS changeable
      UNION ALL SELECT 'servers crashed at random', 5, 'an out-of-memory error caused a kernel panic', 1
      UNION ALL SELECT 'release took 30 hours', 3, 'the workflow cancels in-progress runs on a new push', 1
    ) a
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
    WHERE a.depth >= 5
-- ===== END OF YOUR WORK 3 of 3 =====
  ) a LEFT JOIN (
      SELECT 'servers crashed at random' AS incident
  ) e ON e.incident = a.incident)
THEN 'PASS' ELSE 'FAIL' END AS result;
