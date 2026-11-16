-- Plausible mistake: confidence applied to the profit, not to the value.
-- The cost is paid whether the bet works or not.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  b.bet,
  e.confidence,
  (b.value_k - b.cost_k) * e.confidence AS expected_value_k,
  CASE WHEN (b.value_k - b.cost_k) * e.confidence > 0
       THEN 'fund' ELSE 'buy evidence' END AS decision
FROM bets AS b
JOIN evidence_ladder AS e ON e.evidence = b.evidence;
