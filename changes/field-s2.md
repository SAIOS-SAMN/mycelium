### The field, stage 2: the well's body; a well stands on the field's carrier (track 26 S7; LAW-FIELD v1 as proposed)
- **A recut, no network contract.** No wire byte, act or ledger layout
  changes. The field statement's layout changes (a chart is written as an
  invariant's exact coordinates; a well's stage follows its atlas), and
  the well's carrier changes (`percept`, `dirty`, `volume` and `potential`
  are gone; `stage` and `orbit` are new). Nothing is migrated and no
  reader of the former layout is kept (the dev-state wipe ruling).
- **A chart is an invariant's coordinates.** `carrier::Chart` holds
  `stalk::ChartAxis` values (a count or an exact ratio), with
  `Chart::of_invariant`, so a well's chart and a lifted body's chart are
  one kind of object.
- **The well's body** (`carrier::WellBody`, `holding.rs`): a held
  attractor at a chart, the orbit it binds (none: flat), its membrane
  the throat of the last of its frame planes, its own enclosure; its
  holonomy and precession per revolution read from the orbit, a flat
  well's refused `Unbound`; its inflation the gauge of the frame state
  `H·n_Z + R·n_Q` on the membrane's pole basis, stood inside the membrane
  and read back with its place (`Inflation`). Under the ruling of
  2026-11-06, cohesion is the viscosity: the held share `A/(A + Q)` per
  plane, here `H/(H + R)` (`Frame::share`); the open share is the
  fluidity, never the viscosity. The inflation's docs name the shares so.
- **The well's stage is a coordinate** (`WellStage`: `WellLayer`
  perception or thought, then `ExpansionRing` expanding or rested;
  ordered). It replaces `percept: bool` and `dirty: bool`.
- **`Well` stands on the carrier.** `Well::body`, `centre` (the chart its
  remainder stands on), `planes` (the section's rank plus the lift's
  plane), `inflation` (`H` the charts its cover closed, `R` its sections'
  charts still open), `holonomy`, `precession`, and `orbit`. `volume` and
  `potential(now)` are readings per lane, never stored: the volume the
  lanes of its remainder and each pair's remainder, the potential the
  lanes of everything it holds open with T the receipts since its charts
  moved, no reading where the clock went backwards.
- **Every caller reads.** Hold, merge, the contemplation's `hold`, the
  expansion ring (`drive`), the doors (`shed`), the docket (`plan`), the
  cross-scene meet, `link_new`, the observer's layer and the quote's
  routing volume read the stage and the readings; a merged thought's
  volume is the reading of what it holds, no longer the lane-wise
  largest of what merged.
- **A mind's well stated and reconstructed by a court**
  (`Well::put_statement`, `WellBody::put_statement`, `reconstruct_well`,
  `admit_well`): the statement writes the well's own atlas and its stage,
  no reading; the court rebuilds it, derives the curvature (the holonomy
  per revolution) itself, and admits a claim on a match, refusing
  `Differs` with the pair. Not on the wire (S9).
- **Orbits are geodesics** (operator ruling 2026-11-06; 78.171 amendment
  (f)): every path in the field is a geodesic of its gravitational
  curvature, the wells its attractors, and a bound geodesic is an orbit
  read per revolution (`Orbit::revolution`, `Atlas::revolution`; "lap"
  retired). A closed geodesic with zero precession in a curved field is
  resonant, lawful (`Precession::Resonant`, beside
  `Precession::Advances`). A path through a flat region binds no orbit and
  is refused `Unbound` (by `Orbit::of`); the `Loop` refusal is retired and
  `NotLap` is `NotRevolution`. An attractor binds an orbit or none
  (`Attractor.orbit: Option<Orbit>`); the field's curvature around it is
  its holonomy per revolution, `Unbound` where it binds none. LAW-FIELD
  fixture 2 is restated: a flat region binds no orbit.
- **Claims:** `well_stands_on_its_attractor`,
  `well_membrane_is_its_enclosure`,
  `well_volume_and_potential_are_readings_per_lane`,
  `well_stage_is_a_coordinate_not_a_flag`,
  `mind_well_stated_and_reconstructed_by_a_court`,
  `well_body_stated_and_admitted_by_its_curvature`,
  `flat_region_binds_no_orbit_refused_unbound`,
  `closed_geodesic_with_zero_precession_is_resonant`,
  `reconstruct_well_refuses_an_atlas_of_two_attractors`,
  `chart_of_ratio_invariant_round_trips`; a `compile_fail` doc test on
  `Well.percept`.
- **Left for S9a:** the contemplation's orbits (`cech::orbit::Field`)
  attached to wells as their local orbits; until then every well is flat.
