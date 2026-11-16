DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  b.bet,
  e.confidence,
  b.value_k * e.confidence - b.cost_k AS expected_value_k,
  CASE WHEN b.value_k * e.confidence - b.cost_k > 0
       THEN 'fund' ELSE 'buy evidence' END AS decision
FROM bets AS b
JOIN evidence_ladder AS e ON e.evidence = b.evidence;
