-- Lab 4.1.3 seed.
CREATE TABLE ab_results (
  experiment  TEXT NOT NULL,
  variant     TEXT NOT NULL CHECK (variant IN ('control', 'test')),
  users       INTEGER NOT NULL,
  conversions INTEGER NOT NULL,
  PRIMARY KEY (experiment, variant)
);
INSERT INTO ab_results (experiment, variant, users, conversions) VALUES
  ('new checkout button', 'control', 5000, 400),
  ('new checkout button', 'test',    5000, 460),
  ('short signup form',   'control',  200,  20),
  ('short signup form',   'test',     200,  30),
  ('auto-applied promo',  'control', 4000, 320),
  ('auto-applied promo',  'test',    4000, 250);
