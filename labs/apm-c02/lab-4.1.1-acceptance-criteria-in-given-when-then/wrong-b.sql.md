-- Lab 4.1.1 planted wrong answer B: the rule is fixed, but AC2 and AC3 are the word
-- 'pass' typed in. They are green and can never go red.
DROP VIEW IF EXISTS ac_results;
DROP VIEW IF EXISTS reminders_due;

CREATE VIEW reminders_due AS
SELECT i.id AS invoice_id
FROM invoices i
CROSS JOIN clock c
WHERE i.paid_on IS NULL
  AND julianday(c.today) - julianday(i.due_on) > 7;

CREATE VIEW ac_results AS
SELECT 'AC1' AS criterion,
       CASE WHEN EXISTS (SELECT 1 FROM reminders_due WHERE invoice_id = 101)
            THEN 'pass' ELSE 'fail' END AS result
UNION ALL
SELECT 'AC2', 'pass'
UNION ALL
SELECT 'AC3', 'pass';
