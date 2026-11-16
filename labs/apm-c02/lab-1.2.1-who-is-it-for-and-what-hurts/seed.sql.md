-- Lab 1.2.1 seed: an invoicing app for small businesses, 16 users, 25 support tickets.
DROP TABLE IF EXISTS tickets;
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id      INTEGER PRIMARY KEY,
  segment TEXT NOT NULL,   -- freelancer, agency, shop
  plan    TEXT NOT NULL    -- free, paid
);
CREATE TABLE tickets (
  id           INTEGER PRIMARY KEY,
  user_id      INTEGER NOT NULL REFERENCES users(id),
  opened_on    TEXT NOT NULL,      -- ISO date
  complaint    TEXT NOT NULL,      -- already labelled by support
  minutes_lost INTEGER NOT NULL    -- time the user says the problem cost them
);
INSERT INTO users (id, segment, plan) VALUES
  (1,'freelancer','paid'), (2,'freelancer','free'), (3,'freelancer','paid'), (4,'freelancer','free'),
  (5,'freelancer','paid'), (6,'freelancer','free'), (7,'freelancer','paid'), (8,'freelancer','free'),
  (9,'agency','paid'), (10,'agency','paid'), (11,'agency','paid'), (12,'agency','free'),
  (13,'shop','paid'), (14,'shop','free'), (15,'shop','free'), (16,'shop','paid');
INSERT INTO tickets (id, user_id, opened_on, complaint, minutes_lost) VALUES
  (1,  1, '2026-02-02', 'chasing late payments', 180),
  (2,  9, '2026-02-02', 'login',                  20),
  (3,  2, '2026-02-03', 'chasing late payments', 180),
  (4,  9, '2026-02-03', 'login',                  20),
  (5, 13, '2026-02-04', 'tax export',            120),
  (6,  3, '2026-02-05', 'chasing late payments', 180),
  (7,  9, '2026-02-05', 'login',                  20),
  (8,  7, '2026-02-06', 'tax export',            120),
  (9,  9, '2026-02-09', 'login',                  20),
  (10, 4, '2026-02-09', 'chasing late payments', 180),
  (11, 11,'2026-02-10', 'recurring invoices',     60),
  (12, 9, '2026-02-10', 'login',                  20),
  (13, 5, '2026-02-11', 'chasing late payments', 180),
  (14, 14,'2026-02-11', 'tax export',            120),
  (15, 9, '2026-02-12', 'login',                  20),
  (16, 10,'2026-02-12', 'login',                  20),
  (17, 6, '2026-02-13', 'chasing late payments', 180),
  (18, 9, '2026-02-16', 'login',                  20),
  (19, 8, '2026-02-16', 'tax export',            120),
  (20, 12,'2026-02-17', 'recurring invoices',     60),
  (21, 9, '2026-02-17', 'login',                  20),
  (22, 15,'2026-02-18', 'tax export',            120),
  (23, 1, '2026-02-19', 'chasing late payments', 180),
  (24, 9, '2026-02-19', 'login',                  20),
  (25, 16,'2026-02-20', 'chasing late payments', 180);
