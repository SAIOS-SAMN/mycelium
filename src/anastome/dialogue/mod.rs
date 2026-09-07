// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE 5-ACT DIALOGUE — the closed ANASTOME speech-act algebra:
//!   percept — a sensory cochain broadcast (∂₁Q: the framed scene bodies).
//!   demand  — an unsolved coboundary challenge (an irreducible centered chain
//!             seeking inverse reduction / projective decomposition).
//!   quote   — "what is the highest-yield work?" answered by the yield surface
//!             read from genuine metabolic tension, with the open-demand market
//!             priced per lane (μ⃗).
//!   teach   — canonical element/sentence transmission across the nerve.
//!   settle  — the two-voice consilience receipt (δσ = 0 witnessed) carrying the
//!             polytonic μ⃗ award.
//! Every payload is exact geometry/algebra — chains, addresses, μ⃗ vectors. No
//! English, no semantic tags, no solver hints. Routing state is Vec-only
//! (Law II). The percept codec itself lives in `cognition` (this module owns the
//! act tags and the non-percept payloads).

//!
//! One file per act family (Stage 8b S6): `acts.rs` the act table and the
//! shape of every act; `carrier.rs` the carriers they share; and one file
//! each for speech, demand, quote, docket, cover, receipts, offer, syllabus
//! and percept.

pub mod acts;
pub mod carrier;
pub mod cover;
pub mod demand;
pub mod docket;
pub mod offer;
pub mod percept;
pub mod quote;
pub mod receipts;
pub mod speech;
pub mod syllabus;

pub use acts::*;
pub use carrier::*;
pub use cover::*;
pub use demand::*;
pub use docket::*;
pub use offer::*;
pub use percept::*;
pub use quote::*;
pub use receipts::*;
pub use speech::*;
pub use syllabus::*;

use crate::transducer::a21::{put_addr, take_addr, Address};

#[cfg(test)]
mod tests;
