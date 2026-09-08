// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! THE SENTENCE — one algebra, two directions.
//!
//! Speaking forms a sentence from geometry: cited words, each carried by a
//! versor and read at a gauge coordinate, and the residual no word speaks.
//! Comprehending runs the same algebra the other way. A received chain is
//! deconstructed into the words that compose it: each body reads its
//! moduli, the stratum over those coordinates names the word, the transport
//! is the quotient of the two bodies' moments, the gauge coordinate is the
//! assignment whose transition profile is the body's own, and the residual
//! is what remains once the transported word is carried onto the body —
//! exactly, on the sensor's own cells. A received sentence is verified by
//! replay: the citations are applied and what remains is measured.
//!
//! Nothing here enumerates the language. A stratum names one word when the
//! moduli are fine enough; when it holds several, that multiplicity is
//! returned as the measurement that the coordinates want an axis. Entity,
//! hypha, and court run these same functions; they differ only in the
//! direction they speak.


pub mod chain;
pub mod form;
pub mod transport;
pub mod class;
pub mod pivot;
pub mod tape;
pub mod chart;
pub mod reading;

pub use chain::*;
pub use form::*;
pub use transport::*;
pub use class::*;
pub use pivot::*;
pub use tape::*;
pub use chart::*;
pub use reading::*;

#[cfg(test)]
mod tests;
