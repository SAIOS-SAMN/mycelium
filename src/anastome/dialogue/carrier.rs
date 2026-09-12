// The carriers every act shares: a chain, a mu vector, a grid, a citation
// list, an invariant.
//
//! Each pushes its coordinates and reads them back exactly. A body that
//! does not parse whole is refused; none of these guesses at a length.

use super::*;

pub fn push_cites(out: &mut Vec<u8>, cites: &[CiteWire]) {
    out.extend_from_slice(&(cites.len() as u32).to_le_bytes());
    for c in cites {
        let _ = crate::transducer::address::put_addr(out, &c.addr);
        out.push(c.standing);
        c.transport.to_bytes(out);
        out.extend_from_slice(&c.gauge.to_le_bytes());
        push_chain(out, &c.body);
    }
}

pub fn read_cites(b: &[u8], p: &mut usize) -> Option<Vec<CiteWire>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().checked_sub(*p)? / 41 {
        return None; // untrusted count: address, standing, versor, coordinate, body
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = crate::transducer::address::take_addr(b, p)?;
        let standing = *b.get(*p)?;
        *p += 1;
        let g = crate::transducer::element::Versor::from_bytes(b, p)?;
        let asg = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
        *p += 4;
        let body = read_chain(b, p)?;
        out.push(CiteWire { addr, standing, transport: g, gauge: asg, body });
    }
    Some(out)
}

pub(crate) fn push_invariant(out: &mut Vec<u8>, inv: &crate::transducer::stalk::Invariant) {
    out.push(inv.0.len().min(255) as u8);
    for v in inv.0.iter().take(255) {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

pub(crate) fn read_invariant(b: &[u8], p: &mut usize) -> Option<crate::transducer::stalk::Invariant> {
    let len = *b.get(*p)? as usize;
    *p += 1;
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?));
        *p += 8;
    }
    Some(crate::transducer::stalk::Invariant(v))
}

pub(crate) fn push_grid(out: &mut Vec<u8>, g: &[Vec<u8>]) {
    out.extend_from_slice(&(g.len() as u16).to_le_bytes());
    out.extend_from_slice(&(g.first().map_or(0, |r| r.len()) as u16).to_le_bytes());
    for row in g {
        out.extend_from_slice(row);
    }
}

pub(crate) fn read_grid(b: &[u8], p: &mut usize) -> Option<Vec<Vec<u8>>> {
    let h = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?) as usize;
    let w = u16::from_le_bytes(b.get(*p + 2..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if h * w > b.len().checked_sub(*p)? || h > 1024 || w > 1024 {
        return None; // untrusted dims
    }
    let mut g = Vec::with_capacity(h);
    for _ in 0..h {
        g.push(b.get(*p..*p + w)?.to_vec());
        *p += w;
    }
    Some(g)
}

pub fn push_chain(out: &mut Vec<u8>, chain: &[Cell]) {
    out.extend_from_slice(&(chain.len() as u32).to_le_bytes());
    for (r, c, before, after) in chain {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*before);
        out.push(*after);
    }
}

/// The wire width of one chain cell, measured from the encoder itself (the
/// carrier's own coordinate, never a number chosen elsewhere).
pub fn cell_wire_width() -> usize {
    let mut probe = Vec::new();
    push_chain(&mut probe, &[(0, 0, 0, 0)]);
    probe.len() - 4
}

pub fn read_chain(b: &[u8], p: &mut usize) -> Option<Vec<Cell>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().checked_sub(*p)? / 18 {
        return None; // untrusted count: 18 bytes per cell, or refuse before allocating
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let before = *b.get(*p + 16)?;
        let after = *b.get(*p + 17)?;
        *p += 18;
        out.push((r, c, before, after));
    }
    Some(out)
}

pub(crate) fn push_mu(out: &mut Vec<u8>, mu: &[u64; 4]) {
    for v in mu {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

/// One `u64`, little-endian, or none when the frame ends before it.
pub(crate) fn read_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

/// One `u16`, little-endian, or none when the frame ends before it.
pub(crate) fn read_u16(b: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

pub(crate) fn read_mu(b: &[u8], p: &mut usize) -> Option<[u64; 4]> {
    let mut mu = [0u64; 4];
    for v in &mut mu {
        *v = read_u64(b, p)?;
    }
    Some(mu)
}
