// 22 — THE ORBIT: where a vertex's versor carries a body, ring by ring.

use super::*;

/// End of an orbit (chapter 22). `steps` is instance, not the form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrbitEnd {
    Contact { painted: Vec<Painted>, steps: u32 },
    Torsion { steps: u32 },
    LeftFrame { steps: u32 },
    NoOrbit,
}

pub(crate) fn positions_of(painted: &[Painted]) -> Vec<(i64, i64)> {
    let mut p: Vec<(i64, i64)> = painted.iter().map(|&(r, c, _)| (r, c)).collect();
    p.sort_unstable();
    p.dedup();
    p
}

pub(crate) fn occupy_keys(painted: &[(i64, i64, u8)]) -> crate::network::nerve::Keyed<()> {
    let mut occ = crate::network::nerve::Keyed::new();
    for &(r, c, _) in painted {
        occ.insert(site_key(r, c), ());
    }
    occ
}

/// Smallest peel-shell index at which every image site sits in the
/// `adjacent_ring` expansion of the anchor. Not a metric.
pub(crate) fn ring_dist_peel(image: &[(i64, i64)], anchor: &[(i64, i64)], bound: u32) -> Option<u8> {
    if image.is_empty() || anchor.is_empty() {
        return None;
    }
    let mut have: Vec<(i64, i64, u8)> = anchor.iter().map(|&(r, c)| (r, c, 0)).collect();
    for k in 0..=bound {
        let occ = occupy_keys(&have);
        if image.iter().all(|&(r, c)| occ.contains(&site_key(r, c))) {
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

/// Carry every painted site of a grid's scene by `v`. The image is NOT
/// bounded by the grid's extent: a site carried off it is still the image
/// of that site, and whether the body left the frame is the ORBIT's
/// reading (`OrbitEnd::LeftFrame`), taken after the step. Clipping here
/// would hide the exit. `None` where a site is carried out of the grid's
/// own frame along `e₃`: the orbit walks one grid, and a body carried off
/// it has left it.
pub(crate) fn carry_painted(v: &Versor, painted: &[Painted]) -> Option<Vec<Painted>> {
    let mut out = Vec::new();
    for &(r, c, h) in painted {
        if let Some(image) = element::carry_lamina(v, r, c, 0) {
            for (ir, ic, ik) in image {
                if ik != 0 {
                    return None;
                }
                out.push((ir, ic, h));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

/// THE VERSOR A VERTEX ACTS BY AT ONE NAMED CHART: its rotor under that
/// member's carriage. A keyed read — an orbit is a walk of ONE body, so the
/// chart the body stands on is the coordinate, and reading a representative
/// orbited one body's placement whatever body was handed in.
pub(crate) fn vertex_versor_at(
    v: &Vertex,
    chart: &crate::transducer::stalk::Invariant,
) -> Option<Versor> {
    match v {
        Vertex::Word { derivation, .. } => {
            element::conformal_of_rotor(&derivation.rotor(), &v.acting_on(chart)?.carriage)
        }
        _ => None,
    }
}

/// DOES THIS VERTEX ORBIT — asked of every member it acts by, because the
/// orbit is a property of the vertex and a vertex acting on several charts
/// orbits if any of its members moves the ring distance. A contact vertex
/// carries the identity at every member (extraction's step 6 puts it there),
/// so today every member gives one answer; a vertex whose members differ is
/// answered by the first that moves, and `NoOrbit` only when none does.
pub fn orbit(v: &Vertex, x_painted: &[Painted], anchor: &[(i64, i64)], frame: (usize, usize)) -> OrbitEnd {
    let mut end = OrbitEnd::NoOrbit;
    for chart in v.charts() {
        end = orbit_at(v, &chart, x_painted, anchor, frame);
        if !matches!(end, OrbitEnd::NoOrbit) {
            return end;
        }
    }
    end
}

/// Apply a word until the image's ring distance to `anchor` equals the
/// contact ring (or 0). Bound by the frame. `NoOrbit` when the first step
/// does not move the distance.
pub fn orbit_at(
    v: &Vertex,
    chart: &crate::transducer::stalk::Invariant,
    x_painted: &[Painted],
    anchor: &[(i64, i64)],
    frame: (usize, usize),
) -> OrbitEnd {
    let Some(versor) = vertex_versor_at(v, chart) else {
        return OrbitEnd::NoOrbit;
    };
    let want = match v {
        Vertex::Word { contact: Some(c), .. } => c.ring,
        _ => 0,
    };
    let bound = (frame.0.saturating_add(frame.1)).max(1) as u32;
    let start = ring_dist_peel(&positions_of(x_painted), anchor, bound);
    let Some(next) = carry_painted(&versor, x_painted) else {
        return OrbitEnd::LeftFrame { steps: 1 };
    };
    let d1 = ring_dist_peel(&positions_of(&next), anchor, bound);
    if d1 == start {
        return OrbitEnd::NoOrbit;
    }
    // Canonical form of a placed occupancy: the sorted painted sites as
    // bytes. Position is meaning (orbit instance); this is not the
    // translation-free residual address.
    let state_key = |ps: &[Painted]| -> Vec<u8> {
        let mut painted = ps.to_vec();
        painted.sort_unstable();
        // A reservation, not a bound: a site count whose byte width does
        // not fit the carrier is reserved for as it is written instead.
        let reserve = match painted.len().checked_mul(17) {
            Some(n) => 4 + n,
            None => 4,
        };
        let mut buf = Vec::with_capacity(reserve);
        buf.extend_from_slice(&(painted.len() as u32).to_le_bytes());
        for &(r, c, h) in &painted {
            buf.extend_from_slice(&r.to_le_bytes());
            buf.extend_from_slice(&c.to_le_bytes());
            buf.push(h);
        }
        buf
    };
    let mut painted = next;
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    seen.insert(state_key(x_painted), ());
    seen.insert(state_key(&painted), ());
    for step in 1..=bound {
        let d = ring_dist_peel(&positions_of(&painted), anchor, bound);
        let left = painted.iter().any(|&(r, c, _)| r < 0 || c < 0 || (r as usize) >= frame.0 || (c as usize) >= frame.1);
        if left {
            return OrbitEnd::LeftFrame { steps: step };
        }
        if d == Some(want) {
            let inside: Vec<Painted> = painted
                .into_iter()
                .filter(|&(r, c, _)| r >= 0 && c >= 0 && (r as usize) < frame.0 && (c as usize) < frame.1)
                .collect();
            return OrbitEnd::Contact { painted: inside, steps: step };
        }
        let Some(nxt) = carry_painted(&versor, &painted) else {
            return OrbitEnd::LeftFrame { steps: step };
        };
        let nk = state_key(&nxt);
        if nxt == painted || seen.contains(&nk) {
            return OrbitEnd::Torsion { steps: step };
        }
        seen.insert(nk, ());
        painted = nxt;
    }
    OrbitEnd::LeftFrame { steps: bound }
}
