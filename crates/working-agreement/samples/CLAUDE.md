# Working agreement

This file is read by the AI assistant at the start of every session.

## Code Search Policy

- Never use `grep` or `glob` to search code in this repository.
- Use `pmat query` instead. It returns ranked functions, each with a quality grade, complexity, importance in the call graph and test coverage, not bare lines.
- Example: `pmat query "serialize" --limit 3`.

## Definition of Done

- `make gate` exits 0 on the branch.
- Every story's acceptance criteria have a passing test.
- The change is merged through a pull request.

## Branching

- Work happens on a short-lived feature branch named after its story.
- Nobody pushes to `main`.

## Review

- One human reviewer approves every pull request.
- The assistant may suggest changes; a human merges.
