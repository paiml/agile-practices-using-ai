-- Lab 2.1.0: Score the Bets: Impact, Effort, Confidence
-- Press Run. It prints FAIL until your two answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the score of each bet.
  (SELECT COUNT(*) = 8 AND COUNT(DISTINCT e.bet) = 8 FROM (
    SELECT b.bet AS bet,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           -- TODO: impact x confidence / effort (this placeholder is the impact alone).
           b.impact AS score
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
      SELECT 1 AS id, 'AI ticket triage' AS bet, 8 AS impact, 6 AS effort_weeks, 0.7 AS confidence
      UNION ALL SELECT 2, 'Self-serve onboarding', 6, 3, 0.3
      UNION ALL SELECT 3, 'Enterprise SSO', 6, 4, 0.1
      UNION ALL SELECT 4, 'Usage-based pricing', 9, 10, 0.5
      UNION ALL SELECT 5, 'Mobile app rewrite', 10, 24, 0.1
      UNION ALL SELECT 6, 'Smart search', 5, 4, 0.8
      UNION ALL SELECT 7, 'Bulk export', 3, 1, 0.9
      UNION ALL SELECT 8, 'Dark mode', 2, 2, 0.9
    ) b
  ) a LEFT JOIN (
      SELECT 'AI ticket triage' AS bet, 0.93 AS score
      UNION ALL SELECT 'Self-serve onboarding', 0.6
      UNION ALL SELECT 'Enterprise SSO', 0.15
      UNION ALL SELECT 'Usage-based pricing', 0.45
      UNION ALL SELECT 'Mobile app rewrite', 0.04
      UNION ALL SELECT 'Smart search', 1.0
      UNION ALL SELECT 'Bulk export', 2.7
      UNION ALL SELECT 'Dark mode', 0.9
  ) e ON e.bet = a.bet AND e.score = ROUND(a.score, 2))
  AND
  -- Task 2: the rank of each bet, 1 = the highest score.
  -- The rows below are the right answer to task 1 (the score, to 4 decimals).
  (SELECT COUNT(*) = 8 AND COUNT(DISTINCT e.bet) = 8 FROM (
    SELECT s.bet AS bet,
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
           -- TODO: the rank is 1 plus the number of bets that score higher.
           1 AS rank
-- ===== END OF YOUR WORK 2 of 3 =====
    FROM (
      SELECT 'Dark mode' AS bet, 0.9 AS score
      UNION ALL SELECT 'Bulk export', 2.7
      UNION ALL SELECT 'Smart search', 1.0
      UNION ALL SELECT 'Mobile app rewrite', 0.0417
      UNION ALL SELECT 'Usage-based pricing', 0.45
      UNION ALL SELECT 'Enterprise SSO', 0.15
      UNION ALL SELECT 'Self-serve onboarding', 0.6
      UNION ALL SELECT 'AI ticket triage', 0.9333
    ) s
    LEFT JOIN (
      SELECT 'Dark mode' AS bet, 0.9 AS score
      UNION ALL SELECT 'Bulk export', 2.7
      UNION ALL SELECT 'Smart search', 1.0
      UNION ALL SELECT 'Mobile app rewrite', 0.0417
      UNION ALL SELECT 'Usage-based pricing', 0.45
      UNION ALL SELECT 'Enterprise SSO', 0.15
      UNION ALL SELECT 'Self-serve onboarding', 0.6
      UNION ALL SELECT 'AI ticket triage', 0.9333
    ) o
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
    -- TODO: join each bet to the bets that score higher than it.
    ON o.score = s.score
-- ===== END OF YOUR WORK 3 of 3 =====
    GROUP BY s.bet
  ) a LEFT JOIN (
      SELECT 'AI ticket triage' AS bet, 3 AS rank
      UNION ALL SELECT 'Self-serve onboarding', 5
      UNION ALL SELECT 'Enterprise SSO', 7
      UNION ALL SELECT 'Usage-based pricing', 6
      UNION ALL SELECT 'Mobile app rewrite', 8
      UNION ALL SELECT 'Smart search', 2
      UNION ALL SELECT 'Bulk export', 1
      UNION ALL SELECT 'Dark mode', 4
  ) e ON e.bet = a.bet AND e.rank = a.rank)
THEN 'PASS' ELSE 'FAIL' END AS result;
