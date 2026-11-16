-- Lab 4.1.2 (solution): P50 versus P99: What the Average Hides
-- Press Run. It prints FAIL until your three answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data alone: the 200 requests are built in place.
SELECT CASE WHEN
  -- Task 1: the mean response time of each endpoint.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.endpoint) = 2 FROM (
    SELECT r.endpoint AS endpoint,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           AVG(r.ms) AS mean_ms
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
        -- The 200 requests are built here, not typed: i runs 1 to 100 for each endpoint, the same rows as seed.sql.md.
        SELECT k * 100 + i AS id, CASE k WHEN 0 THEN '/checkout' ELSE '/search' END AS endpoint,
          CASE WHEN k = 0 THEN 190 + (i * 13) % 60 WHEN i <= 97 THEN 80 + (i * 37) % 41 ELSE 2400 + (i - 97) * 300 END AS ms
        FROM (SELECT a.d * 20 + b.d * 4 + c.d + 1 AS i, e.k AS k FROM (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) a, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) b, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) c, (SELECT 0 k UNION ALL SELECT 1) e)
    ) r
    GROUP BY r.endpoint
  ) a LEFT JOIN (
      SELECT '/checkout' AS endpoint, 219.5 AS mean_ms
      UNION ALL SELECT '/search', 187.4
  ) e ON e.endpoint = a.endpoint AND e.mean_ms = ROUND(a.mean_ms, 1))
  AND
  -- Task 2: p50_ms, the 50th of the 100 sorted response times of each endpoint.
  -- A request's place in line is how many requests of its endpoint are at or below its time.
  -- The inner query keeps the requests whose place is at least the one you ask for.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.endpoint) = 2 FROM (
    SELECT x.endpoint AS endpoint, MIN(x.ms) AS p50_ms FROM (
      SELECT r.endpoint AS endpoint, r.ms AS ms
      FROM (
        SELECT k * 100 + i AS id, CASE k WHEN 0 THEN '/checkout' ELSE '/search' END AS endpoint,
          CASE WHEN k = 0 THEN 190 + (i * 13) % 60 WHEN i <= 97 THEN 80 + (i * 37) % 41 ELSE 2400 + (i - 97) * 300 END AS ms
        FROM (SELECT a.d * 20 + b.d * 4 + c.d + 1 AS i, e.k AS k FROM (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) a, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) b, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) c, (SELECT 0 k UNION ALL SELECT 1) e)
      ) r
      JOIN (
        SELECT k * 100 + i AS id, CASE k WHEN 0 THEN '/checkout' ELSE '/search' END AS endpoint,
          CASE WHEN k = 0 THEN 190 + (i * 13) % 60 WHEN i <= 97 THEN 80 + (i * 37) % 41 ELSE 2400 + (i - 97) * 300 END AS ms
        FROM (SELECT a.d * 20 + b.d * 4 + c.d + 1 AS i, e.k AS k FROM (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) a, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) b, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) c, (SELECT 0 k UNION ALL SELECT 1) e)
      ) b ON b.endpoint = r.endpoint AND b.ms <= r.ms
      GROUP BY r.id
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
      HAVING COUNT(*) >= 50
-- ===== END OF YOUR WORK 2 of 3 =====
    ) x
    GROUP BY x.endpoint
  ) a LEFT JOIN (
      SELECT '/checkout' AS endpoint, 219 AS p50_ms
      UNION ALL SELECT '/search', 101
  ) e ON e.endpoint = a.endpoint AND e.p50_ms = a.p50_ms)
  AND
  -- Task 3: p99_ms, the 99th of the 100 sorted response times of each endpoint.
  -- The 100th value is the maximum, not the p99.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.endpoint) = 2 FROM (
    SELECT x.endpoint AS endpoint, MIN(x.ms) AS p99_ms FROM (
      SELECT r.endpoint AS endpoint, r.ms AS ms
      FROM (
        SELECT k * 100 + i AS id, CASE k WHEN 0 THEN '/checkout' ELSE '/search' END AS endpoint,
          CASE WHEN k = 0 THEN 190 + (i * 13) % 60 WHEN i <= 97 THEN 80 + (i * 37) % 41 ELSE 2400 + (i - 97) * 300 END AS ms
        FROM (SELECT a.d * 20 + b.d * 4 + c.d + 1 AS i, e.k AS k FROM (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) a, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) b, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) c, (SELECT 0 k UNION ALL SELECT 1) e)
      ) r
      JOIN (
        SELECT k * 100 + i AS id, CASE k WHEN 0 THEN '/checkout' ELSE '/search' END AS endpoint,
          CASE WHEN k = 0 THEN 190 + (i * 13) % 60 WHEN i <= 97 THEN 80 + (i * 37) % 41 ELSE 2400 + (i - 97) * 300 END AS ms
        FROM (SELECT a.d * 20 + b.d * 4 + c.d + 1 AS i, e.k AS k FROM (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) a, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4) b, (SELECT 0 d UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) c, (SELECT 0 k UNION ALL SELECT 1) e)
      ) b ON b.endpoint = r.endpoint AND b.ms <= r.ms
      GROUP BY r.id
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
      HAVING COUNT(*) >= 99
-- ===== END OF YOUR WORK 3 of 3 =====
    ) x
    GROUP BY x.endpoint
  ) a LEFT JOIN (
      SELECT '/checkout' AS endpoint, 249 AS p99_ms
      UNION ALL SELECT '/search', 3000
  ) e ON e.endpoint = a.endpoint AND e.p99_ms = a.p99_ms)
THEN 'PASS' ELSE 'FAIL' END AS result;
