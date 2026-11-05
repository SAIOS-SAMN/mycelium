### The 2026-11-05 audits' confirmed defects are fixed: a lane past the four is refused, a mind's admission names its refusal, a clause-stated closure carries its modes
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
