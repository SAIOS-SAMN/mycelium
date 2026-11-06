### The field carrier composes turnings on planes that share a unit by fixture 11b's law: `carrier::Rotor`, a motion held by its rate (78.173 §5; ROADMAP track 26 S7 stage 3, in part)
- **No network contract.** No wire byte, act, clause or ledger layout
  changes; the carrier's composition is internal to the mind.
- **`carrier::Rotor`**: a motion held by its rate `B`, an even generator
  of `Cl(p, q)` with rational components. `Rotor::of` (the rate, checked
  by `forms::turning::motion`), `Rotor::of_motion` (the rate read back by
  `forms::turning::generator`), `motion` (read when asked, never stored),
  `then` (`R₁R₂`, whole, the rate read back) and `planes` (each grade-2
  part as its key, units ascending, with its component). The law's
  refusals reach the carrier as `FieldRefusal::Turning`, and a rate's
  parts on no plane as `FieldRefusal::NotPlanes`.
- **`Motion::then`** composes an arriving turning with the one held
  turning whose plane shares a unit with it, through the law, wherever
  both motions are rational: the angle a whole number of quarter turns,
  the only rational motions `cos θ + B·sin θ` at a rational number of
  degrees. The composite's rate is read back and its one plane read; it is
  held as one turning on that plane (the plane between the two is written
  with its units ascending), or as none where the composite is the
  identity. A 0° arriving turning is the identity and is skipped.
- **Refusals:** any other angle is refused `NotExact` with the new
  `Residue::IrrationalMotion { plane, degrees }` in place of `Uncommuted`.
  `Uncommuted` stays for `Turning::then` across two planes, an arriving
  plane that shares a unit with two held planes, and a pair either of which
  carries a winding (how `K` is held beside `R` is open, 78.173 §8.3).
- **Not built, per the approved route:** no `ℚ(ζ_ℓ)` product across
  planes, and neither degree convention changes in code (the carrier's
  one-sided angle, `forms::turning::reading`'s `tan(θ/4) = b`; the factor
  of two is open in #187).
- **Claims** (`src/transducer/field/carrier/tests/rotor.rs`):
  - `rotor_composes_planes_sharing_a_unit_onto_the_plane_between`
    (11b-i: `(16, ±12, 9)/37`);
  - `rotor_on_disjoint_planes_reads_back_the_composite_rate` (11b-ii:
    `4/7`, `3/7`);
  - `rotor_refuses_motion_with_no_rate`;
  - `motion_composes_rational_turnings_sharing_a_unit`;
  - `motion_keeps_disjoint_turnings_beside_the_composite`;
  - `motion_refuses_wound_turnings_and_three_planes_as_uncommuted`;
  - `motion_refuses_irrational_motions_sharing_one_unit`
    (`tests/composition.rs`, formerly
    `motion_refuses_planes_sharing_one_unit`).
- **Docs:** 78.173 §5 records the clause `FORMS-TURNING` v1 as
  reproduced by netstratum 2026-11-06 (its own exact `Cl(3, 3)` over ℚ;
  degree sectors read in floating point only; no veto), signature pending
  its operator, xylarium owed; 78.171's open point, ROADMAP track 26 and
  MU-CLIENT §7.1 item 1.2 restated.
