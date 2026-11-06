### The documents read composed turns with their winding kept (78.171 amendment (c), 78.173 v4)
- `Turn::plus` kept the winding since #129. The sentences that still
  said "composed by addition mod 1" or held a turn "in ℚ/ℤ" now say
  that a composed turn keeps its winding (ℤ, `K = ⌊|θ|/360⌋`) beside its
  phase, and that only a place and a residue read as a phase (ℚ/ℤ):
  MU-SENSING §3.1 and its built-status row for turn motions,
  MU-ANTI-FLATTENING's drift-register row for clause 12, MU-HOSTING's
  crossing content and turn-motion rows (the retired claim name
  `turn_motion_composes_by_addition_mod_one` is replaced by
  `turn_motion_composes_by_addition` and
  `composed_turns_keep_their_winding`), and GLOSSARY *Turn*, *Turn
  motion* and *Crossing*.
- The claim `conformal_accepts_turn_valued_motion` states why a place
  carried across the seam is admitted: the carried place holds a whole
  turn, and a place is read by its phase.
- No code path, wire or ledger layout changes.
