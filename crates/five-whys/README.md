# five-whys

Course chapter: **Continuous improvement**.

A retrospective's root-cause chain as data: a problem, at least five whys
with none repeated, and an action. An assistant can draft the chain from the
retro notes. This check refuses one that stops early, goes in a circle or
changes nothing.

```sh
cargo run -p five-whys -- crates/five-whys/samples/retro.txt     # ok
cargo run -p five-whys -- crates/five-whys/samples/shallow.txt   # REFUSED, exit 1
```

Contract: [`contracts/five-whys-v1.yaml`](../../contracts/five-whys-v1.yaml).
Falsifiers: `falsify_depth_four_is_refused`, `falsify_cycle_is_refused`,
`falsify_missing_action_is_refused`.
