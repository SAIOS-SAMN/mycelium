//! CORE — `Q`, the exact coefficient field (ℚ) every multivector, versor and
//! lane count is written over, and `zlinalg`, the exact ℤ rank A18 reads.

pub mod q;
pub mod zlinalg;

pub use q::Q;
