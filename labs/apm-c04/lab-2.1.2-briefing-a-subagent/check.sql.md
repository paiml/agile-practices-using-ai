-- Lab 2.1.2 check: prints PASS or FAIL.
WITH
exp1(brief_id, missing) AS (VALUES (3, 'check'), (4, 'task'), (6, 'paths')),
exp2(a_id, b_id, path) AS (VALUES (2, 5, 'src/lib.rs')),
exp3(brief_id) AS (VALUES (1), (7)),
exp4(brief_id, text) AS (
  VALUES (1, 'Task: add overdue filter. Paths: src/filter.rs, tests/filter.rs. Check: make test.'),
         (7, 'Task: retry on 503. Paths: src/http.rs. Check: make test.')
)
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 3
  AND NOT EXISTS (SELECT brief_id, missing FROM answer_1 EXCEPT SELECT brief_id, missing FROM exp1)
  AND NOT EXISTS (SELECT brief_id, missing FROM exp1 EXCEPT SELECT brief_id, missing FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 1
  AND NOT EXISTS (SELECT a_id, b_id, path FROM answer_2 EXCEPT SELECT a_id, b_id, path FROM exp2)
  AND NOT EXISTS (SELECT a_id, b_id, path FROM exp2 EXCEPT SELECT a_id, b_id, path FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 2
  AND NOT EXISTS (SELECT brief_id FROM answer_3 EXCEPT SELECT brief_id FROM exp3)
  AND NOT EXISTS (SELECT brief_id FROM exp3 EXCEPT SELECT brief_id FROM answer_3)
  AND (SELECT COUNT(*) FROM answer_4) = 2
  AND NOT EXISTS (SELECT brief_id, text FROM answer_4 EXCEPT SELECT brief_id, text FROM exp4)
  AND NOT EXISTS (SELECT brief_id, text FROM exp4 EXCEPT SELECT brief_id, text FROM answer_4)
THEN 'PASS' ELSE 'FAIL' END;
