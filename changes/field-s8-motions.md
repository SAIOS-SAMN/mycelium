### The field, stage 5: the field's motions as one exact group (track 26 S8; LAW-FIELD v1 §2, a motion; MU-SENSING §3.7)
- **No network contract.** No wire byte, act, task, word or chart address
  changes. The motions are new and nothing persisted reads them.
- **The generators** (`carrier::Generator`, `carrier/motions.rs`):
  - rotate, turnings on disjoint named planes (`Turn`);
  - flip, the reflection in one frame unit (`Flip`). The flip of `e₋ₖ` is
    plane `k`'s pole swap and the inversion in its membrane
    (`Generator::swap`, `Membrane::invert`).
  - deflate and re-inflate, the boost on a frame plane by an exact gauge,
    `(H, R) ↦ (λH, R/λ)` (`Boost`);
  - stretch and thin, the re-period of one phase, a site keeping its index
    (`RePeriod`). No versor carries it.
- **One group** (`carrier::FieldMotion`): a motion is the word of its
  generators. Words compose by concatenation, reduced only where the
  reduction is exact: turnings on one plane or on disjoint planes compose,
  a flip twice cancels, boosts on one plane multiply, and re-periods of
  one phase chain. The inverse is the word reversed with each generator
  undone. A motion acts exactly:
  - on turnings, motions and orbits, by conjugation (`carry_turning`,
    `carry_motion`, `carry_orbit`): a plane's units are carried by quarter
    turns, flips and boosts, and the angle is negated where the image plane
    is reversed;
  - on frame states (`carry_state`);
  - on points (`carry_point`): a phase is carried as a vector on its plane
    and moves to the axis whose plane holds its image.

  What the carrier cannot hold is refused `NotExact` with the new residue
  `Residue::Unnamed { plane }`. That covers a turning by other than a
  quarter turn on a plane sharing a unit, and a boost on a plane holding
  one of the carried units.
- **Claims:**
  - `field_stays_smooth_under_every_isometry`: the curvature carried is
    the curvature of the carried orbit, its order and winding kept, and
    the inverse carries it back;
  - `stretch_is_a_re_period_not_a_versor`;
  - `inversion_in_the_membrane_needs_no_infinity`;
  - `field_motion_and_its_inverse_compose_to_the_identity`.
- **LAW-FIELD fixture 2b**, added at netstratum's request on reading
  amendment (f): flat and resonant are separated by the kept winding.
  `Precession::Resonant` now holds the holonomy whole. Three 120° steps
  are each no identity and compose to 360° with `K = 1`, read resonant; a
  flat path's steps are each the identity, compose to 0° with `K = 0`, and
  it is refused `Unbound` (claim
  `flat_and_resonant_separated_by_kept_winding`). Precession is resonant
  where the holonomy's monodromy is flat, whatever its winding. The
  fixture is owed its reading by netstratum and xylarium.
- **Open:** composing turnings on planes that share a unit at other than a
  quarter turn, which waits on netstratum's composition law (78.173
  fixture 11b; #134). The motions have no production caller yet: stage 4
  lifts a family's pairs as transitions through them.
