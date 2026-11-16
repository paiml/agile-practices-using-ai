-- Lab 2.1.0 seed.
CREATE TABLE bets (
  id           INTEGER PRIMARY KEY,
  bet          TEXT NOT NULL,
  impact       INTEGER NOT NULL,  -- 1 (small) to 10 (large)
  effort_weeks INTEGER NOT NULL,  -- engineer-weeks to ship
  confidence   REAL NOT NULL      -- 0.0 to 1.0, from the evidence ladder
);
INSERT INTO bets (id, bet, impact, effort_weeks, confidence) VALUES
  (1, 'AI ticket triage',       8,  6, 0.7),
  (2, 'Self-serve onboarding',  6,  3, 0.3),
  (3, 'Enterprise SSO',         6,  4, 0.1),
  (4, 'Usage-based pricing',    9, 10, 0.5),
  (5, 'Mobile app rewrite',    10, 24, 0.1),
  (6, 'Smart search',           5,  4, 0.8),
  (7, 'Bulk export',            3,  1, 0.9),
  (8, 'Dark mode',              2,  2, 0.9);
