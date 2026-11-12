### v0.5.0: a mode's amount is its share of the enclosure
- **Network contract:** changed, a recut. The modes a paid solve verdict
  carries (`encode_solve_verdict`) hold new amounts in the same layout:
  each mode's amount is its share of the enclosure, never the mass carried.
  Persisted modes read before the recut are not migrated.
- **The rule** (78.174 §1 item 9, operator 2026-11-05; MU-ENGINEERING §9.3):
  a closure's motion modes are read at the share of the output enclosure its
  carried mass covers (`m` over the enclosure's open-dimensional measure at
  rank `n`); its hue modes at the pair's G share, so a recolour pays G by its
  share (`deflation::form_deflation_read`, `Modes::of_motion`, `Modes::of_gauge`,
  whose argument is now the share).
- **Claim:** `closure_mode_amount_is_its_share_of_the_enclosure`, an enclosure
  at rank 2 (periods 3×6, measure 18) holding its modes at `m/18`.
- MU-ECONOMY §1 states it.
