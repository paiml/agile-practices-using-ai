-- Plausible mistake: the KPI is checked first, so a week that hit the target
-- is called healthy even while the guardrail is breached.
DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  w.week,
  100.0 * w.activated / w.signups AS activation_pct,
  100.0 * w.tickets   / w.signups AS ticket_pct,
  CASE
    WHEN w.activated * 100 >= t.min_activation_pct * w.signups       THEN 'healthy'
    WHEN w.tickets * 100 > t.max_ticket_pct * w.signups              THEN 'guardrail breach'
    ELSE 'kpi miss'
  END AS status
FROM weekly AS w, thresholds AS t;
