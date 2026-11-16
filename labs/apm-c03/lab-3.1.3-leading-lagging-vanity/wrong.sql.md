-- Plausible mistake: average the two channel rates.
-- A channel with 670 sign-ups then counts the same as one with 130.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH per_week AS (
  SELECT
    week,
    SUM(signups)                        AS signups,
    AVG(100.0 * activated / signups)    AS activation_pct,
    AVG(100.0 * paid_w4 / signups)      AS paid_w4_pct
  FROM weekly_signups
  GROUP BY week
)
SELECT
  week,
  SUM(signups) OVER (ORDER BY week) AS total_signups,
  activation_pct,
  paid_w4_pct
FROM per_week;
