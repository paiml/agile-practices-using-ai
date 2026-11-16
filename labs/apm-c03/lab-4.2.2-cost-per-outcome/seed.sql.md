-- Lab 4.2.2 seed.
CREATE TABLE feature_costs (
  feature  TEXT NOT NULL,
  month    INTEGER NOT NULL,   -- 1 to 3
  cost_usd INTEGER NOT NULL,   -- what the feature cost to run that month
  outcomes INTEGER NOT NULL,   -- useful results it delivered (tickets resolved, exports done, ...)
  PRIMARY KEY (feature, month)
);
INSERT INTO feature_costs (feature, month, cost_usd, outcomes) VALUES
  ('AI triage',   1, 1000,  500),
  ('AI triage',   2, 1400,  800),
  ('AI triage',   3, 1800, 1200),
  ('Smart search',1,  800,  800),
  ('Smart search',2,  850,  700),
  ('Smart search',3,  900,  600),
  ('Bulk export', 1,  300,  100),
  ('Bulk export', 2,  320,  110),
  ('Bulk export', 3,  300,  100);
