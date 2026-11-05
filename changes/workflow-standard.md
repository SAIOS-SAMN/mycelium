### Done is two cargo commands; the gate is workspace tests (workflow standard)
- Coordinator ruling: every developer is an AI agent and no person is in
  the loop; a change is done when `cargo clippy --release --workspace
  --all-targets -- -D warnings` and `cargo test --release --workspace`
  pass, and a change to the wire or the mesh also runs
  `scripts/quickstart` (AGENTS.md, *Definition of done*).
- The gate moves from `xtask/` (removed) into the workspace member
  `gate/`, read by its tests alone: the reading bound, the Euclidean
  vocabulary, the numbers gate (its N6 inventory written by
  `numbers_refuses_nothing_in_tree`) and the drift register, each a test
  over the tree. Refusals are unchanged: a seeded tree refused by the old
  gate is refused by the new tests, row for row.
- Every member takes the one lint table (`forms` joins it).
- `scripts/check` and `scripts/ship-scoped` are removed. The durable
  drill is the release-only test `tests/durable.rs` (the script and the
  example client are removed); `drill-undurable` is still its negative
  control.
- `scripts/quickstart` runs `scripts/observer once` over its mesh and
  `scripts/timeline` over the client's traced journal, and fails where
  either does. The observer's `once` now fails on a reading that does not
  answer, honours `CARGO_TARGET_DIR` and reads a mesh with no rendezvous;
  the timeline reads the trace line as written today (no wall-clock
  field), costing each reader by its spend.
- A gate test holds MU-EQUATIONS §11's forms rows to `forms::LAW_IDS`.
- A gate test holds the documents to MU-STANDARDS 7.1: every repository
  path cited in backticks in `docs/`, README, AGENTS, CONTRIBUTING,
  ROADMAP and GLOSSARY exists, or its line says it was removed. The 26
  stale citations found are rebound.
- The Unreleased section is fragments: one file per change under
  `changes/`, holding exactly its entry's markdown, assembled into the
  release's section at release time. The entries that stood under
  Unreleased are moved there unchanged. A gate test refuses a fragment
  that is not a markdown entry opened by its `### ` heading.
