// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A5 — THE RESIDUE GERM: the chain of the exact coboundary A→B, one link
//! per changed site over the union of the two supports, minted when no
//! conformal-and-gauge element closes a body. Anonymous, addressed by form.

use crate::transducer::spelled::{PaintedSite, SectionSite};

/// A residue germ: sorted links of the section (a site of the section and
/// the symbols before and after it) over the union of A's and B's painted
/// sites. The scene's ground means the site is unpainted on that side.
pub type ResidueGerm = Vec<crate::transducer::sentence::SectionLink>;

/// Mint A5's chain from two samples' painted sites — the exact coboundary.
///
/// THE SAMPLE IS THE THEOREM: one position holds one before and one after, so
/// the chain read from a sample pair cannot put two transitions on one site
/// and A5 has nothing to refuse here. The `None` answers the SAMPLES — the
/// two are equal and there is no coboundary to mint — and never A5.
pub fn germinate(a: &[PaintedSite], b: &[PaintedSite]) -> Option<crate::transducer::element::PlacedChain> {
    // Collect the union of sites with the sites' before/after symbols; a
    // side that does not paint a site holds the scene's ground there, the
    // one symbol the open scope writes for it (`raster::void`). Where the
    // scope has none (a vortex, no ground, no family open) a one-sided site
    // has no symbol to write and the chain is not minted: `None`, never 0.
    let void = crate::transducer::raster::void();
    let mut sides: Vec<(SectionSite, Option<u8>, Option<u8>)> = Vec::new();
    for p in a {
        match sides.iter_mut().find(|(s, _, _)| *s == p.site) {
            Some(e) => e.1 = Some(p.symbol),
            None => sides.push((p.site, Some(p.symbol), None)),
        }
    }
    for p in b {
        match sides.iter_mut().find(|(s, _, _)| *s == p.site) {
            Some(e) => e.2 = Some(p.symbol),
            None => sides.push((p.site, None, Some(p.symbol))),
        }
    }
    let mut links: ResidueGerm = Vec::with_capacity(sides.len());
    for (site, before, after) in sides {
        let side = |h: Option<u8>| match h {
            Some(h) => Some(h),
            None => void.ok(),
        };
        links.push(crate::transducer::sentence::SectionLink::new(site, side(before)?, side(after)?));
    }
    // Keep only links that actually change (before != after); a net-zero
    // demand needs no germ.
    links.retain(|l| l.before != l.after);
    if links.is_empty() {
        return None;
    }
    // `upsert` holds one entry per site, so A5 accepts this chain; the
    // `?` collapses into the `None` above rather than adding a refusal.
    // The corner is measured here, at the sensor, and leaves as a
    // translator (law 11).
    crate::transducer::element::place_chain(&links)
}

/// Apply a residue germ to a scene: at each link's site, replace `before`
/// with `after`. Reproduces the demand it was minted from — the germination
/// closes.
pub fn apply(germ: &[crate::transducer::sentence::SectionLink], a: &[PaintedSite]) -> Vec<PaintedSite> {
    let mut out: Vec<PaintedSite> = a.to_vec();
    for l in germ {
        out.retain(|p| p.site != l.site); // clear the site
        if !crate::transducer::raster::is_ground(l.after) {
            out.push(PaintedSite::new(l.site, l.after));
        }
    }
    out.sort_unstable();
    out
}

/// Canonical bytes of a residue germ — its ANASTOME address payload: each
/// link's coordinates along the section's plane in the plane's order, then
/// its two symbols.
pub fn germ_bytes(germ: &[crate::transducer::sentence::SectionLink]) -> Vec<u8> {
    let mut out = Vec::new();
    for l in germ {
        for c in l.site.coordinates() {
            out.extend_from_slice(&c.to_le_bytes());
        }
        out.push(l.before);
        out.push(l.after);
    }
    out
}
