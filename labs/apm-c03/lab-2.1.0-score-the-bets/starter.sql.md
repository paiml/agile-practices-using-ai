DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  bet,
  impact AS score,                                 -- TODO: impact x confidence / effort
  ROW_NUMBER() OVER (ORDER BY impact DESC) AS rank -- TODO: rank by the score, best first
FROM bets;
