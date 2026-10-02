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
use crate::transducer::torsion_witness::{self, HolonomicOrbitWitness};

/// A class that does not survive restriction (15.5): cover-relative, C.
pub const REFUSE_NOT_NATURAL: &str = "class key changes under restriction";
/// CV1, 78.9 — NOT_NATURAL SPLIT: a spoken torsion period that does not
/// re-derive against the mesh's class (23.7) is a claim that does not
/// re-derive, integrity (I), and is its own code.
pub const REFUSE_PERIOD_UNDERIVED: &str = "spoken torsion period does not re-derive against the held class";
/// Refusals of a crystallized form's replay (chapter 34.1).
pub const REFUSE_FORM_TAPE: &str = "form does not parse from its tape";
pub const REFUSE_GLUED_DEPTH: &str = "glued form depth not below sentence depth";
pub const REFUSE_GLUED_UNRULED: &str = "glued form not ruled";
/// 54.7 B5a — AN UNBONDED PORT. A `Vertex::Glued` states that this chart
/// exposes a boundary interface at `(chart, depth, placement, φ)`; a
/// `NerveEdge` states which form that interface mates with. A vertex whose
/// site carries no edge is an open boundary of the manifold and the court
/// says so, rather than leaving it to a reader's silent omission.
///
/// The two readers already fail safe — the tape writer refuses to write a
/// citation whose target the form does not carry, and `speak_on_scene`
/// reports the boundary unspoken — but neither NAMES the defect, and an
/// absence that only shows as a missing paint is the empty-carrier reading
/// that misled five times on 2026-09-20. Stated here it has a coordinate.
pub const REFUSE_ORPHAN_PORT: &str = "glued boundary stands at a site no citation reaches";
/// 54.7 B5a — A PHANTOM BOND, and it is the dangerous direction.
///
/// An orphan port is an incomplete attachment and fails safe everywhere: the
/// tape writer refuses, `speak_on_scene` reports unspoken. A phantom edge is
/// an UNANCHORED CLAIM OF CONNECTIVITY — a citation in the envelope that the
/// complex never constructed — and it does not fail safe: it would be
/// resolved, replayed and PAID FOR against a boundary the form does not hold.
///
/// It is never silently erased. Erasure masks perjury and corrupts the
/// economic attestation that reads the cover for use credit.
pub const REFUSE_PHANTOM_EDGE: &str = "citation claims a site the complex has no boundary at";
/// 54.7 B5a — THE SAME FORM MATED TWICE TO ONE FACET.
///
/// Multi-incidence at a site is a multi-sheeted seam and is admissible when
/// the targets are DISTINCT: the port is a junction between several charts,
/// and each sheet is an independent topological interface. Attaching the
/// SAME external chart twice to the same local facet under the same carriage
/// is redundant topological noise, and it would inflate the yield by
/// declaring one geometric overlap as two.
pub const REFUSE_SHEET_REPEATED: &str = "two citations of one form at one boundary site";
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
/// circuit that runs on and never closes, and a form owes that a
/// `Demand::Dislocation` rather than a filing.
pub const REFUSE_ORBIT: &str = "orbit holonomy is a free part and no dislocation declares it";
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
/// S9.4 — CLOSURE, NOT TRAJECTORY (23.1 step 7, 34.1). A form is refused when
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
/// 78.67 (A): a prefix holding nothing but earlier folds and orbit closes
/// moves no work; folding it would only fold folds, and the fold's own
/// receipt would open the orbit whose close proposed the next one.
pub const REFUSE_FOLD_IDLE: &str = "fold prefix carries no work";
/// 78.68, 78.69: the algorithm did not change in the prefix — no `Return` (a
/// ring completed an age), no `RankStep` (a chart's holonomy algebra gained
/// a dimension), no `Reconnect` (the field's links moved, 78.71), no generator entering the presentation (`Install`,
/// `Ruling(Upheld)`, `Adopted`) — so it is coefficient motion and a fold
/// would cut nothing the physics names. Replaces 78.67 (B)'s byte
/// break-even.
pub const REFUSE_FOLD_NO_TRANSITION: &str = "fold prefix holds no generator admitted, no return and no rank step";
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
/// 78.53: a colour word whose fibre relation is not the one the court
/// re-derives from the family's base — or a family bound to no scene. It is
/// C, relative to this cover: another family may witness the relation.
pub const REFUSE_FIBRE_UNWITNESSED: &str = "colour word's fibre relation is not the one the family's base carries";

/// A REFUSAL WITH ITS COORDINATE (50.9): the reason of 34.1 and where in
/// the sentence the court found it, as content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub reason: &'static str,
    pub at: sentence::At,
    /// CV1 — what kind of obstruction the refusal is, with its content
    /// (78.9): read off what the refusing reader carried by the one reader
    /// where the refusal is a closure's, the ruled arm where it is not.
    pub obstruction: sentence::ClosureObstruction,
}

/// CV1, 78.9 — THE RULED ARMS OF THE CODES THAT ARE NOT A CLOSURE'S: the
/// integrity codes (does not parse, a citation badly formed, a claim that
/// does not re-derive, a claim of nothing that moves, economy and folds,
/// the chain's own integrity) are I; a cited word the court has not ruled
/// is D, held by the court (CV2); a class that does not survive
/// restriction is C (15.5). The four closure codes — ORBIT, NOT_CLOSED,
/// VERTEX_BODY, EDGE — are read by content at their sites
/// ([`Refusal::closure`]); reaching here with one is a refusal no reader
/// placed, UNCLASSIFIED and counted as the defect it is.
pub fn ruled_obstruction(reason: &'static str) -> sentence::ClosureObstruction {
    use sentence::ClosureObstruction as O;
    match reason {
        REFUSE_FORM_TAPE | REFUSE_VERTEX_TAPE | REFUSE_NOT_A_SENTENCE | REFUSE_MALFORMED | REFUSE_DEMAND_UNTYPED => {
            O::Integrity
        }
        REFUSE_GLUED_DEPTH | REFUSE_ORPHAN_PORT | REFUSE_PHANTOM_EDGE | REFUSE_SHEET_REPEATED
        | REFUSE_GLUED_FRAME_GAUGED => O::Integrity,
        REFUSE_DERIVATION | REFUSE_FACE | REFUSE_RANK_LOST | REFUSE_PIVOT_INEXACT | REFUSE_INFLATED_YIELD
        | REFUSE_LETTER_WITNESS | REFUSE_NOT_CONTRACTION | REFUSE_PERIOD_UNDERIVED => O::Integrity,
        REFUSE_REBASE_TRIVIAL | REFUSE_UNFOLD_STILL | REFUSE_ORBIT_STILL => O::Integrity,
        REFUSE_ROOM | REFUSE_WITNESS_ROOM | REFUSE_WITNESS_DEPENDENT | REFUSE_FOLD_OPEN | REFUSE_FOLD_INEXACT => {
            O::Integrity
        }
        REFUSE_FOLD_IDLE | REFUSE_FOLD_NO_TRANSITION => O::Integrity,
        REFUSE_ORIGIN | REFUSE_RECEIPT | REFUSE_RECEIPT_STALE | REFUSE_EQUIVOCATION | REFUSE_NOT_EQUIVOCATION
        | REFUSE_SIGNATURE | REFUSE_MV_INEXACT => O::Integrity,
        REFUSE_GLUED_UNRULED => O::Dialect { coset: Vec::new(), chart: None },
        REFUSE_NOT_NATURAL | REFUSE_FIBRE_UNWITNESSED | REFUSE_SECTION_TURN_OPEN | REFUSE_SECTION_TURN_UNREAD => {
            O::Cover { cocycle: Vec::new() }
        }
        _ => O::Unclassified,
    }
}

impl Refusal {
    /// A refusal whose kind is its code's ruled arm.
    pub fn new(reason: &'static str, at: sentence::At) -> Refusal {
        Refusal { reason, at, obstruction: ruled_obstruction(reason) }
    }

    /// A CLOSURE'S REFUSAL, its kind read off what the refusing reader
    /// carried by the one reader (78.9).
    pub fn closure(reason: &'static str, at: sentence::At, content: sentence::Content) -> Refusal {
        Refusal { reason, at, obstruction: sentence::read_obstruction(content) }
    }

    /// A refusal of the whole tape.
    pub fn of_tape(reason: &'static str, spoken: &Element) -> Refusal {
        Refusal::new(reason, sentence::At::Tape(spoken.address()))
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
            // 54.6 — ONE REASON, TWO KINDS, TOLD APART BY THE COORDINATE.
            //
            // A remainder the form was SILENT on (`spoken == 0`) is a
            // demand the speaker can mount; one it SPOKE WRONGLY is not,
            // and `undeclared_remainder` keeps the latter whatever a demand
            // says of it. `replay_form_at` partitions them and packs one or
            // the other, never both, so the links themselves say which
            // refusal this is — and the kind is read from them rather than
            // from the reason, which cannot tell them apart.
            //
            // Reading it off the reason alone sent both as
            // `RemainderUndeclared`: the speaker contracted over the union,
            // the contraction moved the address, the old seal stopped
            // matching, and the same guaranteed refusal returned at a fresh
            // coordinate — one minted word per cycle.
            REFUSE_NOT_CLOSED => match &self.at {
                sentence::At::Chart(_, links)
                    if links.iter().any(|&(_, _, spoken, _)| spoken != 0) =>
                {
                    sentence::RefusalKind::RemainderWrong
                }
                _ => sentence::RefusalKind::RemainderUndeclared,
            },
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
/// 13.10, 78.44 — the gate on two sections of one chart: a turn relates them
/// that closes at no finite order, so the chart's orbit is not finite.
pub const REFUSE_SECTION_TURN_OPEN: &str = "sections of one chart related by a rotation of no finite order";
/// 13.10 — a finite turn relates two sections and has no versor or class to
/// file as the cocycle.
pub const REFUSE_SECTION_TURN_UNREAD: &str = "sections of one chart related by a rotation with no versor or class";
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
