### The 2026-11-05 audits' confirmed defects are fixed: a lane past the four is refused, a mind's admission names its refusal
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
