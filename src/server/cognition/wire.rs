// 26 — the mind's own codec: a germ, a demand and a percept as bytes.
//
//! These are the acts the kernel itself spells, beside the act table in
//! `anastome::dialogue`. A body that does not parse whole is refused; none
//! of them guesses at a shape.

use super::*;

/// Canonical bytes of an anonymous germ — its coordinate-exact ANASTOME form (the
/// banked word ⊕ the block-diagonal algebra; carries no English, no "kind"). This
/// is the language the hypha teaches back across the nerve; the client treats it
/// opaquely (display only).
pub fn germ_bytes(g: &Element) -> Vec<u8> {
    g.canonical()
}

/// The germ's canonical ADDRESS — a digest of its ANASTOME form. Used to match
/// consilience receipts across voices; a wire/cache identity, never a reasoning
/// operand (the kernel reasons on the element, addresses by this).
pub fn germ_address(g: &Element) -> crate::transducer::a21::Address {
    g.address()
}

pub(crate) fn push_cells(out: &mut Vec<u8>, cells: &[(i64, i64, u8)]) {
    out.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for (r, c, h) in cells {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*h);
    }
}

pub(crate) fn read_cells(b: &[u8], p: &mut usize) -> Option<Vec<(i64, i64, u8)>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 17 {
        return None; // untrusted count: 17 bytes per cell, refuse before allocating
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let h = *b.get(*p + 16)?;
        *p += 17;
        out.push((r, c, h));
    }
    Some(out)
}

/// Encode a demand body: `voice(32) | scene A | scene B`.
pub fn encode_demand(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_cells(&mut out, a);
    push_cells(&mut out, b);
    out
}

/// A DEMAND as it arrives off the wire (26).
pub struct Demand {
    /// The voice that spoke it.
    pub voice: [u8; 32],
    /// The scene BEFORE.
    pub a: Vec<(i64, i64, u8)>,
    /// The scene AFTER. The pair is the demand; neither half alone is one.
    pub b: Vec<(i64, i64, u8)>,
}

pub fn decode_demand(body: &[u8]) -> Option<Demand> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_cells(body, &mut p)?;
    let b = read_cells(body, &mut p)?;
    Some(Demand { voice, a, b })
}

/// THE BODIES OF ONE SCENE: each a connected support with its hues. A scene
/// is not a set of cells but the bodies the cells fall into (A1).
pub type Bodies = Vec<Vec<(i64, i64, u8)>>;

/// CLIENT PERCEPTION — segment each scene into bodies. This is the sensing act;
/// it lives with the client, which reports the resulting percept to its hypha.
pub fn perceive(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> (Bodies, Bodies) {
    (components(a), components(b))
}

pub(crate) fn push_bodies(out: &mut Vec<u8>, bodies: &[Vec<(i64, i64, u8)>]) {
    out.extend_from_slice(&(bodies.len() as u32).to_le_bytes());
    for body in bodies {
        push_cells(out, body);
    }
}

pub(crate) fn read_bodies(b: &[u8], p: &mut usize) -> Option<Vec<Vec<(i64, i64, u8)>>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 4 {
        return None; // untrusted count: each body needs at least its length prefix
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_cells(b, p)?);
    }
    Some(out)
}

/// Encode a client PERCEPT: `voice(32) | A bodies | B bodies` — segmented
/// bodies, not raw cells: the client already perceived.
/// `[voice 32][a bodies][b bodies][frame 4×u32]` — the measured frame
/// (h_in, w_in, h_out, w_out) rides with the bodies so the word germinated
/// from a cross-frame pair carries its frame fiber ρ.
pub fn encode_percept(
    a_bodies: &[Vec<(i64, i64, u8)>],
    b_bodies: &[Vec<(i64, i64, u8)>],
    voice: &[u8; 32],
    frame: &crate::arc::Frame,
    proof: &[Vec<crate::anastome::dialogue::CiteWire>],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_bodies(&mut out, a_bodies);
    push_bodies(&mut out, b_bodies);
    for d in [frame.h_in, frame.w_in, frame.h_out, frame.w_out] {
        out.extend_from_slice(&(d as u32).to_le_bytes());
    }
    // THE PROOF: per germ (in germination order), the speaker's citations.
    out.extend_from_slice(&(proof.len() as u32).to_le_bytes());
    for cites in proof {
        crate::anastome::dialogue::push_cites(&mut out, cites);
    }
    out
}

/// A decoded percept: (voice, bodies A, bodies B, frame, proof per germ).
pub type Percept = (
    [u8; 32],
    Vec<Vec<(i64, i64, u8)>>,
    Vec<Vec<(i64, i64, u8)>>,
    crate::arc::Frame,
    Vec<Vec<crate::anastome::dialogue::CiteWire>>,
);

pub fn decode_percept(body: &[u8]) -> Option<Percept> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_bodies(body, &mut p)?;
    let b = read_bodies(body, &mut p)?;
    let mut d = [0usize; 4];
    for slot in d.iter_mut() {
        *slot = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
    }
    let mut proof = Vec::new();
    if let Some(n) = body.get(p..p + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap()) as usize) {
        p += 4;
        for _ in 0..n {
            proof.push(crate::anastome::dialogue::read_cites(body, &mut p)?);
        }
    }
    Some((voice, a, b, crate::arc::Frame::new(d[0], d[1], d[2], d[3]), proof))
}

/// The frame measured from two cell scenes (their 0-cell extents) — the
/// identity frame when both share dims; used where no grid is at hand.
pub fn frame_of(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> crate::arc::Frame {
    let extent = |s: &[(i64, i64, u8)]| {
        (
            s.iter().map(|(r, _, _)| *r + 1).max().unwrap_or(0).max(0) as usize,
            s.iter().map(|(_, c, _)| *c + 1).max().unwrap_or(0).max(0) as usize,
        )
    };
    let (hi, wi) = extent(a);
    let (ho, wo) = extent(b);
    crate::arc::Frame::new(hi, wi, ho, wo)
}

/// One byte on the wire summarising an outcome (server → client reply).
pub fn outcome_code(o: &Outcome) -> u8 {
    match o {
        Outcome::Decomposed(_) => 1,
        Outcome::GermPending => 2,
        Outcome::Installed(_) => 3,
        Outcome::Sectioned(_) => 6,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
    }
}
