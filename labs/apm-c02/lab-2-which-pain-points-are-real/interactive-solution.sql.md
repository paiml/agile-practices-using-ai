-- Lab 2 (solution): Which Pain Points Are Real
-- Press Run. It prints FAIL until your answer is right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: keep a tag only if at least 3 different interviewees raised it.
  -- The rows must come out most-raised first, ties by tag.
  (SELECT group_concat(a.tag || '|' || a.interviewees, ',')
          = 'confusing-pricing|4,no-invoice|4,slow-checkout|3' FROM (
    SELECT tag,
-- ===== YOUR WORK 1 of 2: edit only between these lines =====
           COUNT(DISTINCT interview_id) AS interviewees
-- ===== END OF YOUR WORK 1 of 2 =====
    FROM (
      SELECT 1 AS interview_id, 'slow-checkout' AS tag, 'Paying took forever' AS quote
      UNION ALL SELECT 1, 'slow-checkout', 'Again, the checkout page hung'
      UNION ALL SELECT 1, 'no-invoice', 'I could not get an invoice'
      UNION ALL SELECT 2, 'slow-checkout', 'Checkout is slow on my phone'
      UNION ALL SELECT 2, 'confusing-pricing', 'Which tier do I need?'
      UNION ALL SELECT 3, 'confusing-pricing', 'Pricing page is a maze'
      UNION ALL SELECT 3, 'slow-checkout', 'Spinner for ten seconds'
      UNION ALL SELECT 3, 'no-invoice', 'Finance needs invoices'
      UNION ALL SELECT 4, 'no-invoice', 'No PDF invoice'
      UNION ALL SELECT 4, 'dark-mode', 'Would like dark mode'
      UNION ALL SELECT 5, 'confusing-pricing', 'Tiers overlap'
      UNION ALL SELECT 5, 'dark-mode', 'Eyes hurt at night'
      UNION ALL SELECT 6, 'no-invoice', 'Still no invoice export'
      UNION ALL SELECT 6, 'confusing-pricing', 'Unclear what is included'
    ) notes
-- ===== YOUR WORK 2 of 2: edit only between these lines =====
    GROUP BY tag
    HAVING COUNT(DISTINCT interview_id) >= 3
    ORDER BY interviewees DESC, tag
-- ===== END OF YOUR WORK 2 of 2 =====
  ) a)
THEN 'PASS' ELSE 'FAIL' END AS result;
