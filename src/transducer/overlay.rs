// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! OVERLAY — congruence as geometric superposition (the perception stack's
//! floor, where RE-COGNITION legitimately lives). Two geometries are smashed
//! together at their best transport; the CONGRUENT part is what coheres, and the
//! incongruent remainders are the directed coboundary BETWEEN the thoughts.
//! Partial overlap is SIGNAL, never failure — this replaces binary equality and
//! exact-subset matching as the recognition primitive. Exact, deterministic,
//! Vec-only. (Comprehension — the C-stack's decomposition of received language —
//! is NOT here; see `cognition::decompose_over`.)

/// One coboundary cell `(row, col, before, after)`.
pub type Cell = (i64, i64, u8, u8);

/// The result of superimposing B onto A at the best transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlay {
    /// The translation applied to B to reach the best congruence.
    pub transport: (i64, i64),
    /// The congruent pairs `(a_cell, transported_b_cell)` — the coherent part.
    pub shared: Vec<(Cell, Cell)>,
    /// A's cells left uncovered — the residual A holds against B.
    pub only_a: Vec<Cell>,
    /// B's cells (transported) left uncovered — the residual B holds against A.
    pub only_b: Vec<Cell>,
}

impl Overlay {
    /// The congruence measure — how much of the geometry coheres.
    pub fn congruent(&self) -> usize {
        self.shared.len()
    }
    /// Full congruence: the geometries coincide entirely at the transport.
    pub fn is_total(&self) -> bool {
        self.only_a.is_empty() && self.only_b.is_empty() && !self.shared.is_empty()
    }
}

/// Overlay with EXACT transitions: cells cohere only when position AND
/// (before, after) agree under the transport.
pub fn overlay(a: &[Cell], b: &[Cell]) -> Overlay {
    best_overlay(a, b, |x, y| x == y)
}

/// Overlay under the SHAPE lens (the quotient reframe — the ascend move's
/// eye): cells cohere on position alone, transitions free. This is how a
/// family (same geometry, different transition assignment) is SEEN.
pub fn overlay_shape(a: &[Cell], b: &[Cell]) -> Overlay {
    best_overlay(a, b, |x, y| (x.0, x.1) == (y.0, y.1))
}

/// Try every transport aligning some B-cell onto some A-cell; keep the one with
/// maximal congruence (ties: smallest transport row-major, so the overlay is
/// deterministic). O(|a|·|b|·|b|) exact — geometry, not hashing.
fn best_overlay(a: &[Cell], b: &[Cell], coheres: fn(&Cell, &Cell) -> bool) -> Overlay {
    let mut candidates: Vec<(i64, i64)> = Vec::new();
    for (ar, ac, _, _) in a {
        for (br, bc, _, _) in b {
            let t = (ar - br, ac - bc);
            if !candidates.contains(&t) {
                candidates.push(t);
            }
        }
    }
    if candidates.is_empty() {
        candidates.push((0, 0)); // one of the sides is empty — trivial transport
    }
    candidates.sort_unstable();
    let mut best: Option<Overlay> = None;
    for &(dr, dc) in &candidates {
        let moved: Vec<Cell> = b.iter().map(|(r, c, x, y)| (r + dr, c + dc, *x, *y)).collect();
        let mut shared = Vec::new();
        let mut only_a = Vec::new();
        let mut used = vec![false; moved.len()];
        for cell in a {
            match moved.iter().enumerate().find(|(k, m)| !used[*k] && coheres(cell, m)) {
                Some((k, m)) => {
                    used[k] = true;
                    shared.push((*cell, *m));
                }
                None => only_a.push(*cell),
            }
        }
        let only_b: Vec<Cell> =
            moved.iter().enumerate().filter(|(k, _)| !used[*k]).map(|(_, m)| *m).collect();
        let cand = Overlay { transport: (dr, dc), shared, only_a, only_b };
        let better = match &best {
            Some(cur) => cand.congruent() > cur.congruent(),
            None => true,
        };
        if better {
            best = Some(cand);
        }
    }
    best.unwrap_or(Overlay { transport: (0, 0), shared: Vec::new(), only_a: a.to_vec(), only_b: b.to_vec() })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Full congruence at a transport: the geometries coincide — total overlay.
    #[test]
    fn identical_shapes_cohere_totally_across_transport() {
        let a = vec![(0, 0, 1, 2), (0, 1, 3, 4)];
        let b = vec![(5, 7, 1, 2), (5, 8, 3, 4)]; // the same, far away
        let o = overlay(&a, &b);
        assert!(o.is_total(), "total congruence: {o:?}");
        assert_eq!(o.transport, (-5, -7), "the transport is read off the overlay");
    }

    /// Partial congruence is SIGNAL: the shared part coheres, both residuals are
    /// exact — the directed coboundary between the two thoughts.
    #[test]
    fn partial_overlap_splits_into_congruent_and_residual() {
        let a = vec![(0, 0, 1, 2), (0, 1, 3, 4), (1, 0, 5, 6)];
        let b = vec![(0, 0, 1, 2), (0, 1, 3, 4), (2, 2, 7, 8)];
        let o = overlay(&a, &b);
        assert_eq!(o.congruent(), 2, "two cells cohere");
        assert_eq!(o.only_a, vec![(1, 0, 5, 6)], "A's residual");
        assert_eq!(o.only_b, vec![(2, 2, 7, 8)], "B's residual");
    }

    /// The SHAPE lens sees a family: same positions, different transitions —
    /// full congruence under the quotient (this is how a puncture family is
    /// detected), while the exact lens sees no congruence at all.
    #[test]
    fn the_shape_lens_sees_the_family_the_exact_lens_cannot() {
        let a = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let b = vec![(3, 3, 5, 9), (3, 4, 5, 9)]; // same shape, other transitions
        assert!(overlay_shape(&a, &b).is_total(), "the quotient reframe coheres");
        assert_eq!(overlay(&a, &b).congruent(), 0, "the exact lens sees nothing shared");
    }

    /// Determinism: equal congruence at several transports resolves to the
    /// smallest transport — one overlay, every time.
    #[test]
    fn the_overlay_is_deterministic_under_ties() {
        let a = vec![(0, 0, 1, 2), (0, 2, 1, 2)];
        let b = vec![(0, 0, 1, 2)]; // fits at (0,0) or (0,2) equally
        let o = overlay(&a, &b);
        assert_eq!(o.transport, (0, 0), "ties collapse to the row-major least transport");
        assert_eq!(o.congruent(), 1);
    }
}
