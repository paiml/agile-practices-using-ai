# backlog-rank

Course chapter: **Backlog and planning with AI**.

Let the assistant propose value, effort and risk. Keep the ranking itself a
pure function: value per effort first, then risk, then id. The same backlog
always gives the same order, and no item is lost or duplicated.

```sh
cargo run -p backlog-rank -- crates/backlog-rank/samples/backlog.csv
```

Contract: [`contracts/backlog-rank-v1.yaml`](../../contracts/backlog-rank-v1.yaml).
Falsifiers: `falsify_rank_is_a_permutation`, `falsify_rank_is_order_independent`,
`falsify_zero_effort_is_refused`.
