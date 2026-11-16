-- A common wrong answer: the threshold is there, but it still counts notes, not different interviewees.
-- Interview 1 raised slow-checkout twice, so slow-checkout shows 4 instead of 3.
SELECT tag, COUNT(*) AS interviewees
FROM notes
GROUP BY tag
HAVING COUNT(*) >= 3
ORDER BY interviewees DESC, tag;
