SELECT tag, COUNT(DISTINCT interview_id) AS interviewees
FROM notes
GROUP BY tag
HAVING COUNT(DISTINCT interview_id) >= 3
ORDER BY interviewees DESC, tag;
