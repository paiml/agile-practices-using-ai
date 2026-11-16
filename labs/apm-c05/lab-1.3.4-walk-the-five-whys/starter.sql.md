-- Lab 1.3.4 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: for each incident, the end of its chain of whys. Return incident, depth (the symptom is
-- depth 1), root_cause (the deepest answer) and changeable (that answer's flag).
-- TODO: this only reads the symptom row. Walk the chain with a recursive query.
CREATE VIEW answer_1 AS
SELECT incident, 1 AS depth, answer AS root_cause, changeable
FROM why
WHERE parent_id IS NULL;

-- Task 2: the incidents whose end of chain the team cannot change. Return incident.
-- TODO: this lists every incident.
CREATE VIEW answer_2 AS
SELECT incident
FROM answer_1;

-- Task 3: the incidents whose chain is five answers deep or more. Return incident.
-- TODO: this lists every incident.
CREATE VIEW answer_3 AS
SELECT incident
FROM answer_1;
