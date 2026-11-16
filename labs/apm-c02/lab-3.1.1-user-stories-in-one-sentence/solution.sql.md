-- Lab 3.1.1 solution.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id AS story_id, 'role' AS missing
FROM stories
WHERE role IS NULL OR TRIM(role) = ''
UNION ALL
SELECT id, 'goal'
FROM stories
WHERE goal IS NULL OR TRIM(goal) = ''
UNION ALL
SELECT id, 'reason'
FROM stories
WHERE reason IS NULL OR TRIM(reason) = '';

CREATE VIEW answer_2 AS
SELECT id AS story_id,
       'As a ' || TRIM(role) || ', I want to ' || TRIM(goal) || ', so that ' || TRIM(reason) || '.' AS sentence
FROM stories
WHERE id NOT IN (SELECT story_id FROM answer_1);

CREATE VIEW answer_3 AS
SELECT r.section AS section
FROM required_sections r
LEFT JOIN page_sections p ON p.section = r.section
WHERE p.section IS NULL OR p.body IS NULL OR TRIM(p.body) = '';

CREATE VIEW answer_4 AS
SELECT p.story_id AS story_id
FROM page_stories p
WHERE p.story_id IN (SELECT story_id FROM answer_1);
