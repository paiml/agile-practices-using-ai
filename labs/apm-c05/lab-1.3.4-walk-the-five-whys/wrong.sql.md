-- Lab 1.3.4 planted wrong answer: stops at the first "why" instead of walking the whole chain, so
-- the root cause of each incident is the answer to the symptom's first question, not the last.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT w.incident AS incident, 2 AS depth, w.answer AS root_cause, w.changeable AS changeable
FROM why AS w
JOIN why AS s ON w.parent_id = s.id
WHERE s.parent_id IS NULL;

CREATE VIEW answer_2 AS
SELECT incident
FROM answer_1
WHERE changeable = 0;

CREATE VIEW answer_3 AS
SELECT incident
FROM answer_1
WHERE depth >= 5;
