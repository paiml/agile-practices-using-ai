-- Lab 3.1.4 check: prints PASS or FAIL.
WITH
exp1(waste_name, hours, share_pct) AS (
  VALUES ('defects', 25, 40.0), ('waiting', 17, 27.0), ('overproduction', 11, 17.0), ('motion and duplication', 10, 16.0)
)
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 4
  AND NOT EXISTS (SELECT waste_name, hours, share_pct FROM answer_1 EXCEPT SELECT waste_name, hours, share_pct FROM exp1)
  AND NOT EXISTS (SELECT waste_name, hours, share_pct FROM exp1 EXCEPT SELECT waste_name, hours, share_pct FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 1
  AND (SELECT value_hours FROM answer_2) = 35
  AND (SELECT COUNT(*) FROM answer_3) = 1
  AND (SELECT waste_name FROM answer_3) = 'defects'
THEN 'PASS' ELSE 'FAIL' END;
