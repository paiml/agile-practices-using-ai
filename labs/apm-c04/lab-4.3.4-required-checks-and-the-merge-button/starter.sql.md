-- Lab 4.3.4 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: for each pull request, each required check that is not 'success'.
-- state is the check's status, or 'missing' when the check never reported.
-- TODO: this reads only the runs that exist, so a check that never reported is invisible.
-- Start from every pull request paired with every required check.
CREATE VIEW answer_1 AS
SELECT c.pr_id AS pr_id, c.name AS check_name, c.status AS state
FROM check_runs c
WHERE c.status = 'failure';

-- Task 2: pull requests that may merge: at least one approval and no row in answer_1.
-- TODO: this lists every pull request; keep only the mergeable ones.
CREATE VIEW answer_2 AS
SELECT id AS pr_id
FROM prs;

-- Task 3: pull requests with at least one required check that never reported.
-- TODO: build it from answer_1.
CREATE VIEW answer_3 AS
SELECT id AS pr_id
FROM prs;

-- Task 4: pull requests in answer_2 that also have a check run that is not 'success' and is not required.
-- Those checks do not block the merge button.
-- TODO: join check_runs to answer_2 and drop the required checks.
CREATE VIEW answer_4 AS
SELECT id AS pr_id
FROM prs;
