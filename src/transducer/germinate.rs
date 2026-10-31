// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A5 — THE RESIDUAL GERM: the chain of the exact coboundary A→B, one link
//! per changed site over the union of the two supports, minted when no
//! conformal-and-gauge element closes a body. Anonymous, addressed by form.

/// A residual germ: sorted `(row, col, hue_before, hue_after)` over the union of
/// A's and B's painted sites. The scene's ground (`arc::ground`: hue 0, or the
/// hue its family is drawn on) means the site is unpainted on that side.
pub type ResidualGerm = Vec<(i64, i64, u8, u8)>;

/// Mint A5's chain from two grids' painted sites — the exact coboundary.
///
/// THE GRID IS THE THEOREM: one position holds one before and one after, so
/// the chain read from a grid pair cannot put two transitions on one site
/// and A5 has nothing to refuse here. The `None` answers the GRIDS — the
/// two are equal and there is no coboundary to mint — and never A5.
pub fn germinate(
    a: &[(i64, i64, u8)],
    b: &[(i64, i64, u8)],
) -> Option<crate::transducer::element::PlacedChain> {
    // Collect the union of sites with the sites' before/after hues.
    let void = crate::arc::ground();
    let mut links: ResidualGerm = Vec::new();
    let upsert = |links: &mut ResidualGerm, r: i64, c: i64, before: u8, after: u8| {
        match links.iter_mut().find(|(rr, cc, _, _)| *rr == r && *cc == c) {
            Some(e) => {
                if before != void {
                    e.2 = before;
                }
                if after != void {
                    e.3 = after;
                }
            }
            None => links.push((r, c, before, after)),
        }
    };
    for (r, c, h) in a {
        upsert(&mut links, *r, *c, *h, void);
    }
    for (r, c, h) in b {
        upsert(&mut links, *r, *c, void, *h);
    }
    // Keep only links that actually change (before != after); a net-zero
    // demand needs no germ.
    links.retain(|(_, _, before, after)| before != after);
    if links.is_empty() {
        return None;
    }
    // `upsert` holds one entry per site, so A5 accepts this chain; the
    // `?` collapses into the `None` above rather than adding a refusal.
    // The corner is measured here, at the sensor, and leaves as a
    // translator (law 11).
    crate::transducer::element::place_chain(&links)
}

/// Apply a residual germ to a scene: at each link's site, replace `before`
/// with `after`. Reproduces the demand it was minted from — the germination
/// closes.
pub fn apply(germ: &[(i64, i64, u8, u8)], a: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let void = crate::arc::ground();
    let mut out: Vec<(i64, i64, u8)> = a.to_vec();
    for (r, c, _before, after) in germ {
        out.retain(|(rr, cc, _)| !(rr == r && cc == c)); // clear the site
        if *after != void {
            out.push((*r, *c, *after));
        }
    }
    out.sort_unstable();
    out
}

/// Canonical bytes of a residual germ — its ANASTOME address payload.
pub fn germ_bytes(germ: &[(i64, i64, u8, u8)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(germ.len() * 18);
    for (r, c, before, after) in germ {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*before);
        out.push(*after);
    }
    out
}
