WITH expected (experiment, lift_pct, z_squared, verdict) AS (
  VALUES
    ('new checkout button', 15.0, 4.58, 'ship'),
    ('short signup form',   50.0, 2.29, 'no call'),
    ('auto-applied promo', -21.9, 9.26, 'harm')
),
got AS (
  SELECT experiment, ROUND(lift_pct, 1) AS lift_pct,
         ROUND(z_squared, 2) AS z_squared, verdict
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 3
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
