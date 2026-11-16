# Lab: User Stories in One Sentence (Course 2, Module 3, lesson 3.1.1)

**Objective (O3):** Write user stories, ground the AI with documentation and standing instructions, and write a specification and a one-page requirement. This lab is aimed at M3a: user stories and the one-page requirement.

A user story is one sentence with three parts: who wants it, what they want, and why. "As a freelancer, I want to see which invoices are overdue on one screen, so that I stop checking my bank to find out." Take any part away and the team has to guess. The part that goes missing most often is the reason, and the reason is what lets a developer choose between two ways to build it.

A one-page requirement puts a few stories in scope and says what is out. It has fixed sections: the problem, the user, the stories, the acceptance criteria, the not-now list and the measure of success.

In this lab the stories are rows with three columns: `role`, `goal` and `reason`. The page is rows too, one per section. You will find the stories with a part missing, write the complete ones as sentences, find the sections the page lacks, and find the stories the page relies on that are not finished.

Look at how a part goes missing. Sometimes the cell is NULL. Sometimes someone typed nothing, or a few spaces. All three are missing, and a check that looks only for NULL lets the other two through.

The block holds four lists of rows, typed in: the stories, the required sections, the sections of the draft page and the stories the page puts in scope. It gives them short names such as `s`, `r` and `p`, and reuses the names in each task.

## Tasks

1. **Task 1: missing parts.** One row for each missing part of each story: the `story_id` and the word `role`, `goal` or `reason`. NULL, an empty string and a string of spaces all count as missing.
2. **Task 2: the sentence.** For each story with no missing part, build the sentence in exactly this form: `As a <role>, I want to <goal>, so that <reason>.`
3. **Task 3: sections the page lacks.** List each required section that is not on the page or is on the page with an empty body.
4. **Task 4: unfinished stories in scope.** List each story the page puts in scope that is among the Task 1 rows.
5. **Not checked by the query:** story 4 says "As a user, I want to have a dashboard". Rewrite it on paper; the block's rows stay as they are. Name the real role (lab 1 gives you one), say what the person wants to do, not what screen they get, and give the reason. Then say what the two sections the page lacks should contain. Work out which rows would then leave the results of Tasks 1, 3 and 4.

## What this lab cannot teach

SQL can check the form of a story. It cannot check its content. This lab does the first and leaves the second to the learner, a reviewer or a later lesson.

| Part of objective M3a | What the lab does | What it does not do |
|---|---|---|
| A story has a role, a goal and a reason | Checked (Tasks 1 and 2) | |
| The role is a real user, not "user" | Not checked. Story 4 has the role "user", and the role test accepts it (story 4 is flagged only for its missing reason) | Tell whether a role is specific enough |
| The goal is something the person wants to do, not a feature | Not checked. "have a dashboard" passes | Tell a goal from a solution |
| The reason is true | Not checked | Tie a reason to an interview note. Lab 2 has the notes; nothing links them to stories |
| Write a story | The not-checked step only, and nothing reads the result | Judge any sentence the learner writes |
| The page has every section | Checked (Task 3) | |
| The page relies only on finished stories | Checked (Task 4) | |
| Write the one-page requirement | The not-checked step only, and nothing reads the result | Judge whether the problem is stated clearly, whether the not-now list is the right one, or whether it fits one page (no length check) |

So this lab teaches you to check that a user story and a one-page requirement are complete. Writing them well is practice you do on your own, because nothing in the block grades it.

## Do it in the course page
The reading holds this lab as one SQL block: the text of `interactive.sql.md`. Edit only between the YOUR WORK markers, press Run (it runs one SELECT statement) and read the word it prints: `FAIL` until your edit is right, then `PASS`. Reset puts the starter text back.

## Check yourself
The block prints one word, `PASS` or `FAIL`. The other files in this directory are what the repository's automated check runs.

## Files
`README.md` (this file), `interactive.sql.md` (the text of the in-page block, to edit), `interactive-solution.sql.md` (the same block with the answer filled in), `seed.sql.md` (tables and rows), `starter.sql.md` (fails on purpose), `solution.sql.md`, `check.sql.md`, `wrong.sql.md` (planted wrong answer: the check must say FAIL), `check.sh`, `RUNS.md` (measured outputs).

## Contract (LAB-STD-001)
- L5 the starter exits non-zero and prints `FAIL`; the solution exits 0 and prints `PASS`, against the same seed and check (`./check.sh starter.sql.md`, `./check.sh solution.sql.md`).
