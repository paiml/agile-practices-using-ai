WITH expected (week, total_signups, activation_pct, paid_w4_pct) AS (
  VALUES
    ('2026-W01',  200, 30.0, 8.0),
    ('2026-W02',  400, 31.0, 8.0),
    ('2026-W03',  650, 30.0, 8.0),
    ('2026-W04', 1150, 15.0, NULL),
    ('2026-W05', 1750, 15.0, NULL),
    ('2026-W06', 2550, 12.0, NULL)
),
got AS (
  SELECT week, total_signups,
         ROUND(activation_pct, 1) AS activation_pct,
         ROUND(paid_w4_pct, 1)    AS paid_w4_pct
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 6
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
