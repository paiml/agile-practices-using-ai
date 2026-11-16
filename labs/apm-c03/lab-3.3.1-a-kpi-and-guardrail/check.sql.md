WITH expected (week, activation_pct, ticket_pct, status) AS (
  VALUES
    (1, 40.0,  5.0, 'healthy'),
    (2, 50.0, 10.0, 'guardrail breach'),
    (3, 30.0,  5.0, 'kpi miss'),
    (4, 50.0,  8.0, 'healthy')
),
got AS (
  SELECT week, ROUND(activation_pct, 1) AS activation_pct,
         ROUND(ticket_pct, 1) AS ticket_pct, status
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 4
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
