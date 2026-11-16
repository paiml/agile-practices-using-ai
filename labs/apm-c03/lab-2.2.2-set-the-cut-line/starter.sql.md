DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  item,
  effort_weeks AS cumulative_weeks,  -- TODO: the running total of effort in priority order
  'ship'       AS decision           -- TODO: 'ship' while the running total fits capacity, else 'cut'
FROM backlog;
