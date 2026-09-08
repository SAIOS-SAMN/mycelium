// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A13, 16.10, 16.11 — THE NERVE: the keyed relation over A21 coordinate
//! bytes, the cover that homes a word by incidence on the chart's lane, and
//! the router that finds a node by identity.

pub mod cover;
pub mod keyed;
pub mod router;

pub use cover::*;
pub use keyed::*;
pub use router::*;

#[cfg(test)]
mod tests;
