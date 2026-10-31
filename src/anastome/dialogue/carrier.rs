// The carriers every act shares: a chain, a mu vector, a grid, a citation
// list, an invariant.
//
//! Each pushes its coordinates and reads them back exactly. A body that
//! does not parse whole is refused; none of these guesses at a length.

use super::*;

/// THE ACT TAG: the byte an act body opens with. Rule 15's question at a
/// wire read is whether the position is the tape's own coordinate or the
/// order things happened to arrive in, and here it is the first: 26 puts
/// the act tag at offset zero, so reading byte zero is reading the field
/// the sender wrote, not taking whichever byte came first.
///
/// It is ONE carrier for that fact. Every decoder asked `b.first()` for
/// itself, so the offset was written out forty times and forty readers
/// each had to be right about it.
pub fn act_tag(body: &[u8]) -> Option<u8> {
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: 26 writes the act tag at offset zero, and this is the one reader of it"
    )]
    body.first().copied()
}

/// Whether an act body opens with `tag`. The refusal every decoder makes
/// before it reads a field.
pub fn act_is(body: &[u8], tag: u8) -> bool {
    act_tag(body) == Some(tag)
}

/// The citations framed whole, or refused whole where an address cannot be
/// framed: nothing is written then, never a partial frame (LD0, rule 11).
pub fn push_cites(out: &mut Vec<u8>, cites: &[CiteWire]) -> Option<()> {
    let mut framed = Vec::new();
    crate::transducer::address::put_u32_count(&mut framed, cites.len())?;
    for c in cites {
        crate::transducer::address::put_address(&mut framed, &c.addr)?;
        framed.push(c.standing);
        c.transport.to_bytes(&mut framed);
        push_chain(&mut framed, &c.body)?;
    }
    out.append(&mut framed);
    Some(())
}

pub fn read_cites(b: &[u8], p: &mut usize) -> Option<Vec<CiteWire>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().checked_sub(*p)? / 37 {
        return None; // untrusted count: address, standing, versor, body — the floor follows the writer
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let addr = crate::transducer::address::take_address(b, p)?;
        let standing = *b.get(*p)?;
        *p += 1;
        let g = crate::transducer::element::Versor::from_bytes(b, p)?;
        let body = read_chain(b, p)?;
        out.push(CiteWire { addr, standing, transport: g, body });
    }
    Some(out)
}

/// A chart's coordinates behind a one-byte axis count. `None` where the
/// chart has more axes than the field can name: a truncated chart is a
/// DIFFERENT chart, and writing 255 of its axes under a count of 255 said
/// so to every reader while the sender believed it named its own.
pub(crate) fn push_invariant(
    out: &mut Vec<u8>,
    inv: &crate::transducer::stalk::Invariant,
) -> Option<()> {
    // THE CHART'S OWN KEY ON THE WIRE (law 2): the canonical coordinates
    // written out, kind-tagged and length-prefixed, which is one encoding
    // shared with the trie and the tape rather than an axis loop per codec.
    let key = inv.key();
    let bytes = key.as_bytes();
    crate::transducer::address::put_u16_count(out, bytes.len())?;
    out.extend_from_slice(bytes);
    Some(())
}

pub(crate) fn read_invariant(b: &[u8], p: &mut usize) -> Option<crate::transducer::stalk::Invariant> {
    let len = crate::transducer::address::take_u16_count(b, p)?;
    let bytes = b.get(*p..*p + len)?;
    *p += len;
    crate::transducer::stalk::Invariant::from_key(bytes)
}

/// `None` where the grid's height or width passes its two bytes (STD1 3c):
/// a grid is refused, never framed as a smaller one.
#[must_use]
pub(crate) fn push_grid(out: &mut Vec<u8>, g: &[Vec<u8>]) -> Option<()> {
    crate::transducer::address::put_u16_count(out, g.len())?;
    crate::transducer::address::put_u16_count(out, crate::arc::grid_width(g))?;
    for row in g {
        out.extend_from_slice(row);
    }
    Some(())
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

/// `None` where the chain's count passes its four bytes (STD1 3c).
#[must_use]
pub fn push_chain(out: &mut Vec<u8>, chain: &[Link]) -> Option<()> {
    crate::transducer::address::put_u32_count(out, chain.len())?;
    for link in chain {
        push_link(out, link);
    }
    Some(())
}

/// ONE LINK on the wire: the one writer `push_chain` and the width share.
fn push_link(out: &mut Vec<u8>, &(r, c, before, after): &Link) {
    out.extend_from_slice(&r.to_le_bytes());
    out.extend_from_slice(&c.to_le_bytes());
    out.push(before);
    out.push(after);
}

/// The wire width of one chain link, measured from the encoder itself (the
/// carrier's own coordinate, never a number chosen elsewhere).
pub fn link_wire_width() -> usize {
    let mut probe = Vec::new();
    push_link(&mut probe, &(0, 0, 0, 0));
    probe.len()
}

pub fn read_chain(b: &[u8], p: &mut usize) -> Option<Vec<Link>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().checked_sub(*p)? / 18 {
        return None; // untrusted count: 18 bytes per link, or refuse before allocating
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
