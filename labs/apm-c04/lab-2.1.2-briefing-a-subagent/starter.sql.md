-- Lab 2.1.2 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: each missing part of each brief, one row per missing part. missing is 'task', 'paths' or 'check'.
-- 'paths' is missing when the brief has no row in brief_paths.
-- TODO: task and check are also missing when empty or only spaces. Add 'paths' and 'check'.
CREATE VIEW answer_1 AS
SELECT id AS brief_id, 'task' AS missing
FROM briefs
WHERE task IS NULL;

-- Task 2: paths two different briefs both write. One row per pair, with a_id smaller than b_id.
-- TODO: join brief_paths to itself on the same path.
CREATE VIEW answer_2 AS
SELECT 0 AS a_id, 0 AS b_id, '' AS path
WHERE 0;

-- Task 3: briefs that can be dispatched in parallel now: no missing part and no path shared with another brief.
-- TODO: this lists every brief; keep only the safe ones.
CREATE VIEW answer_3 AS
SELECT id AS brief_id
FROM briefs;

-- Task 4: the brief text for each brief in answer_3, in exactly this form:
--   Task: <task>. Paths: <path>, <path>. Check: <check>.
-- Paths are in alphabetical order, separated by a comma and a space.
-- TODO: build the text.
CREATE VIEW answer_4 AS
SELECT id AS brief_id, task AS text
FROM briefs;
