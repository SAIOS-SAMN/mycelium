// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.154 (4), 78.152 TU-6, issue #159 (ruled 2026-11-06) — RETRIEVE: a
// COORDINATE RETRIEVAL, `ACT_DOMAIN` sub-kind 19. It reads a domain's ledger
// at a final point's tick, priced at the court's read price and drawn from
// the staker's stake as a claim. The draw is the act's price, never a wallet
// act.
//
//! The request names the staker, the point retrieved, the final point the
//! staker cites, and a nonce, all under the staker's signature. PINPOINT,
//! UNPACK and UNROLL stay in process and are served only through this priced
//! retrieval: a free read invites flooding. Any staker may ask, a mind or a
//! kernel (`Session::retrieve`).

use super::atlas::{byte, head, key32};
use crate::chronometry::tuniversal::TPoint;
use crate::chronometry::KStepReceipt;
use crate::economy::Mu;

/// FORMAT: sub-kind byte of ACT_DOMAIN for a coordinate retrieval.
pub const KIND_RETRIEVE: u8 = 19;

const RETRIEVE: &[u8] = b"mycelium retrieve v1";

/// THE RETRIEVAL: the domain whose ledger is read, the staker who pays, the
/// point retrieved, the final point the staker cites, a nonce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Retrieve {
    pub domain: [u8; 32],
    pub staker: [u8; 32],
    pub point: TPoint,
    pub cited: TPoint,
    pub nonce: [u8; 32],
}

impl Retrieve {
    /// What the staker signs: `"mycelium retrieve v1" ‖ domain ‖ staker ‖
    /// point ‖ cited ‖ nonce`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = RETRIEVE.to_vec();
        out.extend_from_slice(&self.domain);
        out.extend_from_slice(&self.staker);
        self.point.put(&mut out);
        self.cited.put(&mut out);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedRetrieve {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedRetrieve { retrieve: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedRetrieve {
    pub retrieve: Retrieve,
    pub sig: [u8; 64],
}

impl SignedRetrieve {
    /// Whether the staker's signature verifies over the request.
    pub fn verifies(&self) -> bool {
        use ed25519_dalek::Verifier;
        let Ok(vk) = ed25519_dalek::VerifyingKey::from_bytes(&self.retrieve.staker) else { return false };
        vk.verify(&self.retrieve.signed_bytes(), &ed25519_dalek::Signature::from_bytes(&self.sig)).is_ok()
    }
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][19][staker 32][point][cited][nonce
/// 32][sig 64]`, each point in its wire writing (`TPoint::put`).
pub fn encode_retrieve(s: &SignedRetrieve) -> Vec<u8> {
    let r = &s.retrieve;
    let mut out = head(&r.domain, KIND_RETRIEVE);
    out.extend_from_slice(&r.staker);
    r.point.put(&mut out);
    r.cited.put(&mut out);
    out.extend_from_slice(&r.nonce);
    out.extend_from_slice(&s.sig);
    out
}

pub(crate) fn take_retrieve(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedRetrieve> {
    let staker = key32(b, p)?;
    let (point, cited) = (TPoint::take(b, p)?, TPoint::take(b, p)?);
    let nonce = key32(b, p)?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    Some(SignedRetrieve { retrieve: Retrieve { domain, staker, point, cited, nonce }, sig })
}

/// WHY A RETRIEVAL WAS NOT SERVED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetrieveRefusal {
    /// 1 — the staker's signature does not verify.
    Unsigned,
    /// 2 — the domain is not founded on this court.
    UnknownDomain,
    /// 3 — the point or the cited point is not final in this court's view.
    NotFinal,
    /// 4 — the point and the cited point stand at different origins, or the
    /// point stands above the point cited.
    Origins,
    /// 5 — the tick is folded on this court: it is read through
    /// `ACT_RECEIPTS`'s replay arm.
    Folded,
    /// 6 — the stake does not cover the read cost on a lane.
    Short,
    /// 7 — a cost passes its carrier, or the claim's id is drawn already.
    Overflow,
}

impl RetrieveRefusal {
    fn code(self) -> u8 {
        match self {
            RetrieveRefusal::Unsigned => 1,
            RetrieveRefusal::UnknownDomain => 2,
            RetrieveRefusal::NotFinal => 3,
            RetrieveRefusal::Origins => 4,
            RetrieveRefusal::Folded => 5,
            RetrieveRefusal::Short => 6,
            RetrieveRefusal::Overflow => 7,
        }
    }

    fn of(code: u8) -> Option<RetrieveRefusal> {
        Some(match code {
            1 => RetrieveRefusal::Unsigned,
            2 => RetrieveRefusal::UnknownDomain,
            3 => RetrieveRefusal::NotFinal,
            4 => RetrieveRefusal::Origins,
            5 => RetrieveRefusal::Folded,
            6 => RetrieveRefusal::Short,
            7 => RetrieveRefusal::Overflow,
            _ => return None,
        })
    }
}

/// WHAT THE COURT ANSWERED a retrieval: the fee claimed on the stake, exact
/// per lane, and the receipts held at the tick, each carrying its stamp; or
/// the refusal by name. The court's own signed position is written beside
/// it by the door ([`RetrieveReply`]).
#[derive(Clone, Debug)]
pub enum Retrieved {
    Served { fee: Mu, receipts: Vec<KStepReceipt> },
    Refused(RetrieveRefusal),
}

/// THE REPLY AS THE RETRIEVER READS IT: on success the receipts beside the
/// court's 47.1 section over its own head at the answer, whose payload is
/// [`answered_payload`]. A priced read the caller cannot verify, or present
/// as an equivocation where it is false, is not accountable.
#[derive(Clone)]
pub enum RetrieveReply {
    Served { fee: Mu, placed: Vec<Placed>, section: Box<super::ReceiptSection> },
    Refused(RetrieveRefusal),
}

/// A RECEIPT RETRIEVED WITH ITS ANCESTRY TO THE SIGNED HEAD (#159, ruled
/// 2026-11-06): the receipts on one path from it up to the head, each
/// written out whole, in order. The first names the receipt among its
/// predecessor and parents, each names the one before it, and the head
/// names the last. A receipt that is the head itself has an empty path.
/// Height alone would let a court return a receipt at a lower height that
/// is not in its chain.
#[derive(Clone, Debug)]
pub struct Placed {
    pub receipt: KStepReceipt,
    pub path: Vec<KStepReceipt>,
}

const ANSWERED: &[u8] = b"mycelium retrieved v1";

/// The payload the court's section signs over: `"mycelium retrieved v1" ‖`
/// the request's signed bytes, written out. The section binds the answer to
/// the request it answers.
pub fn answered_payload(r: &Retrieve) -> Vec<u8> {
    let mut out = ANSWERED.to_vec();
    out.extend_from_slice(&r.signed_bytes());
    out
}

fn put_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u32::try_from(n).ok()?.to_le_bytes());
    Some(())
}

fn take_count(b: &[u8], p: &mut usize) -> Option<u32> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(n)
}

/// `[1][fee μ⃗][count u32]([receipt][path count u32][path receipt …]) …
/// [section]` (each receipt in its own writing, `KStepReceipt::write`; the
/// section as `encode_receipt_section` writes it) or `[0][code]`. `paths`
/// holds one path per receipt served, in order. `None` where a receipt or
/// the section cannot be written, a count passes its carrier, or a served
/// answer has no section or not one path per receipt: a reply is never cut
/// short and never unsigned.
pub fn encode_retrieve_reply(r: &Retrieved, answer: Option<(&super::ReceiptSection, &[Vec<KStepReceipt>])>) -> Option<Vec<u8>> {
    match r {
        Retrieved::Refused(why) => Some(vec![0, why.code()]),
        Retrieved::Served { fee, receipts } => {
            let (s, paths) = answer?;
            if paths.len() != receipts.len() {
                return None;
            }
            let mut out = vec![1];
            fee.put(&mut out);
            put_count(&mut out, receipts.len())?;
            for (x, path) in receipts.iter().zip(paths) {
                x.write(&mut out)?;
                put_count(&mut out, path.len())?;
                path.iter().try_for_each(|y| y.write(&mut out))?;
            }
            out.extend_from_slice(&super::encode_receipt_section(&s.prefix, &s.sig, &s.receipt)?);
            Some(out)
        }
    }
}

pub fn decode_retrieve_reply(b: &[u8]) -> Option<RetrieveReply> {
    let mut p = 0;
    match byte(b, &mut p)? {
        0 => {
            let why = RetrieveRefusal::of(byte(b, &mut p)?)?;
            (p == b.len()).then_some(RetrieveReply::Refused(why))
        }
        1 => {
            let fee = Mu::take_from(b, &mut p)?;
            let placed = (0..take_count(b, &mut p)?)
                .map(|_| {
                    let receipt = KStepReceipt::read(b, &mut p)?;
                    let path = (0..take_count(b, &mut p)?).map(|_| KStepReceipt::read(b, &mut p)).collect::<Option<Vec<_>>>()?;
                    Some(Placed { receipt, path })
                })
                .collect::<Option<Vec<_>>>()?;
            let section = super::decode_receipt_section(b, p)?;
            let end = super::receipt_section_end(b, p)?;
            (end == b.len()).then_some(RetrieveReply::Served { fee, placed, section: Box::new(section) })
        }
        _ => None,
    }
}

/// WHY A SERVED RETRIEVAL DOES NOT STAND for the retriever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetrievalBreach {
    /// The section's signature is not the court's over its prefix.
    Unsigned,
    /// The section's prefix and receipt are not one claim: the place, the
    /// height or the predecessor it commits to is not its receipt's.
    NotItsHead,
    /// The section signs over another request than the one asked.
    OtherRequest,
    /// A receipt does not re-derive its own hash: forged.
    Forged { at: crate::chronometry::Stamp },
    /// A receipt stands at another tick than the point asked.
    OffTick { at: crate::chronometry::Stamp },
    /// A receipt's path does not lead from it to the court's signed head: a
    /// step names no predecessor or parent of the step before, or the head
    /// does not name the last. The receipt is off the head's chain.
    OffChain { at: crate::chronometry::Stamp },
}

/// Whether `x` names `below` as its predecessor or among its parents, by
/// stamp and chain hash.
fn names(x: &KStepReceipt, below: &crate::chronometry::Parent) -> bool {
    x.prev.iter().chain(&x.parents).any(|p| crate::court::admission::Predecessor::cites(p, below))
}

/// THE RETRIEVER'S CHECK of a served answer, against the request it sent
/// and the court's key: the section verifies under `court` and is one claim
/// with its head receipt, it signs over this request, and every receipt
/// re-derives, stands at the point's tick, and its path leads to the head:
/// every step re-derives and names the one below it, and the head names the
/// last. A receipt of the right height off the head's chain is refused
/// `OffChain`.
pub fn verify_retrieved(request: &Retrieve, court: &[u8; 32], placed: &[Placed], section: &super::ReceiptSection) -> Result<(), RetrievalBreach> {
    use crate::chronometry::Parent;
    use crate::court::admission::{verify_signature, Commitment, Predecessor, Reconstructed};
    let p = &section.prefix;
    verify_signature(p, court, &section.sig).map_err(|_| RetrievalBreach::Unsigned)?;
    let head = Reconstructed::receipt(&section.receipt).ok_or(RetrievalBreach::NotItsHead)?;
    if p.at != head.object().stamp.place() || p.height != head.object().stamp.h || !Commitment::signed(p).binds(&head) {
        return Err(RetrievalBreach::NotItsHead);
    }
    if p.payload != answered_payload(request) {
        return Err(RetrievalBreach::OtherRequest);
    }
    let head = head.object();
    for x in placed {
        let r = &x.receipt;
        let at = r.stamp;
        if Reconstructed::receipt(r).is_none() {
            return Err(RetrievalBreach::Forged { at });
        }
        if at.h != request.point.h {
            return Err(RetrievalBreach::OffTick { at });
        }
        let mut below = Parent::of(r);
        for step in &x.path {
            if Reconstructed::receipt(step).is_none() {
                return Err(RetrievalBreach::Forged { at: step.stamp });
            }
            if !names(step, &below) {
                return Err(RetrievalBreach::OffChain { at });
            }
            below = Parent::of(step);
        }
        let reached = names(head, &below) || (x.path.is_empty() && Predecessor::cites(&Parent::of(head), &below));
        if !reached {
            return Err(RetrievalBreach::OffChain { at });
        }
    }
    Ok(())
}
