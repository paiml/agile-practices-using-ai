-- Lab 3.3.3 seed: eight "done" claims from three agents, each with whatever receipt it attached.
DROP TABLE IF EXISTS claims;
CREATE TABLE claims (
  id        INTEGER PRIMARY KEY,
  agent     TEXT NOT NULL,
  said_done INTEGER NOT NULL,   -- 1 when the agent said "done"
  command   TEXT,               -- the command the agent ran
  exit_code INTEGER,            -- what the command exited with
  output    TEXT                -- what the command printed
);
INSERT INTO claims (id, agent, said_done, command, exit_code, output) VALUES
  (1, 'agent-a', 1, 'make test', 0,    'ok 42 tests'),
  (2, 'agent-a', 1, 'make test', 1,    'FAILED 3 tests'),
  (3, 'agent-b', 1, '',          0,    'ok'),
  (4, 'agent-b', 1, 'make lint', 0,    '   '),
  (5, 'agent-b', 1, 'make test', NULL, ''),
  (6, 'agent-c', 1, 'make test', 0,    'ok 12 tests'),
  (7, 'agent-c', 1, 'make lint', 0,    'no warnings'),
  (8, 'agent-c', 0, 'make docs', 2,    'error: missing file');
