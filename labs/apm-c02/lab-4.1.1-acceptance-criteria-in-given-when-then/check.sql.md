-- Lab 4.1.1 check: a short script, not one query. It prints PASS or FAIL and nothing else.
-- It asks three things:
--   1. Are AC1, AC2 and AC3 all 'pass'?
--   2. Does the rule list exactly invoices 101 and 105?
--   3. Can each test fail? The Given of each criterion is reversed (101 is paid,
--      102 is unpaid, 103 is 8 days late); every test must then say 'fail'.
-- The invoices table is put back as it was before the last line.
DROP TABLE IF EXISTS temp.chk_obs;
DROP TABLE IF EXISTS temp.chk_inv;
CREATE TEMP TABLE chk_obs (k TEXT, ok INTEGER);
CREATE TEMP TABLE chk_inv AS SELECT * FROM invoices;

INSERT INTO chk_obs
SELECT 'all three pass',
       (SELECT COUNT(*) FROM ac_results) = 3
   AND (SELECT COUNT(DISTINCT criterion) FROM ac_results
        WHERE criterion IN ('AC1', 'AC2', 'AC3') AND result = 'pass') = 3;

INSERT INTO chk_obs
SELECT 'rule lists 101 and 105 only',
       (SELECT COUNT(*) FROM reminders_due) = 2
   AND (SELECT COUNT(*) FROM reminders_due WHERE invoice_id IN (101, 105)) = 2;

UPDATE invoices SET paid_on = '2026-03-09' WHERE id = 101;
UPDATE invoices SET paid_on = NULL         WHERE id = 102;
UPDATE invoices SET due_on  = '2026-03-02' WHERE id = 103;

INSERT INTO chk_obs
SELECT 'each test can fail',
       (SELECT COUNT(DISTINCT criterion) FROM ac_results
        WHERE criterion IN ('AC1', 'AC2', 'AC3') AND result = 'fail') = 3;

DELETE FROM invoices;
INSERT INTO invoices SELECT * FROM chk_inv;

SELECT CASE WHEN (SELECT COUNT(*) FROM chk_obs WHERE ok = 1) = 3
            THEN 'PASS' ELSE 'FAIL' END;

DROP TABLE temp.chk_obs;
DROP TABLE temp.chk_inv;
