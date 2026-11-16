WITH expected (item, cumulative_weeks, decision) AS (
  VALUES
    ('Fix signup bug',      3, 'ship'),
    ('AI ticket triage',    7, 'ship'),
    ('Bulk export',        10, 'ship'),
    ('Usage-based pricing',15, 'cut'),
    ('Dark mode',          17, 'cut'),
    ('Smart search',       20, 'cut')
),
got AS (
  SELECT item, cumulative_weeks, decision FROM answer
)
SELECT CASE
  WHEN (SELECT COUNT(*) FROM got) = 6
   AND NOT EXISTS (SELECT * FROM got EXCEPT SELECT * FROM expected)
   AND NOT EXISTS (SELECT * FROM expected EXCEPT SELECT * FROM got)
  THEN 'PASS' ELSE 'FAIL' END;
