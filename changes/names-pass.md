### Names pass: the 2026-11-05/06 vocabulary rulings in code
- Names only: no behaviour, value, byte tag or wire layout changes.
  Every served, printed or hashed string keeps its text (the act name
  `b"uttered"` under the operator digest, refusal strings, axiom texts,
  console and soak labels).
- **Expression, not utterance.** `client/sdk/mind/utter.rs` ->
  `express.rs`, `client/sdk/utterance.rs` -> `expression.rs`
  (`Utterance` -> `Expression`), `Action::Uttered` -> `Action::Asserted`
  (byte `0x4B`), `forget_utterance` -> `forget_expression`, `uttered_n` /
  `uttered` -> `asserted_n` / `asserted`, `verify_utterance` ->
  `verify_expression`, `StrokeOutcome::uttered` -> `asserted`; comments
  read expression / assert.
- **"Frame", by MU-ENGINEERING §9.3.** `WireFrame` -> `Delimited`,
  `read_frame` / `write_frame` -> `read_delimited` / `write_delimited`,
  `PageEnd::Frame` -> `PageEnd::Budget`, `append_frame` ->
  `append_record`; `frame_spelling` -> `sample_spelling`,
  `section_of_frames` -> `samples_of_section`, `HeldPose::OneFrame` ->
  `OneSample`; `LayerKind::Frame` -> `Enclosure`, `VertexAt::Frame` ->
  `VertexAt::Enclosure`, `frame_faces` / `frame_boundary` /
  `share_of_frame` / `frame_fixed_on_ground` -> `enclosure_*` /
  `share_of_enclosure`; `element::Frame` -> `element::Reperiod`;
  `Spoken::frame` -> `Spoken::extents`, `Unstated::Unframed` ->
  `NoEnclosure`; the court's `frame_of` -> `measure_of`, `orbit_frame` ->
  `orbit_measure`, the clock's `unframed` -> `unmeasured`;
  `forms/src/frame.rs` -> `forms/src/reference.rs` (`forms::reference`),
  a mouth's `from_frame` / `to_frame` -> `from_chart` / `to_chart`,
  `frame_base_point` -> `pose_point`; `DualFrame` -> `ReciprocalBasis`,
  the pose reader's `on_frame` -> `on_basis`. The passage's `Frame`
  (hold, reach) is the lawful pole basis and keeps its name.
- **The holonomic lane.** `network::wire::Lane::Harmonic` ->
  `Lane::Holonomic` (byte 3). The harmonic sense keeps its name.
- **Orbits and relations, not loops.** `handoff::Loop` -> `MouthOrbit`,
  `HandoffRequest::Loops` -> `Orbits`, `encode_loops_read` /
  `encode_loops_reply` / `decode_loops_reply` -> `*_orbits_*`,
  `mouth_loops` -> `mouth_orbits` (sub-kind byte 10 unchanged);
  `LoopReason` -> `OrbitReason`; `AxisLoops` -> `AxisRelations`;
  `hue_loop` -> `hue_relation`.
- Left for the branches that own them: the identifiers whose uses reach
  `src/transducer/field/` or `client/sdk/mind/chatter.rs` (`frame_key`,
  `perceive_frames`, `Vertex::Frame`, `Constellation::frames`,
  `frame_budget`, `hear_frame`, `Inflow::Frame`, `chatter::loop_of`,
  `same_loop`), and "door", which neither GLOSSARY nor MU-ENGINEERING
  names a replacement for.
