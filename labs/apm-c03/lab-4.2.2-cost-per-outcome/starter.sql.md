DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  feature,
  cost_usd AS first_cpo,   -- TODO: cost per outcome in month 1 (cost / outcomes)
  cost_usd AS last_cpo,    -- TODO: cost per outcome in month 3
  'hold'   AS decision     -- TODO: 'scale' if it fell, 'review' if it rose, else 'hold'
FROM feature_costs
WHERE month = 1;
