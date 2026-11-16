-- Lab 1.3.1 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: each missing part of each ticket, one row per missing part. missing is 'title' or 'done_condition'.
-- TODO: a part is also missing when it is an empty string or only spaces. Add the test for done_condition too.
CREATE VIEW answer_1 AS
SELECT id AS ticket_id, 'title' AS missing
FROM tickets
WHERE title IS NULL;

-- Task 2: tickets whose done condition is present but vague.
-- TODO: keep a ticket when its done_condition contains any word in vague_words (LIKE is case-insensitive for plain letters).
CREATE VIEW answer_2 AS
SELECT id AS ticket_id
FROM tickets;

-- Task 3: tickets an agent can take now: no missing part, no vague done condition, plan approved.
-- TODO: this lists every ticket; keep only the ready ones.
CREATE VIEW answer_3 AS
SELECT id AS ticket_id
FROM tickets;

-- Task 4: tickets whose plan was approved but that are not ready.
-- TODO: plan_approved = 1 and the ticket is not in answer_3.
CREATE VIEW answer_4 AS
SELECT id AS ticket_id
FROM tickets;
