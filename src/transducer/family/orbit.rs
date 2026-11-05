// 22 — THE ORBIT: whether a vertex's motion moves a body, read by one exact
// application (78.62). The march that carried a body ring by ring — a walk
// with a hop-distance shell, a set of seen states and a cap from the grid's
// extent — is retired: the court and the mint asked only whether the body
// moves, which the first application decides (measured: 680 of 680 cases
// agreed before the march was removed).

use super::*;

/// THE SITES A SCENE PAINTS, each once, in canonical order.
pub(crate) fn positions_of(painted: &[PaintedSite]) -> Vec<SectionSite> {
    let mut p: Vec<SectionSite> = painted.iter().map(|s| s.site).collect();
    p.sort_unstable();
    p.dedup();
    p
}

pub(crate) fn occupy_keys(painted: &[PaintedSite]) -> crate::network::nerve::Keyed<()> {
    let mut occ = crate::network::nerve::Keyed::new();
    for p in painted {
        occ.insert(site_key(p.site), ());
    }
    occ
}

/// Smallest peel-shell index at which every image site sits in the
/// `adjacent_ring` expansion of the anchor. Not a metric.
///
/// In the grid's own count: the shell reaches the grid's height plus width,
/// and `k as u8` wrapped a far shell onto a near one, which could read a
/// contact at a ring the body never stood on.
pub(crate) fn ring_dist_peel(image: &[SectionSite], anchor: &[SectionSite], bound: usize) -> Option<usize> {
    if image.is_empty() || anchor.is_empty() {
        return None;
    }
    let mut have: Vec<PaintedSite> = anchor.iter().map(|&s| PaintedSite::new(s, 0)).collect();
    for k in 0..=bound {
        let occ = occupy_keys(&have);
        if image.iter().all(|&s| occ.contains(&site_key(s))) {
            return Some(k);
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
/// of that site — clipping here would hide that the motion moved it. `None`
/// where a site is carried out of the grid's own frame along `e₃`: the
/// motion is read within one grid, and a body carried off it has moved.
pub(crate) fn carry_painted(v: &Versor, painted: &[PaintedSite]) -> Option<Vec<PaintedSite>> {
    let mut out = Vec::new();
    for p in painted {
        let [r, c] = p.site.coordinates();
        if let Some(image) = element::carry_compartment(v, r, c, 0) {
            for (ir, ic, ik) in image {
                if ik != 0 {
                    return None;
                }
                out.push(PaintedSite::on_plane([ir, ic], p.symbol));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

/// THE VERSOR A VERTEX ACTS BY AT ONE NAMED CHART: its rotor under that
/// member's carriage. A keyed read — an orbit is a traversal of ONE body, so the
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

/// 22, 78.43, 78.62 — DOES THE VERTEX'S MOTION MOVE THE BODY: one exact
/// application, never a walk. The body is read as the set of its sites; the
/// motion a member acts by carries it once, and the body STANDS STILL where
/// its image is non-empty and every image site is a site of the body — the
/// motion fixes it as a set. A member with no conformal versor moves nothing;
/// one that carries a site off the grid along `e₃` moves it. Asked of every
/// member: the vertex moves the body where any member does. An empty body
/// has nothing to move.
pub fn moves(v: &Vertex, painted: &[PaintedSite]) -> bool {
    if painted.is_empty() {
        return false;
    }
    let body = occupy_keys(painted);
    v.charts().iter().any(|chart| {
        let Some(versor) = vertex_versor_at(v, chart) else { return false };
        match carry_painted(&versor, painted) {
            None => true,
            Some(image) => image.is_empty() || !image.iter().all(|p| body.contains(&site_key(p.site))),
        }
    })
}
