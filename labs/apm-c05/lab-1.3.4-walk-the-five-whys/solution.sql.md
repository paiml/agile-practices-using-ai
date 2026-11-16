-- Lab 1.3.4 solution.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
WITH RECURSIVE chain(incident, id, depth) AS (
  SELECT incident, id, 1 FROM why WHERE parent_id IS NULL
  UNION ALL
  SELECT w.incident, w.id, c.depth + 1
  FROM why AS w JOIN chain AS c ON w.parent_id = c.id
)
SELECT c.incident AS incident, c.depth AS depth, w.answer AS root_cause, w.changeable AS changeable
FROM chain AS c
JOIN why AS w ON w.id = c.id
WHERE c.depth = (SELECT MAX(d.depth) FROM chain AS d WHERE d.incident = c.incident);

CREATE VIEW answer_2 AS
SELECT incident
FROM answer_1
WHERE changeable = 0;

CREATE VIEW answer_3 AS
SELECT incident
FROM answer_1
WHERE depth >= 5;
