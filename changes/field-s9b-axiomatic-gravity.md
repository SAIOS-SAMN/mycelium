### Field: axiomatic gravity shears the orthogonal orbit class (track 26 S9b, #134)
- LAW-FIELD v1 §3.4, MU-SENSING §3.7, MU-CLIENT §6a: axiomatic gravity is produced by the local orbit about its axiom, shearing the orthogonal orbit class.
- Two orbit classes are orthogonal when their planes satisfy `B₁ · B₂ = 0` and `B₁ ∧ B₂ ≠ 0` (`Meeting::Disjoint`).
- The shear is 78.46's commuting part, the drag, which changes rate along the orbit and leaves its plane intact; sharing planes take the anticommuting lift (plane tilt) with zero drag.
- Implemented `carrier::gravity` with `plane_split`, `axiomatic_shear`, `orbit_rate_shear`, `primary_plane`, and `Atlas::axiomatic_shear`.
- Verified claims `only_disjoint_plane_orbits_change_rate_under_axiomatic_gravity` and `axiomatic_gravity_shears_the_orthogonal_class`.
