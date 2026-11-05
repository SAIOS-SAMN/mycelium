// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A9 — THE RECURSIVE SELECTION ALGEBRA: an anchored word is
//! γ_substance ∘ π_anchor, where π is a term composed from the sites and
//! the edge adjacency the sensor measures, evaluated exactly on a scene.

pub mod codec;
pub mod derive;
pub mod selection;

pub use derive::*;

/// ONE PIECE OF EVIDENCE for a selection term: the footprint a word touched
/// (sites of the section) and the scene it touched it on (its painted
/// sites) (A9; MU-SENSING §3.6). The family of these is what `π` is derived
/// from; one of them underdetermines it.
pub type Evidence = (Vec<SectionSite>, Vec<PaintedSite>);

/// Evidence carried with whatever names the word that produced it — an
/// `Address` in the mind, raw key bytes on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceOf<A> {
    /// The word the evidence is FOR.
    pub addr: A,
    /// The footprint it touched.
    pub footprint: Vec<SectionSite>,
    /// The scene it touched it on.
    pub scene: Vec<PaintedSite>,
}

/// Evidence carried with the address that produced it.
pub type AddressedEvidence = EvidenceOf<crate::transducer::address::Address>;
pub use selection::*;

use super::blade::components;
use super::spelled::{PaintedSite, SectionSite};

#[cfg(test)]
mod tests;
