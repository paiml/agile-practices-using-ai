-- Lab 4.3.2 seed.
CREATE TABLE scorecard (
  metric    TEXT PRIMARY KEY,
  value     REAL NOT NULL,
  target    REAL NOT NULL,
  direction TEXT NOT NULL CHECK (direction IN ('up', 'down'))  -- 'up': higher is better; 'down': lower is better
);
INSERT INTO scorecard (metric, value, target, direction) VALUES
  ('Activation rate (%)',          42,   40,  'up'),
  ('p99 latency (ms)',            900,  800,  'down'),
  ('Cost per outcome (USD)',     1.45, 1.50,  'down'),
  ('Week-4 retention (%)',         36,   40,  'up'),
  ('NPS',                          28,   35,  'up'),
  ('Tickets per 100 users',         4,    5,  'down');
