-- Lab 1.3.1 planted wrong answer: treats only NULL as missing, so the empty done condition (ticket 3)
-- and the spaces-only title (ticket 4) pass as present.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id AS ticket_id, 'title' AS missing
FROM tickets
WHERE title IS NULL
UNION ALL
SELECT id, 'done_condition'
FROM tickets
WHERE done_condition IS NULL;

CREATE VIEW answer_2 AS
SELECT t.id AS ticket_id
FROM tickets t
WHERE EXISTS (SELECT 1 FROM vague_words w WHERE t.done_condition LIKE '%' || w.word || '%');

CREATE VIEW answer_3 AS
SELECT id AS ticket_id
FROM tickets
WHERE plan_approved = 1
  AND id NOT IN (SELECT ticket_id FROM answer_1)
  AND id NOT IN (SELECT ticket_id FROM answer_2);

CREATE VIEW answer_4 AS
SELECT id AS ticket_id
FROM tickets
WHERE plan_approved = 1
  AND id NOT IN (SELECT ticket_id FROM answer_3);
