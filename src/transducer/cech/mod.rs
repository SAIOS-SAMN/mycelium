// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 14 — THE ČECH COMPLEX: vertices are the holders whose cover contains
//! the chart, one section each; a k-simplex is a (k+1)-fold overlap that
//! has volume. No simplex forms over a remainder.
//!
//! A TRANSITION IS `D(ρ)·T(t)·R` AND IS CARRIED AS TWO FIELDS (L11e): the
//! rotor in `Cl(4,0)` over all six planes, the carriage in `Cl(3,1)`. One
//! versor field cannot hold both: `Cl(3,1)` is the conformal algebra of a
//! two-dimensional base and holds one rotation plane, so it carries the
//! residues at the price of five of the rotor's planes; and a rotor alone
//! files finite orders correctly, because conjugation by a translation
//! preserves the rotor class, while every standing class is invisible to
//! it.

pub mod board;
pub mod coboundary;
pub mod coefficient;
pub mod complex;
pub mod cover;
pub mod connecting;
pub mod coxeter;
pub mod flux;
pub mod hole;
pub mod nerve;
pub mod orbit;
pub mod orbit_algebra;
pub mod simplex;
pub mod persist;
pub mod quotient;
pub mod remainder;

pub use board::{independent, Applied, Board, Pivot};
pub use coboundary::*;
pub use complex::*;
pub use connecting::{connecting, Glued, Unglued};
pub use hole::{Capped, Condensed, Flex, Punctured};
pub use coxeter::*;
pub use nerve::*;
pub use orbit_algebra::*;
pub use simplex::*;
pub use quotient::{quotient, Outside, Quotient};
pub use remainder::{RemainderCount, RemainderReading};

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{Element, Versor};

#[cfg(test)]
mod tests;
