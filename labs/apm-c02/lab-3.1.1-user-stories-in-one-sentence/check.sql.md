-- Lab 3.1.1 check: prints PASS or FAIL.
WITH
exp1(story_id, missing) AS (
  VALUES (3, 'reason'), (4, 'reason'), (5, 'role'), (6, 'goal')
),
exp2(story_id, sentence) AS (
  VALUES (1, 'As a freelancer, I want to see which invoices are overdue on one screen, so that I stop checking my bank to find out.'),
         (2, 'As a freelancer, I want to have overdue clients reminded automatically, so that I get my Friday afternoon back.'),
         (7, 'As a studio owner, I want to see which teammate sent each invoice, so that I can answer a client without asking around.')
),
exp3(section) AS (VALUES ('acceptance'), ('measure')),
exp4(story_id) AS (VALUES (4))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 4
  AND NOT EXISTS (SELECT story_id, missing FROM answer_1 EXCEPT SELECT story_id, missing FROM exp1)
  AND NOT EXISTS (SELECT story_id, missing FROM exp1 EXCEPT SELECT story_id, missing FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 3
  AND NOT EXISTS (SELECT story_id, sentence FROM answer_2 EXCEPT SELECT story_id, sentence FROM exp2)
  AND NOT EXISTS (SELECT story_id, sentence FROM exp2 EXCEPT SELECT story_id, sentence FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 2
  AND NOT EXISTS (SELECT section FROM answer_3 EXCEPT SELECT section FROM exp3)
  AND NOT EXISTS (SELECT section FROM exp3 EXCEPT SELECT section FROM answer_3)
  AND (SELECT COUNT(*) FROM answer_4) = 1
  AND NOT EXISTS (SELECT story_id FROM answer_4 EXCEPT SELECT story_id FROM exp4)
  AND NOT EXISTS (SELECT story_id FROM exp4 EXCEPT SELECT story_id FROM answer_4)
THEN 'PASS' ELSE 'FAIL' END;
