//! CORE — the exact coefficient field.
//!
//! `Q` is ℚ. It is the field the COEFFICIENTS of a multivector in
//! `Cl(3,1)(ℚ)` are written in, so that every coordinate is exact or refused
//! (rule 11). It is not what the engine counts in: the engine counts in
//! versors and blades, in μ⃗ per lane, and in ℤ-ranks. In the algebra
//! "scalar" names grade 0 — one part of a multivector, never the whole of
//! one — and a reading that collapses a multivector or a μ⃗ to one number is
//! the flattening law 5 refuses.
//!
//! `zlinalg` is the exact ℤ rank A18 reads. Those two are the whole of this
//! module.

pub mod q;
pub mod zlinalg;

pub use q::Q;
