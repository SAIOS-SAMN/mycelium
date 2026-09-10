// ACT_PROBE — a tentative sentence against an open listing.
//
//! The demand and the routed demand that were spelled here carried a
//! coboundary chain to a court (law 8, ruled 2026-09-10); both are deleted
//! with their acts, and the door refuses their tags.

use super::*;

pub fn encode_probe(voice: &[u8; 32], target: impl AsRef<[u8]>, cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_PROBE];
    out.extend_from_slice(voice);
    let _ = put_addr(&mut out, target);
    push_cites(&mut out, cites);
    out
}

pub fn decode_probe(body: &[u8]) -> Option<([u8; 32], Address, Vec<CiteWire>)> {
    if body.first() != Some(&ACT_PROBE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let target = take_addr(body, &mut p)?;
    let cites = read_cites(body, &mut p)?;
    Some((voice, target, cites))
}

pub fn encode_probe_reply(code: u8, paid: &[u64; 4], remaining: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![code];
    push_mu(&mut out, paid);
    push_mu(&mut out, remaining);
    out
}

pub fn decode_probe_reply(body: &[u8]) -> Option<(u8, [u64; 4], [u64; 4])> {
    let code = *body.first()?;
    let mut p = 1usize;
    let paid = read_mu(body, &mut p)?;
    let remaining = read_mu(body, &mut p)?;
    Some((code, paid, remaining))
}
