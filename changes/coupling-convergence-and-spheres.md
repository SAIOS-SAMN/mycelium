### Wire `forms::coupling` sphere meets into `observe` and generalize enclosure scaling (#378)

- Added `COUPLING: [&str; 3]` (`converging`, `bound`, `diverging`) and `MindReading::coupling` in `src/observe/mind.rs` (wire-compatible `put`/`take`/`lines`).
- Wired `forms::coupling::Enclosure` and `forms::coupling::meets` into `Scenes::read` (`src/client/sdk/mind/converge.rs`) and `Mind::ground_from_percept` (`src/client/sdk/mind/perceive.rs`) to record real-time sphere convergence and divergence.
- Replaced rank-2 `uniform_scale([usize; 2], [usize; 2])` in `perceive.rs` with rank-general `uniform_enclosure_scale(&[usize], &[usize])`.
