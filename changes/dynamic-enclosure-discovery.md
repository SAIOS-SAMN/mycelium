### Dynamic phase-period enclosure discovery from unranked streams (#32)

- Added `transducer::family::transduce::discover_enclosure` and `Discovered` (`src/transducer/family/transduce/discover.rs`) to infer an unranked stream's phase-circle periods `L = p₀ · … · pₙ₋₁` (1D, 2D, 3D, …) strictly from topological closure (`β₀` connected components) and exact rational interior coherence (`M / E` via `u128` cross-multiplication), under `Budget`, with zero floating point and zero `HashMap`s.
- Added `src/transducer/family/transduce/tests/discover.rs` proving 1D retention, 2D square/rectangle folding, and 3D cube folding from raw unranked byte streams.
