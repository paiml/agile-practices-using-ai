DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  endpoint,
  AVG(ms) AS mean_ms,
  AVG(ms) AS p50_ms,  -- TODO: the response time half of the requests beat
  AVG(ms) AS p99_ms   -- TODO: the response time 99 of every 100 requests beat
FROM requests
GROUP BY endpoint;
