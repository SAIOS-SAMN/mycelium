### The client surface speaks the 2026-11-04/05 rulings: angles as degrees on a named plane, the multi-dimensional ledger, holonomy unpriced, the Concordance signed by reconstruction
- **Network contract: no byte changes.** The codec and wire bytes are
  unchanged. The seat's kind codes 1–6 for the plane symmetries are
  unchanged.
- **SDK surface: `Symmetry`'s rotations are named by plane and degrees.**
  `Symmetry::Rotate90`, `Rotate180` and `Rotate270` are renamed
  `RotateE2E1By90`, `RotateE2E1By180` and `RotateE2E1By270`: the rotation
  by 90°, 180° and 270° in the `e₂e₁` plane of the perspective's two plane
  axes (`e₂` carried onto `e₁`). Re-exported unchanged in place by
  `mycelium_sdk::rule` and the prelude. The mirrors' docs read along a
  plane axis, never a row or a column.
