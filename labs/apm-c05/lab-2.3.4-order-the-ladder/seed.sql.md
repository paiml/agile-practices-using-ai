-- Lab 2.3.4 seed: the checks one pipeline can run. seconds = how long one run takes.
-- catches = defects the check caught per 100 changes.
DROP TABLE IF EXISTS gate;
CREATE TABLE gate (
  name    TEXT PRIMARY KEY,
  seconds INTEGER,
  catches INTEGER
);
INSERT INTO gate (name, seconds, catches) VALUES
  ('format',            2, 12),
  ('lint',              8, 20),
  ('type check',       25, 15),
  ('unit tests',       90, 30),
  ('integration',     420, 12),
  ('clean-room build', 600,  6),
  ('spell check',       5,  0),
  ('old smoke test',  300,  0);
