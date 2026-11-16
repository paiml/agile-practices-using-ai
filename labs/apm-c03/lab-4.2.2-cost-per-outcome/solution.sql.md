DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH unit AS (
  SELECT feature, month, 1.0 * cost_usd / outcomes AS cpo
  FROM feature_costs
)
SELECT
  f.feature,
  f.cpo AS first_cpo,
  l.cpo AS last_cpo,
  CASE
    WHEN l.cpo < f.cpo THEN 'scale'
    WHEN l.cpo > f.cpo THEN 'review'
    ELSE 'hold'
  END AS decision
FROM unit AS f
JOIN unit AS l ON l.feature = f.feature
WHERE f.month = 1 AND l.month = 3;
