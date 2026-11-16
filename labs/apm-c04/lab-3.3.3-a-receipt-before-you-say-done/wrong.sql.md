-- Lab 3.3.3 planted wrong answer: accepts any claim whose exit code is 0, so a claim with no command (3)
-- and a claim with blank output (4) are accepted, and claim 5 (NULL exit code) is accepted because a NULL comparison is never true.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id AS claim_id, 'failed' AS reason
FROM claims
WHERE said_done = 1 AND exit_code <> 0;

CREATE VIEW answer_2 AS
SELECT id AS claim_id
FROM claims
WHERE said_done = 1
  AND id NOT IN (SELECT claim_id FROM answer_1);

CREATE VIEW answer_3 AS
SELECT agent,
       SUM(said_done) AS claimed,
       SUM(CASE WHEN id IN (SELECT claim_id FROM answer_2) THEN 1 ELSE 0 END) AS accepted
FROM claims
GROUP BY agent
HAVING SUM(said_done) > 0;

CREATE VIEW answer_4 AS
SELECT agent
FROM answer_3
WHERE accepted = 0;
