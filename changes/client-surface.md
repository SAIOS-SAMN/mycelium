### The client surface speaks the 2026-11-04/05 rulings: angles as degrees on a named plane, the multi-dimensional ledger, holonomy unpriced, the Concordance signed by reconstruction
- **Network contract: wire strings changed; no codec bytes.** The codec
  and wire bytes are unchanged, and the seat's kind codes 1–6 for the
  plane symmetries are unchanged. Refusal reasons carried on the wire
  change wording:
  - a claim of factorisation naming no task held: "no such task on this
    book" becomes "no such task at this court" (`judge_factorisation`);
  - a solve on a task held with no content: "no such task on this book:
    its ground is not read" becomes "no such task at this court: its
    ground is not read";
  - `REFUSE_EQUIVOCATION`: "two receipts at one K with different chain
    hashes" becomes "two receipts at one line position i with different
    chain hashes";
  - `REFUSE_NOT_EQUIVOCATION`: "two receipts that do not share key and K,
    or do not differ" becomes "two receipts that do not share key and line
    position i, or do not differ".
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
- **SDK surface: `Modes` in the prelude.** `mycelium_sdk::prelude` adds
  `Modes` (the carrier of μ, from `mycelium_sdk::task`) beside `Mu`, and
  `Mu` is documented as retiring in v0.5.0 (MU-ECONOMY §9). No type is
  removed. The wallet is documented as a reading of the holding's
  coordinate on the one multi-dimensional ledger, never "flat"; the
  readings' entries as events of that ledger. The client example and the
  conformance solve line print the verdict's modes beside the μ paid.
  The mind's comments say "the court's open listings" and "recorded" for
  "the open book" and "booked"; the mind's trace label reads "admission:
  the open listings".
- **Holonomy is read, never priced** (the reference mind, crate-private).
  `Mind::price_hearings` is renamed `read_hearings`, `Mind::price`
  `read_loops`, `Carrier::priced` `loops_read`, `Mind::lapse_unpriced`
  `lapse_field_waiting`, the board's `unpriced`/`unpriced_crossed`
  `field_waiting`/`field_waiting_crossed`, and the types `Pricing`,
  `Priced`, `FieldWork::Price` and `Inflow::Priced` `HeardLoops`,
  `LoopsRead`, `FieldWork::ReadLoops` and `Inflow::LoopsRead`. Their docs
  state the loops' holonomy as meaning read; a price attaches only to
  traversal. The mind's trace labels read "admission: hearings read" and
  "admission: a hearing (loops read)".
- **The algebra and the completion test.** The SDK's crate description
  and the rule seam's, the house's and the seat mind's docs name the
  algebra `Cl(p, q)` with the signature as data, `Mv` its conformal case,
  never `Cl(4,1)`. The mind's count of paid solves is documented as an
  observation, never a goal. The README's kernel sections already state
  the Concordance signed clause by clause by reconstructing its fixtures.
- **Replay, grid and K.** `ReceiptsRequest.replay` is documented as an
  era's receipts read as history, each reconstructed apart, never as
  verification. The README's client loop reads `TaskEnvelope::section()`
  (the removed `rasters()` no longer appears), and "raster" stands only at
  intake in the SDK's task docs. Cognition comments say "site" where a
  sample's place was a "cell". The receipt index is the line position `i`
  in the quote act's layout doc, `YieldQuote::k`'s doc, the chronometry
  module doc and the court's head docs; `K` means closed orbits. The
  identifiers `YieldQuote::k`, `k_step`, `head_k` and
  `KStepReceipt::step_index` keep their names for a later rename.
