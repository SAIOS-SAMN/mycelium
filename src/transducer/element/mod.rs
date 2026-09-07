// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ELEMENT — the uniform carrier of an ANASTOME germ. An element IS the
//! DESCRIPTION of a transition; the SOLUTION is its inverse deduction `g⁻¹`,
//! never stored as a second word. Description ∘ solution = identity.
//!
//! The action is a versor, never a matrix (18.3). `Element { tape, action,
//! sub, mag }`: S is one versor `V = carriage · linear_versor(word)`, G the
//! hue permutation, T an extent in receipts, H the shell coordinates.
//! compose is the geometric product on S and permutation composition on G;
//! inverse is the versor inverse and refuses when it does not exist;
//! same_action is projective equality on V and equality of the permutation.
//! A cell is carried by the sandwich `V p Ṽ` and read at the sensor only.
//! English never appears here; addresses are wire identity, never reasoned on.

pub mod carrier;
pub mod gauge;
pub mod letters;
pub mod residual;
pub mod sensor;
pub mod tape;
pub mod vertex;

pub use carrier::*;
pub use gauge::*;
pub use letters::*;
pub use residual::*;
pub use sensor::*;
pub use tape::*;

#[cfg(test)]
mod tests;
