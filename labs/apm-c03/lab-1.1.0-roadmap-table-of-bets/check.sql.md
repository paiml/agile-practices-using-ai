WITH expected (bet, confidence, expected_value_k, decision) AS (
  VALUES
    ('AI ticket triage',      0.7,  160.0, 'fund'),
    ('Self-serve onboarding', 0.3,   30.0, 'fund'),
    ('Enterprise SSO',        0.1, -100.0, 'buy evidence'),
    ('Usage-based pricing',   0.5,  250.0, 'fund'),
    ('Mobile app rewrite',    0.1, -480.0, 'buy evidence'),
    ('Smart search',          0.9,  145.0, 'fund')
),
got AS (
  SELECT bet, ROUND(confidence, 2) AS confidence,
         ROUND(expected_value_k, 1) AS expected_value_k, decision
  FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 6
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
