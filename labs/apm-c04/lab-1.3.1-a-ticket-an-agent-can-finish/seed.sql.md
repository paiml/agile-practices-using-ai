-- Lab 1.3.1 seed: seven draft tickets for an invoicing app and the words that make a done condition vague.
DROP TABLE IF EXISTS vague_words;
DROP TABLE IF EXISTS tickets;
CREATE TABLE tickets (
  id            INTEGER PRIMARY KEY,
  title         TEXT,
  done_condition TEXT,
  plan_approved INTEGER NOT NULL DEFAULT 0   -- 1 when a human approved the agent's plan
);
CREATE TABLE vague_words (
  word TEXT PRIMARY KEY   -- a done condition containing one of these cannot be measured
);
INSERT INTO tickets (id, title, done_condition, plan_approved) VALUES
  (1, 'Add overdue filter',  'make test exits 0 and the new test overdue_filter passes', 1),
  (2, 'Speed up export',     'Export is fast',                                            1),
  (3, 'Fix rounding',        '',                                                          0),
  (4, '   ',                 'make lint prints 0 warnings',                               1),
  (5, 'Add CSV header',      'The CSV has a header row and make test exits 0',            0),
  (6, 'Reminder email',      'The reminder looks nicer and works',                        0),
  (7, 'Retry on 503',        'curl against the stub returns 200 after 2 retries; make test exits 0', 1);
INSERT INTO vague_words (word) VALUES ('fast'), ('nicer'), ('better'), ('works'), ('good');
