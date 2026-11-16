-- Lab 3.1.4 seed: one sprint of an AI-assisted team, and the list of kinds of work that are waste.
DROP TABLE IF EXISTS waste;
DROP TABLE IF EXISTS work_log;
CREATE TABLE work_log (
  id    INTEGER PRIMARY KEY,
  what  TEXT,
  kind  TEXT,      -- what the hours went to
  hours INTEGER
);
CREATE TABLE waste (
  kind       TEXT PRIMARY KEY,   -- a kind of work that is a waste
  waste_name TEXT                -- what the waste is called
);
INSERT INTO work_log (id, what, kind, hours) VALUES
  ( 1, 'new export feature customers asked for',   'feature',        12),
  ( 2, 'agent wrote three helpers nobody calls',   'unused code',     6),
  ( 3, 'waiting for the nightly job to finish',    'waiting',         9),
  ( 4, 'fix a bug that tests would have caught',   'rework',         10),
  ( 5, 'billing page customers asked for',         'feature',        14),
  ( 6, 'second agent re-did the first agent''s task', 'duplicate work', 7),
  ( 7, 'waiting for review on a 900-line PR',      'waiting',         8),
  ( 8, 'agent tuned a function nobody runs',       'unused code',     5),
  ( 9, 're-run the flaky test until green',        'rework',          4),
  (10, 'fix a bug that escaped to production',     'rework',         11),
  (11, 'search page customers asked for',          'feature',         9),
  (12, 'two agents opened the same ticket',        'duplicate work',  3);
INSERT INTO waste (kind, waste_name) VALUES
  ('unused code',    'overproduction'),
  ('waiting',        'waiting'),
  ('rework',         'defects'),
  ('duplicate work', 'motion and duplication');
