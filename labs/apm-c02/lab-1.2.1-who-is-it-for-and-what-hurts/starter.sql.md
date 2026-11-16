-- Lab 1.2.1 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: tickets per segment.
-- TODO: join tickets to users and count the tickets (the 0 is a placeholder).
CREATE VIEW answer_1 AS
SELECT u.segment AS segment,
       0         AS tickets
FROM users u
GROUP BY u.segment;

-- Task 2: one row per segment and complaint.
-- TODO: users = how many different people complained; minutes_lost = the total.
CREATE VIEW answer_2 AS
SELECT u.segment  AS segment,
       t.complaint AS complaint,
       COUNT(*)   AS tickets,
       0          AS users,
       0          AS minutes_lost
FROM tickets t
JOIN users u ON u.id = t.user_id
GROUP BY u.segment, t.complaint;

-- Task 3: the one segment and complaint the product is for.
-- TODO: decide which column of answer_2 ranks the rows, and say why.
CREATE VIEW answer_3 AS
SELECT segment, complaint
FROM answer_2
ORDER BY tickets DESC
LIMIT 1;
