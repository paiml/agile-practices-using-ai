-- Plausible mistake: impact / effort_weeks runs first, and both are integers,
-- so 8 / 6 becomes 1 and 5 / 4 becomes 1 before confidence is applied.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  bet,
  impact / effort_weeks * confidence AS score,
  ROW_NUMBER() OVER (ORDER BY impact / effort_weeks * confidence DESC) AS rank
FROM bets;
