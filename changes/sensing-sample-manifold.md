### The sample as a manifold: phase circles and intervals; a periodic sample a torus (track 26, #32)
- **The reading** (ruled 2026-11-06 on #32, the envelope;
  `transduce::SampleManifold`). Each phase of a sample is a circle of its
  period, where the symbols repeat along it with a period dividing its
  extent (`PhaseManifold::Circle { period }`, the least such the
  circumference), or an interval chart with boundary
  (`PhaseManifold::Interval { extent }`). The sample is their product
  (`SampleManifold::of`), or a fibration where an axis is taken apart
  (`SampleManifold::nested`, `AxisManifold::Fibred { base, fibre }`). A
  fully periodic sample is a torus (`SampleManifold::torus`).
- Claims: `periodic_sample_reads_as_a_torus`,
  `aperiodic_phase_reads_as_an_interval`, `partial_period_reads_as_an_interval`,
  `nested_axis_reads_as_a_fibration`.
- A reading, computed when asked and never stored; no wire, task or
  address change.
