-- Lab 2.2.2 seed.
CREATE TABLE capacity (
  weeks INTEGER NOT NULL            -- engineer-weeks the team has this cycle
);
INSERT INTO capacity (weeks) VALUES (10);

CREATE TABLE backlog (
  priority     INTEGER PRIMARY KEY, -- 1 = most important, already ranked
  item         TEXT NOT NULL,
  effort_weeks INTEGER NOT NULL
);
INSERT INTO backlog (priority, item, effort_weeks) VALUES
  (1, 'Fix signup bug',       3),
  (2, 'AI ticket triage',     4),
  (3, 'Bulk export',          3),
  (4, 'Usage-based pricing',  5),
  (5, 'Dark mode',            2),
  (6, 'Smart search',         3);
