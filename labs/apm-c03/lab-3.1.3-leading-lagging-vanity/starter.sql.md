DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  week,
  SUM(signups) AS total_signups,  -- TODO: make this the running total since W01
  NULL         AS activation_pct, -- TODO: activated / signups for the week, in percent
  NULL         AS paid_w4_pct     -- TODO: paid_w4 / signups for the week, in percent
FROM weekly_signups
GROUP BY week;
