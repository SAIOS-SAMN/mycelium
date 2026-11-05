### Transport, carrier and coordinate: the three layers documented and gated
- **Network contract:** none. No wire byte, act, ledger layout or signed
  byte changes; no source under `src/` changes.
- **The model** stands in MU-ENGINEERING §9: the transport moves bytes
  (connection, channel security, a length, deferral) and never holds
  meaning; a carrier holds meaning at full dimension and never projects
  to fewer; a coordinate routes and never identifies, and TUniversal
  stamps and never drives. §9.2 lists the known violations, each with its
  fix and release. Linked from AGENTS (*The regressions that recur*);
  GLOSSARY adds *Transport* and *Utterance* and aligns *Carrier* and
  *Frame*.
- **The gate keeps the transport free of meaning** (`gate/src/layers.rs`,
  tree test `transport_names_no_carrier`): the transport sources
  (`src/network/wire.rs`, `src/network/noise.rs`,
  `src/bin/mycelium-server/transport.rs`, listed once) name no carrier
  type (`Mv`, `Modes`, `Mu`, `Coordinate`, `Sample`, `SectionSample`,
  `SectionSite`, `Turn`, `TPoint`, `Lane`), read as tokens so comments
  are not read. Five listed exceptions, each until v0.5.0: `Lane` in the
  wire and the door (the lane byte), and `TPoint` in the wire, the Noise
  channel and the door (the pointed frame's trailer). A stale exception
  is refused. Fixture tests: one refusal, one pass.
- **"Frame" audited:** ten meanings, with counts and a proposed name for
  each by function, in MU-ENGINEERING §9.3 (*Names to settle*). No code
  identifier is renamed yet.
