-- Lab 4.1.2 seed.
CREATE TABLE requests (
  id       INTEGER PRIMARY KEY,
  endpoint TEXT NOT NULL,
  ms       INTEGER NOT NULL   -- response time of one request
);

-- 100 requests to /checkout: steady, between 190 and 249 ms.
WITH RECURSIVE n (i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < 100)
INSERT INTO requests (endpoint, ms)
SELECT '/checkout', 190 + (i * 13) % 60 FROM n;

-- 100 requests to /search: 97 fast ones (80 to 120 ms) and 3 that hang for seconds.
WITH RECURSIVE n (i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < 100)
INSERT INTO requests (endpoint, ms)
SELECT '/search',
       CASE WHEN i <= 97 THEN 80 + (i * 37) % 41
            ELSE 2400 + (i - 97) * 300 END
FROM n;
