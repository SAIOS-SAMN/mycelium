### `forms::coupling` holds LAW-COUPLING v1: the dipole parcel per plane, the coupling on a turning plane and the light ledger, exact over ℚ (proposal 78.175)
- **Network contract: a new law form and a new clause.** A new shell,
  `FORMS-COUPLING` v1, joins `forms::LAW_IDS` (and the SDK's
  `FORM_LAW_IDS`), ruled by netstratum's and mycelium's operators
  2026-11-06 through MU-RELEASE §3a, extending LAW-FIELD (netstratum
  docs/121 §9–§13, docs/122 §2). The clause register serves it as a third
  clause, `FORMS-COUPLING` v1 at kernel standing (signable and vetoable),
  on `ACT_LAWS` mode 7, and `CLAUSES` (and so the conformance statement)
  names it. New writing: fixture tag 12 (`Fixture::Coupling`, an
  operation byte 0–5 then its exact values) and object tag 9
  (`Object::Coupling`, a reading byte 0–6). A kernel that signed the
  register must reconstruct the new clause's 25 fixtures or veto it; no
  act or ledger layout changes. The SDK reconstructs it through
  `forms::coupling` (`reconstruct_fixture`).
- **Shared degrees are round** (ruled 2026-11-06): rotation angles on
  Clifford turning planes; an L1 projection is a named reading
  (`hemisphere_l1`, `Parcel::arc_l1`).
- **F1–F2, per plane:** `Parcel` (`Parcel::kind`: `3Z − U` held, `U`
  open) and its `Dipole`, the pole `σ = (z − q)/(z + q)` and the open
  share `o = ν(q/z)`; `open_share` (`ν(t) = t/(1 + t)`),
  `Parcel::inverted` (`ν(1/t) = 1 − ν(t)`); `Parcels`, a body's frames on
  each turning plane, founded by the even split (`Parcels::founded`), a
  bond or peel changing one plane (`Parcels::bond`).
- **F3:** `Alignment` at a round separation on a named turning plane,
  `h = hemisphere_l1(Δ) = 1 − Δ/90°`; `Neighbour`, `nearness`, `field`
  (`F = Σ w·σ·o·n·h / Σ w`), `pull` (`x′ = x ⊕ y`,
  `y = −σ·F·o·(x̄ ⊖ x)`, by `forms::coordinate::flow`), `relative` (the
  flow law's inverse), `Enclosure` and `meets`.
- **F4:** `Face`, `unlocked`, `take`, `ledger`; a ledger that does not
  balance is refused `Unbalanced` with its residue.
- **Fixtures:** 78.175 §5, claimed in `forms/src/tests/coupling.rs`.
