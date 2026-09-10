// 22 — THE ORBIT: where a vertex's versor carries a body, ring by ring.

use super::*;

/// End of an orbit (chapter 22). `steps` is instance, not the form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrbitEnd {
    Contact { cells: Vec<Cell3>, steps: u32 },
    Torsion { steps: u32 },
    LeftFrame { steps: u32 },
    NoOrbit,
}

pub(crate) fn positions_of(cells: &[Cell3]) -> Vec<(i64, i64)> {
    let mut p: Vec<(i64, i64)> = cells.iter().map(|&(r, c, _)| (r, c)).collect();
    p.sort_unstable();
    p.dedup();
    p
}

pub(crate) fn occupy_keys(cells: &[(i64, i64, u8)]) -> crate::network::nerve::Keyed<()> {
    let mut occ = crate::network::nerve::Keyed::new();
    for &(r, c, _) in cells {
        occ.insert(cell_pos_key(r, c), ());
    }
    occ
}

/// Smallest peel-shell index at which every image cell sits in the
/// `adjacent_ring` expansion of the anchor. Not a metric.
pub(crate) fn ring_dist_peel(image: &[(i64, i64)], anchor: &[(i64, i64)], bound: u32) -> Option<u8> {
    if image.is_empty() || anchor.is_empty() {
        return None;
    }
    let mut have: Vec<(i64, i64, u8)> = anchor.iter().map(|&(r, c)| (r, c, 0)).collect();
    for k in 0..=bound {
        let occ = occupy_keys(&have);
        if image.iter().all(|&(r, c)| occ.contains(&cell_pos_key(r, c))) {
            return Some(k as u8);
        }
        let ring = crate::transducer::blade::adjacent_ring(&have);
        if ring.is_empty() {
            break;
        }
        have.extend(ring);
    }
    None
}

/// Carry every cell of a body by `v`. The image is NOT bounded by any
/// frame: a cell carried off the frame is still the image of that cell, and
/// whether the body left the frame is the ORBIT's reading (`OrbitEnd::
/// LeftFrame`), taken after the step. Clipping here would hide the exit.
pub(crate) fn apply_versor_cells(v: &Versor, cells: &[Cell3]) -> Vec<Cell3> {
    let mut out = Vec::new();
    for &(r, c, h) in cells {
        if let Some(image) = element::carry_cell(v, r, c) {
            for (ir, ic) in image {
                out.push((ir, ic, h));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub(crate) fn vertex_versor(v: &Vertex) -> Option<Versor> {
    match v {
        Vertex::Word { derivation, carriage, .. } => {
            element::conformal_of_rotor(&derivation.rotor(), carriage)
        }
        _ => None,
    }
}

/// Apply a word until the image's ring distance to `anchor` equals the
/// contact ring (or 0). Bound by the frame. `NoOrbit` when the first step
/// does not move the distance.
pub fn orbit(v: &Vertex, x_cells: &[Cell3], anchor: &[(i64, i64)], frame: (usize, usize)) -> OrbitEnd {
    let Some(versor) = vertex_versor(v) else {
        return OrbitEnd::NoOrbit;
    };
    let want = match v {
        Vertex::Word { contact: Some(c), .. } => c.ring,
        _ => 0,
    };
    let bound = (frame.0.saturating_add(frame.1)).max(1) as u32;
    let start = ring_dist_peel(&positions_of(x_cells), anchor, bound);
    let next = apply_versor_cells(&versor, x_cells);
    let d1 = ring_dist_peel(&positions_of(&next), anchor, bound);
    if d1 == start {
        return OrbitEnd::NoOrbit;
    }
    // Canonical form of a placed occupancy: sorted cells as bytes, then
    // blake3. Position is meaning (orbit instance); this is not the
    // translation-free residual address.
    let state_key = |cs: &[Cell3]| -> Vec<u8> {
        let mut cells = cs.to_vec();
        cells.sort_unstable();
        let mut buf = Vec::with_capacity(4 + cells.len().saturating_mul(17));
        buf.extend_from_slice(&(cells.len() as u32).to_le_bytes());
        for &(r, c, h) in &cells {
            buf.extend_from_slice(&r.to_le_bytes());
            buf.extend_from_slice(&c.to_le_bytes());
            buf.push(h);
        }
        buf
    };
    let mut cells = next;
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    seen.insert(state_key(x_cells), ());
    seen.insert(state_key(&cells), ());
    for step in 1..=bound {
        let d = ring_dist_peel(&positions_of(&cells), anchor, bound);
        let left = cells.iter().any(|&(r, c, _)| r < 0 || c < 0 || (r as usize) >= frame.0 || (c as usize) >= frame.1);
        if left {
            return OrbitEnd::LeftFrame { steps: step };
        }
        if d == Some(want) {
            let inside: Vec<Cell3> = cells
                .into_iter()
                .filter(|&(r, c, _)| r >= 0 && c >= 0 && (r as usize) < frame.0 && (c as usize) < frame.1)
                .collect();
            return OrbitEnd::Contact { cells: inside, steps: step };
        }
        let nxt = apply_versor_cells(&versor, &cells);
        let nk = state_key(&nxt);
        if nxt == cells || seen.contains(&nk) {
            return OrbitEnd::Torsion { steps: step };
        }
        seen.insert(nk, ());
        cells = nxt;
    }
    OrbitEnd::LeftFrame { steps: bound }
}
