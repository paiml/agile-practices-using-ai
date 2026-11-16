-- Lab 1.2.1 solution.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT u.segment AS segment,
       COUNT(*)  AS tickets
FROM tickets t
JOIN users u ON u.id = t.user_id
GROUP BY u.segment;

CREATE VIEW answer_2 AS
SELECT u.segment                 AS segment,
       t.complaint               AS complaint,
       COUNT(*)                  AS tickets,
       COUNT(DISTINCT t.user_id) AS users,
       SUM(t.minutes_lost)       AS minutes_lost
FROM tickets t
JOIN users u ON u.id = t.user_id
GROUP BY u.segment, t.complaint;

-- Rank by people, not by tickets: one person can file many tickets.
CREATE VIEW answer_3 AS
SELECT segment, complaint
FROM answer_2
ORDER BY users DESC, minutes_lost DESC
LIMIT 1;
