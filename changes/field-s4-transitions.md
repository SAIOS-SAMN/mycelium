### A family's pairs lifted as transitions (track 26 S7 stage 4 in part; #134)
- **No recut and no network contract.** The lift writes the same degrees,
  charts and addresses as before. Only the reading of a phase as a vector
  inside `FieldMotion::carry_point` moves, and nothing persisted or on the
  wire reads it.
- **The lift's convention** (78.171 §8, agreed between the coordinators'
  readings, 2026-11-06, netstratum docs/117 §11.6; operators to confirm):
  every axis keeps the one shared lift unit `e₊₍ₙ₊₁₎`, and its phase is
  measured from it. A phase `θ` is `cos θ·e₊₍ₙ₊₁₎ − sin θ·e₊ᵢ`.
  `carry_point` reads phases so, where before it measured them from `e₊ᵢ`.
- **`carrier::motion_of`:** a factorisation's motion (a signed permutation
  and the re-period of each moved phase) lifted whole as a motion of the
  field. A reversal of axis `i` is the reflection in `e₊ᵢ` alone, then one
  whole turning on its plane: the identity as a motion, carrying the
  winding the transduction's `p − s` writes. A transposition `(i j)` is
  the 90° turning on `e₊ᵢe₊ⱼ`, then the flip of `e₊ᵢ`. A re-period is
  `RePeriod`. No signed permutation is refused, and none is lifted in
  part.
- **`carrier::transitions_of`, `PairTransition`:** a pair lifted as
  transitions from each input body's chart to the chart of the output
  body it is carried onto. A body that is split or merged keeps its
  refusal.
- **`FieldRefusal::Transduction`:** a motion the transduction refuses to
  speak, as it refused it.
- **Claims:** `every_signed_permutation_lifts_whole_as_a_motion_of_the_field`,
  `reversal_is_the_reflection_in_its_own_unit_alone`,
  `pair_lifted_as_transitions_between_its_bodies_charts`.
- **Owed:** the nesting as a fibration of levels (stage 4's second half).
