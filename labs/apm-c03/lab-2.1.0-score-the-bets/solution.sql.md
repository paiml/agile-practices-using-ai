DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  bet,
  impact * confidence / effort_weeks AS score,
  ROW_NUMBER() OVER (ORDER BY impact * confidence / effort_weeks DESC) AS rank
FROM bets;
