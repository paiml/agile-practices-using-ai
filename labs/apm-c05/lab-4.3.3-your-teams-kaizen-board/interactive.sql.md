-- Lab 4.3.3: Your Team's Kaizen Board
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the next three kaizen. Only open ideas count. The score of an idea is
  -- impact * 1.0 / effort. Return id, idea and score (1 decimal) for the three highest scores.
  -- Ties go to the lower id.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.id) = 3 FROM (
    SELECT
-- ===== YOUR WORK 1 of 5: edit only between these lines =====
           -- TODO: the score of an idea is impact * 1.0 / effort, rounded to 1 decimal.
           k.id AS id,
           k.idea AS idea,
           ROUND(k.impact * 1.0 / k.effort, 1) AS score
-- ===== END OF YOUR WORK 1 of 5 =====
    FROM (
      SELECT 1 AS id, 'Add andon check to CI' AS idea, 'open' AS status, 8 AS impact, 2 AS effort
      UNION ALL SELECT 2, 'Rewrite the build system', 'open', 9, 9
      UNION ALL SELECT 3, 'Cache dependency downloads', 'open', 6, 2
      UNION ALL SELECT 4, 'Pair on flaky test', 'done', 7, 1
      UNION ALL SELECT 5, 'Weekly metrics review', 'open', 5, 1
      UNION ALL SELECT 6, 'Del' || 'ete dead feature flags', 'open', 3, 3
      UNION ALL SELECT 7, 'Auto-label stale tickets', 'open', 4, 1
    ) k
-- ===== YOUR WORK 2 of 5: edit only between these lines =====
    -- TODO: this sorts by impact alone and does not skip done ideas.
    ORDER BY k.impact DESC
-- ===== END OF YOUR WORK 2 of 5 =====
    LIMIT 3
  ) a LEFT JOIN (
      SELECT 5 AS id, 'Weekly metrics review' AS idea, 5.0 AS score
      UNION ALL SELECT 1, 'Add andon check to CI', 4.0
      UNION ALL SELECT 7, 'Auto-label stale tickets', 4.0
  ) e ON e.id = a.id AND e.idea = a.idea AND e.score = a.score)
  AND
  -- Task 2: the effort the team commits to if it takes the three ideas of task 1. One column: total_effort.
  -- The rows named a below are the right answer to task 1 (they have no effort column).
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.total_effort) = 1 FROM (
    SELECT
-- ===== YOUR WORK 3 of 5: edit only between these lines =====
           SUM(k.effort) AS total_effort
-- ===== END OF YOUR WORK 3 of 5 =====
    FROM (
      SELECT 1 AS id, 'Add andon check to CI' AS idea, 'open' AS status, 8 AS impact, 2 AS effort
      UNION ALL SELECT 2, 'Rewrite the build system', 'open', 9, 9
      UNION ALL SELECT 3, 'Cache dependency downloads', 'open', 6, 2
      UNION ALL SELECT 4, 'Pair on flaky test', 'done', 7, 1
      UNION ALL SELECT 5, 'Weekly metrics review', 'open', 5, 1
      UNION ALL SELECT 6, 'Del' || 'ete dead feature flags', 'open', 3, 3
      UNION ALL SELECT 7, 'Auto-label stale tickets', 'open', 4, 1
    ) k,
    (
      SELECT 1 AS id, 'Add andon check to CI' AS idea, 4.0 AS score
      UNION ALL SELECT 7, 'Auto-label stale tickets', 4.0
      UNION ALL SELECT 5, 'Weekly metrics review', 5.0
    ) a
-- ===== YOUR WORK 4 of 5: edit only between these lines =====
    -- TODO: this pairs every idea with every row. Pair each idea with its own row.
    WHERE 1 = 1
-- ===== END OF YOUR WORK 4 of 5 =====
  ) a LEFT JOIN (
      SELECT 4 AS total_effort
  ) e ON e.total_effort = a.total_effort)
  AND
  -- Task 3: the open ideas the team does NOT take this round (open, and not in task 1). Return id.
  -- The rows named a below are the right answer to task 1.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.id) = 3 FROM (
    SELECT k.id AS id
    FROM (
      SELECT 1 AS id, 'Add andon check to CI' AS idea, 'open' AS status, 8 AS impact, 2 AS effort
      UNION ALL SELECT 2, 'Rewrite the build system', 'open', 9, 9
      UNION ALL SELECT 3, 'Cache dependency downloads', 'open', 6, 2
      UNION ALL SELECT 4, 'Pair on flaky test', 'done', 7, 1
      UNION ALL SELECT 5, 'Weekly metrics review', 'open', 5, 1
      UNION ALL SELECT 6, 'Del' || 'ete dead feature flags', 'open', 3, 3
      UNION ALL SELECT 7, 'Auto-label stale tickets', 'open', 4, 1
    ) k
    LEFT JOIN (
      SELECT 1 AS id, 'Add andon check to CI' AS idea, 4.0 AS score
      UNION ALL SELECT 7, 'Auto-label stale tickets', 4.0
      UNION ALL SELECT 5, 'Weekly metrics review', 5.0
    ) a ON a.id = k.id
-- ===== YOUR WORK 5 of 5: edit only between these lines =====
    -- TODO: this lists every idea. Keep the open ideas that are not taken this round.
    -- The join above leaves a empty (NULL) for an idea that is not taken.
    WHERE 1 = 1
-- ===== END OF YOUR WORK 5 of 5 =====
  ) a LEFT JOIN (
      SELECT 2 AS id
      UNION ALL SELECT 3
      UNION ALL SELECT 6
  ) e ON e.id = a.id)
THEN 'PASS' ELSE 'FAIL' END AS result;
