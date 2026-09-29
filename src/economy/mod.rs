// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26, E1, E2 — THE ECONOMY: what a word measures, who is bonded, what is
//! set aside against a listing, what each closed act pays, and the ledger
//! on disk. Every balance is a polytonic μ⃗ (𝒮, 𝒢, 𝒯, 𝒽), never a scalar.

pub mod escrow;
pub mod fold;
pub mod lanes;
pub mod pay;
pub mod persist;
pub mod settle;
pub mod treasury;

pub use fold::{nets_of, Folded, Nets};
pub use lanes::*;
pub use pay::{Held, Unpaid};
pub use treasury::*;

use crate::transducer::element::Element;

#[cfg(test)]
mod tests;
