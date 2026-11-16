-- Lab 3.1.3 seed.
CREATE TABLE weekly_signups (
  week      TEXT NOT NULL,     -- ISO week of sign-up
  channel   TEXT NOT NULL,
  signups   INTEGER NOT NULL,  -- new accounts that week
  activated INTEGER NOT NULL,  -- of those, finished setup within 7 days
  paid_w4   INTEGER,           -- of those, paying 4 weeks later; NULL = not known yet
  PRIMARY KEY (week, channel)
);
INSERT INTO weekly_signups (week, channel, signups, activated, paid_w4) VALUES
  ('2026-W01', 'organic',  100, 40, 12),
  ('2026-W01', 'paid_ads', 100, 20,  4),
  ('2026-W02', 'organic',  110, 44, 13),
  ('2026-W02', 'paid_ads',  90, 18,  3),
  ('2026-W03', 'organic',  120, 48, 14),
  ('2026-W03', 'paid_ads', 130, 27,  6),
  ('2026-W04', 'organic',  120, 48, NULL),
  ('2026-W04', 'paid_ads', 380, 27, NULL),
  ('2026-W05', 'organic',  125, 50, NULL),
  ('2026-W05', 'paid_ads', 475, 40, NULL),
  ('2026-W06', 'organic',  130, 52, NULL),
  ('2026-W06', 'paid_ads', 670, 44, NULL);
