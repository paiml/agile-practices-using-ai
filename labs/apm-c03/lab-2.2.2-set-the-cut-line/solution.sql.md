DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  item,
  SUM(effort_weeks) OVER (ORDER BY priority) AS cumulative_weeks,
  CASE WHEN SUM(effort_weeks) OVER (ORDER BY priority) <= (SELECT weeks FROM capacity)
       THEN 'ship' ELSE 'cut' END AS decision
FROM backlog;
