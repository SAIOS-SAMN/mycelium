// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A9 — THE RECURSIVE SELECTION ALGEBRA: an anchored word is
//! γ_substance ∘ π_anchor, where π is a term composed from the 0-cells and
//! the 1-cell adjacency the sensor measures, evaluated exactly on a scene.

pub mod derive;
pub mod sel;
pub mod tape;

pub use derive::*;

/// ONE PIECE OF EVIDENCE for a selection term: the footprint a word touched
/// and the scene it touched it on (A9). The family of these is what `π` is
/// derived from; one of them underdetermines it.
pub type Evidence = (Vec<(i64, i64)>, Vec<(i64, i64, u8)>);

/// Evidence carried with whatever names the word that produced it — an
/// `Address` in the mind, raw key bytes on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceOf<A> {
    /// The word the evidence is FOR.
    pub addr: A,
    /// The footprint it touched.
    pub footprint: Vec<(i64, i64)>,
    /// The scene it touched it on.
    pub scene: Vec<(i64, i64, u8)>,
}

/// Evidence carried with the address that produced it.
pub type AddressedEvidence = EvidenceOf<crate::transducer::address::Address>;
pub use sel::*;

use super::blade::components;

#[cfg(test)]
mod tests;
