// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A5 — THE RESIDUAL GERM: the chain of the exact coboundary A→B, one cell
//! per changed position over the union of the two supports, minted when no
//! conformal-and-gauge element closes a body. Anonymous, addressed by form.

/// A residual germ: sorted `(row, col, hue_before, hue_after)` over the union of
/// A's and B's occupied positions. hue 0 means the cell is absent on that side.
pub type ResidualGerm = Vec<(i64, i64, u8, u8)>;

/// Mint the residual germ from raw cell lists `a → b` — the exact coboundary.
pub fn germinate(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Option<ResidualGerm> {
    // Collect the union of positions with their before/after hues.
    let mut cells: ResidualGerm = Vec::new();
    let upsert = |cells: &mut ResidualGerm, r: i64, c: i64, before: u8, after: u8| {
        match cells.iter_mut().find(|(rr, cc, _, _)| *rr == r && *cc == c) {
            Some(e) => {
                if before != 0 {
                    e.2 = before;
                }
                if after != 0 {
                    e.3 = after;
                }
            }
            None => cells.push((r, c, before, after)),
        }
    };
    for (r, c, h) in a {
        upsert(&mut cells, *r, *c, *h, 0);
    }
    for (r, c, h) in b {
        upsert(&mut cells, *r, *c, 0, *h);
    }
    // Keep only cells that actually change (before != after); a net-zero demand
    // needs no germ.
    cells.retain(|(_, _, before, after)| before != after);
    if cells.is_empty() {
        return None;
    }
    cells.sort_unstable();
    Some(cells)
}

/// Apply a residual germ to a scene: at each germ cell, replace `before` with
/// `after`. Reproduces the demand it was minted from — the germination closes.
pub fn apply(germ: &ResidualGerm, a: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let mut out: Vec<(i64, i64, u8)> = a.to_vec();
    for (r, c, _before, after) in germ {
        out.retain(|(rr, cc, _)| !(rr == r && cc == c)); // clear the cell
        if *after != 0 {
            out.push((*r, *c, *after));
        }
    }
    out.sort_unstable();
    out
}

/// Canonical bytes of a residual germ — its ANASTOME address payload.
pub fn germ_bytes(germ: &ResidualGerm) -> Vec<u8> {
    let mut out = Vec::with_capacity(germ.len() * 18);
    for (r, c, before, after) in germ {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*before);
        out.push(*after);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(mut v: Vec<(i64, i64, u8)>) -> Vec<(i64, i64, u8)> {
        v.sort_unstable();
        v
    }

    /// A non-conformal change (a single cell recoloured, others untouched — not a
    /// rigid motion of the body) germinates a residual germ that reproduces A→B.
    #[test]
    fn a_residual_germ_reproduces_its_demand() {
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // only (0,1): 2→9
        let g = germinate(&a, &b).expect("a change germinates");
        assert_eq!(g, vec![(0, 1, 2, 9)], "the germ IS the exact residual");
        assert_eq!(norm(apply(&g, &a)), norm(b.to_vec()), "it closes A→B (topological)");
    }

    /// Creation/annihilation is carried too (hue 0 on the absent side).
    #[test]
    fn creation_is_carried_as_a_residual() {
        let a = [(0, 0, 1u8)];
        let b = [(0, 0, 1u8), (0, 1, 5)]; // a cell appears
        let g = germinate(&a, &b).expect("appearance germinates");
        assert_eq!(g, vec![(0, 1, 0, 5)]);
        assert_eq!(norm(apply(&g, &a)), norm(b.to_vec()));
    }

    /// No net change → no germ owed.
    #[test]
    fn no_change_no_germ() {
        let a = [(0, 0, 1u8), (0, 1, 2)];
        assert!(germinate(&a, &a).is_none());
    }
}
