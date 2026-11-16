-- Lab 3.3.3 check: prints PASS or FAIL.
WITH
exp1(claim_id, reason) AS (VALUES (2, 'failed'), (3, 'no command'), (4, 'no output'), (5, 'no exit code')),
exp2(claim_id) AS (VALUES (1), (6), (7)),
exp3(agent, claimed, accepted) AS (VALUES ('agent-a', 2, 1), ('agent-b', 3, 0), ('agent-c', 2, 2)),
exp4(agent) AS (VALUES ('agent-b'))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 4
  AND NOT EXISTS (SELECT claim_id, reason FROM answer_1 EXCEPT SELECT claim_id, reason FROM exp1)
  AND NOT EXISTS (SELECT claim_id, reason FROM exp1 EXCEPT SELECT claim_id, reason FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 3
  AND NOT EXISTS (SELECT claim_id FROM answer_2 EXCEPT SELECT claim_id FROM exp2)
  AND NOT EXISTS (SELECT claim_id FROM exp2 EXCEPT SELECT claim_id FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 3
  AND NOT EXISTS (SELECT agent, claimed, accepted FROM answer_3 EXCEPT SELECT agent, claimed, accepted FROM exp3)
  AND NOT EXISTS (SELECT agent, claimed, accepted FROM exp3 EXCEPT SELECT agent, claimed, accepted FROM answer_3)
  AND (SELECT COUNT(*) FROM answer_4) = 1
  AND NOT EXISTS (SELECT agent FROM answer_4 EXCEPT SELECT agent FROM exp4)
  AND NOT EXISTS (SELECT agent FROM exp4 EXCEPT SELECT agent FROM answer_4)
THEN 'PASS' ELSE 'FAIL' END;
