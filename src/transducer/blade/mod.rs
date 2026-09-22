// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, A6, 70.4 — THE VOLUMETRIC BLADE: a hue-blind connected component
//! lifted into Cl(3,1) as a non-null dual sphere carrying four orthogonal
//! strata, and the exact scale-free moduli read off its own moments.

pub mod body;
pub mod harmonics;
pub mod carry;
pub mod shells;
pub mod surgery;
pub mod continuous;
pub mod frame;
pub mod strain;
pub mod tower;

pub use body::*;
pub use carry::*;
pub use shells::*;
pub use surgery::*;
pub use continuous::*;
pub use frame::*;
pub use strain::*;
pub use tower::*;

use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

#[cfg(test)]
mod tests;
