-- Lab 3.1.1 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: every missing part of every story, one row per missing part.
-- missing is 'role', 'goal' or 'reason'.
-- TODO: a part is also missing when it is an empty string or only spaces.
-- TODO: add the same test for goal and for reason.
CREATE VIEW answer_1 AS
SELECT id AS story_id, 'role' AS missing
FROM stories
WHERE role IS NULL;

-- Task 2: each complete story as one sentence, exactly in this form:
--   As a <role>, I want to <goal>, so that <reason>.
-- TODO: leave out every story that appears in answer_1, and build the sentence.
CREATE VIEW answer_2 AS
SELECT id AS story_id,
       role AS sentence
FROM stories;

-- Task 3: required sections the page lacks. A section with an empty body is lacking.
-- TODO: this lists every required section; keep only the lacking ones.
CREATE VIEW answer_3 AS
SELECT r.section AS section
FROM required_sections r;

-- Task 4: stories the page puts in scope that are not complete.
-- TODO: this lists every story in scope; keep only those with a row in answer_1.
CREATE VIEW answer_4 AS
SELECT p.story_id AS story_id
FROM page_stories p;
