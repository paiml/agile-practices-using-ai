-- Plausible mistake: every metric is scored as value / target, so a lower-is-better
-- metric that is over its target (p99 latency) looks like an overachiever.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH scored AS (
  SELECT metric, 100.0 * value / target AS attainment_pct
  FROM scorecard
)
SELECT
  metric,
  attainment_pct,
  CASE
    WHEN attainment_pct >= 100 THEN 'green'
    WHEN attainment_pct >= 90  THEN 'amber'
    ELSE 'red'
  END AS status
FROM scored;
