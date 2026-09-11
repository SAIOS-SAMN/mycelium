// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 14 — THE ČECH COMPLEX: vertices are the holders whose cover contains
//! the chart, one section each; a k-cell is a (k+1)-fold overlap that has
//! volume. No cell forms over a remainder.
//!
//! A TRANSITION IS `D(ρ)·T(t)·R` AND IS CARRIED AS TWO FIELDS (L11e): the
//! rotor in `Cl(4,0)` over all six planes, the carriage in `Cl(3,1)`.
//! Sections and cells held the rotor alone, so `extend` saw a transition's
//! rotation and nothing else — finite rotor orders filed correctly, because
//! conjugation by a translation preserves the rotor class, and every
//! standing class was invisible. One versor field instead was tried and
//! refused by measurement: `Cl(3,1)` is the conformal algebra of a
//! two-dimensional base and holds one rotation plane, so it carries the
//! residues at the price of five of the rotor's planes.

pub mod cell;
pub mod coboundary;
pub mod complex;
pub mod tape;

pub use cell::*;
pub use coboundary::*;
pub use complex::*;

use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{Element, Versor};

#[cfg(test)]
mod tests;
