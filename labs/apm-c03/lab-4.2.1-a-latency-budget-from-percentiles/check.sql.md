WITH expected (percentile, total_ms, headroom_ms, status, biggest_step) AS (
  VALUES
    ('p50', 280,   0, 'ok',   'ranking'),
    ('p99', 660, -60, 'over', 'search')
),
got AS (
  SELECT percentile, total_ms, headroom_ms, status, biggest_step FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 2
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
