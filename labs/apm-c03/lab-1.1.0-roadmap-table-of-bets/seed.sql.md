-- Lab 1.1.0 seed.
CREATE TABLE evidence_ladder (
  evidence   TEXT PRIMARY KEY,
  rung       INTEGER NOT NULL,
  confidence REAL NOT NULL
);
INSERT INTO evidence_ladder (evidence, rung, confidence) VALUES
  ('opinion',             1, 0.1),
  ('customer_interviews', 2, 0.3),
  ('prototype_test',      3, 0.5),
  ('paid_pilot',          4, 0.7),
  ('live_ab_test',        5, 0.9);

CREATE TABLE bets (
  id       INTEGER PRIMARY KEY,
  bet      TEXT NOT NULL,
  value_k  INTEGER NOT NULL,  -- yearly value in $k if the bet works
  cost_k   INTEGER NOT NULL,  -- cost in $k to build it
  evidence TEXT NOT NULL REFERENCES evidence_ladder (evidence)
);
INSERT INTO bets (id, bet, value_k, cost_k, evidence) VALUES
  (1, 'AI ticket triage',      400, 120, 'paid_pilot'),
  (2, 'Self-serve onboarding', 300,  60, 'customer_interviews'),
  (3, 'Enterprise SSO',        500, 150, 'opinion'),
  (4, 'Usage-based pricing',   900, 200, 'prototype_test'),
  (5, 'Mobile app rewrite',   1200, 600, 'opinion'),
  (6, 'Smart search',          250,  80, 'live_ab_test');
