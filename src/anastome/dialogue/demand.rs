// ACT_PROBE — a tentative sentence against an open listing.
//
//! The demand and the routed demand that were spelled here carried a
//! coboundary chain to a court (law 8, ruled 2026-09-10); both are deleted
//! with their acts, and the door refuses their tags.

use super::*;

/// `None` where an address cannot be framed: the probe is not spoken, never
/// a partial frame (LD0).
pub fn encode_probe(voice: &[u8; 32], target: impl AsRef<[u8]>, cites: &[CiteWire]) -> Option<Vec<u8>> {
    let mut out = vec![ACT_PROBE];
    out.extend_from_slice(voice);
    put_addr(&mut out, target)?;
    push_cites(&mut out, cites)?;
    Some(out)
}

pub fn decode_probe(body: &[u8]) -> Option<([u8; 32], Address, Vec<CiteWire>)> {
    if !act_is(body, ACT_PROBE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let target = take_addr(body, &mut p)?;
    let cites = read_cites(body, &mut p)?;
    Some((voice, target, cites))
}

/// PR1 — THE PROBE'S READING ON THE WIRE: the settled μ⃗, the closure (its
/// kind, and a residual's versor behind a flag, as its ray), the remaining μ⃗.
pub fn encode_probe_reply(r: &ProbeReading) -> Vec<u8> {
    let mut out = Vec::new();
    push_mu(&mut out, &r.settled);
    match &r.closure {
        ProbeClosure::Closed => out.push(0),
        ProbeClosure::Residual(v) => {
            out.push(1);
            match v {
                Some(v) => {
                    out.push(1);
                    v.put_canonical(&mut out);
                }
                None => out.push(0),
            }
        }
        ProbeClosure::Nothing => out.push(2),
        ProbeClosure::Gone => out.push(3),
    }
    push_mu(&mut out, &r.remaining);
    out
}

pub fn decode_probe_reply(body: &[u8]) -> Option<ProbeReading> {
    let mut p = 0usize;
    let settled = read_mu(body, &mut p)?;
    let kind = *body.get(p)?;
    p += 1;
    let closure = match kind {
        0 => ProbeClosure::Closed,
        1 => {
            let flag = *body.get(p)?;
            p += 1;
            ProbeClosure::Residual(match flag {
                0 => None,
                1 => Some(crate::transducer::cga::Mv::from_bytes(body, &mut p)?),
                _ => return None,
            })
        }
        2 => ProbeClosure::Nothing,
        3 => ProbeClosure::Gone,
        _ => return None,
    };
    let remaining = read_mu(body, &mut p)?;
    (p == body.len()).then_some(ProbeReading { settled, closure, remaining })
}
