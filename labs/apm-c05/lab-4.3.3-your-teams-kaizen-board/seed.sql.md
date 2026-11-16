-- Lab 4.3.3 seed: a team's kaizen board, one row per improvement idea.
DROP TABLE IF EXISTS kaizen;
CREATE TABLE kaizen (
  id     INTEGER PRIMARY KEY,
  idea   TEXT,
  status TEXT,      -- 'open' or 'done'
  impact INTEGER,   -- 1 to 10
  effort INTEGER    -- 1 to 10
);
INSERT INTO kaizen (id, idea, status, impact, effort) VALUES
  (1, 'Add andon check to CI',        'open', 8, 2),
  (2, 'Rewrite the build system',     'open', 9, 9),
  (3, 'Cache dependency downloads',   'open', 6, 2),
  (4, 'Pair on flaky test',           'done', 7, 1),
  (5, 'Weekly metrics review',        'open', 5, 1),
  (6, 'Delete dead feature flags',    'open', 3, 3),
  (7, 'Auto-label stale tickets',     'open', 4, 1);
