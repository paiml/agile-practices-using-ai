-- Lab 1.3.4 seed: three incident write-ups. Each row is one "why?" answer.
-- parent_id points at the answer it explains (NULL is the symptom itself).
-- changeable = 1 when the team can change that cause.
DROP TABLE IF EXISTS why;
CREATE TABLE why (
  id         INTEGER PRIMARY KEY,
  incident   TEXT,
  parent_id  INTEGER,
  answer     TEXT,
  changeable INTEGER
);
INSERT INTO why (id, incident, parent_id, answer, changeable) VALUES
  ( 1, 'servers crashed at random', NULL, 'servers crashed at random', 0),
  ( 2, 'servers crashed at random',    1, 'there were no log files to read', 0),
  ( 3, 'servers crashed at random',    2, 'logs lived on ephemeral storage that vanishes on reboot', 1),
  ( 4, 'servers crashed at random',    3, 'the machines had no swap and ran out of memory', 1),
  ( 5, 'servers crashed at random',    4, 'an out-of-memory error caused a kernel panic', 1),
  (11, 'release took 30 hours',     NULL, 'release took 30 hours', 0),
  (12, 'release took 30 hours',       11, 'every new push cancelled the run in flight', 1),
  (13, 'release took 30 hours',       12, 'the workflow cancels in-progress runs on a new push', 1),
  (21, 'build went red on Friday',  NULL, 'build went red on Friday', 0),
  (22, 'build went red on Friday',    21, 'a dependency published a broken version', 0),
  (23, 'build went red on Friday',    22, 'we do not control the registry', 0);
