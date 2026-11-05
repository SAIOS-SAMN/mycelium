### The 2026-11-05 audits' confirmed defects are fixed: a lane past the four is refused
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
