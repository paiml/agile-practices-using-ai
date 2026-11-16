DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  b.bet,
  1.0                   AS confidence,         -- TODO: take it from evidence_ladder
  b.value_k - b.cost_k  AS expected_value_k,   -- TODO: value x confidence - cost
  'fund'                AS decision            -- TODO: 'fund' above zero, else 'buy evidence'
FROM bets AS b;
