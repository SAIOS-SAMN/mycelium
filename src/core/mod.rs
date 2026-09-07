//! CORE — the one scalar the whole engine counts in.
//!
//! `Q`, the exact rational, and `zlinalg`, the exact ℤ rank A18 reads. That
//! is all of it. What used to stand here — the antisymmetric Δ, the rational
//! canonical form and its conjugacy hash, `QMatrix` and the conjugating
//! `Operator`, the Taylor exponential with its clamp, the CCL Laws, the
//! SAIOS superstructure — was the representation the mycelium layers
//! replaced: an action is an algebra element and never a matrix (law 11,
//! rule 16), an identity is content and never a digest of it (law 2), and a
//! constant that stands in for a measurement of the world is refused
//! (rule 5). Deleted in F2, 2026-09-07.

pub mod q;
pub mod zlinalg;

pub use q::Q;
