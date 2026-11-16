-- Plausible mistake: the decision follows the cost chart (total dollars), not the
-- cost per outcome. AI triage costs more every month, but each result got cheaper.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH unit AS (
  SELECT feature, month, 1.0 * cost_usd / outcomes AS cpo, cost_usd
  FROM feature_costs
)
SELECT
  f.feature,
  f.cpo AS first_cpo,
  l.cpo AS last_cpo,
  CASE
    WHEN l.cost_usd < f.cost_usd THEN 'scale'
    WHEN l.cost_usd > f.cost_usd THEN 'review'
    ELSE 'hold'
  END AS decision
FROM unit AS f
JOIN unit AS l ON l.feature = f.feature
WHERE f.month = 1 AND l.month = 3;
