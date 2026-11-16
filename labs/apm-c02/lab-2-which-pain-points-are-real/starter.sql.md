-- TODO: a theme is real only if at least 3 DIFFERENT interviewees raised it.
-- Return tag and interviewees (distinct interviews), most-raised first, ties by tag.
SELECT tag, COUNT(*) AS interviewees
FROM notes
GROUP BY tag
ORDER BY interviewees DESC, tag;
