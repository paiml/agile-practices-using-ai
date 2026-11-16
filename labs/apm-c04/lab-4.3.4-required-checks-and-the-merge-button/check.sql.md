-- Lab 4.3.4 check: prints PASS or FAIL.
WITH
exp1(pr_id, check_name, state) AS (VALUES (2, 'test', 'failure'), (3, 'lint', 'missing'), (5, 'test', 'pending')),
exp2(pr_id) AS (VALUES (1), (6)),
exp3(pr_id) AS (VALUES (3)),
exp4(pr_id) AS (VALUES (1), (6))
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 3
  AND NOT EXISTS (SELECT pr_id, check_name, state FROM answer_1 EXCEPT SELECT pr_id, check_name, state FROM exp1)
  AND NOT EXISTS (SELECT pr_id, check_name, state FROM exp1 EXCEPT SELECT pr_id, check_name, state FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 2
  AND NOT EXISTS (SELECT pr_id FROM answer_2 EXCEPT SELECT pr_id FROM exp2)
  AND NOT EXISTS (SELECT pr_id FROM exp2 EXCEPT SELECT pr_id FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 1
  AND NOT EXISTS (SELECT pr_id FROM answer_3 EXCEPT SELECT pr_id FROM exp3)
  AND NOT EXISTS (SELECT pr_id FROM exp3 EXCEPT SELECT pr_id FROM answer_3)
  AND (SELECT COUNT(*) FROM answer_4) = 2
  AND NOT EXISTS (SELECT pr_id FROM answer_4 EXCEPT SELECT pr_id FROM exp4)
  AND NOT EXISTS (SELECT pr_id FROM exp4 EXCEPT SELECT pr_id FROM answer_4)
THEN 'PASS' ELSE 'FAIL' END;
