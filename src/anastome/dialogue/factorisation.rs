// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §3.9, §6.2; ROADMAP track 26 S4c — A CLAIM OF FACTORISATION ON
// THE WIRE: a mind states, for one train pair of a task, the motion that
// carries the pair (a signed permutation of the phases, and the enclosure
// the moved frame is re-periodded onto) and the residue it leaves (links of
// the pair's own rank and width), with the ground the claim was spoken
// under, signed. The court reconstructs the motion on its copy of the posted
// samples, derives the residue itself, and answers the verdict.
//
//! The act:
//!
//! `[ACT_FACTORISATION][voice 32][task citation 32][pair][rank u16][(source
//! u16)(sense u8) · rank][period u16 · rank][link count][link · count]`,
//! then optionally `[GRND][len u32][symbol]`, then `[SIGN][sig 64]`.
//!
//! - `pair` and `link count` are counts written by their own coordinate
//!   (LEB128, minimal); `pair` is the train pair's index in the family.
//! - Output phase `j` of the motion reads source phase `source` in `sense`
//!   (`0` kept, `1` reversed), so the signed permutation is written whole
//!   and exactly; a writing that is no signed permutation is refused at the
//!   reader (`SignedPermutation::new`).
//! - The re-period is written as the periods of the enclosure the moved
//!   frame is taken onto, each in the `u16` field an extent stands in; the
//!   input's periods are the posted pair's own, so each phase's re-period
//!   `p′/p` is exact from the two.
//! - Each link is a site of rank `n` and its two symbols
//!   (`sentence::Link::put`).
//! - The voice signs the domain `mycelium factorisation v1` and the act up
//!   to its signature.
//!
//! The reply is the verdict: admitted, the residue of the residues
//! (`unclaimed`, `uncarried`), or a refusal by name. An admitted claim pays
//! nothing on this wire (MU-SENSING §3.9: μ is the traversal's cost, and a
//! factorisation of a train pair discharges no withheld pair).

use super::{act_is, ACT_FACTORISATION};
use crate::transducer::address::{put_bytes, put_key_bytes, put_key_count, put_u16_count, take_bytes, take_key_bytes, take_key_count, take_u16_count};
use crate::transducer::family::transduce::{Enclosure, Motion, Sense, SignedPermutation};
use crate::transducer::sentence::Link;

/// The signed bytes' domain of a claim of factorisation.
const SIGNED_FACTORISATION: &[u8] = b"mycelium factorisation v1";

/// A CLAIM OF FACTORISATION AS THE COURT READS IT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorisationAct {
    /// The speaking voice, its key's verifying half.
    pub voice: [u8; 32],
    /// The task's citation.
    pub task: [u8; 32],
    /// The train pair's index in the family.
    pub pair: usize,
    /// The motion stated.
    pub motion: Motion,
    /// The residue stated: links of the pair's rank and width.
    pub residue: Vec<Link>,
    /// The ground the claim was spoken under (MU-SENSING §6.2): one symbol
    /// at the source's width, or none.
    pub ground: Option<Vec<u8>>,
    /// Whether the voice signed the act.
    pub signed: bool,
}

/// The sense's byte.
fn sense_byte(s: Sense) -> u8 {
    match s {
        Sense::Kept => 0,
        Sense::Reversed => 1,
    }
}

/// The sense a byte names, or none.
fn sense_of(b: u8) -> Option<Sense> {
    match b {
        0 => Some(Sense::Kept),
        1 => Some(Sense::Reversed),
        _ => None,
    }
}

/// THE MOTION WRITTEN: its rank, each output phase's source and sense, then
/// the periods it is taken onto. `None` where a rank, a source or a period
/// passes its `u16` field, or where the permutation and the enclosure differ
/// in rank: nothing is written short.
pub fn put_motion(out: &mut Vec<u8>, m: &Motion) -> Option<()> {
    let rank = m.permutation.rank();
    if m.onto.rank() != rank {
        return None;
    }
    put_u16_count(out, rank)?;
    for &(source, sense) in m.permutation.images() {
        put_u16_count(out, source)?;
        out.push(sense_byte(sense));
    }
    for &period in m.onto.periods() {
        put_u16_count(out, period)?;
    }
    Some(())
}

/// The motion read back, or `None` where it is no signed permutation or no
/// enclosure.
fn take_motion(b: &[u8], p: &mut usize) -> Option<Motion> {
    let rank = take_u16_count(b, p)?;
    // Each image and each period takes bytes: a rank past the bytes that
    // remain is refused before anything is allocated for it.
    if rank > b.len().checked_sub(*p)? {
        return None;
    }
    let mut images = Vec::with_capacity(rank);
    for _ in 0..rank {
        let source = take_u16_count(b, p)?;
        let sense = sense_of(*b.get(*p)?)?;
        *p += 1;
        images.push((source, sense));
    }
    let mut periods = Vec::with_capacity(rank);
    for _ in 0..rank {
        periods.push(take_u16_count(b, p)?);
    }
    let permutation = SignedPermutation::new(images).ok()?;
    let onto = Enclosure::new(periods).ok()?;
    Some(Motion { permutation, onto })
}

/// Links written: their count by its own coordinate, then each.
fn put_links(out: &mut Vec<u8>, links: &[Link]) {
    put_key_count(out, links.len());
    for l in links {
        l.put(out);
    }
}

/// Links read back: each read from the bytes that hold it, so a count the
/// bytes do not carry ends the read with `None` before anything past them
/// is allocated.
fn take_links(b: &[u8], p: &mut usize) -> Option<Vec<Link>> {
    let n = take_key_count(b, p)?;
    let mut out = Vec::new();
    for _ in 0..n {
        out.push(Link::take(b, p)?);
    }
    Some(out)
}

/// The bytes a claim's voice signs: the domain, then the act up to its
/// signature.
fn signed_bytes(unsigned: &[u8]) -> Vec<u8> {
    let mut b = SIGNED_FACTORISATION.to_vec();
    b.extend_from_slice(unsigned);
    b
}

/// A CLAIM OF FACTORISATION WRITTEN AND SIGNED by `key`: the train pair
/// `pair` of the task `citation`, its `motion` and `residue`, and the
/// `ground` it was spoken under. `None` where a field passes its field's
/// width: a claim is never sent short.
pub fn encode_factorisation(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    pair: usize,
    motion: &Motion,
    residue: &[Link],
    ground: Option<&[u8]>,
) -> Option<Vec<u8>> {
    use ed25519_dalek::Signer;
    let mut out = vec![ACT_FACTORISATION];
    out.extend_from_slice(&key.verifying_key().to_bytes());
    out.extend_from_slice(citation);
    put_key_count(&mut out, pair);
    put_motion(&mut out, motion)?;
    put_links(&mut out, residue);
    if let Some(g) = ground {
        out.extend_from_slice(b"GRND");
        put_bytes(&mut out, g)?;
    }
    let sig = key.sign(&signed_bytes(&out)).to_bytes();
    out.extend_from_slice(b"SIGN");
    out.extend_from_slice(&sig);
    Some(out)
}

/// A claim of factorisation read back whole: `None` where it does not
/// decode, or where bytes stand after it that are no signature; `signed`
/// whether its voice signed it. A stated ground is one symbol, never empty.
pub fn decode_factorisation(body: &[u8]) -> Option<FactorisationAct> {
    if !act_is(body, ACT_FACTORISATION) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let task: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let pair = take_key_count(body, &mut p)?;
    let motion = take_motion(body, &mut p)?;
    let residue = take_links(body, &mut p)?;
    let ground = match body.get(p..p + 4) == Some(&b"GRND"[..]) {
        true => {
            p += 4;
            let g = take_bytes(body, &mut p)?;
            if g.is_empty() {
                return None;
            }
            Some(g)
        }
        false => None,
    };
    let signed = match body.get(p..) {
        Some([]) => false,
        Some([b'S', b'I', b'G', b'N', sig @ ..]) => {
            let sig: [u8; 64] = sig.try_into().ok()?;
            crate::court::identity::verify(&voice, &signed_bytes(body.get(..p)?), &sig)
        }
        _ => return None,
    };
    Some(FactorisationAct { voice, task, pair, motion, residue, ground, signed })
}

/// THE COURT'S VERDICT ON A CLAIM OF FACTORISATION.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FactorVerdict {
    /// The court reconstructed the motion and derived the residue claimed.
    Admitted,
    /// The residue the court derived is not the residue claimed: the links
    /// the court derived and the claim left out (`unclaimed`), and the links
    /// the claim stated and the motion does not leave (`uncarried`).
    Residue { unclaimed: Vec<Link>, uncarried: Vec<Link> },
    /// Refused by name, before or at the reconstruction: unsigned, no such
    /// task or pair, a stated ground the court does not read, or a motion
    /// that is no motion of the pair.
    Refused { reason: Vec<u8> },
}

/// The verdict written: `[0]` admitted; `[1][unclaimed][uncarried]` the
/// residue of the residues, each as links; `[2][reason]` a refusal.
pub fn encode_factor_verdict(v: &FactorVerdict) -> Vec<u8> {
    let mut out = Vec::new();
    match v {
        FactorVerdict::Admitted => out.push(0),
        FactorVerdict::Residue { unclaimed, uncarried } => {
            out.push(1);
            put_links(&mut out, unclaimed);
            put_links(&mut out, uncarried);
        }
        FactorVerdict::Refused { reason } => {
            out.push(2);
            put_key_bytes(&mut out, reason);
        }
    }
    out
}

/// The verdict read back whole, or `None`.
pub fn decode_factor_verdict(body: &[u8]) -> Option<FactorVerdict> {
    let mut p = 1usize;
    let v = match super::act_tag(body)? {
        0 => FactorVerdict::Admitted,
        1 => {
            let unclaimed = take_links(body, &mut p)?;
            let uncarried = take_links(body, &mut p)?;
            FactorVerdict::Residue { unclaimed, uncarried }
        }
        2 => FactorVerdict::Refused { reason: take_key_bytes(body, &mut p)? },
        _ => return None,
    };
    (p == body.len()).then_some(v)
}
