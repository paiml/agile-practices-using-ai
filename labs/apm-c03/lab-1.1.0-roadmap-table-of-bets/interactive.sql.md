-- Lab 1.1.0: A Roadmap Is a Table of Bets
-- Press Run. It prints FAIL until your three answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the confidence of each bet.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.bet) = 6 FROM (
    SELECT b.bet AS bet,
-- ===== YOUR WORK 1 of 4: edit only between these lines =====
           -- TODO: take the confidence from the evidence ladder (the 1.0 is a placeholder).
           1.0 AS confidence
-- ===== END OF YOUR WORK 1 of 4 =====
    FROM (
      SELECT 1 AS id, 'AI ticket triage' AS bet, 400 AS value_k, 120 AS cost_k, 'paid_pilot' AS evidence
      UNION ALL SELECT 2, 'Self-serve onboarding', 300, 60, 'customer_interviews'
      UNION ALL SELECT 3, 'Enterprise SSO', 500, 150, 'opinion'
      UNION ALL SELECT 4, 'Usage-based pricing', 900, 200, 'prototype_test'
      UNION ALL SELECT 5, 'Mobile app rewrite', 1200, 600, 'opinion'
      UNION ALL SELECT 6, 'Smart search', 250, 80, 'live_ab_test'
    ) b
    JOIN (
      SELECT 'opinion' AS evidence, 1 AS rung, 0.1 AS confidence
      UNION ALL SELECT 'customer_interviews', 2, 0.3
      UNION ALL SELECT 'prototype_test', 3, 0.5
      UNION ALL SELECT 'paid_pilot', 4, 0.7
      UNION ALL SELECT 'live_ab_test', 5, 0.9
    ) l
-- ===== YOUR WORK 2 of 4: edit only between these lines =====
    -- TODO: join each bet to the rung of its own evidence.
    ON 1 = 1
-- ===== END OF YOUR WORK 2 of 4 =====
  ) a LEFT JOIN (
      SELECT 'AI ticket triage' AS bet, 0.7 AS confidence
      UNION ALL SELECT 'Self-serve onboarding', 0.3
      UNION ALL SELECT 'Enterprise SSO', 0.1
      UNION ALL SELECT 'Usage-based pricing', 0.5
      UNION ALL SELECT 'Mobile app rewrite', 0.1
      UNION ALL SELECT 'Smart search', 0.9
  ) e ON e.bet = a.bet AND e.confidence = ROUND(a.confidence, 2))
  AND
  -- Task 2: the expected value of each bet, in $k.
  -- The rows below already carry the right confidence from task 1.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.bet) = 6 FROM (
    SELECT b.bet AS bet,
-- ===== YOUR WORK 3 of 4: edit only between these lines =====
           -- TODO: value x confidence - cost (this placeholder ignores the confidence).
           b.value_k - b.cost_k AS expected_value_k
-- ===== END OF YOUR WORK 3 of 4 =====
    FROM (
      SELECT 'AI ticket triage' AS bet, 400 AS value_k, 120 AS cost_k, 0.7 AS confidence
      UNION ALL SELECT 'Self-serve onboarding', 300, 60, 0.3
      UNION ALL SELECT 'Enterprise SSO', 500, 150, 0.1
      UNION ALL SELECT 'Usage-based pricing', 900, 200, 0.5
      UNION ALL SELECT 'Mobile app rewrite', 1200, 600, 0.1
      UNION ALL SELECT 'Smart search', 250, 80, 0.9
    ) b
  ) a LEFT JOIN (
      SELECT 'AI ticket triage' AS bet, 160.0 AS expected_value_k
      UNION ALL SELECT 'Self-serve onboarding', 30.0
      UNION ALL SELECT 'Enterprise SSO', -100.0
      UNION ALL SELECT 'Usage-based pricing', 250.0
      UNION ALL SELECT 'Mobile app rewrite', -480.0
      UNION ALL SELECT 'Smart search', 145.0
  ) e ON e.bet = a.bet AND e.expected_value_k = ROUND(a.expected_value_k, 1))
  AND
  -- Task 3: decide 'fund' or 'buy evidence'.
  -- The rows below already carry the right expected value from task 2.
  (SELECT COUNT(*) = 6 AND COUNT(DISTINCT e.bet) = 6 FROM (
    SELECT b.bet AS bet,
-- ===== YOUR WORK 4 of 4: edit only between these lines =====
           -- TODO: 'fund' when the expected value is above zero, else 'buy evidence'.
           'fund' AS decision
-- ===== END OF YOUR WORK 4 of 4 =====
    FROM (
      SELECT 'AI ticket triage' AS bet, 400 AS value_k, 120 AS cost_k, 0.7 AS confidence, 160.0 AS expected_value_k
      UNION ALL SELECT 'Self-serve onboarding', 300, 60, 0.3, 30.0
      UNION ALL SELECT 'Enterprise SSO', 500, 150, 0.1, -100.0
      UNION ALL SELECT 'Usage-based pricing', 900, 200, 0.5, 250.0
      UNION ALL SELECT 'Mobile app rewrite', 1200, 600, 0.1, -480.0
      UNION ALL SELECT 'Smart search', 250, 80, 0.9, 145.0
    ) b
  ) a LEFT JOIN (
      SELECT 'AI ticket triage' AS bet, 'fund' AS decision
      UNION ALL SELECT 'Self-serve onboarding', 'fund'
      UNION ALL SELECT 'Enterprise SSO', 'buy evidence'
      UNION ALL SELECT 'Usage-based pricing', 'fund'
      UNION ALL SELECT 'Mobile app rewrite', 'buy evidence'
      UNION ALL SELECT 'Smart search', 'fund'
  ) e ON e.bet = a.bet AND e.decision = a.decision)
THEN 'PASS' ELSE 'FAIL' END AS result;
