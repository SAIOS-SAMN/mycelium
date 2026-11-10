### The Manifold carrier (78.179 §1, #32 item 6)
- **Carrier:** `transducer::manifold::{Manifold, Axis, Phase}`.
  The geometric manifold an observation lifts into, one factor per phase,
  replacing legacy bounds and envelopes.
  - `Phase::Circle { period }` where content repeats along an axis with least
    proper divisor period `p`;
  - `Phase::Interval { extent }` otherwise;
  - `Axis::One` or `Fibred { base, fibre }`;
  - `Manifold::torus` for the fully periodic torus `Tⁿ`;
  - `sample.lift()` or `Manifold::lift_from(&sample)`.
- Replaces the sample's legacy "envelope".
- Claims: `periodic_content_becomes_circle`,
  `constant_content_becomes_circle_period_one`,
  `non_repeating_content_becomes_interval`,
  `fully_periodic_2d_sample_is_torus_t2`,
  `torus_constructor_creates_all_circle_phases`.
- No wire or layout change.
