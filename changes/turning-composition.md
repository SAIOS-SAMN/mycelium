### `forms::turning` holds the composition of turnings, LAW-COORDINATE fixture 11b: the Cayley form on the generator, exact in the even subalgebra of `Cl(p, q)` (proposal 78.173 §5)
- **Network contract: a new law form, no new clause.** A new shell,
  `FORMS-TURNING` v1, joins `forms::LAW_IDS` (and the SDK's
  `FORM_LAW_IDS`). It states the composition law netstratum read and
  stated on 2026-11-06 for fixture 11b (netstratum docs/117 §11.2–§11.4).
  The clause register is unchanged: `FORMS-COORDINATE` stays v4 with the
  fixtures it serves, and 11b is not served (the register's fixture form
  has no operation for a composition, and serving it is a new fixture tag
  on `ACT_LAWS` mode 7, left to the clause's drafting). No wire byte, act
  or ledger layout changes.
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
  - `inverses_that_do_not_exist_are_refused_by_name`;
  - `degrees_read_as_certified_sectors`.
- **Docs:** 78.173 §5 records 11b as read and stated by netstratum
  (xylarium owed); MU-EQUATIONS §11 tables `FORMS-TURNING` v1.
- **Open:** the generator read back from a composite in six or more frame
  generators may carry a grade-6 part (its reverse is still its
  negation); the law names `B` a bivector, which holds in up to five. The
  field carrier's use of the law and the proposed state change for H are
  separate items.
