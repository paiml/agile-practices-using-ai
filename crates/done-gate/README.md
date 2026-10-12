# done-gate

Course chapter: **Definition of Done**.

One binary runs every check in a list and exits 0 only if there is at least
one check and all of them pass. A red check is never reported green, and an
empty list is not Done. Checks run without a shell.

```sh
cargo run -p done-gate -- crates/done-gate/samples/checks.txt
```

[`samples/pre-push`](samples/pre-push) is a git hook that runs the gate
before every push: copy it to `.git/hooks/pre-push`.

Contract: [`contracts/done-gate-v1.yaml`](../../contracts/done-gate-v1.yaml).
Falsifiers: `falsify_one_red_check_is_not_done`, `falsify_empty_gate_is_not_done`,
`falsify_missing_program_is_red`.
