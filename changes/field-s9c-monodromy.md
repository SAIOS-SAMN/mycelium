### Field: vortex candidate monodromy and stated well reconstructed by a court (track 26 S9c, #134)
- LAW-FIELD v1 §3.2, MU-SENSING §3.8, MU-CLIENT §7.1: a vortex candidate's monodromy is evaluated over the atlas's charts as `Monodromy` (flat, torsion of order $\ell$, or rupture).
- Implemented `Atlas::candidate_monodromy`, evaluating the geodesic revolution through charts; resonant closed geodesics read `Monodromy::Flat` (order 1), rational advances read `Monodromy::Torsion { order }`, and paths through flat regions are refused `Unbound`.
- Implemented `Orbit::monodromy`, `Attractor::monodromy`, `WellBody::monodromy`, `Well::monodromy`, and `Curvature::{monodromy, precession}`.
- Implemented exact motion inversion `Motion::inverse` over pairwise disjoint turning planes.
- Reconstructed and admitted stated wells in court, deriving holonomy, precession, and monodromy afresh without stored readings.
- Verified claims `candidate_monodromy_reads_flat_torsion_and_unbound_between_charts`, `motion_and_its_inverse_compose_to_the_identity`, and `mind_well_stated_and_reconstructed_by_a_court`.
