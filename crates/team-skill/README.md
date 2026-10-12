# team-skill

Course chapter: **Continuous improvement**.

A skill is a procedure the assistant loads by name. This example has one,
[`samples/skills/sprint-review/SKILL.md`](samples/skills/sprint-review/SKILL.md),
and a checker that fails a skill whose frontmatter has no `name` or
`description`.

```sh
cargo run -p team-skill -- crates/team-skill/samples/skills/sprint-review/SKILL.md
```

Contract: [`contracts/team-skill-v1.yaml`](../../contracts/team-skill-v1.yaml).
Falsifiers: `falsify_missing_description_fails`, `falsify_unclosed_frontmatter_fails`.
