### Client: mind proposes rewrites (Refine, AxisOpen, Translation, Reanchor) (MU-CLIENT §5.4, #146)
- **Rewrite proposal methods on `Mind`:**
  - Implemented `refine_of` carrying `PivotBy::OpenAxis`, refining a chart within an existing dimension, verified by peel.
  - Implemented `axis_open_of` for germination's Extend, proposing a new dimension of perception with the orbit's holonomy rotor.
  - Implemented `translation_of` carrying `PivotBy::Placement` across chord overlaps.
  - Implemented `reanchor_of` proposing re-anchoring when re-gauging moves spatial coordinates while preserving class invariance.
  - Added `propose_rewrite` ensuring rewrites are asserted once and not duplicated.
- **Test coverage:**
  - Added unit test cases verifying reconstruction and tape encoding for each rewrite proposal in `src/client/sdk/tests/rewrite.rs`.
