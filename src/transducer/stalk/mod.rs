// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! M4, A6 — THE ATLAS OF THE BASE MANIFOLD: charts on M = P/G for the
//! bundle whose structure group is the rational conformal group. A word is
//! filed under exact scale-free coordinates; the members are addresses.

pub mod chart;
pub mod stalks;
pub mod stratum;

pub use chart::*;
pub use stalks::*;
pub(crate) use stratum::*;

use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Versor};
use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;

#[cfg(test)]
mod tests;
