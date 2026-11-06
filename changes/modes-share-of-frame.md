### v0.5.0: a mode's amount is its share of the frame
- **Network contract:** changed, a recut. The modes a paid solve verdict
  carries (`encode_solve_verdict`) hold new amounts in the same layout:
  each mode's amount is its share of the frame, never the mass carried.
  Persisted modes read before the recut are not migrated.
- **The rule** (78.174 §1 item 9, operator 2026-11-05): a closure's motion
  modes are read at the share of the output frame its carried mass covers
  (`m` over the frame's measure); its hue modes at the pair's G share, so
  a recolour pays G by its share (`deflation::form_deflation_read`,
  `Modes::of_motion`, `Modes::of_gauge`, whose argument is now the share).
- **Claim:** `closure_mode_amount_is_its_share_of_the_frame`, a slide
  across a 3×6 frame holding its modes at `m/18`.
- MU-ECONOMY §1 states it.
