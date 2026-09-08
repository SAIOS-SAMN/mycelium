// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE RECURSIVE SELECTION ALGEBRA (alpha ruling): selection is never a
//! predefined enum of spatial filters — it is a recursive composition of
//! 0-cell boundary markings. An anchored word is γ_substance ∘ π_anchor where
//! the projector π is a TERM in this algebra, derived bottom-up from the two
//! lattice invariants the sensor actually measures:
//!
//!   Level 0 — 0-cells (occupied lattice points) and 1-cell adjacency
//!             (‖x−y‖₁ = 1); nothing else enters.
//!   Level 1 — atomic supports and morphisms: K_h (hue support), the
//!             coboundary δK = {x ∈ K | ∃ y ∉ K adjacent}, the extremal
//!             projections onto the substrate axes.
//!   Level 2+ — algebraic composition: ∩, ∖, the star/dilation (1-cell
//!             neighbourhood), connected-component projection — from which
//!             interior (K ∖ δK), relative interface (K₁ ∩ star K₂), hulls
//!             and every deeper predicate DERIVE rather than being named.
//!
//! The constructors of `Sel` are the OPERATORS of this algebra (like compose/
//! bracket on Element) — not labels over semantic categories. A term is
//! re-evaluated exactly on any scene; deriving the minimal term whose
//! evaluation carries a word's domain is deduction over the algebra, verified
//! downstream by replay (a wrong term simply fails to re-derive its train
//! evidence). All carriers are Vec-of-relations (Law II); evaluation is exact.

//!
//! Three organs (Stage 8b S32): `sel.rs` the selection term and how it
//! reads a scene (A9); `derive.rs` the selection a family agrees on (A9);
//! `tape.rs` the codec (19).

pub mod derive;
pub mod sel;
pub mod tape;

pub use derive::*;

/// ONE PIECE OF EVIDENCE for a selection term: the footprint a word touched
/// and the scene it touched it on (A9). The family of these is what `π` is
/// derived from; one of them underdetermines it.
pub type Evidence = (Vec<(i64, i64)>, Vec<(i64, i64, u8)>);

/// Evidence carried with the address that produced it.
pub type AddressedEvidence = (crate::transducer::a21::Address, Vec<(i64, i64)>, Vec<(i64, i64, u8)>);
pub use sel::*;

use super::blade::components;

#[cfg(test)]
mod tests;
