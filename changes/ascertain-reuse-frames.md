### Reuse perceived frames across `ascertain_on` folds (#349)

- Added `ascertain_with_frames` in `src/client/sdk/mind/family.rs` so leave-one-out perspective checks reuse `extract_on`'s already-perceived `Frames` (or perceive the family once instead of `N` times) and honour `physics::halted()` between folds.
- Updated `perceive()` in `src/client/sdk/mind/perceive.rs` to pass `t.frames` into `ascertain_with_frames`.
