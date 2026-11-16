-- Lab 3.1.4: Find the Muda in One Sprint
-- Press Run. It prints FAIL until your answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: the hours of each waste, and its share of ALL wasted hours (whole percent).
  -- A feature row is value work: it is not in the waste list, so it is not wasted hours.
  -- Return waste_name, hours and share_pct.
  (SELECT COUNT(*) = 4 AND COUNT(DISTINCT e.waste_name) = 4 FROM (
    SELECT w.waste_name AS waste_name,
-- ===== YOUR WORK 1 of 3: edit only between these lines =====
           -- TODO: hours = the hours of this waste.
           -- share_pct = its share of the wasted hours, in whole percent.
           -- t.all_hours is every hour of the sprint, features included.
           -- t.waste_hours is only the hours of work whose kind is a waste.
           -- This takes the share of ALL hours.
           SUM(l.hours) AS hours,
           ROUND(SUM(l.hours) * 100.0 / t.all_hours, 0) AS share_pct
-- ===== END OF YOUR WORK 1 of 3 =====
    FROM (
      SELECT 1 AS id, 'new export feature customers asked for' AS what, 'feature' AS kind, 12 AS hours
      UNION ALL SELECT 2, 'agent wrote three helpers nobody calls', 'unused code', 6
      UNION ALL SELECT 3, 'waiting for the nightly job to finish', 'waiting', 9
      UNION ALL SELECT 4, 'fix a bug that tests would have caught', 'rework', 10
      UNION ALL SELECT 5, 'billing page customers asked for', 'feature', 14
      UNION ALL SELECT 6, 'second agent re-did the first agent''s task', 'duplicate work', 7
      UNION ALL SELECT 7, 'waiting for review on a 900-line PR', 'waiting', 8
      UNION ALL SELECT 8, 'agent tuned a function nobody runs', 'unused code', 5
      UNION ALL SELECT 9, 're-run the flaky test until green', 'rework', 4
      UNION ALL SELECT 10, 'fix a bug that escaped to production', 'rework', 11
      UNION ALL SELECT 11, 'search page customers asked for', 'feature', 9
      UNION ALL SELECT 12, 'two agents opened the same ticket', 'duplicate work', 3
    ) l
    JOIN (
      SELECT 'unused code' AS kind, 'overproduction' AS waste_name
      UNION ALL SELECT 'waiting', 'waiting'
      UNION ALL SELECT 'rework', 'defects'
      UNION ALL SELECT 'duplicate work', 'motion and duplication'
    ) w ON w.kind = l.kind,
    (
      SELECT SUM(x.hours) AS all_hours,
             SUM(CASE WHEN y.kind IS NULL THEN 0 ELSE x.hours END) AS waste_hours
      FROM (
      SELECT 1 AS id, 'new export feature customers asked for' AS what, 'feature' AS kind, 12 AS hours
      UNION ALL SELECT 2, 'agent wrote three helpers nobody calls', 'unused code', 6
      UNION ALL SELECT 3, 'waiting for the nightly job to finish', 'waiting', 9
      UNION ALL SELECT 4, 'fix a bug that tests would have caught', 'rework', 10
      UNION ALL SELECT 5, 'billing page customers asked for', 'feature', 14
      UNION ALL SELECT 6, 'second agent re-did the first agent''s task', 'duplicate work', 7
      UNION ALL SELECT 7, 'waiting for review on a 900-line PR', 'waiting', 8
      UNION ALL SELECT 8, 'agent tuned a function nobody runs', 'unused code', 5
      UNION ALL SELECT 9, 're-run the flaky test until green', 'rework', 4
      UNION ALL SELECT 10, 'fix a bug that escaped to production', 'rework', 11
      UNION ALL SELECT 11, 'search page customers asked for', 'feature', 9
      UNION ALL SELECT 12, 'two agents opened the same ticket', 'duplicate work', 3
      ) x
      LEFT JOIN (
      SELECT 'unused code' AS kind, 'overproduction' AS waste_name
      UNION ALL SELECT 'waiting', 'waiting'
      UNION ALL SELECT 'rework', 'defects'
      UNION ALL SELECT 'duplicate work', 'motion and duplication'
      ) y ON y.kind = x.kind
    ) t
    GROUP BY w.waste_name
  ) a LEFT JOIN (
      SELECT 'defects' AS waste_name, 25 AS hours, 40 AS share_pct
      UNION ALL SELECT 'waiting', 17, 27
      UNION ALL SELECT 'overproduction', 11, 17
      UNION ALL SELECT 'motion and duplication', 10, 16
  ) e ON e.waste_name = a.waste_name AND e.hours = a.hours AND e.share_pct = a.share_pct)
  AND
  -- Task 2: the hours of value work (kinds that are not a waste). One column: value_hours.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.value_hours) = 1 FROM (
    SELECT SUM(l.hours) AS value_hours
    FROM (
      SELECT 1 AS id, 'new export feature customers asked for' AS what, 'feature' AS kind, 12 AS hours
      UNION ALL SELECT 2, 'agent wrote three helpers nobody calls', 'unused code', 6
      UNION ALL SELECT 3, 'waiting for the nightly job to finish', 'waiting', 9
      UNION ALL SELECT 4, 'fix a bug that tests would have caught', 'rework', 10
      UNION ALL SELECT 5, 'billing page customers asked for', 'feature', 14
      UNION ALL SELECT 6, 'second agent re-did the first agent''s task', 'duplicate work', 7
      UNION ALL SELECT 7, 'waiting for review on a 900-line PR', 'waiting', 8
      UNION ALL SELECT 8, 'agent tuned a function nobody runs', 'unused code', 5
      UNION ALL SELECT 9, 're-run the flaky test until green', 'rework', 4
      UNION ALL SELECT 10, 'fix a bug that escaped to production', 'rework', 11
      UNION ALL SELECT 11, 'search page customers asked for', 'feature', 9
      UNION ALL SELECT 12, 'two agents opened the same ticket', 'duplicate work', 3
    ) l
    LEFT JOIN (
      SELECT 'unused code' AS kind, 'overproduction' AS waste_name
      UNION ALL SELECT 'waiting', 'waiting'
      UNION ALL SELECT 'rework', 'defects'
      UNION ALL SELECT 'duplicate work', 'motion and duplication'
    ) w ON w.kind = l.kind
-- ===== YOUR WORK 2 of 3: edit only between these lines =====
    -- TODO: this adds up every hour. Keep only the rows whose kind is not a waste.
    -- The join above leaves w empty (NULL) for a kind that is not in the waste list.
    WHERE 1 = 1
-- ===== END OF YOUR WORK 2 of 3 =====
  ) a LEFT JOIN (
      SELECT 35 AS value_hours
  ) e ON e.value_hours = a.value_hours)
  AND
  -- Task 3: the biggest waste. One row: waste_name. If two wastes tie, the lower name in the alphabet.
  -- The rows below are the right answer to task 1, in no useful order.
  (SELECT COUNT(*) = 1 AND SUM(a.waste_name = 'defects') = 1 FROM (
    SELECT s.waste_name AS waste_name
    FROM (
      SELECT 'overproduction' AS waste_name, 11 AS hours, 17 AS share_pct
      UNION ALL SELECT 'waiting', 17, 27
      UNION ALL SELECT 'motion and duplication', 10, 16
      UNION ALL SELECT 'defects', 25, 40
    ) s
-- ===== YOUR WORK 3 of 3: edit only between these lines =====
    -- TODO: this returns the smallest waste. Biggest hours first, then the lower name.
    ORDER BY s.hours, s.waste_name
-- ===== END OF YOUR WORK 3 of 3 =====
    LIMIT 1
  ) a)
THEN 'PASS' ELSE 'FAIL' END AS result;
