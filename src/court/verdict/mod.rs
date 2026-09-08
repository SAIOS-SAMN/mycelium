// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23, 42, 34.1 — THE VERDICT: a spoken form re-derived from its tape, the
//! same replay with nothing supplied, the ruling over the federated basis,
//! the π* selection term, and the REFUSE_* constants a court answers with.

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
use crate::transducer::torsion_witness::{self, HolonomicLoopWitness};

/// Spoken torsion period does not re-derive against the mesh's class.
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
/// S9.4 — CLOSURE, NOT DESCENT. A form is refused when it leaves a
/// remainder on some train pair: it does not close the family it claims.
/// What it MEASURED is priced by the economy and is not a verdict here.
pub const REFUSE_NO_CLOSURE: &str = "the form leaves a remainder — it closes nothing";
pub const REFUSE_RANK_LOST: &str = "form rank below what its tape re-derives";
pub const REFUSE_PIVOT_INEXACT: &str = "pivot certificate does not re-derive";
pub const REFUSE_ROOM: &str = "lane room does not cover the draw";
/// 13.9 step 8. Two organs re-derive one object and reach DIFFERENT
/// addresses: not a disagreement for a third to settle, but inconsistent
/// data, which can only arise from tapes that do not re-derive.
pub const REFUSE_MV_INEXACT: &str = "Mayer–Vietoris exactness fails: cochains are inconsistent";

#[cfg(test)]
mod tests;
