WITH expected (bet, score, rank) AS (
  VALUES
    ('Bulk export',           2.70, 1),
    ('Smart search',          1.00, 2),
    ('AI ticket triage',      0.93, 3),
    ('Dark mode',             0.90, 4),
    ('Self-serve onboarding', 0.60, 5),
    ('Usage-based pricing',   0.45, 6),
    ('Enterprise SSO',        0.15, 7),
    ('Mobile app rewrite',    0.04, 8)
),
got AS (
  SELECT bet, ROUND(score, 2) AS score, rank FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 8
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
