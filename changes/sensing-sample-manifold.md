### Sample manifold carrier (78.179 §1, #32 item 6)
- **Carrier:** `transducer::sample_manifold::{SampleManifold, AxisManifold, PhaseManifold}`.
  A sample is built as a manifold once lifted into the field: one factor per
  phase.
  - `PhaseManifold::Circle { period }` where the content repeats along the axis
    with least proper divisor period `p`;
  - `PhaseManifold::Interval { extent }` otherwise;
  - `AxisManifold::One` or `Fibred { base, fibre }`;
  - `SampleManifold::torus` for the fully periodic torus `Tⁿ`.
- Replaces the sample's legacy "envelope".
- Claims: `periodic_content_becomes_circle`,
  `constant_content_becomes_circle_period_one`,
  `non_repeating_content_becomes_interval`,
  `fully_periodic_2d_sample_is_torus_t2`,
  `torus_constructor_creates_all_circle_phases`.
- No wire or layout change.
