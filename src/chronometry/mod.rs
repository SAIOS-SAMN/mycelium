// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 45 — CHRONOMETRY: the receipt is the clock. K orders events inside one
//! kernel, TUniversal is the height of the receipt ledger, and no kernel
//! reads a wall clock.

pub mod chain;
pub mod era;
pub mod pack;
pub mod receipt;

pub use chain::*;
pub use era::*;
pub use receipt::*;

#[cfg(test)]
mod tests;
