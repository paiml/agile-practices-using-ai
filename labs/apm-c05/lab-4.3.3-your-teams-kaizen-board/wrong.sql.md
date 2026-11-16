-- Lab 4.3.3 planted wrong answer: ranks by the score but forgets that done ideas do not count,
-- so "Pair on flaky test" (already done, score 7.0) takes the first place.
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id, idea, ROUND(impact * 1.0 / effort, 1) AS score
FROM kaizen
ORDER BY impact * 1.0 / effort DESC, id
LIMIT 3;

CREATE VIEW answer_2 AS
SELECT SUM(effort) AS total_effort
FROM kaizen
WHERE id IN (SELECT id FROM answer_1);

CREATE VIEW answer_3 AS
SELECT id
FROM kaizen
WHERE status = 'open' AND id NOT IN (SELECT id FROM answer_1);
