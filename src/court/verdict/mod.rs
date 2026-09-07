// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Verification core for the court binary (#162, three-binary split).
//!
//! The court runs on this module and the shared exact algebra only — no
//! market, no door, no epigenome, no task registry. Replaying a candidate
//! reduction needs the minimal cellular replayer:
//!
//!   replay: (W, ∂₁Q) → (δσ̂ = 0)  ∧  strict ≺_DM descent
//!
//! plus the two-docket ruling over the federated basis and the π*
//! derivation from pooled evidence. Keeping this separate lets the court
//! binary run in constrained validator environments without linking the
//! server's role-specific machinery.

//!
//! Four organs (Stage 8b S15): `replay.rs` a spoken form re-derived from
//! its tape (23, 42); `blind.rs` the same with nothing supplied (42);
//! `ruling.rs` what a court decides (42); `selection.rs` the anchor a
//! family derives (A9).

pub mod blind;
pub mod replay;
pub mod ruling;
pub mod selection;

pub use blind::*;
pub use replay::*;
pub use ruling::*;
pub use selection::*;

use crate::anastome::dialogue;
use crate::network::nerve::Keyed;
use crate::transducer::anchor::{derive_carrying_all, Sel};
use crate::transducer::cech;
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::sentence::{self, Form, Vertex};
use crate::transducer::stalk::{self, Stalks};
use crate::transducer::toroid::{self, HolonomicLoopWitness};

/// Spoken torsion period does not re-derive against the mesh's class.
/// A demand whose residual the alphabet does not spell: no tape, so no word
/// of the language, so nothing the market can price (16.2).
pub const REFUSE_UNSPELLED: &str = "residual has no tape to spell";

pub const REFUSE_NOT_NATURAL: &str = "not natural";
/// Refusals of a crystallized form's replay (chapter 34.1).
pub const REFUSE_FORM_TAPE: &str = "form does not parse from its tape";
pub const REFUSE_GLUED_DEPTH: &str = "glued form depth not below sentence depth";
pub const REFUSE_GLUED_UNRULED: &str = "glued form not ruled";
pub const REFUSE_GLUED_FRAME_GAUGED: &str = "glued form frame is not identity";
pub const REFUSE_VERTEX_TAPE: &str = "vertex does not re-derive from its tape";
pub const REFUSE_FACE: &str = "face does not re-derive from its vertices";
pub const REFUSE_LOOP: &str = "loop holonomy is not identity";
pub const REFUSE_REBASE_TRIVIAL: &str = "rebased form equals its base";
pub const REFUSE_UNFOLD_STILL: &str = "unfolding measure does not move";
pub const REFUSE_ORBIT_STILL: &str = "orbit does not move";
pub const REFUSE_NO_DESCENT: &str = "no strict descent — the form closes nothing";
pub const REFUSE_RANK_LOST: &str = "form rank below what its tape re-derives";
pub const REFUSE_PIVOT_INEXACT: &str = "pivot certificate does not re-derive";
pub const REFUSE_ROOM: &str = "lane room does not cover the draw";
/// 13.9 step 8. Two organs re-derive one object and reach DIFFERENT
/// addresses: not a disagreement for a third to settle, but inconsistent
/// data, which can only arise from tapes that do not re-derive.
pub const REFUSE_MV_INEXACT: &str = "Mayer–Vietoris exactness fails: cochains are inconsistent";

#[cfg(test)]
mod tests;
