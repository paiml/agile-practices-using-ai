-- Lab 4.2.1 seed.
CREATE TABLE budget (
  percentile TEXT PRIMARY KEY,   -- 'p50' or 'p99'
  budget_ms  INTEGER NOT NULL    -- what the whole request may take at that percentile
);
INSERT INTO budget (percentile, budget_ms) VALUES ('p50', 280), ('p99', 600);

CREATE TABLE steps (
  step   TEXT PRIMARY KEY,
  p50_ms INTEGER NOT NULL,       -- typical time spent in this step
  p99_ms INTEGER NOT NULL        -- slow-tail time spent in this step
);
INSERT INTO steps (step, p50_ms, p99_ms) VALUES
  ('auth',     15,  40),
  ('search',  100, 380),
  ('ranking', 120, 150),
  ('render',   45,  90);
