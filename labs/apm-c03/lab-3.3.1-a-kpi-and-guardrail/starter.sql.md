DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  week,
  activated AS activation_pct,  -- TODO: activated as a percent of signups
  tickets   AS ticket_pct,      -- TODO: tickets as a percent of signups
  'healthy' AS status           -- TODO: 'guardrail breach', 'kpi miss' or 'healthy'
FROM weekly;
