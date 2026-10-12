# working-agreement

Course chapter: **Sprint execution with an AI assistant**.

`CLAUDE.md` is the assistant's memory of how the team works. This lint fails
unless it has non-empty `## Definition of Done`, `## Branching` and
`## Review` sections. See [`samples/CLAUDE.md`](samples/CLAUDE.md).

```sh
cargo run -p working-agreement -- crates/working-agreement/samples/CLAUDE.md
```

Contract: [`contracts/working-agreement-v1.yaml`](../../contracts/working-agreement-v1.yaml).
Falsifiers: `falsify_missing_definition_of_done_fails`, `falsify_empty_section_fails`.
