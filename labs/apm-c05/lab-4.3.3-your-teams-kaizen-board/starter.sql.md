-- Lab 4.3.3 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: the next three kaizen. Only 'open' ideas count. The score of an idea is
-- impact * 1.0 / effort. Return id, idea and score (1 decimal) for the three highest scores.
-- Ties go to the lower id.
-- TODO: this sorts by impact alone and does not skip done ideas.
CREATE VIEW answer_1 AS
SELECT id, idea, ROUND(impact * 1.0 / effort, 1) AS score
FROM kaizen
ORDER BY impact DESC
LIMIT 3;

-- Task 2: the effort the team commits to if it takes the three ideas of Task 1.
-- TODO: this adds up every idea on the board.
CREATE VIEW answer_2 AS
SELECT SUM(effort) AS total_effort
FROM kaizen;

-- Task 3: the open ideas the team does NOT take this round (open, and not in answer_1). Return id.
-- TODO: this lists every idea.
CREATE VIEW answer_3 AS
SELECT id
FROM kaizen;
