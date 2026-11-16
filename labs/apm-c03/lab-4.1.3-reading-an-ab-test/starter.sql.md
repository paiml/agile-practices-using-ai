DROP VIEW IF EXISTS answer;
CREATE VIEW answer AS
SELECT
  experiment,
  0.0       AS lift_pct,   -- TODO: (test rate - control rate) / control rate, in percent
  0.0       AS z_squared,  -- TODO: (rb - ra)^2 / (p * (1 - p) * (1/na + 1/nb)), p = pooled rate
  'no call' AS verdict     -- TODO: 'ship', 'harm' or 'no call' (z_squared above 3.8416 is a real gap)
FROM ab_results
GROUP BY experiment;
