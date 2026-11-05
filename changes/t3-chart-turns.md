### The chart reads a section as turns of its own periods; the conformal lift retires from `section_moduli` (T3, #26)
- **Network contract:** none. No wire byte, act or ledger layout changes,
  and no chart, word, task or clause address moves: every chart reading
  is unchanged, axis for axis, and the oracle golden reproduces.
- `blade::SectionTurns` lifts a section by enclosure: its span along the
  plane is its frame, one phase per plane axis, and each site the turn of
  its offset from the span's least corner (`Turn::of`). The chart
  (`SectionTurns::chart`) reads the cube complex of the support the turns
  land on (`Turn::lands`) and each second moment in the frame's own
  gauge (a turn's offset from the mean turn times the period, with each
  round's extent), laid out by `Harmonics::axes`. `blade::section_moduli`
  reads it; the conformal blade (`lift`'s sphere, quartic moments and
  towers) is no longer built for a chart.
- Claim `chart_reads_sections_as_turns`: all 4 095 non-empty sections of
  a `3 × 4` window, a ring, a bar on each axis and a section at negative
  coordinates, in place and translated, read the lift's chart exactly,
  and each turn lands on a site of its period.
- The soak's chart instrument (`mycelium-soak --chart`) times the turns,
  the topology and the chart, and reads the moments from the turns; its
  trace names the stages `turns` and `chart` where it named `lift` and
  `harmonics`.
