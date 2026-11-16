-- Lab 3.3.1 seed.
CREATE TABLE thresholds (
  min_activation_pct INTEGER NOT NULL,  -- the KPI target: activated / signups, in percent
  max_ticket_pct     INTEGER NOT NULL   -- the guardrail: support tickets / signups, in percent
);
INSERT INTO thresholds (min_activation_pct, max_ticket_pct) VALUES (40, 8);

CREATE TABLE weekly (
  week      INTEGER PRIMARY KEY,
  signups   INTEGER NOT NULL,
  activated INTEGER NOT NULL,  -- signups that reached the first useful action
  tickets   INTEGER NOT NULL   -- support tickets opened by those signups
);
INSERT INTO weekly (week, signups, activated, tickets) VALUES
  (1, 200,  80, 10),
  (2, 250, 125, 25),
  (3, 220,  66, 11),
  (4, 300, 150, 24);
