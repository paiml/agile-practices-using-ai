-- Lab 3.1.1: User Stories in One Sentence
-- Press Run. It prints FAIL until your four answers are right, then PASS.
-- Edit only the lines between the YOUR WORK markers. Leave the data rows alone.
SELECT CASE WHEN
  -- Task 1: every missing part of every story, one row per missing part.
  -- missing is 'role', 'goal' or 'reason'. NULL, an empty string and only spaces all count as missing.
  (SELECT COUNT(*) = 4 AND COUNT(DISTINCT e.story_id || '/' || e.missing) = 4 FROM (
    SELECT s.id AS story_id, 'role' AS missing
    FROM (
      SELECT 1 AS id, 'freelancer' AS role, 'see which invoices are overdue on one screen' AS goal, 'I stop checking my bank to find out' AS reason
      UNION ALL SELECT 2, 'freelancer', 'have overdue clients reminded automatically', 'I get my Friday afternoon back'
      UNION ALL SELECT 3, 'freelancer', 'export a year of invoices as one CSV', ''
      UNION ALL SELECT 4, 'user', 'have a dashboard', NULL
      UNION ALL SELECT 5, NULL, 'send an invoice from my phone', 'I can bill before I leave the client site'
      UNION ALL SELECT 6, 'bookkeeper', '   ', 'April takes an hour, not a week'
      UNION ALL SELECT 7, 'studio owner', 'see which teammate sent each invoice', 'I can answer a client without asking around'
    ) s
-- ===== YOUR WORK 1 of 7: edit only between these lines =====
    -- TODO: a part is also missing when it is an empty string or only spaces.
    WHERE s.role IS NULL
-- ===== END OF YOUR WORK 1 of 7 =====
    UNION ALL
    SELECT s.id, 'goal'
    FROM (
      SELECT 1 AS id, 'freelancer' AS role, 'see which invoices are overdue on one screen' AS goal, 'I stop checking my bank to find out' AS reason
      UNION ALL SELECT 2, 'freelancer', 'have overdue clients reminded automatically', 'I get my Friday afternoon back'
      UNION ALL SELECT 3, 'freelancer', 'export a year of invoices as one CSV', ''
      UNION ALL SELECT 4, 'user', 'have a dashboard', NULL
      UNION ALL SELECT 5, NULL, 'send an invoice from my phone', 'I can bill before I leave the client site'
      UNION ALL SELECT 6, 'bookkeeper', '   ', 'April takes an hour, not a week'
      UNION ALL SELECT 7, 'studio owner', 'see which teammate sent each invoice', 'I can answer a client without asking around'
    ) s
-- ===== YOUR WORK 2 of 7: edit only between these lines =====
    -- TODO: add the same test for goal.
    WHERE 0
-- ===== END OF YOUR WORK 2 of 7 =====
    UNION ALL
    SELECT s.id, 'reason'
    FROM (
      SELECT 1 AS id, 'freelancer' AS role, 'see which invoices are overdue on one screen' AS goal, 'I stop checking my bank to find out' AS reason
      UNION ALL SELECT 2, 'freelancer', 'have overdue clients reminded automatically', 'I get my Friday afternoon back'
      UNION ALL SELECT 3, 'freelancer', 'export a year of invoices as one CSV', ''
      UNION ALL SELECT 4, 'user', 'have a dashboard', NULL
      UNION ALL SELECT 5, NULL, 'send an invoice from my phone', 'I can bill before I leave the client site'
      UNION ALL SELECT 6, 'bookkeeper', '   ', 'April takes an hour, not a week'
      UNION ALL SELECT 7, 'studio owner', 'see which teammate sent each invoice', 'I can answer a client without asking around'
    ) s
-- ===== YOUR WORK 3 of 7: edit only between these lines =====
    -- TODO: add the same test for reason.
    WHERE 0
-- ===== END OF YOUR WORK 3 of 7 =====
  ) a LEFT JOIN (
      SELECT 3 AS story_id, 'reason' AS missing
      UNION ALL SELECT 4, 'reason'
      UNION ALL SELECT 5, 'role'
      UNION ALL SELECT 6, 'goal'
  ) e ON e.story_id = a.story_id AND e.missing = a.missing)
  AND
  -- Task 2: each complete story as one sentence.
  -- The rows of m below are the right answer to task 1: the stories with a missing part.
  (SELECT COUNT(*) = 3 AND COUNT(DISTINCT e.story_id) = 3 FROM (
    SELECT s.id AS story_id,
-- ===== YOUR WORK 4 of 7: edit only between these lines =====
           -- TODO: build the sentence, exactly in this form:
           --   As a <role>, I want to <goal>, so that <reason>.
           s.role AS sentence
-- ===== END OF YOUR WORK 4 of 7 =====
    FROM (
      SELECT 1 AS id, 'freelancer' AS role, 'see which invoices are overdue on one screen' AS goal, 'I stop checking my bank to find out' AS reason
      UNION ALL SELECT 2, 'freelancer', 'have overdue clients reminded automatically', 'I get my Friday afternoon back'
      UNION ALL SELECT 3, 'freelancer', 'export a year of invoices as one CSV', ''
      UNION ALL SELECT 4, 'user', 'have a dashboard', NULL
      UNION ALL SELECT 5, NULL, 'send an invoice from my phone', 'I can bill before I leave the client site'
      UNION ALL SELECT 6, 'bookkeeper', '   ', 'April takes an hour, not a week'
      UNION ALL SELECT 7, 'studio owner', 'see which teammate sent each invoice', 'I can answer a client without asking around'
    ) s
    LEFT JOIN (
      SELECT 3 AS story_id, 'reason' AS missing
      UNION ALL SELECT 4, 'reason'
      UNION ALL SELECT 5, 'role'
      UNION ALL SELECT 6, 'goal'
    ) m ON m.story_id = s.id
-- ===== YOUR WORK 5 of 7: edit only between these lines =====
    -- TODO: leave out every story that has a row in m.
    WHERE 1
-- ===== END OF YOUR WORK 5 of 7 =====
  ) a LEFT JOIN (
      SELECT 1 AS story_id, 'As a freelancer, I want to see which invoices are overdue on one screen, so that I stop checking my bank to find out.' AS sentence
      UNION ALL SELECT 2, 'As a freelancer, I want to have overdue clients reminded automatically, so that I get my Friday afternoon back.'
      UNION ALL SELECT 7, 'As a studio owner, I want to see which teammate sent each invoice, so that I can answer a client without asking around.'
  ) e ON e.story_id = a.story_id AND e.sentence = a.sentence)
  AND
  -- Task 3: required sections the page lacks.
  (SELECT COUNT(*) = 2 AND COUNT(DISTINCT e.section) = 2 FROM (
    SELECT r.section AS section
    FROM (
      SELECT 'problem' AS section
      UNION ALL SELECT 'user'
      UNION ALL SELECT 'stories'
      UNION ALL SELECT 'acceptance'
      UNION ALL SELECT 'not now'
      UNION ALL SELECT 'measure'
    ) r
    LEFT JOIN (
      SELECT 'problem' AS section, 'Freelancers find out an invoice is overdue only when they check the bank. Chasing it costs about three hours each time.' AS body
      UNION ALL SELECT 'user', 'A freelancer who sends 5 to 30 invoices a month and has no bookkeeper.'
      UNION ALL SELECT 'stories', 'Stories 1, 2 and 4.'
      UNION ALL SELECT 'not now', 'Tax export. Team accounts. A phone app.'
      UNION ALL SELECT 'measure', ''
    ) p ON p.section = r.section
-- ===== YOUR WORK 6 of 7: edit only between these lines =====
    -- TODO: keep only the lacking sections. A section with an empty body is lacking.
    WHERE 1
-- ===== END OF YOUR WORK 6 of 7 =====
  ) a LEFT JOIN (
      SELECT 'acceptance' AS section
      UNION ALL SELECT 'measure'
  ) e ON e.section = a.section)
  AND
  -- Task 4: stories the page puts in scope that are not complete.
  -- The rows of m below are the right answer to task 1.
  (SELECT COUNT(*) = 1 AND COUNT(DISTINCT e.story_id) = 1 FROM (
    SELECT p.story_id AS story_id
    FROM (
      SELECT 1 AS story_id
      UNION ALL SELECT 2
      UNION ALL SELECT 4
    ) p
    LEFT JOIN (
      SELECT 3 AS story_id, 'reason' AS missing
      UNION ALL SELECT 4, 'reason'
      UNION ALL SELECT 5, 'role'
      UNION ALL SELECT 6, 'goal'
    ) m ON m.story_id = p.story_id
-- ===== YOUR WORK 7 of 7: edit only between these lines =====
    -- TODO: keep only the stories that have a row in m.
    WHERE 1
-- ===== END OF YOUR WORK 7 of 7 =====
  ) a LEFT JOIN (
      SELECT 4 AS story_id
  ) e ON e.story_id = a.story_id)
THEN 'PASS' ELSE 'FAIL' END AS result;
