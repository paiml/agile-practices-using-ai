-- Lab 3.3.3 solution.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

CREATE VIEW answer_1 AS
SELECT id AS claim_id,
       CASE
         WHEN command IS NULL OR TRIM(command) = '' THEN 'no command'
         WHEN exit_code IS NULL THEN 'no exit code'
         WHEN exit_code <> 0 THEN 'failed'
         WHEN output IS NULL OR TRIM(output) = '' THEN 'no output'
       END AS reason
FROM claims
WHERE said_done = 1
  AND (command IS NULL OR TRIM(command) = ''
       OR exit_code IS NULL OR exit_code <> 0
       OR output IS NULL OR TRIM(output) = '');

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
