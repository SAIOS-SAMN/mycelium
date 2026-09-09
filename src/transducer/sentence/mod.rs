// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 2.2 — THE SENTENCE: a form of vertices, faces, loops and remainder,
//! one algebra run in two directions — speaking a sentence from geometry, and
//! comprehending a received chain back into the words that compose it.

pub mod chain;
pub mod form;
pub mod transport;
pub mod class;
pub mod pivot;
pub mod tape;
pub mod chart;
pub mod reading;
pub mod demand;

pub use chain::*;
pub use demand::*;
pub use form::*;
pub use transport::*;
pub use class::*;
pub use pivot::*;
pub use tape::*;
pub use chart::*;
pub use reading::*;

#[cfg(test)]
mod tests;
