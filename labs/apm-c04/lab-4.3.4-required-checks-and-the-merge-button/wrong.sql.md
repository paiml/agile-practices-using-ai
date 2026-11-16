-- Lab 4.3.4 planted wrong answer: reads only the check runs that exist, so a required check that never
-- reported (pull request 3, lint) counts as a pass and that pull request is allowed to merge.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT c.pr_id AS pr_id, c.name AS check_name, c.status AS state
FROM check_runs c
JOIN required_checks r ON r.name = c.name
WHERE c.status <> 'success';

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
