WITH expected (feature, first_cpo, last_cpo, decision) AS (
  VALUES
    ('AI triage',    2.0, 1.5, 'scale'),
    ('Smart search', 1.0, 1.5, 'review'),
    ('Bulk export',  3.0, 3.0, 'hold')
),
got AS (
  SELECT feature, ROUND(first_cpo, 2) AS first_cpo,
         ROUND(last_cpo, 2) AS last_cpo, decision
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 3
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
