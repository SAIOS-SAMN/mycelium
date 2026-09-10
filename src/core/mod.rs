//! CORE — the exact arithmetic everything above is written in.
//!
//! - `q` — `Q`, the coefficient field ℚ that every multivector, versor and
//!   lane count is written over (law 4: exact, or refuse).
//! - `lattice` — exact ℤ linear algebra: the rank A18 reads (16.18) and the
//!   Hermite form with its transform, which is the factorisation A12 looks
//!   for (16.12).

pub mod gaussian;
pub mod lattice;
pub mod q;

pub use q::Q;
