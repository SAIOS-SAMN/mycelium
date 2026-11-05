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
- **Angles as degrees on a named plane.** The house's conformal law on a
  motion stated phase by phase reads each phase's advance as the degrees
  `360°·r/h` on its plane per bell. The conformance probes' rotor is "the
  rotor in the e₁e₂ plane at the rational point (3/5, 4/5)", and the
  conformance report's lines name it so. The mind's docs and the SDK's
  tests name the plane by its axes (`e₁e₂`), never "the sensor plane" or
  a turn without a plane.
- **Lanes as the coordinate's components.** `TPoint` is documented as
  LAW-COORDINATE v3's one-component, non-negative case (each base lane a
  vector of one exact non-negative component). The wire version's μ
  fields, the deferral's deficit and the quote's `[u64; 4]` fields are
  documented as readings per lane, never a holding. `Deferral`'s
  `Display` labels each lane (`deficit S 1, G 0, T 2, H 0`).
  `mycelium-client --init` prints `genesis line: voice <vk> <μ on S> <μ
  on G> <μ on T> <μ on H>`; the fields a genesis file reads are unchanged.
