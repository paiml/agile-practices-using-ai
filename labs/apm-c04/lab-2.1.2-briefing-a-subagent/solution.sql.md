-- Lab 2.1.2 solution.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id AS brief_id, 'task' AS missing
FROM briefs
WHERE task IS NULL OR TRIM(task) = ''
UNION ALL
SELECT b.id, 'paths'
FROM briefs b
WHERE NOT EXISTS (SELECT 1 FROM brief_paths p WHERE p.brief_id = b.id)
UNION ALL
SELECT id, 'check'
FROM briefs
WHERE check_cmd IS NULL OR TRIM(check_cmd) = '';

CREATE VIEW answer_2 AS
SELECT a.brief_id AS a_id, b.brief_id AS b_id, a.path AS path
FROM brief_paths a
JOIN brief_paths b ON a.path = b.path AND a.brief_id < b.brief_id;

CREATE VIEW answer_3 AS
SELECT id AS brief_id
FROM briefs
WHERE id NOT IN (SELECT brief_id FROM answer_1)
  AND id NOT IN (SELECT a_id FROM answer_2)
  AND id NOT IN (SELECT b_id FROM answer_2);

CREATE VIEW answer_4 AS
SELECT b.id AS brief_id,
       'Task: ' || TRIM(b.task)
       || '. Paths: ' || (SELECT group_concat(path, ', ')
                          FROM (SELECT path FROM brief_paths WHERE brief_id = b.id ORDER BY path))
       || '. Check: ' || TRIM(b.check_cmd) || '.' AS text
FROM briefs b
WHERE b.id IN (SELECT brief_id FROM answer_3);
