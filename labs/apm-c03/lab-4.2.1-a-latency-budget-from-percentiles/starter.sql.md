DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  percentile,
  budget_ms AS total_ms,      -- TODO: the sum of the step times at that percentile
  0         AS headroom_ms,   -- TODO: budget minus total
  'ok'      AS status,        -- TODO: 'ok' when the total is within budget, else 'over'
  'auth'    AS biggest_step   -- TODO: the step that takes the most time at that percentile
FROM budget;
