// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE EXACT ALGEBRA: the multivectors of `Cl(p, q)` over ℚ, each value
//! carrying its `n` and the mask of its negative generators (MU-SENSING
//! §3.7), the geometric product written once for every `n` and every
//! signature, and the wire writing of a value of `Cl(n+1, 1)` with the key
//! it is written into (A4, 71, A21).
//!
//! It stands below the law forms (`forms`) and the library (`mycelium`),
//! and depends on the exact rationals alone, so a law form states its
//! product in the algebra without reaching the mind or the court
//! (Concordance 2.10). The library re-exports every item here at
//! `transducer::cga`, where the conformal model's constructions stand.

pub mod address;
pub mod component;
pub mod multivector;
mod ray;
mod signature;

pub use multivector::{axes, base_axis, blades, pseudo, slots, Mv, E1, E2, E3, EM, EP, MAX_N};
pub use ray::projective_ratio;
pub use signature::{Clifford, SignatureRefusal};

/// The crate's README, compiled as a doctest, so its example is the API's.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
