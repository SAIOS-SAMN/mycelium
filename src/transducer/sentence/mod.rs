// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 2.2 — THE SENTENCE: a form of vertices, faces, orbits and remainder,
//! one algebra run in two directions — speaking a sentence from geometry, and
//! comprehending a received chain back into the words that compose it.

pub mod axis;
pub mod chain;
pub mod form;
pub mod transport;
pub mod class;
pub mod pivot;
pub mod address;
pub mod chart;
pub mod glue;
pub mod rebase;
pub mod inverse;
mod obstruction;
mod reading;
pub mod demand;
pub mod compose;
pub mod coordinates;
pub mod rung;
pub mod hearing;
pub mod theorem;
pub mod reasons;

pub use reasons::{Reasons, ReasonsTally};
pub use chain::*;
pub use demand::*;
pub use form::*;
pub use transport::*;
pub use class::*;
pub use pivot::*;
pub use address::*;
pub use chart::*;
pub use reading::*;
pub use inverse::*;
pub use obstruction::*;

#[cfg(test)]
pub(crate) mod tests;
