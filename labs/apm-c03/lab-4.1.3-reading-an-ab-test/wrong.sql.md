-- Plausible mistake: any positive lift is called a win, with no test of whether
-- the gap could be noise. The small short-form test jumps 50 percent on 200 users.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH pivot AS (
  SELECT
    experiment,
    SUM(CASE WHEN variant = 'control' THEN users END)       AS na,
    SUM(CASE WHEN variant = 'control' THEN conversions END) AS ca,
    SUM(CASE WHEN variant = 'test'    THEN users END)       AS nb,
    SUM(CASE WHEN variant = 'test'    THEN conversions END) AS cb
  FROM ab_results
  GROUP BY experiment
),
stats AS (
  SELECT
    experiment,
    1.0 * ca / na AS ra,
    1.0 * cb / nb AS rb,
    1.0 * (ca + cb) / (na + nb) AS p,
    1.0 / na + 1.0 / nb AS inv
  FROM pivot
)
SELECT
  experiment,
  100.0 * (rb - ra) / ra AS lift_pct,
  (rb - ra) * (rb - ra) / (p * (1 - p) * inv) AS z_squared,
  CASE WHEN rb > ra THEN 'ship' ELSE 'harm' END AS verdict
FROM stats;
