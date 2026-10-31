// 78.92 (2) — ACT_WHY: ask a court why one act on its ledger stands — the
// words the act names, each with the reasons read off the word itself.
//
//! The ask locates the act by its coordinates: a place `K.i` on the court's
//! own line, or a stamp on any signer's line it holds. The answer is the
//! act — its stamp, kind and frame — and every word it names that the court
//! holds, each with the stamp that admitted it, the head of the task it was
//! spoken for, and its [`Reasons`] (`None` for a word that is not a
//! sentence). A word the act names and the court no longer holds is counted,
//! never dropped silently. Heads are for the console to name things by,
//! never keys: the word's address stays the court's.

use super::*;
use crate::chronometry::{ActKind, Place, Stamp};
use crate::transducer::sentence::Reasons;

/// WHICH ACT IS ASKED ABOUT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhyAsk {
    /// A place on the asked court's own line.
    Own(Place),
    /// An act at a stamp, on any line the court holds.
    Of(Stamp),
}

/// ONE WORD THE ACT NAMES, as the court holds it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Named {
    pub head: [u8; 8],
    pub admitted: Stamp,
    pub task: Option<[u8; 8]>,
    pub reasons: Option<Reasons>,
}

/// 78.93 B0 — ONE REFUSAL THE ACT RECORDS: its kind
/// (`RefusalKind::byte`) and the head of the refused sentence's chart.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Refusal {
    pub kind: u8,
    pub chart: [u8; 8],
}

/// WHY AN ACT STANDS: the act, the words it names, the named words this
/// court no longer holds, and the refusals it records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Why {
    pub at: Stamp,
    pub kind: ActKind,
    pub mu: [u64; 4],
    pub named: Vec<Named>,
    pub unheld: u64,
    pub refused: Vec<Refusal>,
}

/// The first eight bytes of an address, for naming it on a console.
pub fn head_of(a: &[u8]) -> [u8; 8] {
    let mut h = [0u8; 8];
    for (d, s) in h.iter_mut().zip(a) {
        *d = *s;
    }
    h
}

pub fn encode_why(ask: WhyAsk) -> Vec<u8> {
    let mut out = vec![ACT_WHY];
    match ask {
        WhyAsk::Own(p) => {
            out.push(0);
            p.put_canonical(&mut out);
        }
        WhyAsk::Of(s) => {
            out.push(1);
            s.put_canonical(&mut out);
        }
    }
    out
}

/// The ask, read whole or not at all.
pub fn decode_why(body: &[u8]) -> Option<WhyAsk> {
    if !act_is(body, ACT_WHY) {
        return None;
    }
    let mut p = 2usize;
    let ask = match *body.get(1)? {
        0 => WhyAsk::Own(Place::take_canonical(body, &mut p)?),
        1 => WhyAsk::Of(Stamp::take_canonical(body, &mut p)?),
        _ => return None,
    };
    (p == body.len()).then_some(ask)
}

fn put_u64s(out: &mut Vec<u8>, xs: &[u64]) {
    for x in xs {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(x)
}

fn take_flag(b: &[u8], p: &mut usize) -> Option<bool> {
    let x = *b.get(*p)?;
    *p += 1;
    match x {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

fn take_head(b: &[u8], p: &mut usize) -> Option<[u8; 8]> {
    let h: [u8; 8] = b.get(*p..*p + 8)?.try_into().ok()?;
    *p += 8;
    Some(h)
}

/// `[ACT_WHY][0]` where the court holds no act there; otherwise `[ACT_WHY][1]`,
/// the act's stamp, kind and frame, the unheld count, and each named word.
pub fn encode_why_reply(why: Option<&Why>) -> Vec<u8> {
    let mut out = vec![ACT_WHY];
    let Some(w) = why else {
        out.push(0);
        return out;
    };
    out.push(1);
    w.at.put_canonical(&mut out);
    out.push(w.kind.byte());
    put_u64s(&mut out, &w.mu);
    put_u64s(&mut out, &[w.unheld]);
    crate::transducer::address::put_key_count(&mut out, w.named.len());
    for n in &w.named {
        out.extend_from_slice(&n.head);
        n.admitted.put_canonical(&mut out);
        match &n.task {
            None => out.push(0),
            Some(t) => {
                out.push(1);
                out.extend_from_slice(t);
            }
        }
        match &n.reasons {
            None => out.push(0),
            Some(r) => {
                out.push(1);
                r.put(&mut out);
            }
        }
    }
    crate::transducer::address::put_key_count(&mut out, w.refused.len());
    for r in &w.refused {
        out.push(r.kind);
        out.extend_from_slice(&r.chart);
    }
    out
}

/// The reply, read whole: `Some(None)` where the court holds no act at the
/// coordinates asked, `None` where the bytes do not read.
pub fn decode_why_reply(body: &[u8]) -> Option<Option<Why>> {
    if !act_is(body, ACT_WHY) {
        return None;
    }
    let mut p = 1usize;
    if !take_flag(body, &mut p)? {
        return (p == body.len()).then_some(None);
    }
    let at = Stamp::take_canonical(body, &mut p)?;
    let kind = ActKind::of_sealed(*body.get(p)?)?;
    p += 1;
    let mu = [take_u64(body, &mut p)?, take_u64(body, &mut p)?, take_u64(body, &mut p)?, take_u64(body, &mut p)?];
    let unheld = take_u64(body, &mut p)?;
    let mut named = Vec::new();
    for _ in 0..crate::transducer::address::take_key_count(body, &mut p)? {
        let head = take_head(body, &mut p)?;
        let admitted = Stamp::take_canonical(body, &mut p)?;
        let task = match take_flag(body, &mut p)? {
            true => Some(take_head(body, &mut p)?),
            false => None,
        };
        let reasons = match take_flag(body, &mut p)? {
            true => Some(Reasons::take(body, &mut p)?),
            false => None,
        };
        named.push(Named { head, admitted, task, reasons });
    }
    let mut refused = Vec::new();
    for _ in 0..crate::transducer::address::take_key_count(body, &mut p)? {
        let kind = *body.get(p)?;
        p += 1;
        refused.push(Refusal { kind, chart: take_head(body, &mut p)? });
    }
    (p == body.len()).then_some(Some(Why { at, kind, mu, named, unheld, refused }))
}
