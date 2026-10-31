// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-DERIVATIONS D3.2 — THE TRANSDUCTION: two frames related body to body,
//! in turns, never through a common lattice.
//!
//! LIFT → ACT → PROJECT. Past the sensor a frame of extent `h × w` is two
//! circles of turns, `ℚ/ℤ`, with periods `h` and `w`: a site boundary is the
//! turn `r/h`, held as the pair (index, period) and compared with another by
//! integer cross-multiplication, never converted to a point, never reduced
//! to a common denominator. The seam `0 ≡ 1` of each circle is held as the
//! frame's boundary and never wrapped. A body is held by its BOUNDARY: its
//! bands, the maximal runs of one hue whose boundary turns stand on both
//! circles, never its interior sites. Two frames relate through the incidence
//! of the two frames' bands on the one turn circle of each axis: a re-period
//! of each circle (no versor carries an anisotropic scale), an offset, and
//! one of the eight signed swaps of the two circles. Sites are written only
//! where a grid is: the sensor's read, and the projection that speaks onto
//! an output frame.

mod enclosed;
mod record;
mod refusal;
mod turn;

#[cfg(test)]
mod tests;

pub use enclosed::{Band, Enclosed};
pub use record::{arcs, family_share, need, need_of, Incidence, Transduction};
pub use refusal::TransductionRefusal;
pub use turn::{Enclosure, Turn};
