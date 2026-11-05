// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23, 42, 34.1 — THE VERDICT: a spoken form re-derived from its tape, the
//! same reconstruction with nothing supplied, the ruling over the federated basis,
//! the π* selection term, and the REFUSE_* constants a court answers with.

pub mod blind;
pub mod body;
pub mod factorisation;
pub mod reconstruct;
mod refusals;
pub mod resume;
pub mod ruling;
pub mod selection;
pub mod terrain;

#[cfg(test)]
mod tests;

pub use blind::*;
pub use reconstruct::*;
pub use refusals::*;
pub use resume::*;
pub(crate) use terrain::*;
pub use ruling::*;
#[cfg(test)]
pub use selection::*;
