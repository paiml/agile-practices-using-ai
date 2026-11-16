WITH expected (endpoint, mean_ms, p50_ms, p99_ms) AS (
  VALUES
    ('/checkout', 219.5, 219,  249),
    ('/search',   187.4, 101, 3000)
),
got AS (
  SELECT endpoint, ROUND(mean_ms, 1) AS mean_ms,
         ROUND(p50_ms, 0) AS p50_ms, ROUND(p99_ms, 0) AS p99_ms
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 2
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
