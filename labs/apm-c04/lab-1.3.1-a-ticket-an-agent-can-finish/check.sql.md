-- Lab 1.3.1 check: prints PASS or FAIL.
WITH
exp1(ticket_id, missing) AS (VALUES (3, 'done_condition'), (4, 'title')),
exp2(ticket_id) AS (VALUES (2), (6)),
exp3(ticket_id) AS (VALUES (1), (7)),
exp4(ticket_id) AS (VALUES (2), (4))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 2
  AND NOT EXISTS (SELECT ticket_id, missing FROM answer_1 EXCEPT SELECT ticket_id, missing FROM exp1)
  AND NOT EXISTS (SELECT ticket_id, missing FROM exp1 EXCEPT SELECT ticket_id, missing FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 2
  AND NOT EXISTS (SELECT ticket_id FROM answer_2 EXCEPT SELECT ticket_id FROM exp2)
  AND NOT EXISTS (SELECT ticket_id FROM exp2 EXCEPT SELECT ticket_id FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 2
  AND NOT EXISTS (SELECT ticket_id FROM answer_3 EXCEPT SELECT ticket_id FROM exp3)
  AND NOT EXISTS (SELECT ticket_id FROM exp3 EXCEPT SELECT ticket_id FROM answer_3)
  AND (SELECT COUNT(*) FROM answer_4) = 2
  AND NOT EXISTS (SELECT ticket_id FROM answer_4 EXCEPT SELECT ticket_id FROM exp4)
  AND NOT EXISTS (SELECT ticket_id FROM exp4 EXCEPT SELECT ticket_id FROM answer_4)
THEN 'PASS' ELSE 'FAIL' END;
