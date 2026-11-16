-- Lab 4.1.1 solution.
DROP VIEW IF EXISTS ac_results;
DROP VIEW IF EXISTS reminders_due;

-- Task 3: "more than 7 days" is > 7, not >= 7.
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
SELECT 'AC2',
       CASE WHEN NOT EXISTS (SELECT 1 FROM reminders_due WHERE invoice_id = 102)
            THEN 'pass' ELSE 'fail' END
UNION ALL
SELECT 'AC3',
       CASE WHEN NOT EXISTS (SELECT 1 FROM reminders_due WHERE invoice_id = 103)
            THEN 'pass' ELSE 'fail' END;
