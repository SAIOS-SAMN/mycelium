// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.3 — THE ELEMENT: the uniform carrier of an ANASTOME germ, and the
//! DESCRIPTION of a transition whose solution is its inverse `g⁻¹`.
//! `Element { tape, action, sub, mag }`; the action is a versor, never a matrix.

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
