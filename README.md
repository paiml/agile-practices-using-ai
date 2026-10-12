# Agile Practices Using AI: code examples

Runnable Rust examples for the course **Agile Practices Using AI**. Each
example takes one agile practice that an AI assistant now helps with, and
writes down the part a team should not leave to the assistant's judgment as
code with a contract.

Every example has:

- a small library and a command-line tool (`cargo run -p <example> -- <file>`);
- sample input in `crates/<example>/samples/`;
- a contract in `contracts/<example>-v1.yaml` that states what the code
  guarantees and names the test that would prove it wrong;
- a binding in `contracts/<example>/binding.yaml` that ties each contract
  equation to a Rust function.

## The examples

| Example | Practice | Course chapter |
|---|---|---|
| [`story-contract`](crates/story-contract) | User stories with Given/When/Then acceptance criteria | Backlog and planning with AI |
| [`backlog-rank`](crates/backlog-rank) | Ranking a backlog by value per effort | Backlog and planning with AI |
| [`capacity-plan`](crates/capacity-plan) | Planning sprints against capacity | Backlog and planning with AI |
| [`working-agreement`](crates/working-agreement) | A `CLAUDE.md` working agreement with a Definition of Done | Sprint execution with an AI assistant |
| [`done-gate`](crates/done-gate) | One gate that says Done, run from a git hook and CI | Definition of Done |
| [`team-skill`](crates/team-skill) | A team skill the assistant loads by name | Continuous improvement |
| [`five-whys`](crates/five-whys) | A five whys root-cause chain from a retrospective | Continuous improvement |

The CI workflow in [`.github/workflows/ci.yml`](.github/workflows/ci.yml) is
an example too: it runs the same gate on every pull request.

## Quick start

You need Rust (stable) and, for the contract checks, `pv` and `jq`:

```sh
cargo install aprender-contracts-cli --version 0.71.1 --locked   # installs pv

cargo run -p story-contract -- crates/story-contract/samples/stories.txt
cargo run -p backlog-rank -- crates/backlog-rank/samples/backlog.csv
cargo run -p capacity-plan -- crates/capacity-plan/samples/estimates.csv 13 2
cargo run -p five-whys -- crates/five-whys/samples/shallow.txt   # refused: too shallow

make gate
```

## The gate

`make gate` is this repository's Definition of Done. It passes only when all
of these pass:

1. `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test`.
2. `pv validate` on every contract.
3. `pv lint` for every example, with its binding and the strict test-binding
   check, so every falsification test a contract names is a real `#[test]`.
   An example without a contract and a binding fails the gate.
4. `pv ontology export --owl` and `pv ontology tbox` on
   [`contracts/ontology.yaml`](contracts/ontology.yaml), the signature that
   names the concepts the contracts use: stories, criteria, backlog items,
   sprints, checks and so on.

Code that does not pass the gate does not go into this repository.

## Licence

MIT. See [LICENSE](LICENSE).
