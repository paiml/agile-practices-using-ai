DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  w.week,
  100.0 * w.activated / w.signups AS activation_pct,
  100.0 * w.tickets   / w.signups AS ticket_pct,
  CASE
    WHEN w.tickets * 100 > t.max_ticket_pct * w.signups              THEN 'guardrail breach'
    WHEN w.activated * 100 < t.min_activation_pct * w.signups        THEN 'kpi miss'
    ELSE 'healthy'
  END AS status
FROM weekly AS w, thresholds AS t;
