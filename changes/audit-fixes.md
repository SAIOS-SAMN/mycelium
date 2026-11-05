### The 2026-11-05 audits' confirmed defects are fixed: a lane past the four is refused, a mind's admission names its refusal, a clause-stated closure carries its modes, a seated mind's solve is paid to its own key
- **A lane past the four is refused by name, never wrapped**
  (`lane_past_the_four_refused_never_wrapped`). `HomeTrie` indexed its
  lanes with `(lane as usize) & 3`, so lane 5 was filed, read and rooted
  as lane 1: a peer's cover page naming lane 5 stood a hypha on a lane it
  never named. A lane byte of 4 or more is now `LaneRefused { lane }`
  ("lane 5 refused: the base lanes are S, G, T, H (0 to 3)"). `admit`,
  `retract`, `rehome`, `fibre`, `multiplicity`, `charts` and `root` return
  `Result<_, LaneRefused>`; the exchange and the SDK session refuse a cover
  page at its first chart on a lane past the four and stop its walk; the
  court's own sites, which read only the four lanes, file nothing on a
  refusal. **Network contract: no wire change.** The cover page's bytes
  are unchanged; a lane byte of 4 or more, which was masked, is refused.
- **A mind's admission names its refusal** (`mind_admit_names_its_refusal`;
  reference mind, crate-private). `transducer::mind::admit` answered a
  `bool` its callers dropped with `let _ =`, and dropped each lane step's
  refusal inside it: a long law's admission landing where no mind thought
  (a guard settling perception between ticks) was lost without a word. It
  now answers `Result<Landed, AdmissionRefused>`: `Landed` reads the flux
  readings, words taught and covers joined; `NoMindThinking` hands the
  admission back whole; `LanePastItsCarrier { ledger, lane, held, add }`
  names the lane that would pass a `u64`, checked before anything lands, so
  the lanes land whole or not at all. The mind's two callers land a handed
  back admission by thinking in the mind's own field and report any other
  refusal.
- **A clause-stated closure carries the modes its carriages moved**
  (`paid_verdict_carries_the_modes_paid`). The deflation's motion reader
  (`form_motions`) read word vertices alone, so a form speaking by clauses
  (78.143, `Form::elements`) read as moving nothing: a solve its carriage
  closed was paid with an empty `Modes` (the claim's fixture, before the
  fix: "paid [2/9, 0, 0, 0]; modes no modes"). It now reads each
  `RuleElement::Carriage`'s versor and gauge beside the word vertices', and
  `rule_flux` reads the same motions, so the T flux and the T modes come
  from one set of motions: a clause carriage's measure and pay now carry
  the T flux a word vertex's motion already carried. **Not fixed, a
  contradiction of MU-ECONOMY §1 recorded for a ruling.** The quickstart's
  two paid verdicts ("paid μ [0, 2/3, 0, 0]; modes no modes" on
  `00576224`, "paid μ [11/50, 0, 0, 92]; modes no modes" on `00dbd492`)
  are closures by clause elements on a form with no word vertex, whose
  elements state no motion and no hue map the modes reader can name (on
  `00576224` a `Tile` of mirrored blocks). The closure moved no mode
  of §1's four kinds, while μ was paid from the closed sites' shares of
  the frame. A G-only closure by word vertices whose hue maps moved (for
  example `08ed6ac7`) also carries no modes, because each pair's modes are
  read at the S mass it carried, which a recolour does not move (§12
  item 4). No mode is invented for either.
- **A seated mind's solve is filed under its own key and paid to it**
  (`seated_mind_solve_paid_to_its_own_key`; Concordance 6.2a, 78.165). The
  SDK's seat path (`solve_before`) sent a task's solve to the house alone,
  as the in-game call `FN_SOLVE`, so a house that filed it with the court
  did so under the house's key and was paid for the seat's work. The seat
  path now files the court solve itself, signed by the key the mind sat by
  (`Mind::file_seat_solve`, over the new `Mind::submit_solve_signed`, which
  `submit_solve_rule` now calls with the voice's key), and still calls
  `FN_SOLVE` for houses that want the seat's answer in their game. The
  court pays the key that signs; `FN_SOLVE`'s docs, the house example and
  MU-HOSTING §5a.6 say so. **Network contract: no wire change.** The solve
  is the existing `ACT_SOLVE`; a seated mind now sends one per task it
  solves, held for its return as every solve of the mind is.
