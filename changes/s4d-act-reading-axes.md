### The act reading's last planar carriers held by plane axis (track 26 S4d, stage 2d)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no task, word, chart or clause address moves: the panes
  keep their split byte (1 along the plane's first axis, 0 along its
  second) and the tiling its counts in the plane's order.
- The act reading's own cells (`family::acts`) are painted sites and
  section sites: the residue, its pieces (connected through a shared face
  generated at the section's rank), the owner of each site, the
  connectors, the echoes, the slide and the bends. The `(row, column,
  symbol)` adapter is removed.
- `Act::Panes { along_rows: bool }` is `Act::Panes { along: PlaneAxis }`;
  `Act::Tile { rows, cols }` is `Act::Tile { blocks }`, one count per
  plane axis; the speaking of panes, tiles, the square's symmetries and
  the scaling reads the plane through the perspective. They stay rank 2.
  Claims `frame_act_round_trips_and_reads_its_planar_bytes_by_axis`,
  `extract_rule_reads_panes_along_first_plane_axis`.
- `statement::Spoken.frame` is the output's extents per plane axis. Claim
  `speak_clauses_frames_output_by_plane_axis`.
- **SDK surface change, inside v0.4.0's break:** `Speech::grid` is
  `Speech::sample`. A client writes `s.sample()` where it wrote
  `s.grid()`.
- Test fixtures and names by function: `grid(rows)` is `digits(rows)`;
  `speech_sample_refuses_site_outside_frame`,
  `germinate_refuses_unchanged_scenes`,
  `slot_of_axis_stands_for_each_base_axis`.
- **Remaining planar, outside the act reading:** `Speech.frame` and
  `perceive::frame_of` (read with the measured `Frame`, a defect), and the
  ray of `Act::Slide` and `Act::Extend`, a clause field written as a pair.
