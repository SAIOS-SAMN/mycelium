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

/// A class that does not survive restriction (15.5), or a spoken torsion
/// period that does not re-derive against the mesh's class (23.7).
pub const REFUSE_NOT_NATURAL: &str = "class key changes under restriction";
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
/// S9.4 — CLOSURE, NOT DESCENT (23.1 step 7, 34.1). A form is refused when
/// it leaves a remainder on some train pair: it does not close the family
/// it claims. What it MEASURED is priced by the economy and is not a
/// verdict here.
pub const REFUSE_NOT_CLOSED: &str = "form leaves a remainder on the family";
pub const REFUSE_RANK_LOST: &str = "form rank below what its tape re-derives";
pub const REFUSE_PIVOT_INEXACT: &str = "pivot certificate does not replay";
/// 23.8: a lane of the voice's room did not cover the draw. Nothing is
/// docketed; the refusal travels to the voice.
pub const REFUSE_ROOM: &str = "voice room does not cover the utterance on an occupied lane";
/// 13.9 step 8. Two organs re-derive one object and reach DIFFERENT
/// addresses: not a disagreement for a third to settle, but inconsistent
/// data, which can only arise from tapes that do not re-derive.
pub const REFUSE_MV_INEXACT: &str = "Mayer–Vietoris exactness fails: cochains are inconsistent";
/// 47.2 step 1: the frame's genesis or epoch is not this node's.
pub const REFUSE_ORIGIN: &str = "receipt names another genesis or origin epoch";
/// 47.2 step 4: the claimed height does not re-derive from the parents
/// named, or the prefix and the receipt it travels with disagree.
pub const REFUSE_RECEIPT: &str = "receipt height does not re-derive from its parents";
/// 47.2 step 3: a duplicate frame at a K already held, ignored.
pub const REFUSE_RECEIPT_STALE: &str = "receipt index at or below the signer's known head";
/// 47.4: two receipts at one K on one chain with different hashes.
pub const REFUSE_EQUIVOCATION: &str = "two receipts at one K with different chain hashes";
/// 47.2 step 2: the signature does not verify over the 47.1 prefix under
/// the key the frame claims.
pub const REFUSE_SIGNATURE: &str = "signature does not verify over the signed prefix";
/// 34.1 (I18): an act whose frame parsed but whose ruling names a payload
/// the frame does not carry — a REBIND with no selection, an ATTACH with no
/// base and transport. Decided before any carrier is touched.
pub const REFUSE_MALFORMED: &str = "act names a payload it does not carry";

#[cfg(test)]
mod tests;
