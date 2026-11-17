// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.185 (approved 2026-11-17) — A HYPHA'S ATTACHMENT AS AN ADJUNCTION OF
// THE COVER, ITS DETACHMENT AS A RETRACTION, each carried as a crossing.
//
//! Two sub-kinds of `ACT_DOMAIN`, keyed by the hypha's own key and signed
//! by it, each carrying its crossing record (X1, #160):
//!
//! ```text
//! ADJUNCTION [ACT_DOMAIN][hypha 32][0x80 0x00][21][n u8](node 32, g Mv, [m u8](phase ℚ, pole ℚ)…)…[nonce 32][crossing][sig 64]
//! RETRACTION [ACT_DOMAIN][hypha 32][0x80 0x00][22][nonce 32][crossing][sig 64]
//! ```
//!
//! An adjunction states its tethers: each node it links, the motion `g`
//! carrying that node's readings onto the hypha's, and the planes of their
//! relative phase state. The court admits it under `forms::cover`: the link
//! a simplex of the nerve, the cocycle closed on every triangle, a tether at
//! rest. A retraction states only its withdrawal. Nothing is registered: a
//! hypha is attached exactly when an adjunction of it is admitted.

use super::atlas::{byte, head, key32};
use super::crossing_record::{crossed, Crossing};
use crate::chronometry::Stamp;
use crate::transducer::cga::Mv;
use num_rational::BigRational as Q;

/// FORMAT: sub-kind byte of ACT_DOMAIN for an adjunction.
pub const KIND_ADJUNCTION: u8 = 21;
/// FORMAT: sub-kind byte of ACT_DOMAIN for a retraction.
pub const KIND_RETRACTION: u8 = 22;
const ADJUNCTION: &[u8] = b"mycelium adjunction v1";
const RETRACTION: &[u8] = b"mycelium retraction v1";

/// ONE TETHER the hypha claims: the node, the motion carrying the node's
/// readings onto the hypha's, and their relative phase state's planes, each
/// `(phase, pole)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TetherStated {
    pub node: [u8; 32],
    pub g: Mv,
    pub planes: Vec<(Q, Q)>,
}

/// AN ADJUNCTION: the hypha, its tethers and a nonce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Adjunction {
    pub hypha: [u8; 32],
    pub tethers: Vec<TetherStated>,
    pub nonce: [u8; 32],
}

/// A RETRACTION: the hypha and a nonce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Retraction {
    pub hypha: [u8; 32],
    pub nonce: [u8; 32],
}

/// Either act, signed by the hypha's key over its bytes and its crossing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signed<T> {
    pub act: T,
    pub crossing: Crossing,
    pub sig: [u8; 64],
}

fn put_q(out: &mut Vec<u8>, q: &Q) {
    Mv::scalar(q.clone()).to_bytes(out);
}

fn take_q(b: &[u8], p: &mut usize) -> Option<Q> {
    Some(Mv::from_bytes(b, p)?.scalar_part())
}

pub(crate) fn put_tethers(out: &mut Vec<u8>, ts: &[TetherStated]) -> Option<()> {
    out.push(u8::try_from(ts.len()).ok()?);
    for t in ts {
        out.extend_from_slice(&t.node);
        t.g.to_bytes(out);
        out.push(u8::try_from(t.planes.len()).ok()?);
        for (phase, pole) in &t.planes {
            put_q(out, phase);
            put_q(out, pole);
        }
    }
    Some(())
}

pub(crate) fn take_tethers(b: &[u8], p: &mut usize) -> Option<Vec<TetherStated>> {
    (0..byte(b, p)?)
        .map(|_| {
            let node = key32(b, p)?;
            let g = Mv::from_bytes(b, p)?;
            let planes = (0..byte(b, p)?).map(|_| Some((take_q(b, p)?, take_q(b, p)?))).collect::<Option<Vec<_>>>()?;
            Some(TetherStated { node, g, planes })
        })
        .collect()
}

impl Adjunction {
    /// `"mycelium adjunction v1" ‖ hypha ‖ tethers ‖ nonce`.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        let mut out = ADJUNCTION.to_vec();
        out.extend_from_slice(&self.hypha);
        put_tethers(&mut out, &self.tethers)?;
        out.extend_from_slice(&self.nonce);
        Some(out)
    }

    /// Signed by the hypha's key with the crossing it carries.
    pub fn sign(self, crossing: Crossing, key: &ed25519_dalek::SigningKey) -> Option<Signed<Adjunction>> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&crossed(self.signed_bytes()?, &crossing)?).to_bytes();
        Some(Signed { act: self, crossing, sig })
    }
}

impl Retraction {
    /// `"mycelium retraction v1" ‖ hypha ‖ nonce`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = RETRACTION.to_vec();
        out.extend_from_slice(&self.hypha);
        out.extend_from_slice(&self.nonce);
        out
    }

    /// Signed by the hypha's key with the crossing it carries.
    pub fn sign(self, crossing: Crossing, key: &ed25519_dalek::SigningKey) -> Option<Signed<Retraction>> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&crossed(self.signed_bytes(), &crossing)?).to_bytes();
        Some(Signed { act: self, crossing, sig })
    }
}

impl Signed<Adjunction> {
    /// The bytes the signature is over: the act's, then its crossing.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        crossed(self.act.signed_bytes()?, &self.crossing)
    }
}

impl Signed<Retraction> {
    /// The bytes the signature is over: the act's, then its crossing.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        crossed(self.act.signed_bytes(), &self.crossing)
    }
}

/// The ADJUNCTION act on the wire.
pub fn encode_adjunction(s: &Signed<Adjunction>) -> Option<Vec<u8>> {
    let mut out = head(&s.act.hypha, KIND_ADJUNCTION);
    put_tethers(&mut out, &s.act.tethers)?;
    out.extend_from_slice(&s.act.nonce);
    s.crossing.put(&mut out)?;
    out.extend_from_slice(&s.sig);
    Some(out)
}

/// The RETRACTION act on the wire.
pub fn encode_retraction(s: &Signed<Retraction>) -> Option<Vec<u8>> {
    let mut out = head(&s.act.hypha, KIND_RETRACTION);
    out.extend_from_slice(&s.act.nonce);
    s.crossing.put(&mut out)?;
    out.extend_from_slice(&s.sig);
    Some(out)
}

fn take_tail(b: &[u8], p: &mut usize) -> Option<([u8; 32], Crossing, [u8; 64])> {
    let nonce = key32(b, p)?;
    let crossing = Crossing::take(b, p).ok()?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    Some((nonce, crossing, sig))
}

/// The ADJUNCTION's body after its sub-kind, read whole.
pub(crate) fn take_adjunction(hypha: [u8; 32], b: &[u8], p: &mut usize) -> Option<Signed<Adjunction>> {
    let tethers = take_tethers(b, p)?;
    let (nonce, crossing, sig) = take_tail(b, p)?;
    Some(Signed { act: Adjunction { hypha, tethers, nonce }, crossing, sig })
}

/// The RETRACTION's body after its sub-kind, read whole.
pub(crate) fn take_retraction(hypha: [u8; 32], b: &[u8], p: &mut usize) -> Option<Signed<Retraction>> {
    let (nonce, crossing, sig) = take_tail(b, p)?;
    Some(Signed { act: Retraction { hypha, nonce }, crossing, sig })
}

/// WHY AN ADJUNCTION OR A RETRACTION WAS REFUSED, by name (78.185).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoverRefused {
    /// 1 — the signature is not the hypha's key's.
    Unsigned,
    /// 2 — the crossing record does not verify.
    Crossing,
    /// 3 — the hypha is attached already, or the retracted one is not.
    Held,
    /// 4 — a tether names a node the nerve does not hold.
    NoNode([u8; 32]),
    /// 5 — two nodes of the link do not overlap: a new cycle.
    OpensCycle([u8; 32], [u8; 32]),
    /// 6 — the readings disagree on the triangle of these two nodes.
    Cocycle([u8; 32], [u8; 32]),
    /// 7 — no tether stands at rest.
    Untethered,
    /// 8 — the retraction would disconnect the nerve.
    Disconnects,
    /// 9 — a motion that is not a unit motion.
    Motion,
    /// 10 — a claim still stands on the hypha's stake.
    Claimed,
    /// 11 — the court's line could not take the act.
    LineRefused,
    /// 12 — a genesis hypha: it is the cover's ground, never retracted.
    Genesis,
}

/// AN ADJUNCTION OR A RETRACTION ANSWERED: sealed at its stamp and receipt,
/// or refused by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Covered {
    Sealed { at: Stamp, receipt: [u8; 32] },
    Refused(CoverRefused),
}

/// `[0][stamp][receipt 32]` or `[1][code][node 32 …]`.
pub fn encode_cover_reply(c: &Covered) -> Vec<u8> {
    match c {
        Covered::Sealed { at, receipt } => {
            let mut out = vec![0u8];
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out
        }
        Covered::Refused(r) => {
            let mut out = vec![1u8];
            match r {
                CoverRefused::Unsigned => out.push(1),
                CoverRefused::Crossing => out.push(2),
                CoverRefused::Held => out.push(3),
                CoverRefused::NoNode(k) => {
                    out.push(4);
                    out.extend_from_slice(k);
                }
                CoverRefused::OpensCycle(a, b) => {
                    out.push(5);
                    out.extend_from_slice(a);
                    out.extend_from_slice(b);
                }
                CoverRefused::Cocycle(a, b) => {
                    out.push(6);
                    out.extend_from_slice(a);
                    out.extend_from_slice(b);
                }
                CoverRefused::Untethered => out.push(7),
                CoverRefused::Disconnects => out.push(8),
                CoverRefused::Motion => out.push(9),
                CoverRefused::Claimed => out.push(10),
                CoverRefused::LineRefused => out.push(11),
                CoverRefused::Genesis => out.push(12),
            }
            out
        }
    }
}

pub fn decode_cover_reply(b: &[u8]) -> Option<Covered> {
    let mut p = 0usize;
    let c = match byte(b, &mut p)? {
        0 => Covered::Sealed { at: Stamp::take_canonical(b, &mut p)?, receipt: key32(b, &mut p)? },
        1 => Covered::Refused(match byte(b, &mut p)? {
            1 => CoverRefused::Unsigned,
            2 => CoverRefused::Crossing,
            3 => CoverRefused::Held,
            4 => CoverRefused::NoNode(key32(b, &mut p)?),
            5 => CoverRefused::OpensCycle(key32(b, &mut p)?, key32(b, &mut p)?),
            6 => CoverRefused::Cocycle(key32(b, &mut p)?, key32(b, &mut p)?),
            7 => CoverRefused::Untethered,
            8 => CoverRefused::Disconnects,
            9 => CoverRefused::Motion,
            10 => CoverRefused::Claimed,
            11 => CoverRefused::LineRefused,
            12 => CoverRefused::Genesis,
            _ => return None,
        }),
        _ => return None,
    };
    (p == b.len()).then_some(c)
}
