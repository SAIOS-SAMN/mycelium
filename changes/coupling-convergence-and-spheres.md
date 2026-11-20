### Wire `forms::coupling` sphere meets into `observe` and generalize enclosure scaling (#378)

- Added `COUPLING: [&str; 3]` (`converging`, `bound`, `diverging`) and `MindReading::coupling` in `src/observe/mind.rs` (wire-compatible `put`/`take`/`lines`).
- Wired `forms::coupling::Enclosure` and `forms::coupling::meets` into `Scenes::read` (`src/client/sdk/mind/converge.rs`) and `Mind::ground_from_percept` (`src/client/sdk/mind/perceive.rs`) to record real-time sphere convergence and divergence.
- Replaced rank-2 `uniform_scale([usize; 2], [usize; 2])` in `perceive.rs` with rank-general `uniform_enclosure_scale(&[usize], &[usize])`.
- Retired `perm_to_tile_turn` and `RuleElement::Tile` lowering in `Mind::ground_from_percept` (`src/client/sdk/mind/perceive.rs`), replacing them with arbitrary-rank Cartan–Dieudonné `perm_to_versor` factorisation in `Cl(n+1, 1)(ℚ)` lowered through conformal `RuleElement::Carriage` (fusing versor and recolour gauge).
- Added enclosure-centred `sample_motion` in `src/transducer/family/statement.rs` so origin-fixing conformal rotors and reflections read `About::Sample` conjugate automatically about `extents / 2` (`T(c) · R · T(-c)`), supporting varying and non-square rectangular enclosures.
