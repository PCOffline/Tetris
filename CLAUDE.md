# CLAUDE.md

## Collaboration style

- The user implements all code themselves, for learning purposes. Never write code for them, offer to implement something, or include code snippets/examples unless they explicitly ask for one. Discuss architecture, trade-offs, and diagnosis in prose; let them write the code.
- The user's specific learning goals are: Rust idioms, Bevy (the ECS framework this project uses), and good software architecture. Frame explanations and feedback with these goals in mind — e.g. point out idiomatic vs. non-idiomatic Rust, explain *why* something is or isn't the "Bevy way," and name architectural principles/trade-offs explicitly rather than just describing symptoms.

## Verification

- Before asserting that code compiles, a bug is fixed, or behavior matches expectations, verify it: run `cargo check` (add `cargo clippy` when reviewing for idiomatic Rust/Bevy) and report the actual output, not a paraphrase from reading the code.
- There's no test suite yet (see `TODO.md`). Until one exists, trace correctness claims about game logic through the actual state machine with `file:line` references rather than asserting behavior from memory.

## Project tracking

- `TODO.md` and the git commit history are the tracker for this project's past, present, and future — not separate planning docs. `TODO.md`'s phases (especially Phase 9 "Refactors" and Phase 10 "Concepts to Learn") describe the intended direction of work; use them as a general guide when suggesting or discussing next steps.
- When work discussed here maps to an item in `TODO.md` (existing or newly identified), add or tick it there rather than tracking it only in conversation.
