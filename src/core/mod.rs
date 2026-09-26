//! CORE — the exact arithmetic everything above is written in.
//!
//! - `q` — `Q`, the coefficient field ℚ that every multivector, versor and
//!   lane count is written over (law 4: exact, or refuse).
//! - `span` — the span of vectors as the blade it is, held by its factors:
//!   a vector joins where its wedge with the blade is non-zero, and the
//!   grade is the dimension A18 reads (16.18). No matrix and no rank of one.
//! - `exponents` — a word's factorisation over the words a mind holds: the
//!   integer exponents each generator is cited to, and the residue (16.12).
//!   Integer because an exponent is a count of letters, by meaning.
//! - `lanes` — the one checked step of a four-lane measure (STD1, rule 11).

pub mod exponents;
pub mod gaussian;
pub mod lanes;
pub mod q;
pub mod span;

#[cfg(test)]
mod tests;

pub use q::Q;
