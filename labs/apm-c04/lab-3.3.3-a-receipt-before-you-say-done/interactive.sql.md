-- Lab 3.3.3: A Receipt Before You Say Done
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: why each done claim has no valid receipt. Only claims with said_done = 1 count.
  -- The reason is the FIRST of these that applies, in this order:
  --   'no command'   the command is NULL, empty or only spaces
  --   'no exit code' the exit code is NULL
  --   'failed'       the exit code is not 0
  --   'no output'    the output is NULL, empty or only spaces
  (SELECT COUNT(*) = 4 AND COUNT(DISTINCT e.claim_id) = 4 FROM (
    SELECT c.id AS claim_id,
-- ===== YOUR WORK 1 of 6: edit only between these lines =====
           -- TODO: the reason is the FIRST test that applies, in the order above. Add the others.
           CASE WHEN c.exit_code <> 0 THEN 'failed' END AS reason
-- ===== END OF YOUR WORK 1 of 6 =====
    FROM (
      SELECT 1 AS id, 'agent-a' AS agent, 1 AS said_done, 'make test' AS command, 0 AS exit_code, 'ok 42 tests' AS output
      UNION ALL SELECT 2, 'agent-a', 1, 'make test', 1, 'FAILED 3 tests'
      UNION ALL SELECT 3, 'agent-b', 1, '', 0, 'ok'
      UNION ALL SELECT 4, 'agent-b', 1, 'make lint', 0, '   '
      UNION ALL SELECT 5, 'agent-b', 1, 'make test', NULL, ''
      UNION ALL SELECT 6, 'agent-c', 1, 'make test', 0, 'ok 12 tests'
      UNION ALL SELECT 7, 'agent-c', 1, 'make lint', 0, 'no warnings'
      UNION ALL SELECT 8, 'agent-c', 0, 'make docs', 2, 'error: missing file'
    ) c
-- ===== YOUR WORK 2 of 6: edit only between these lines =====
    -- TODO: keep the claims that said done and have a reason. This starter keeps only failed exit codes.
    WHERE c.said_done = 1 AND c.exit_code <> 0
-- ===== END OF YOUR WORK 2 of 6 =====
  ) a LEFT JOIN (
      SELECT 2 AS claim_id, 'failed' AS reason
      UNION ALL SELECT 3, 'no command'
      UNION ALL SELECT 4, 'no output'
      UNION ALL SELECT 5, 'no exit code'
  ) e ON e.claim_id = a.claim_id AND e.reason = a.reason)
  AND
  -- Task 2: the accepted claims: said done and no reason.
  -- The rows in r are the right answer to task 1 (the claims with a reason).
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.claim_id) = 3 FROM (
    SELECT c.id AS claim_id
    FROM (
      SELECT 1 AS id, 'agent-a' AS agent, 1 AS said_done, 'make test' AS command, 0 AS exit_code, 'ok 42 tests' AS output
      UNION ALL SELECT 2, 'agent-a', 1, 'make test', 1, 'FAILED 3 tests'
      UNION ALL SELECT 3, 'agent-b', 1, '', 0, 'ok'
      UNION ALL SELECT 4, 'agent-b', 1, 'make lint', 0, '   '
      UNION ALL SELECT 5, 'agent-b', 1, 'make test', NULL, ''
      UNION ALL SELECT 6, 'agent-c', 1, 'make test', 0, 'ok 12 tests'
      UNION ALL SELECT 7, 'agent-c', 1, 'make lint', 0, 'no warnings'
      UNION ALL SELECT 8, 'agent-c', 0, 'make docs', 2, 'error: missing file'
    ) c
    LEFT JOIN (
      SELECT 2 AS claim_id, 'failed' AS reason
      UNION ALL SELECT 3, 'no command'
      UNION ALL SELECT 4, 'no output'
      UNION ALL SELECT 5, 'no exit code'
    ) r ON r.claim_id = c.id
-- ===== YOUR WORK 3 of 6: edit only between these lines =====
    -- TODO: keep the claims that said done and that have no row in r (the join gives NULL).
    -- This starter keeps every claim.
    WHERE 1
-- ===== END OF YOUR WORK 3 of 6 =====
  ) a LEFT JOIN (
      SELECT 1 AS claim_id
      UNION ALL SELECT 6
      UNION ALL SELECT 7
  ) e ON e.claim_id = a.claim_id)
  AND
  -- Task 3: for each agent that claimed done at least once: agent, claimed (done claims) and accepted.
  -- The rows in a are the right answer to task 2 (the accepted claims).
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.agent) = 3 FROM (
    SELECT c.agent AS agent,
-- ===== YOUR WORK 4 of 6: edit only between these lines =====
           -- TODO: claimed = how many done claims the agent made, accepted = how many of them have a row in a.
           0 AS claimed,
           0 AS accepted
-- ===== END OF YOUR WORK 4 of 6 =====
    FROM (
      SELECT 1 AS id, 'agent-a' AS agent, 1 AS said_done, 'make test' AS command, 0 AS exit_code, 'ok 42 tests' AS output
      UNION ALL SELECT 2, 'agent-a', 1, 'make test', 1, 'FAILED 3 tests'
      UNION ALL SELECT 3, 'agent-b', 1, '', 0, 'ok'
      UNION ALL SELECT 4, 'agent-b', 1, 'make lint', 0, '   '
      UNION ALL SELECT 5, 'agent-b', 1, 'make test', NULL, ''
      UNION ALL SELECT 6, 'agent-c', 1, 'make test', 0, 'ok 12 tests'
      UNION ALL SELECT 7, 'agent-c', 1, 'make lint', 0, 'no warnings'
      UNION ALL SELECT 8, 'agent-c', 0, 'make docs', 2, 'error: missing file'
    ) c
    LEFT JOIN (
      SELECT 1 AS claim_id
      UNION ALL SELECT 6
      UNION ALL SELECT 7
    ) a ON a.claim_id = c.id
-- ===== YOUR WORK 5 of 6: edit only between these lines =====
    -- TODO: list each agent that claimed done at least once.
    GROUP BY c.agent
-- ===== END OF YOUR WORK 5 of 6 =====
  ) a LEFT JOIN (
      SELECT 'agent-a' AS agent, 2 AS claimed, 1 AS accepted
      UNION ALL SELECT 'agent-b', 3, 0
      UNION ALL SELECT 'agent-c', 2, 2
  ) e ON e.agent = a.agent AND e.claimed = a.claimed AND e.accepted = a.accepted)
  AND
  -- Task 4: agents that claimed done at least once and had none accepted.
  -- The rows in s are the right answer to task 3.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.agent) = 1 FROM (
    SELECT s.agent AS agent
    FROM (
      SELECT 'agent-a' AS agent, 2 AS claimed, 1 AS accepted
      UNION ALL SELECT 'agent-b', 3, 0
      UNION ALL SELECT 'agent-c', 2, 2
    ) s
-- ===== YOUR WORK 6 of 6: edit only between these lines =====
    -- TODO: keep the agents that claimed done at least once and have nothing accepted.
    -- This starter keeps every agent.
    WHERE 1
-- ===== END OF YOUR WORK 6 of 6 =====
  ) a LEFT JOIN (
      SELECT 'agent-b' AS agent
  ) e ON e.agent = a.agent)
THEN 'PASS' ELSE 'FAIL' END AS result;
