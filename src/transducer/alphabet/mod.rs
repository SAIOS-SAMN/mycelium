// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.1, law 8 — THE CAYLEY ALPHABET: seventeen immutable letters on the
//! Euclidean rank-4 chart `e0..e3`, each the one pair `Φ` with its carried
//! `Φ⁻¹`, exact over ℚ. The alphabet never grows; the language grows by words.

pub mod block;
pub mod bundle;
pub mod derivation;
pub mod factor;
pub mod letters;
pub mod plane;
pub mod rewrite;
pub mod rotor;
pub mod spelling;
pub mod turn;
pub mod word;

pub(crate) use block::*;
pub use derivation::*;
pub use factor::*;
pub use letters::*;
pub use plane::*;
pub use rotor::*;
pub use spelling::*;
pub use turn::*;
pub use word::*;

use num_rational::BigRational;

pub type Q = BigRational;

#[cfg(test)]
mod tests;
