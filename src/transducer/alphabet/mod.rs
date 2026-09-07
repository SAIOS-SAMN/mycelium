// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE CAYLEY ALPHABET (LAW VIII) — seventeen immutable letters.
//!
//! The alphabet is exactly seventeen generators on the Euclidean rank-4
//! chart `e0..e3`: one identity, six quarter turns (one per pair plane),
//! six Pythagorean (3,4,5) rotors and four Pythagorean (5,12,13) rotors.
//! Every letter is the one pair `Φ` with its carried `Φ⁻¹`, exact over ℚ:
//!
//! * quarter turn on `e_i∧e_j`:  `Φ = 1 + e_ij`,        `Φ⁻¹ = (1 − e_ij)/2`
//! * (3,4,5):                     `Φ = 3/5 + 4/5·e_ij`,   `Φ⁻¹ = 3/5 − 4/5·e_ij`
//! * (5,12,13):                   `Φ = 5/13 + 12/13·e_ij`, `Φ⁻¹ mirrored`
//!
//! A letter acts by the sandwich `Φ v Φ̃ / (Φ Φ̃)` — the rotor's rotation,
//! which is twice the rotor's own half-angle. No angles, no radians, no
//! roots: exact Pythagorean transports only. The alphabet never grows; the
//! language grows by WORDS — oriented tapes of letters whose product is one
//! rotor — minted on measured obstruction and adjudicated by the court.
//!
//! `rot90`, `rot180`, the flips and the transposition are not letters we
//! add: they are words already in the seventeen. A flip of the sensor's
//! plane is the shadow of a half turn in a plane that uses the third axis,
//! so the alphabet has no odd element and needs none.
//!
//! SPELLING is the peel that reduces a measured similarity of the plane to
//! the alphabet: the orientation sheet is peeled first, then the prime-5
//! letters, then the prime-13 letters, then the unit turns — each shell
//! reads a valuation and removes exactly one syllable, never trying and
//! verifying candidates. A rotation whose Gaussian factors lie outside the
//! alphabet's primes stands as a remainder: honest dialect, not a word.
//!
//! Base geometry: everything above (the conformal versors of `cga`, the
//! lattice isometries, the frame) is realised FROM a word, never beside it.

//!
//! One organ per file (Stage 8b S8): `rotor.rs` the exact even versor and
//! its arithmetic; `letters.rs` the seventeen letters and their relators;
//! `word.rs` a path through the alphabet and its normal form; `plane.rs`
//! the coefficient group and its conjugacy; `spelling.rs` which word spells
//! a block.

pub mod letters;
pub mod plane;
pub mod rotor;
pub mod spelling;
pub mod word;

pub use letters::*;
pub use plane::*;
pub use rotor::*;
pub use spelling::*;
pub use word::*;

use num_rational::BigRational;

pub type Q = BigRational;

#[cfg(test)]
mod tests;
