### Cohesion is a named reading per plane: the held share, the viscosity, beside the fluidity (MU-ANASTOME §9.4; #149)
- **No network contract changes.** No act, wire layout or served fixture
  changes; `FORMS-COUPLING` stays at version 1 with its 27 fixtures. The
  held share is a reading of v1's own inversion `ν(1/t) = 1 − ν(t)`, so
  it stands in code and tests only, outside the register.
- **`forms::coupling`.** `held_share(t) = 1/(1 + t)`, refused `NotRatio`
  for `t < 0`, with `held_share(t) = 1 − open_share(t)` and, for `t > 0`,
  `held_share(t) = open_share(1/t)`; `Parcel::held_share`; and
  `Parcels::cohesion` and `Parcels::fluidity`, each a
  `forms::coordinate::Vector` keyed by the body's turning planes, never
  summed. A per-plane vector the coordinate form refuses other than by a
  repeated plane is carried as `CouplingRefusal::Form`.
- **The mind's field.** `field::carrier::Cohesion`, read from frame
  states keyed by their plane's two axes (a turning plane, or one frame
  plane's own pair, `frame_plane(k)`), with its fluidity per plane and no
  total; `WellBody::cohesion(hold, reach)` and `Well::cohesion()`, read
  from the well's inflation (field stage 2) on its membrane's frame plane,
  every call, never a field of the well.
- **Claims.** `passage_exchanges_cohesion_and_fluidity` (the pole swap
  read through `Frame::share`), `cohesion_and_fluidity_half_at_the_throat`,
  `cohesion_read_per_plane_never_summed` and
  `held_share_is_open_share_of_inverse` in `forms`; the well's
  `well_cohesion_is_the_held_share_of_its_inflation`,
  `well_body_cohesion_half_at_the_throat_and_exchanged_by_the_passage` and
  `well_cohesion_read_per_plane_never_summed`; two `compile_fail` doctests
  (no total, no stored cohesion).
- **Owed (#149).** That a whole too cohesive sticks at the throat and one
  too fluid tears: neither LAW-PASSAGE v3 nor LAW-TRAVERSAL v1 gives an
  exact reading of either yet, and no threshold stands in their place.
  Frames per turning plane of a held whole: at field stage 2 a well holds
  one frame state, so its reading holds one plane.
