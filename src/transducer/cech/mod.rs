// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 14 — THE ČECH COMPLEX: vertices are the holders whose cover contains
//! the chart, one section each; a k-cell is a (k+1)-fold overlap that has
//! volume, with Rotor coefficients. No cell forms over a remainder.

pub mod cell;
pub mod coboundary;
pub mod complex;
pub mod tape;

pub use cell::*;
pub use coboundary::*;
pub use complex::*;

use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::Element;

#[cfg(test)]
mod tests;
