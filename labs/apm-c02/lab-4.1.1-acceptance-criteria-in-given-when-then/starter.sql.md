-- Lab 4.1.1 starter. Keep the view names and column names.
DROP VIEW IF EXISTS ac_results;
DROP VIEW IF EXISTS reminders_due;

-- The rule under test, as a developer first wrote it. Requirement:
--   "An invoice gets a reminder when it is unpaid and MORE THAN 7 days past its due date."
-- Do not edit this view until Task 3.
CREATE VIEW reminders_due AS
SELECT i.id AS invoice_id
FROM invoices i
CROSS JOIN clock c
WHERE i.paid_on IS NULL
  AND julianday(c.today) - julianday(i.due_on) >= 7;

-- The acceptance criteria as runnable tests: one row each, result 'pass' or 'fail'.
CREATE VIEW ac_results AS
-- AC1  Given invoice 101 is unpaid and 8 days past due,
--      When  the reminders are listed on 10 March,
--      Then  invoice 101 is in the list.
SELECT 'AC1' AS criterion,
       CASE WHEN EXISTS (SELECT 1 FROM reminders_due WHERE invoice_id = 101)
            THEN 'pass' ELSE 'fail' END AS result
UNION ALL
-- AC2  Given invoice 102 was paid (late),
--      When  the reminders are listed on 10 March,
--      Then  invoice 102 is NOT in the list.
-- TODO (Task 1): replace 'todo' with the test, written like AC1.
SELECT 'AC2', 'todo'
UNION ALL
-- AC3  Given invoice 103 is unpaid and exactly 7 days past due,
--      When  the reminders are listed on 10 March,
--      Then  invoice 103 is NOT in the list.
-- TODO (Task 2): replace 'todo' with the test.
SELECT 'AC3', 'todo';
