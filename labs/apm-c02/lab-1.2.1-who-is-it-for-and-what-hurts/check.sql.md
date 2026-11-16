-- Lab 1.2.1 check: prints PASS or FAIL.
WITH
exp1(segment, tickets) AS (
  VALUES ('agency', 12), ('freelancer', 9), ('shop', 4)
),
exp2(segment, complaint, tickets, users, minutes_lost) AS (
  VALUES ('freelancer', 'chasing late payments',  7, 6, 1260),
         ('freelancer', 'tax export',             2, 2,  240),
         ('agency',     'login',                 10, 2,  200),
         ('agency',     'recurring invoices',     2, 2,  120),
         ('shop',       'tax export',             3, 3,  360),
         ('shop',       'chasing late payments',  1, 1,  180)
),
exp3(segment, complaint) AS (
  VALUES ('freelancer', 'chasing late payments')
)
SELECT CASE WHEN
      (SELECT COUNT(*) FROM answer_1) = 3
  AND NOT EXISTS (SELECT segment, tickets FROM answer_1 EXCEPT SELECT segment, tickets FROM exp1)
  AND NOT EXISTS (SELECT segment, tickets FROM exp1 EXCEPT SELECT segment, tickets FROM answer_1)
  AND (SELECT COUNT(*) FROM answer_2) = 6
  AND NOT EXISTS (SELECT segment, complaint, tickets, users, minutes_lost FROM answer_2
                  EXCEPT SELECT segment, complaint, tickets, users, minutes_lost FROM exp2)
  AND NOT EXISTS (SELECT segment, complaint, tickets, users, minutes_lost FROM exp2
                  EXCEPT SELECT segment, complaint, tickets, users, minutes_lost FROM answer_2)
  AND (SELECT COUNT(*) FROM answer_3) = 1
  AND NOT EXISTS (SELECT segment, complaint FROM answer_3 EXCEPT SELECT segment, complaint FROM exp3)
THEN 'PASS' ELSE 'FAIL' END;
