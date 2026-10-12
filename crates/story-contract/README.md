# story-contract

Course chapter: **Backlog and planning with AI**.

An assistant can turn rough notes into user stories in seconds. This example
is the check that does not trust the draft: a story is accepted only when it
has at least one acceptance criterion, and every criterion has a Given, a
When and a Then.

```sh
cargo run -p story-contract -- crates/story-contract/samples/stories.txt     # ok
cargo run -p story-contract -- crates/story-contract/samples/bad-story.txt   # REFUSED, exit 1
```

Contract: [`contracts/story-contract-v1.yaml`](../../contracts/story-contract-v1.yaml).
Falsifiers: `falsify_story_without_criteria_is_refused`, `falsify_blank_part_is_refused`.
