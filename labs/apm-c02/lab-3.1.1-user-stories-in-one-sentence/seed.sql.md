-- Lab 3.1.1 seed: seven draft user stories for the invoicing app and a draft
-- one-page requirement that cites three of them.
DROP TABLE IF EXISTS page_stories;
DROP TABLE IF EXISTS page_sections;
DROP TABLE IF EXISTS required_sections;
DROP TABLE IF EXISTS stories;
CREATE TABLE stories (
  id     INTEGER PRIMARY KEY,
  role   TEXT,   -- who:  "As a <role>,"
  goal   TEXT,   -- what: "I want to <goal>,"
  reason TEXT    -- why:  "so that <reason>."
);
CREATE TABLE required_sections (
  section TEXT PRIMARY KEY    -- what every one-page requirement must hold
);
CREATE TABLE page_sections (
  section TEXT PRIMARY KEY,   -- the draft page, one row per section
  body    TEXT
);
CREATE TABLE page_stories (
  story_id INTEGER PRIMARY KEY REFERENCES stories(id)   -- stories the page puts in scope
);
INSERT INTO stories (id, role, goal, reason) VALUES
  (1, 'freelancer',   'see which invoices are overdue on one screen', 'I stop checking my bank to find out'),
  (2, 'freelancer',   'have overdue clients reminded automatically',  'I get my Friday afternoon back'),
  (3, 'freelancer',   'export a year of invoices as one CSV',         ''),
  (4, 'user',         'have a dashboard',                             NULL),
  (5, NULL,           'send an invoice from my phone',                'I can bill before I leave the client site'),
  (6, 'bookkeeper',   '   ',                                          'April takes an hour, not a week'),
  (7, 'studio owner', 'see which teammate sent each invoice',         'I can answer a client without asking around');
INSERT INTO required_sections (section) VALUES
  ('problem'), ('user'), ('stories'), ('acceptance'), ('not now'), ('measure');
INSERT INTO page_sections (section, body) VALUES
  ('problem', 'Freelancers find out an invoice is overdue only when they check the bank. Chasing it costs about three hours each time.'),
  ('user',    'A freelancer who sends 5 to 30 invoices a month and has no bookkeeper.'),
  ('stories', 'Stories 1, 2 and 4.'),
  ('not now', 'Tax export. Team accounts. A phone app.'),
  ('measure', '');
INSERT INTO page_stories (story_id) VALUES (1), (2), (4);
