-- Lab 4.1.3 (solution): Reading an A/B Test
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
-- In each row below: ra and rb are the control and test conversion rates, p is the pooled rate, inv is 1/na + 1/nb.
SELECT CASE WHEN
  -- Task 1: lift_pct, (test rate - control rate) / control rate, in percent.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.experiment) = 3 FROM (
    SELECT s.experiment AS experiment,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           100.0 * (s.rb - s.ra) / s.ra AS lift_pct
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
      SELECT p.experiment AS experiment, 1.0 * p.ca / p.na AS ra, 1.0 * p.cb / p.nb AS rb,
        1.0 * (p.ca + p.cb) / (p.na + p.nb) AS p, 1.0 / p.na + 1.0 / p.nb AS inv
      FROM (
        SELECT g.experiment AS experiment,
          SUM(CASE WHEN g.variant = 'control' THEN g.users END) AS na,
          SUM(CASE WHEN g.variant = 'control' THEN g.conversions END) AS ca,
          SUM(CASE WHEN g.variant = 'test' THEN g.users END) AS nb,
          SUM(CASE WHEN g.variant = 'test' THEN g.conversions END) AS cb
        FROM (
          SELECT 'new checkout button' AS experiment, 'control' AS variant, 5000 AS users, 400 AS conversions
          UNION ALL SELECT 'new checkout button', 'test', 5000, 460
          UNION ALL SELECT 'short signup form', 'control', 200, 20
          UNION ALL SELECT 'short signup form', 'test', 200, 30
          UNION ALL SELECT 'auto-applied promo', 'control', 4000, 320
          UNION ALL SELECT 'auto-applied promo', 'test', 4000, 250
        ) g
        GROUP BY g.experiment
      ) p
    ) s
  ) a LEFT JOIN (
      SELECT 'new checkout button' AS experiment, 15.0 AS lift_pct
      UNION ALL SELECT 'short signup form', 50.0
      UNION ALL SELECT 'auto-applied promo', -21.9
  ) e ON e.experiment = a.experiment AND e.lift_pct = ROUND(a.lift_pct, 1))
  AND
  -- Task 2: z_squared, (rb - ra)^2 / (p * (1 - p) * (1/na + 1/nb)).
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.experiment) = 3 FROM (
    SELECT s.experiment AS experiment,
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
           (s.rb - s.ra) * (s.rb - s.ra) / (s.p * (1 - s.p) * s.inv) AS z_squared
-- ===== END OF YOUR WORK 2 of 3 =====
    FROM (
      SELECT p.experiment AS experiment, 1.0 * p.ca / p.na AS ra, 1.0 * p.cb / p.nb AS rb,
        1.0 * (p.ca + p.cb) / (p.na + p.nb) AS p, 1.0 / p.na + 1.0 / p.nb AS inv
      FROM (
        SELECT g.experiment AS experiment,
          SUM(CASE WHEN g.variant = 'control' THEN g.users END) AS na,
          SUM(CASE WHEN g.variant = 'control' THEN g.conversions END) AS ca,
          SUM(CASE WHEN g.variant = 'test' THEN g.users END) AS nb,
          SUM(CASE WHEN g.variant = 'test' THEN g.conversions END) AS cb
        FROM (
          SELECT 'new checkout button' AS experiment, 'control' AS variant, 5000 AS users, 400 AS conversions
          UNION ALL SELECT 'new checkout button', 'test', 5000, 460
          UNION ALL SELECT 'short signup form', 'control', 200, 20
          UNION ALL SELECT 'short signup form', 'test', 200, 30
          UNION ALL SELECT 'auto-applied promo', 'control', 4000, 320
          UNION ALL SELECT 'auto-applied promo', 'test', 4000, 250
        ) g
        GROUP BY g.experiment
      ) p
    ) s
  ) a LEFT JOIN (
      SELECT 'new checkout button' AS experiment, 4.58 AS z_squared
      UNION ALL SELECT 'short signup form', 2.29
      UNION ALL SELECT 'auto-applied promo', 9.26
  ) e ON e.experiment = a.experiment AND e.z_squared = ROUND(a.z_squared, 2))
  AND
  -- Task 3: verdict. z_squared above 3.8416 is a real gap: 'ship' if the test rate is higher, else 'harm'. Otherwise 'no call'.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.experiment) = 3 FROM (
    SELECT s.experiment AS experiment,
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
           CASE
             WHEN (s.rb - s.ra) * (s.rb - s.ra) / (s.p * (1 - s.p) * s.inv) <= 3.8416 THEN 'no call'
             WHEN s.rb > s.ra THEN 'ship'
             ELSE 'harm'
           END AS verdict
-- ===== END OF YOUR WORK 3 of 3 =====
    FROM (
      SELECT p.experiment AS experiment, 1.0 * p.ca / p.na AS ra, 1.0 * p.cb / p.nb AS rb,
        1.0 * (p.ca + p.cb) / (p.na + p.nb) AS p, 1.0 / p.na + 1.0 / p.nb AS inv
      FROM (
        SELECT g.experiment AS experiment,
          SUM(CASE WHEN g.variant = 'control' THEN g.users END) AS na,
          SUM(CASE WHEN g.variant = 'control' THEN g.conversions END) AS ca,
          SUM(CASE WHEN g.variant = 'test' THEN g.users END) AS nb,
          SUM(CASE WHEN g.variant = 'test' THEN g.conversions END) AS cb
        FROM (
          SELECT 'new checkout button' AS experiment, 'control' AS variant, 5000 AS users, 400 AS conversions
          UNION ALL SELECT 'new checkout button', 'test', 5000, 460
          UNION ALL SELECT 'short signup form', 'control', 200, 20
          UNION ALL SELECT 'short signup form', 'test', 200, 30
          UNION ALL SELECT 'auto-applied promo', 'control', 4000, 320
          UNION ALL SELECT 'auto-applied promo', 'test', 4000, 250
        ) g
        GROUP BY g.experiment
      ) p
    ) s
  ) a LEFT JOIN (
      SELECT 'new checkout button' AS experiment, 'ship' AS verdict
      UNION ALL SELECT 'short signup form', 'no call'
      UNION ALL SELECT 'auto-applied promo', 'harm'
  ) e ON e.experiment = a.experiment AND e.verdict = a.verdict)
THEN 'PASS' ELSE 'FAIL' END AS result;
