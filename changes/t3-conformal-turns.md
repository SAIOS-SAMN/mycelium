### The Conformal law accepts a motion stated as turns, checked in ℚ/ℤ (T3, #26; MU-HOSTING §5a.10)
- **Network contract:** none. No wire byte, act, verdict encoding, law
  version or ledger layout changes. A turn-valued motion is read in
  process and through the SDK; its wire form rides the crossing record
  (track 25, X1). The versor transition's encoding is unchanged.
- `physics::laws::verify_turns` reads a `TurnTransition` (a frame's
  periods, a `TurnMotion` of one turn per phase, each body's place before
  and after) under `Law::Conformal`: each place is carried phase by phase
  by addition mod 1 and compared in ℚ/ℤ, and a disagreement is the
  residue `t_claim − t_rec` per phase (`TurnBreach::NotCarried`). No turn
  becomes a rotor; only multiples of a quarter turn have exact rotors
  over ℚ (Niven), and every turn is checked as a turn.
- Refusals by name: `LeavesPeriods { phase, turn, period }` (a turn whose
  index on the phase's period is no whole site), `Rank`, `PlaceRank`, and
  `Unread` where a composed period passes the machine word.
- `Turn::plus` (the sum in ℚ/ℤ on the lcm of the two periods, the period
  of the composed orbit), `Turn::less` and `Turn::is_whole`;
  `TurnMotion::then` and `TurnMotion::carry`.
- Claims `conformal_accepts_turn_valued_motion`,
  `turn_motion_composes_by_addition_mod_one`,
  `verify_turns_refuses_turn_leaving_frame_periods`,
  `verify_turns_names_residue_of_uncarried_place`.
- **SDK surface, additive:** `mycelium_sdk::house` re-exports
  `verify_turns`, `TurnMotion`, `TurnTransition`, `TurnBreach`,
  `TurnVerdict`, `Turn` and `Enclosure`.
