-- Lab 2.1.2 seed: seven draft briefs for subagents and the paths each may write.
DROP TABLE IF EXISTS brief_paths;
DROP TABLE IF EXISTS briefs;
CREATE TABLE briefs (
  id        INTEGER PRIMARY KEY,
  task      TEXT,   -- what the subagent must do
  check_cmd TEXT    -- the command that proves it is done
);
CREATE TABLE brief_paths (
  brief_id INTEGER NOT NULL REFERENCES briefs(id),
  path     TEXT NOT NULL,   -- a file this subagent may write
  PRIMARY KEY (brief_id, path)
);
INSERT INTO briefs (id, task, check_cmd) VALUES
  (1, 'add overdue filter',      'make test'),
  (2, 'add reminder email',      'make test'),
  (3, 'export CSV',              NULL),
  (4, '   ',                     'make lint'),
  (5, 'update library exports',  'make test'),
  (6, 'write the user guide',    'make docs'),
  (7, 'retry on 503',            'make test');
INSERT INTO brief_paths (brief_id, path) VALUES
  (1, 'src/filter.rs'), (1, 'tests/filter.rs'),
  (2, 'src/mail.rs'),   (2, 'src/lib.rs'),
  (3, 'src/export.rs'),
  (4, 'src/ui.rs'),
  (5, 'src/lib.rs'),
  (7, 'src/http.rs');
