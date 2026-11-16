-- Plausible mistake: the slowest request reported as p99.
-- p99 is the 99th of 100 sorted values, not the 100th.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
WITH ranked AS (
  SELECT
    endpoint,
    ms,
    ROW_NUMBER() OVER (PARTITION BY endpoint ORDER BY ms) AS rn,
    COUNT(*)     OVER (PARTITION BY endpoint)             AS n
  FROM requests
)
SELECT
  endpoint,
  AVG(ms) AS mean_ms,
  MIN(CASE WHEN rn * 100 >= 50 * n THEN ms END) AS p50_ms,
  MAX(ms) AS p99_ms
FROM ranked
GROUP BY endpoint;
