### `forms::turning` holds the composition of turnings, LAW-COORDINATE fixture 11b: the Cayley form on the generator, exact in the even subalgebra of `Cl(p, q)` (proposal 78.173 §5)
- **Network contract: a new law form and a new clause.** A new shell,
  `FORMS-TURNING` v1, joins `forms::LAW_IDS` (and the SDK's
  `FORM_LAW_IDS`). It states the composition law netstratum read and
  stated on 2026-11-06 for fixture 11b (netstratum docs/117 §11.2–§11.4).
  The clause register serves it as a fourth clause, `FORMS-TURNING` v1 at
  kernel standing (signable and vetoable), on `ACT_LAWS` mode 7, and
  `CLAUSES` (and so the conformance statement) names it. New writing:
  fixture tag 13 (`Fixture::Turning`, an operation byte 0–3, a motion, a
  generator, two motions composed or two rates composed, then its values
  of `Cl(p, q)`); each object is a value, the existing object tag 0
  (`Object::Multivector`). A kernel that signed the register must
  reconstruct the new clause's 12 fixtures (11b i–iii) or veto it.
  `FORMS-COORDINATE` stays v4 with the fixtures it serves; no act or
  ledger layout changes. The SDK reconstructs it through `forms::turning`
  (`reconstruct_fixture`) and re-exports `TurningFixture`.
- **The law** (`forms::turning`): `motion` (`R = (1 + B)(1 − B)⁻¹`),
  `generator` (`B = (R − 1)(R + 1)⁻¹`), `compose` (`R₁R₂`, whole, never
  plane by plane) and `compose_rates`, over `algebra::Clifford`, the
  signature read from the data. The inverse is the Cayley form's alone
  (78.32(1)): it solves `u·y = 1` exactly in the subalgebra `u`'s blades
  generate, in every signature. Refusals by name (`TurningRefusal`):
  `NotGenerator`, `NotEven`, `NotUnit`, `NoMotion` (`1 − B` has no
  inverse), `NoGenerator` (`R + 1` has none, `R = −1` among them),
  `Mismatched`.
- **One plane and the reading** (`forms::turning::reading`): `Regime` of a
  plane read from its square; `on_plane`; `composed`, the closed forms
  `(b₁ + b₂)/(1 − b₁b₂)` elliptic and `(b₁ + b₂)/(1 + b₁b₂)` hyperbolic;
  `degrees`, `tan(θ/4) = b` as a certified sexagesimal sector, display
  only, from exact bounds on Euler's series.
- **Claims** (`forms/src/tests/turning.rs`), fixture 11b with
  `cayley(back(R)) = R` and `R·R̃ = 1` on every motion:
  - `planes_sharing_a_unit_gain_the_turning_between_them_and_no_volume`
    (11b-i);
  - `disjoint_planes_commute_and_keep_the_volume` (11b-ii, the generator
    reading `4/7`, `3/7`, never `½`, `⅓`);
  - `one_plane_composes_by_its_regime` (11b-iii: `1` elliptic, `5/7`
    hyperbolic);
  - `closed_forms_on_one_plane_agree_with_the_general_law`;
  - `turning_inverse_refuses_nonexistent`;
  - `degrees_read_as_certified_sectors`;
  - `register_reaches_stated_turning_values`
    (`src/physics/laws/clauses/tests/turning.rs`: the served clause at
    kernel standing and its twelve values). The register's own claims
    cover the new clause with the others: `put_clause_round_trips` and
    `put_object_round_trips` (fixture tag 13 reads back), and the SDK's
    `reconstruct_fixture_agrees_with_register` and
    `sign_concordance_admits_served_register` (a kernel signs the clause
    by reconstruction, or vetoes it).
- **Docs:** 78.173 §5 records 11b as read and stated by netstratum
  (xylarium owed); MU-EQUATIONS §11 tables `FORMS-TURNING` v1.
- **Open:** the generator read back from a composite in six or more frame
  generators may carry a grade-6 part (its reverse is still its
  negation); the law names `B` a bivector, which holds in up to five. The
  field carrier's use of the law and the proposed state change for H are
  separate items.
