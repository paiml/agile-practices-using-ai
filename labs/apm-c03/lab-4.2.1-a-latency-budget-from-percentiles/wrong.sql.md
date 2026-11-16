-- Plausible mistake: the budget is tested with < instead of <=.
-- The p50 path adds up to exactly 280 ms, which is inside a 280 ms budget.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH per_step (percentile, step, ms) AS (
  SELECT 'p50', step, p50_ms FROM steps
  UNION ALL
  SELECT 'p99', step, p99_ms FROM steps
),
totals AS (
  SELECT percentile, SUM(ms) AS total_ms, MAX(ms) AS biggest_ms
  FROM per_step
  GROUP BY percentile
)
SELECT
  t.percentile,
  t.total_ms,
  b.budget_ms - t.total_ms AS headroom_ms,
  CASE WHEN t.total_ms < b.budget_ms THEN 'ok' ELSE 'over' END AS status,
  (SELECT s.step FROM per_step AS s
    WHERE s.percentile = t.percentile AND s.ms = t.biggest_ms) AS biggest_step
FROM totals AS t
JOIN budget AS b ON b.percentile = t.percentile;
