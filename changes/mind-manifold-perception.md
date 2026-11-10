### Mind Manifold Perception: Depth Perception Integration (78.179 §1, #134)
- **Engine Perception:** `transducer::family::transduce::{PairFactored, TaskPercept}`.
  - `PairFactored` now records `input_manifold: Manifold` and `output_manifold: Manifold`, lifted directly from observed samples without dimensional collapse.
  - `TaskPercept` holds `test_manifolds: Vec<Manifold>`, lifting each test observation into the manifold carrier.
  - Added helper accessors `pair_manifold(&self, pair: usize) -> Option<(&Manifold, &Manifold)>`, `test_manifold(&self, test: usize) -> Option<&Manifold>`, and `train_manifolds(&self)`.
- **Client Mind Board:**
  - Added `Mind::task_percept(&self, id: &[u8]) -> Option<&TaskPercept>` accessing the full open-dimensional manifold geometry recorded on `self.board.percepts`.
  - Rephrased pair body perceptions (`PairBodies`, `FamilyBodies`) to eliminate 2D frame compaction assumptions.
- Claims: `no_axis_is_privileged_in_perception` verifies manifold carriers across permuted axes and open ranks.
- No wire format change.
