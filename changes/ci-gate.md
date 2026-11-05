### CI runs the definition of done on every pull request; corpus tests refuse without a corpus
- **Network contract:** none. No wire byte, act, ledger layout, task,
  word or chart address changes.
- `.github/workflows/definition-of-done.yml` (workflow `definition of done`,
  job `clippy and tests`, named by function) runs on every
  pull request and every push to main: it fetches the ARC-AGI-2 corpus
  into `MYCELIUM_CORPUS`, then runs
  `cargo clippy --release --workspace --all-targets -- -D warnings` and
  `cargo test --release --workspace`, four build jobs, 60 minutes bound,
  `~/.cargo` and `target/` cached on `Cargo.lock`.
- The four corpus tests that passed having read nothing
  (`sources/arc-agi/tests/retract.rs`'s three, `ground_census.rs`) now
  refuse where the corpus does not read, naming `MYCELIUM_CORPUS`.
- The pull request template is what changed, why and the network
  contract; its self-ticked checklist is gone, as CI runs the checks.
  AGENTS.md, CONTRIBUTING.md and ROADMAP.md say so.
