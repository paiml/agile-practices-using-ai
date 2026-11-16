-- Lab 3.3.3 starter. Finish each view. Keep the view names and column names.
DROP VIEW IF EXISTS answer_4;
DROP VIEW IF EXISTS answer_3;
DROP VIEW IF EXISTS answer_2;
DROP VIEW IF EXISTS answer_1;

-- Task 1: why each "done" claim has no valid receipt. Only claims with said_done = 1 count.
-- reason is the FIRST that applies, in this order:
--   'no command'   the command is NULL, empty or only spaces
--   'no exit code' the exit code is NULL
--   'failed'       the exit code is not 0
--   'no output'    the output is NULL, empty or only spaces
-- TODO: the CASE below knows only one reason; add the others, in this order.
CREATE VIEW answer_1 AS
SELECT id AS claim_id,
       CASE WHEN exit_code <> 0 THEN 'failed' END AS reason
FROM claims
WHERE said_done = 1 AND exit_code <> 0;

-- Task 2: the claims accepted: said "done" and have no reason in answer_1.
-- TODO: this lists every claim; keep only the accepted ones.
CREATE VIEW answer_2 AS
SELECT id AS claim_id
FROM claims;

-- Task 3: for each agent, how many "done" claims it made and how many were accepted.
-- Columns: agent, claimed, accepted.
-- TODO: count claims with said_done = 1 and the ones in answer_2.
CREATE VIEW answer_3 AS
SELECT agent, 0 AS claimed, 0 AS accepted
FROM claims
GROUP BY agent;

-- Task 4: agents that claimed "done" at least once and had none accepted.
-- TODO: build it from answer_3.
CREATE VIEW answer_4 AS
SELECT agent
FROM claims;
