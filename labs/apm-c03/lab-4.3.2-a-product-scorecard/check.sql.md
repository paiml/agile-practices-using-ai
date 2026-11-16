WITH expected (metric, attainment_pct, status) AS (
  VALUES
    ('Activation rate (%)',      105.0, 'green'),
    ('p99 latency (ms)',          88.9, 'red'),
    ('Cost per outcome (USD)',   103.4, 'green'),
    ('Week-4 retention (%)',      90.0, 'amber'),
    ('NPS',                       80.0, 'red'),
    ('Tickets per 100 users',    125.0, 'green')
),
got AS (
  SELECT metric, ROUND(attainment_pct, 1) AS attainment_pct, status FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 6
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
