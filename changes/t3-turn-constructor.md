### A turn has one constructor, `Turn::of(r, h)`: MU-ANTI-FLATTENING 12's guard registered (T3, #26)
- **Network contract:** none. No wire byte, act, ledger layout, task,
  word or chart address changes.
- `transduce::Turn`'s index and period are private; `Turn::new` is
  `Turn::of(r, h)`, and a phase reversed is `Turn::reversed`. A turn is
  read back on a period only by `Turn::lands`. The `compile_fail` claim
  `turn_has_one_constructor` (a turn written by its fields; a field
  read) enters the drift register (`xtask/lint/drift.rs`).
- **Library surface change, inside v0.4.0's break:** a caller writes
  `Turn::of(r, h)` where it wrote `Turn::new(index, period)`.
