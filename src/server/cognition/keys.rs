// A21 — the keys the mind indexes by. Every one of them writes its
// coordinates out and decodes back to the object it names; none is a
// digest of content (law 2).

use super::*;

pub(crate) fn chart_simplex_key(inv: &crate::transducer::stalk::Invariant, sim: &[u8]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::address::TAG_CELL];
    crate::transducer::address::put_bytes(&mut buf, &inv.key());
    crate::transducer::address::put_bytes(&mut buf, sim);
    buf
}

/// Canonical encoding of a voice's K: the index itself.
pub(crate) fn utterance_k_key(k: u64) -> Vec<u8> {
    k.to_le_bytes().to_vec()
}

pub(crate) fn loop_key(chart: &crate::transducer::stalk::Invariant, voice: &[u8; 32]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, &chart.key());
    buf.extend_from_slice(voice);
    buf
}

/// Canonical key of one evidence row: address, sorted footprint, sorted scene.
pub(crate) fn footprint_key(addr: &[u8], fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<u8> {
    let mut points = fp.to_vec();
    points.sort_unstable();
    let mut cells = scene.to_vec();
    cells.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, addr);
    buf.extend_from_slice(&(points.len() as u32).to_le_bytes());
    for &(r, c) in &points {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    buf.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for &(r, c, h) in &cells {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
        buf.push(h);
    }
    buf
}

/// M1 action key of an empty-sub group element: canonical versor plus gauge.
pub(crate) fn action_index_key(e: &Element) -> Option<Vec<u8>> {
    if !e.sub().is_empty() {
        return None;
    }
    let mut buf = Vec::new();
    match e.versor() {
        Some(v) => {
            let k = v.key()?;
            crate::transducer::address::put_bytes(&mut buf, k.as_bytes());
        }
        None => buf.push(0),
    }
    for &(a, b) in e.gauge() {
        buf.push(a);
        buf.push(b);
    }
    Some(buf)
}

/// Positions of a chain — same_base and the condense family are a get.
pub(crate) fn chart_footprint_key(_inv: &crate::transducer::stalk::Invariant, chain: &[(i64, i64, u8, u8)]) -> Vec<u8> {
    let mut pos = element::positions(chain);
    pos.sort_unstable();
    let mut buf = Vec::new();
    buf.extend_from_slice(&(pos.len() as u32).to_le_bytes());
    for (r, c) in pos {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    buf
}

/// A 1-simplex is exactly two vertices. Any other fiber is not an edge.
pub(crate) fn edge_pair(addrs: &[Address]) -> Option<(Address, Address)> {
    match addrs {
        [a, b] => Some((a.clone(), b.clone())),
        _ => None,
    }
}


pub(crate) fn sentence_rule_of(form: &Element) -> Option<crate::transducer::sentence::SentenceRule> {
    use crate::transducer::sentence::{from_element_form, rule_of_form};
    Some(rule_of_form(from_element_form(form)?))
}

/// The consilience address of a SECTION prospect: the carrying thought's stable
/// base address ⊕ the assignment — so two kernels independently proposing the
/// same axis extension meet at one receipt (wire identity, never reasoned on).
pub(crate) fn section_address(thought: &[u8], asg: &[(u8, u8)]) -> Vec<u8> {
    let mut sigma = asg.to_vec();
    sigma.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, thought);
    buf.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
    for (b, a) in sigma {
        buf.push(b);
        buf.push(a);
    }
    buf
}
