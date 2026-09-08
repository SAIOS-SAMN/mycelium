// ACT_DEMAND, ACT_DEMAND_ROUTED, ACT_PROBE — an unsolved coboundary put to
// a court, the same demand forwarded once to its home, and a tentative
// sentence against an open listing.

use super::*;

pub fn encode_routed_demand(origin: &[u8; 32], demand: &[u8]) -> Vec<u8> {
    let mut out = vec![ACT_DEMAND_ROUTED];
    out.extend_from_slice(origin);
    out.extend_from_slice(demand);
    out
}

pub fn decode_routed_demand(body: &[u8]) -> Option<([u8; 32], &[u8])> {
    if body.first() != Some(&ACT_DEMAND_ROUTED) || body.len() < 34 {
        return None;
    }
    let origin: [u8; 32] = body[1..33].try_into().ok()?;
    Some((origin, &body[33..]))
}

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

/// `[ACT_DEMAND][voice 32][chain][cites][K u64]` — the speaker's own
/// reduction of the chain over its genome rides as citations; an empty
/// citation list attests the chain irreducible for this voice. K is the
/// voice's receipt index (23.8); a frame that omits it is refused.
pub fn encode_demand(voice: &[u8; 32], chain: &[Cell], cites: &[CiteWire]) -> Vec<u8> {
    encode_demand_at(voice, 0, chain, cites)
}

pub fn encode_demand_at(voice: &[u8; 32], k: u64, chain: &[Cell], cites: &[CiteWire]) -> Vec<u8> {
    let mut out = vec![ACT_DEMAND];
    out.extend_from_slice(voice);
    push_chain(&mut out, chain);
    push_cites(&mut out, cites);
    out.extend_from_slice(&k.to_le_bytes());
    out
}

/// The same act read by a caller that does not use the demander's K.
pub fn decode_demand(body: &[u8]) -> Option<DemandAct> {
    decode_demand_at(body)
}

pub fn decode_demand_at(body: &[u8]) -> Option<DemandAct> {
    if body.first() != Some(&ACT_DEMAND) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let chain = read_chain(body, &mut p)?;
    let cites = read_cites(body, &mut p).unwrap_or_default();
    let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    Some(DemandAct { voice, chain, cites, k })
}
