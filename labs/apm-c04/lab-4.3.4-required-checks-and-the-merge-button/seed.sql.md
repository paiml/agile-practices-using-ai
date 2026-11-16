-- Lab 4.3.4 seed: three required checks, six pull requests and the check runs each one reported.
DROP TABLE IF EXISTS check_runs;
DROP TABLE IF EXISTS prs;
DROP TABLE IF EXISTS required_checks;
CREATE TABLE required_checks (
  name TEXT PRIMARY KEY   -- a check that must be 'success' before merge
);
CREATE TABLE prs (
  id        INTEGER PRIMARY KEY,
  title     TEXT NOT NULL,
  approvals INTEGER NOT NULL
);
CREATE TABLE check_runs (
  pr_id  INTEGER NOT NULL REFERENCES prs(id),
  name   TEXT NOT NULL,
  status TEXT NOT NULL,   -- 'success', 'failure' or 'pending'
  PRIMARY KEY (pr_id, name)
);
INSERT INTO required_checks (name) VALUES ('build'), ('test'), ('lint');
INSERT INTO prs (id, title, approvals) VALUES
  (1, 'Add overdue filter',   1),
  (2, 'Add reminder email',   1),
  (3, 'Export as CSV',        1),
  (4, 'Retry on 503',         0),
  (5, 'Update library docs',  2),
  (6, 'Rename a variable',    1);
INSERT INTO check_runs (pr_id, name, status) VALUES
  (1, 'build', 'success'), (1, 'test', 'success'), (1, 'lint', 'success'), (1, 'bench', 'failure'),
  (2, 'build', 'success'), (2, 'test', 'failure'), (2, 'lint', 'success'),
  (3, 'build', 'success'), (3, 'test', 'success'),
  (4, 'build', 'success'), (4, 'test', 'success'), (4, 'lint', 'success'),
  (5, 'build', 'success'), (5, 'test', 'pending'), (5, 'lint', 'success'),
  (6, 'build', 'success'), (6, 'test', 'success'), (6, 'lint', 'success'), (6, 'bench', 'pending');
