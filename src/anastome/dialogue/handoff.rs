// 78.126 — ACT_HANDOFF: matter, or a delivered word, crossing from one
// domain to another on one court (2(b), the coordinator's rulings
// 2026-10-07; 78.59 P2: matter does not net like money).
//
//! A unit of matter stands in exactly one place, so a crossing between
//! domains is no rate on an edge: it is a HANDOFF one court serialises, two
//! sides each consumed exactly once.
//!
//! - OFFER, signed by the sending tier: the nets leaving its domain (its
//!   identities, negative) and arriving in the other (the receiver's
//!   identities, positive), summing to zero on every lane, on S (matter)
//!   and H (a delivered word) only — G crosses by rates and μ, never by
//!   handoff, and T is reserved for contracts. Sealed, the matter is IN
//!   FLIGHT. The offer carries the sender's charter commitment (zero until
//!   charters exist), signed, unchecked.
//! - ACCEPT, by the receiving tier, whole or not at all, within `W` of the
//!   offer; REFUSE by the receiver; EXPIRY claimed by the sender past `W`.
//!   Refused or expired, the handoff is RETURNED.
//! - A fold consumes a SIDE: OUT by the sender, in any state after the
//!   offer; IN by the receiver once accepted, or by the sender once
//!   returned. Each side once; a second is a double-spend, refused.

use super::uplink::{put_nets, take_nets, Net};
use crate::chronometry::Stamp;

/// The signed bytes' domain.
const SIGNED: &[u8] = b"mycelium handoff v1";

/// THE LANES A HANDOFF MAY CARRY: S (matter) and H (a delivered word).
pub const CARRIED: [bool; 4] = [true, false, false, true];

/// WHICH SIDE OF A HANDOFF A FOLD CONSUMES: OUT — the sender's, its nets
/// leaving; IN — the receiver's once accepted (its nets arriving), or the
/// sender's once returned (what left, come back).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Out,
    In,
}

impl Side {
    pub fn byte(self) -> u8 {
        match self {
            Side::Out => 0,
            Side::In => 1,
        }
    }

    pub fn of(b: u8) -> Option<Side> {
        match b {
            0 => Some(Side::Out),
            1 => Some(Side::In),
            _ => None,
        }
    }
}

/// A HANDOFF OFFERED: who sends, who receives, what leaves and what arrives,
/// and the sender's charter commitment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub from_tier: [u8; 32],
    pub to_tier: [u8; 32],
    pub out_nets: Vec<Net>,
    pub in_nets: Vec<Net>,
    pub charter: [u8; 32],
    /// The sender's nonce, so two like offers are two handoffs.
    pub nonce: [u8; 32],
}

/// WHY AN OFFER IS NOT A HANDOFF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unbalanced {
    /// A list not in its identities' strictly ascending order.
    Unordered,
    /// A lane a handoff does not carry is nonzero (G or T).
    Lane { lane: usize },
    /// Out and in do not sum to zero on this lane: matter made or lost.
    Unconserved { lane: usize },
    /// Out holds a positive net or in a negative one: a side running backward.
    Backward { lane: usize },
    /// A sum passed its carrier.
    Overflow { lane: usize },
}

fn ascending(nets: &[Net]) -> bool {
    nets.windows(2).all(|w| matches!(w, [a, b] if a.0 < b.0))
}

impl Offer {
    /// The offer's one writing.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.from_tier);
        out.extend_from_slice(&self.to_tier);
        put_nets(&mut out, &self.out_nets);
        put_nets(&mut out, &self.in_nets);
        out.extend_from_slice(&self.charter);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Offer> {
        let key = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        };
        let (from_tier, to_tier) = (key(b, p)?, key(b, p)?);
        let (out_nets, in_nets) = (take_nets(b, p)?, take_nets(b, p)?);
        Some(Offer { from_tier, to_tier, out_nets, in_nets, charter: key(b, p)?, nonce: key(b, p)? })
    }

    /// THE HANDOFF'S ID: the hash of its one writing.
    pub fn id(&self) -> [u8; 32] {
        crate::chronometry::payload_hash(&self.bytes())
    }

    /// CONSERVATION across the boundary: each list ascending, only S and H
    /// carried, out running out and in running in, and on every lane out
    /// and in summing to exactly zero.
    pub fn conserves(&self) -> Result<(), Unbalanced> {
        if !ascending(&self.out_nets) || !ascending(&self.in_nets) {
            return Err(Unbalanced::Unordered);
        }
        let mut sum = [0i128; 4];
        for (nets, sign) in [(&self.out_nets, -1i128), (&self.in_nets, 1)] {
            for (_, v) in nets {
                for (l, x) in v.iter().enumerate() {
                    if !CARRIED[l] && *x != 0 {
                        return Err(Unbalanced::Lane { lane: l });
                    }
                    if x.signum() == -sign {
                        return Err(Unbalanced::Backward { lane: l });
                    }
                    sum[l] = sum[l].checked_add(*x).ok_or(Unbalanced::Overflow { lane: l })?;
                }
            }
        }
        match (0..4).find(|&l| sum[l] != 0) {
            Some(lane) => Err(Unbalanced::Unconserved { lane }),
            None => Ok(()),
        }
    }
}

/// WHAT IS ASKED OF A HANDOFF.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Handoff {
    Offer(Box<Offer>),
    /// The receiver takes it whole.
    Accept { id: [u8; 32] },
    /// The receiver refuses it: returned.
    Refuse { id: [u8; 32] },
    /// The sender claims it returned, its window passed unanswered.
    Expire { id: [u8; 32] },
}

impl Handoff {
    fn kind(&self) -> u8 {
        match self {
            Handoff::Offer(_) => 0,
            Handoff::Accept { .. } => 1,
            Handoff::Refuse { .. } => 2,
            Handoff::Expire { .. } => 3,
        }
    }

    fn put_body(&self, out: &mut Vec<u8>) {
        match self {
            Handoff::Offer(o) => out.extend_from_slice(&o.bytes()),
            Handoff::Accept { id } | Handoff::Refuse { id } | Handoff::Expire { id } => out.extend_from_slice(id),
        }
    }

    /// The handoff it names.
    pub fn id(&self) -> [u8; 32] {
        match self {
            Handoff::Offer(o) => o.id(),
            Handoff::Accept { id } | Handoff::Refuse { id } | Handoff::Expire { id } => *id,
        }
    }

    /// The bytes the filer signs.
    pub fn signed_bytes(&self, who: &[u8; 32]) -> Vec<u8> {
        let mut out = SIGNED.to_vec();
        out.extend_from_slice(who);
        out.push(self.kind());
        self.put_body(&mut out);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedHandoff {
        use ed25519_dalek::Signer;
        let who = key.verifying_key().to_bytes();
        let sig = key.sign(&self.signed_bytes(&who)).to_bytes();
        SignedHandoff { who, handoff: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedHandoff {
    pub who: [u8; 32],
    pub handoff: Handoff,
    pub sig: [u8; 64],
}

/// WHERE A HANDOFF STANDS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Offered, in flight, awaiting the receiver until `until` (a height).
    Offered { until: u64 },
    Accepted,
    Returned,
}

/// WHAT A FRAME ASKS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandoffRequest {
    Signed(SignedHandoff),
    /// A handoff's standing, read; nothing sealed.
    Read([u8; 32]),
}

const KIND_READ: u8 = 4;

/// `[ACT_HANDOFF][kind][who 32][body][sig 64]`.
pub fn encode_handoff(h: &SignedHandoff) -> Vec<u8> {
    let mut out = vec![super::ACT_HANDOFF, h.handoff.kind()];
    out.extend_from_slice(&h.who);
    h.handoff.put_body(&mut out);
    out.extend_from_slice(&h.sig);
    out
}

/// `[ACT_HANDOFF][4][id 32]`.
pub fn encode_handoff_read(id: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_HANDOFF, KIND_READ];
    out.extend_from_slice(&id);
    out
}

pub fn decode_handoff_request(b: &[u8]) -> Option<HandoffRequest> {
    let [act, kind, ..] = b else { return None };
    if *act != super::ACT_HANDOFF {
        return None;
    }
    let mut p = 2;
    let id = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
        let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(k)
    };
    let request = match *kind {
        KIND_READ => HandoffRequest::Read(id(b, &mut p)?),
        k => {
            let who = id(b, &mut p)?;
            let handoff = match k {
                0 => Handoff::Offer(Box::new(Offer::take(b, &mut p)?)),
                1 => Handoff::Accept { id: id(b, &mut p)? },
                2 => Handoff::Refuse { id: id(b, &mut p)? },
                3 => Handoff::Expire { id: id(b, &mut p)? },
                _ => return None,
            };
            let sig: [u8; 64] = b.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            HandoffRequest::Signed(SignedHandoff { who, handoff, sig })
        }
    };
    (p == b.len()).then_some(request)
}

/// WHY A HANDOFF ACT WAS NOT TAKEN.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandoffRefusal {
    Unsigned,
    /// The offer does not conserve, by why.
    Unbalanced(Unbalanced),
    /// The sender (offer, expiry) or receiver (accept, refuse) is not the filer.
    NotTheTier,
    /// A tier no domain on this court names; or both tiers in one domain.
    Unbound,
    /// The same offer sealed already.
    Offered,
    /// No handoff of this id.
    Unknown,
    /// Answered already — accepted or returned.
    Answered,
    /// Accepted or refused past its window; or expiry claimed before it closed.
    Window,
    LineRefused,
}

/// THE COURT'S ANSWER.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Handed {
    /// Sealed: the receipt, the handoff's id and its state now.
    Sealed { at: Stamp, receipt: [u8; 32], id: [u8; 32], state: State },
    Refused(HandoffRefusal),
}

fn put_state(out: &mut Vec<u8>, s: &State) {
    match s {
        State::Offered { until } => {
            out.push(0);
            out.extend_from_slice(&until.to_le_bytes());
        }
        State::Accepted => out.push(1),
        State::Returned => out.push(2),
    }
}

fn take_state(b: &[u8], p: &mut usize) -> Option<State> {
    let tag = *b.get(*p)?;
    *p += 1;
    Some(match tag {
        0 => {
            let until = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            State::Offered { until }
        }
        1 => State::Accepted,
        2 => State::Returned,
        _ => return None,
    })
}

fn refusal_bytes(r: &HandoffRefusal) -> [u8; 2] {
    match r {
        HandoffRefusal::Unsigned => [0, 0],
        HandoffRefusal::Unbalanced(u) => match u {
            Unbalanced::Unordered => [1, 0],
            Unbalanced::Lane { lane } => [2, lane_byte(*lane)],
            Unbalanced::Unconserved { lane } => [3, lane_byte(*lane)],
            Unbalanced::Backward { lane } => [4, lane_byte(*lane)],
            Unbalanced::Overflow { lane } => [5, lane_byte(*lane)],
        },
        HandoffRefusal::NotTheTier => [6, 0],
        HandoffRefusal::Unbound => [7, 0],
        HandoffRefusal::Offered => [8, 0],
        HandoffRefusal::Unknown => [9, 0],
        HandoffRefusal::Answered => [10, 0],
        HandoffRefusal::Window => [11, 0],
        HandoffRefusal::LineRefused => [12, 0],
    }
}

#[expect(
    clippy::manual_unwrap_or,
    reason = "rule 11 — `unwrap_or` is the spelling this refuses; a lane past a byte is named at the wall, never wrapped"
)]
fn lane_byte(lane: usize) -> u8 {
    match u8::try_from(lane) {
        Ok(l) => l,
        Err(_) => u8::MAX,
    }
}

fn refusal_of(code: u8, lane: u8) -> Option<HandoffRefusal> {
    let lane = usize::from(lane);
    Some(match code {
        0 => HandoffRefusal::Unsigned,
        1 => HandoffRefusal::Unbalanced(Unbalanced::Unordered),
        2 => HandoffRefusal::Unbalanced(Unbalanced::Lane { lane }),
        3 => HandoffRefusal::Unbalanced(Unbalanced::Unconserved { lane }),
        4 => HandoffRefusal::Unbalanced(Unbalanced::Backward { lane }),
        5 => HandoffRefusal::Unbalanced(Unbalanced::Overflow { lane }),
        6 => HandoffRefusal::NotTheTier,
        7 => HandoffRefusal::Unbound,
        8 => HandoffRefusal::Offered,
        9 => HandoffRefusal::Unknown,
        10 => HandoffRefusal::Answered,
        11 => HandoffRefusal::Window,
        12 => HandoffRefusal::LineRefused,
        _ => return None,
    })
}

pub fn encode_handoff_reply(h: &Handed) -> Vec<u8> {
    let mut out = Vec::new();
    match h {
        Handed::Sealed { at, receipt, id, state } => {
            out.push(0);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out.extend_from_slice(id);
            put_state(&mut out, state);
        }
        Handed::Refused(r) => {
            out.push(1);
            out.extend_from_slice(&refusal_bytes(r));
        }
    }
    out
}

pub fn decode_handoff_reply(b: &[u8]) -> Option<Handed> {
    let mut p = 1;
    let h = match b.first_chunk::<1>()? {
        [0] => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            let id: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Handed::Sealed { at, receipt, id, state: take_state(b, &mut p)? }
        }
        [1] => {
            let (code, lane) = (*b.get(1)?, *b.get(2)?);
            p = 3;
            Handed::Refused(refusal_of(code, lane)?)
        }
        _ => return None,
    };
    (p == b.len()).then_some(h)
}

/// A handoff's standing as read: `[0]` none, or `[1][state][out consumed
/// u8][in consumed u8]`.
pub fn encode_read_reply(read: Option<(State, bool, bool)>) -> Vec<u8> {
    match read {
        None => vec![0],
        Some((s, out, inn)) => {
            let mut v = vec![1];
            put_state(&mut v, &s);
            v.extend_from_slice(&[u8::from(out), u8::from(inn)]);
            v
        }
    }
}

pub fn decode_read_reply(b: &[u8]) -> Option<Option<(State, bool, bool)>> {
    let mut p = 1;
    let read = match b.first_chunk::<1>()? {
        [0] => None,
        [1] => {
            let s = take_state(b, &mut p)?;
            let flag = |x: u8| match x {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            };
            let (out, inn) = (flag(*b.get(p)?)?, flag(*b.get(p + 1)?)?);
            p += 2;
            Some((s, out, inn))
        }
        _ => return None,
    };
    (p == b.len()).then_some(read)
}
