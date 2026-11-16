-- Lab 4.3.4 solution.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT p.id AS pr_id, r.name AS check_name, COALESCE(c.status, 'missing') AS state
FROM prs p
CROSS JOIN required_checks r
LEFT JOIN check_runs c ON c.pr_id = p.id AND c.name = r.name
WHERE c.status IS NULL OR c.status <> 'success';

CREATE VIEW answer_2 AS
SELECT id AS pr_id
FROM prs
WHERE approvals >= 1
  AND id NOT IN (SELECT pr_id FROM answer_1);

CREATE VIEW answer_3 AS
SELECT DISTINCT pr_id
FROM answer_1
WHERE state = 'missing';

CREATE VIEW answer_4 AS
SELECT DISTINCT c.pr_id AS pr_id
FROM check_runs c
WHERE c.pr_id IN (SELECT pr_id FROM answer_2)
  AND c.status <> 'success'
  AND c.name NOT IN (SELECT name FROM required_checks);
