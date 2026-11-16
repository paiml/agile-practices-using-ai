DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH scored AS (
  SELECT
    metric,
    CASE direction
      WHEN 'up' THEN 100.0 * value / target
      ELSE 100.0 * target / value
    END AS attainment_pct
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
