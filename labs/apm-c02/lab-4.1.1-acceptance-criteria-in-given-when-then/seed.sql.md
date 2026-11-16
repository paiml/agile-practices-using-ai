-- Lab 4.1.1 seed: five invoices and the date the reminder job runs.
-- These rows are the "Given" of each acceptance criterion.
DROP TABLE IF EXISTS invoices;
DROP TABLE IF EXISTS clock;
CREATE TABLE clock (
  today TEXT NOT NULL          -- ISO date; one row
);
CREATE TABLE invoices (
  id           INTEGER PRIMARY KEY,
  client       TEXT NOT NULL,
  amount_cents INTEGER NOT NULL,
  due_on       TEXT NOT NULL,  -- ISO date
  paid_on      TEXT            -- ISO date, NULL while unpaid
);
INSERT INTO clock (today) VALUES ('2026-03-10');
INSERT INTO invoices (id, client, amount_cents, due_on, paid_on) VALUES
  (101, 'Harbor Dental',      48000, '2026-03-02', NULL),          -- unpaid, 8 days past due
  (102, 'Pine Street Bakery', 12500, '2026-02-20', '2026-03-05'),  -- paid, was 13 days late
  (103, 'Lumen Studio',       90000, '2026-03-03', NULL),          -- unpaid, exactly 7 days past due
  (104, 'Orchard Vets',       30000, '2026-03-20', NULL),          -- unpaid, not due yet
  (105, 'Kite Legal',         15000, '2026-02-01', NULL);          -- unpaid, 37 days past due
