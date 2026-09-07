// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH COMPLEX — vertices are the hyphas (or germs) whose cover contains
//! the chart, one section each. A k-cell is a (k+1)-fold overlap and exists
//! only when that overlap has volume (open listings / shared support), a
//! polytonic μ⃗, never a word count. Coefficients are Rotor of Cl(4,0).
//! Work per glue is the link of one vertex, climbing degree only while
//! volume persists. A transition that does not realise as a rotor is a
//! remainder; no cell forms over it. The tower has no top.

//!
//! Four organs (Stage 8b S16): `cell.rs` a simplex, its transition and its
//! fill (13); `complex.rs` every cell a mind holds and the walks that read
//! it (13); `coboundary.rs` what closes a cell and the class that remains
//! (13, 14); `tape.rs` the complex on disk (28).

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
