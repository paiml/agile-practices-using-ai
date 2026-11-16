DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH per_week AS (
  SELECT
    week,
    SUM(signups)   AS signups,
    SUM(activated) AS activated,
    SUM(paid_w4)   AS paid_w4
  FROM weekly_signups
  GROUP BY week
)
SELECT
  week,
  SUM(signups) OVER (ORDER BY week) AS total_signups,
  100.0 * activated / signups       AS activation_pct,
  100.0 * paid_w4 / signups         AS paid_w4_pct
FROM per_week;
