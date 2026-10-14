// 21 step 3, 47.3 — THE CITATION ASKED BY ADDRESS: a sub-kind of ACT_FETCH.
// A court holding a sentence that waits on a word it has not ruled asks a peer
// court for exactly that word; the peer answers with it if its language holds
// it. Courts exchange what a citation needs, never a stratum's whole lack
// (2026-10-14: the bulk page pull is retired).
//
//! Request: `[ACT_FETCH][voice 32][0xFFFF_FFFE][n u16]{[addr]}·n` — the
//! marker stands where a fetch writes its chart count, a count no fetch can
//! carry, and is not the presentation's (`0xFFFF_FFFF`), so none of the three
//! reads as another. Addresses are `put_addr`'s, read whole.
//!
//! Reply: the syllabus page shape (`encode_syllabus_reply`): each asked word
//! the peer installed, at the stamp that admitted it; a word it does not hold
//! is absent, never a placeholder.

use super::*;

/// The marker a citation ask writes where a fetch writes its count.
const CITED: u32 = u32::MAX - 1;

/// `None` where a count or an address passes the field that carries it
/// (STD1 3c): the ask is refused whole, never framed short.
pub fn encode_cited_request(voice: &[u8; 32], addrs: &[Address]) -> Option<Vec<u8>> {
    let mut out = vec![ACT_FETCH];
    out.extend_from_slice(voice);
    out.extend_from_slice(&CITED.to_le_bytes());
    crate::transducer::address::put_u16_count(&mut out, addrs.len())?;
    for a in addrs {
        put_addr(&mut out, a)?;
    }
    Some(out)
}

/// The asker and the addresses it asks, read whole; `None` for any other
/// body, a fetch and a presentation ask included.
pub fn decode_cited_request(body: &[u8]) -> Option<([u8; 32], Vec<Address>)> {
    if !act_is(body, ACT_FETCH) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    if body.get(33..37)? != CITED.to_le_bytes().as_slice() {
        return None;
    }
    let n = usize::from(u16::from_le_bytes(body.get(37..39)?.try_into().ok()?));
    let mut p = 39usize;
    // Each address is at least its own length field: a count past the body
    // allocates nothing (rule 10).
    if n > body.len().checked_sub(p)? / 2 {
        return None;
    }
    let mut addrs = Vec::with_capacity(n);
    for _ in 0..n {
        addrs.push(take_addr(body, &mut p)?);
    }
    (p == body.len()).then_some((voice, addrs))
}
