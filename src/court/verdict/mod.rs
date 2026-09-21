// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23, 42, 34.1 — THE VERDICT: a spoken form re-derived from its tape, the
//! same replay with nothing supplied, the ruling over the federated basis,
//! the π* selection term, and the REFUSE_* constants a court answers with.

pub mod blind;
pub mod body;
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
/// 37.7 P2: a `RotorPrime` demand whose witness does not carry the prime
/// it names. `gcd_{ℤ[i]}(s + t·i, p)` has a norm that is not `p`, so
/// there is no letter to open and the demand claims one anyway. Refused
/// at the demand vertex, so a bad witness never reaches the docket.
pub const REFUSE_LETTER_WITNESS: &str = "rotor prime witness does not carry its prime";
/// 50.2 step 2: the cited generators do not compose to the word the
/// vertex carries. The court re-derived the derivation and disagreed —
/// a refusal at that vertex, never a smaller answer.
pub const REFUSE_DERIVATION: &str = "vertex derivation does not compose to its word";
pub const REFUSE_FACE: &str = "face does not re-derive from its vertices";
/// 54.5 (ruled 2026-09-20) — THE NAME IS THE OLD LAW AND THE READING IS
/// THE NEW ONE. It read "not identity", which was `H¹ = 0`: every 1-cycle
/// trivial. A compact holonomy about a fixed point RETURNS and is `ℤ/ℓℤ`
/// torsion, which is content (law 7); what refuses is a free part, a
/// circuit that walks and never closes, and a form owes that a
/// `Demand::Dislocation` rather than a filing.
pub const REFUSE_LOOP: &str = "loop holonomy is a free part and no dislocation declares it";
/// 54.5 — PERJURY ABOUT THE TERRAIN, which is the one thing in a receipt
/// the complex can contradict.
///
/// A receipt is a SUB-SELECTION of the cycles a stroke discharged: it says
/// WHICH classes, and the complex says what `ℓ` each closes at. A class the
/// form does not carry yields nothing and is not a refusal — an observer
/// may track cycles outside this form's horizon without defrauding anyone.
/// But a receipt that carries a period and names one the geometry does not
/// derive is claiming terrain, and `Σ(ℓ − 1)` prices reserve regeneration
/// (E2), so an unchecked claim mints capacity out of nothing.
pub const REFUSE_INFLATED_YIELD: &str = "receipt attests a period the complex does not derive";
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
/// 45.8, 34.1 (ruled 2026-09-09): a fold behind a prefix that is not
/// fully final, and a fold whose epoch rows do not conserve.
pub const REFUSE_FOLD_OPEN: &str = "fold prefix is not final";
pub const REFUSE_FOLD_INEXACT: &str = "fold does not conserve";
pub const REFUSE_DEMAND_UNTYPED: &str = "demand vertex names no kind";
pub const REFUSE_EDGE: &str = "edge transport is neither in the stabilizer nor the exchange nor a remainder edge";
/// 50.6, 34.1 (L7): a tape with no vertex that acts is a chain, refused
/// whole; a word vertex that does not carry its body onto its image on the
/// court's copy of the family is not a word for that body (50.1 step 1).
pub const REFUSE_NOT_A_SENTENCE: &str = "utterance is a chain, not a sentence";
pub const REFUSE_VERTEX_BODY: &str = "vertex does not carry its body onto its image";
/// 50.9 (L8): a sentence citing a refusal receipt is not the contraction
/// of the refused sentence at the coordinate the court named.
pub const REFUSE_NOT_CONTRACTION: &str = "sentence is not the contraction of the refused sentence at the coordinate";

/// A REFUSAL WITH ITS COORDINATE (50.9): the reason of 34.1 and where in
/// the sentence the court found it, as content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub reason: &'static str,
    pub at: sentence::At,
}

impl Refusal {
    /// A refusal of the whole tape.
    pub fn of_tape(reason: &'static str, spoken: &Element) -> Refusal {
        Refusal { reason, at: sentence::At::Tape(spoken.address()) }
    }

    /// The kind the reason travels as (34.1, 50.9): `VertexBody` is
    /// `REFUSE_VERTEX_BODY`, `EdgeHolonomy` is `REFUSE_EDGE`,
    /// `RemainderUndeclared` is `REFUSE_NOT_CLOSED`, `OrbitStill` is
    /// `REFUSE_ORBIT_STILL`, `FaceDerivation` is `REFUSE_FACE`, and every
    /// other refusal of the tape is a `MalformedComplex`.
    pub fn kind(&self) -> sentence::RefusalKind {
        match self.reason {
            REFUSE_VERTEX_BODY => sentence::RefusalKind::VertexBody,
            REFUSE_EDGE => sentence::RefusalKind::EdgeHolonomy,
            REFUSE_NOT_CLOSED => sentence::RefusalKind::RemainderUndeclared,
            REFUSE_ORBIT_STILL => sentence::RefusalKind::OrbitStill,
            REFUSE_FACE => sentence::RefusalKind::FaceDerivation,
            _ => sentence::RefusalKind::MalformedComplex,
        }
    }
}

/// WHAT A COURT HOLDS OF A SENTENCE IT REFUSED (50.9): the kind, the
/// coordinate and the receipt it answered with, keyed by the refused
/// address, so the contraction the voice cites next can be verified.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    pub kind: sentence::RefusalKind,
    pub at: sentence::At,
    pub receipt: [u8; 32],
}
/// 47.3, 34.1: a second organ must hold bond outside the home's
/// delegation closure and room on the lanes the act occupies.
pub const REFUSE_WITNESS_DEPENDENT: &str = "witness bond lies inside the home's delegation closure";
pub const REFUSE_WITNESS_ROOM: &str = "witness room does not cover the escrow on an occupied lane";
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
/// 34.1 (I21): an `ACT_EQUIVOCATION` whose two receipts are not 47.4's
/// proof — two keys, two K, or one chain hash (a duplicate frame). Decided
/// by `admission::admit_equivocation` before any carrier is touched.
pub const REFUSE_NOT_EQUIVOCATION: &str = "two receipts that do not share key and K, or do not differ";

#[cfg(test)]
mod tests;
