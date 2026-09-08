// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, A6 — THE VOLUMETRIC BLADE: a connected component of one hue lifted
//! into Cl(3,1) as a non-null dual sphere carrying four orthogonal strata,
//! and the exact scale-free moduli read off its own moments.

pub mod body;
pub mod harmonics;
pub mod carry;
pub mod shells;

pub use body::*;
pub use carry::*;
pub use shells::*;

use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

#[cfg(test)]
mod tests;
