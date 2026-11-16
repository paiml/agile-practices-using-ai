DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  metric,
  value   AS attainment_pct,  -- TODO: how much of the target is met, in percent (mind the direction)
  'green' AS status           -- TODO: 'green' at 100 or more, 'amber' at 90 or more, else 'red'
FROM scorecard;
