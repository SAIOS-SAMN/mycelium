// A21 — the keys the mind indexes by. Every one of them writes its
// coordinates out and decodes back to the object it names; none is a
// digest of content (law 2).

use super::*;

pub(crate) fn chart_simplex_key(inv: &crate::transducer::stalk::Invariant, sim: &[u8]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::address::TAG_SIMPLEX];
    crate::transducer::address::put_key_bytes(&mut buf, &inv.key());
    crate::transducer::address::put_key_bytes(&mut buf, sim);
    buf
}

pub(crate) fn orbit_key(chart: &crate::transducer::stalk::Invariant, voice: &[u8; 32]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_key_bytes(&mut buf, &chart.key());
    buf.extend_from_slice(voice);
    buf
}

/// Canonical key of one evidence row: address, sorted footprint, sorted
/// scene, each site written along the plane in the plane's order.
pub(crate) fn footprint_key(
    address: &[u8],
    fp: &[crate::transducer::spelled::SectionSite],
    scene: &[crate::transducer::spelled::PaintedSite],
) -> Vec<u8> {
    let mut points = fp.to_vec();
    points.sort_unstable();
    let mut painted = scene.to_vec();
    painted.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::address::put_key_bytes(&mut buf, address);
    crate::transducer::address::put_key_count(&mut buf, points.len());
    for site in &points {
        site.put_canonical(&mut buf);
    }
    crate::transducer::address::put_key_count(&mut buf, painted.len());
    for p in &painted {
        p.site.put_canonical(&mut buf);
        buf.push(p.symbol);
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
            v.canonical()?;
            v.put_canonical(&mut buf);
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
pub(crate) fn chart_footprint_key(_inv: &crate::transducer::stalk::Invariant, chain: &[crate::transducer::sentence::SectionLink]) -> Vec<u8> {
    let mut pos = element::positions(chain);
    pos.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::address::put_key_count(&mut buf, pos.len());
    for site in pos {
        site.put_canonical(&mut buf);
    }
    buf
}

/// A 1-simplex is exactly two vertices. Any other fibre is not an edge.
pub(crate) fn edge_pair(addrs: &[Address]) -> Option<(Address, Address)> {
    match addrs {
        [a, b] => Some((a.clone(), b.clone())),
        _ => None,
    }
}
