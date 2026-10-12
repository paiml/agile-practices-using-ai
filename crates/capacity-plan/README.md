# capacity-plan

Course chapter: **Backlog and planning with AI**.

Estimates go in, in backlog order. Each item goes into the first sprint with
room for it. No sprint goes over capacity, and anything that fits nowhere is
listed as overflow, never dropped.

```sh
cargo run -p capacity-plan -- crates/capacity-plan/samples/estimates.csv 13 2
```

Contract: [`contracts/capacity-plan-v1.yaml`](../../contracts/capacity-plan-v1.yaml).
Falsifiers: `falsify_no_sprint_over_capacity`, `falsify_work_is_conserved`.
