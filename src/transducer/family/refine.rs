// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! #97, law 5, law 11 — THE COMMON REFINEMENT COMPLEX: two grids of unequal
//! extent read on one cell complex, exactly, so a reshaped pair is a chain
//! and not a bucket of points.
//!
//! When `h_in × w_in ≠ h_out × w_out` the pairing has no cell-for-cell
//! correspondence to read a transition on, and perception falls to
//! `Demand::TopologicalDeformation` — a set of cells with the relation
//! between them dropped. The relation is not missing; it is a RATIO, and a
//! ratio of integers is exact.
//!
//! `K_lcm = lcm(h_in, h_out) × lcm(w_in, w_out)` is the coarsest complex both
//! grids embed in without rounding: each input cell is an `s_in` block of it
//! and each output cell an `s_out` block, both integer. On `K_lcm` the two
//! grids are cell-for-cell and the transition is the ordinary chain the rest
//! of the engine already reads.

use crate::transducer::family::perceive::Grid;

/// The embedding of two grids in their common refinement. All four scales
/// are exact integers — `lcm(a, b) / a` divides by construction — so nothing
/// here rounds, samples or interpolates.
#[derive(Clone, Copy, Debug)]
pub struct CommonRefinement {
    pub lcm_h: usize,
    pub lcm_w: usize,
    /// Rows and columns of `K_lcm` per input cell.
    pub scale_in: (usize, usize),
    /// Rows and columns of `K_lcm` per output cell.
    pub scale_out: (usize, usize),
}

impl CommonRefinement {
    /// `None` on an empty extent: a grid with no rows or no columns embeds in
    /// nothing, and a refinement of it would be a complex invented for a
    /// scene that is not there (rule 11).
    pub fn of(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Option<CommonRefinement> {
        if h_in == 0 || w_in == 0 || h_out == 0 || w_out == 0 {
            return None;
        }
        let lcm_h = num_integer::lcm(h_in, h_out);
        let lcm_w = num_integer::lcm(w_in, w_out);
        Some(CommonRefinement {
            lcm_h,
            lcm_w,
            scale_in: (lcm_h / h_in, lcm_w / w_in),
            scale_out: (lcm_h / h_out, lcm_w / w_out),
        })
    }

    /// THE FRAME RATIO the refinement measures, as a pair of exact rationals
    /// `(h_out/h_in, w_out/w_in)`. It is the dilation the pair stands at and
    /// it rides on the carriage, never on a float and never rounded.
    pub fn ratio(
        h_in: usize,
        w_in: usize,
        h_out: usize,
        w_out: usize,
    ) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        if h_in == 0 || w_in == 0 {
            return None;
        }
        let q = |n: usize, d: usize| {
            num_rational::BigRational::new(
                num_bigint::BigInt::from(n),
                num_bigint::BigInt::from(d),
            )
        };
        Some((q(h_out, h_in), q(w_out, w_in)))
    }

    /// Is this pair already cell-for-cell? Then the refinement is the grids
    /// themselves and there is nothing for this organ to do.
    pub fn is_trivial(&self) -> bool {
        self.scale_in == (1, 1) && self.scale_out == (1, 1)
    }

    /// THE RUNS OF ONE AXIS: the intervals of `K_lcm` on which BOTH source
    /// cells are constant, as `(start, len, i_in, i_out)`.
    ///
    /// READ AS RUNS, NOT AS CELLS, and this is the difference between an
    /// organ that is bounded by its objects and one that is bounded by their
    /// least common multiple. `lcm(29, 30)` is 870 for two grids of thirty
    /// rows; over both axes that is three quarters of a million coordinates
    /// to visit to answer a question about at most sixty boundaries.
    ///
    /// The pair `(i_in, i_out)` changes only where a subdivision boundary
    /// falls, and the boundaries are the multiples of `s_in` and of `s_out` —
    /// at most `h_in + h_out` of them. So the runs ARE the two grids' own
    /// boundaries, which is what rule 10 allows a loop to be bounded by, and
    /// the answer is identical because the pair is constant between them.
    fn runs(n: usize, s_in: usize, s_out: usize) -> Vec<(usize, usize, usize, usize)> {
        let mut out = Vec::new();
        let mut at = 0usize;
        while at < n {
            let i_in = at / s_in;
            let i_out = at / s_out;
            // The next boundary on either side, which is where the pair can
            // change and nowhere else.
            let next_in = (i_in + 1) * s_in;
            let next_out = (i_out + 1) * s_out;
            let end = next_in.min(next_out).min(n);
            out.push((at, end - at, i_in, i_out));
            at = end;
        }
        out
    }

    /// THE BIPARTITE COCHAIN ∂₁Q over `K_lcm`: one cell per coordinate where
    /// the two grids disagree, carrying the hue before and after.
    ///
    /// This is the same object `word_or_demand` reads on an equal-extent
    /// pair — `(row, col, hue_before, hue_after)` — so a reshaped pair
    /// reaches the rest of the engine as an ordinary chain and the curvature
    /// between the frames survives perception.
    ///
    /// `None` where either grid is ragged: a row shorter than the frame says
    /// the grid is not the extent it claims, and reading a hue past it would
    /// be inventing one (rule 11).
    pub fn bipartite_cochain(
        &self,
        grid_in: &Grid,
        grid_out: &Grid,
    ) -> Option<Vec<(i64, i64, u8, u8)>> {
        let (s_r_in, s_c_in) = self.scale_in;
        let (s_r_out, s_c_out) = self.scale_out;
        let rows = Self::runs(self.lcm_h, s_r_in, s_r_out);
        let cols = Self::runs(self.lcm_w, s_c_in, s_c_out);
        let mut out = Vec::new();
        for &(r0, rn, r_in, r_out) in &rows {
            let before_row = grid_in.get(r_in)?;
            let after_row = grid_out.get(r_out)?;
            for &(c0, cn, c_in, c_out) in &cols {
                let before = *before_row.get(c_in)?;
                let after = *after_row.get(c_out)?;
                if before == after {
                    continue;
                }
                // The run is constant, so every coordinate in it carries the
                // same transition. They are written out because a chain is
                // cells: the run is how the answer was COMPUTED, not a
                // compression of what it says.
                for r in r0..r0 + rn {
                    for c in c0..c0 + cn {
                        out.push((r as i64, c as i64, before, after));
                    }
                }
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// #97 — TWO GRIDS OF UNEQUAL EXTENT READ ONE COMPLEX, exactly.
    ///
    /// A 2×2 doubled to 4×4 has `lcm = 4`: the input embeds two-for-one and
    /// the output one-for-one, so a cell that changed hue contributes its
    /// whole block and one that did not contributes nothing.
    #[test]
    fn a_doubled_grid_refines_without_rounding() {
        let r = CommonRefinement::of(2, 2, 4, 4).expect("both extents stand");
        assert_eq!((r.lcm_h, r.lcm_w), (4, 4));
        assert_eq!(r.scale_in, (2, 2), "each input cell is a 2×2 block of K_lcm");
        assert_eq!(r.scale_out, (1, 1), "each output cell is one cell of K_lcm");
        assert!(!r.is_trivial());

        // Input all hue 1; output all hue 1 but for its top-left cell.
        let a: Grid = vec![vec![1, 1], vec![1, 1]];
        let mut b: Grid = vec![vec![1; 4]; 4];
        b[0][0] = 7;
        let chain = r.bipartite_cochain(&a, &b).expect("both grids are square");
        assert_eq!(chain.len(), 1, "one output cell differs, so one cell of K_lcm does");
        assert_eq!(chain[0], (0, 0, 1, 7));
    }

    /// THE RATIO IS EXACT ℚ and never a float: 3 rows to 2 is three halves,
    /// which has no binary expansion and must not acquire one.
    #[test]
    fn the_frame_ratio_is_an_exact_rational() {
        let (rh, rw) = CommonRefinement::ratio(2, 4, 3, 2).expect("extents stand");
        assert_eq!(rh, num_rational::BigRational::new(3.into(), 2.into()));
        assert_eq!(rw, num_rational::BigRational::new(2.into(), 4.into()));
    }

    /// RUNS ARE THE BOUNDARIES OF THE TWO GRIDS, not the cells of their least
    /// common multiple. Coprime extents are where that matters: 3 and 4 have
    /// `lcm` 12, and the pair changes at six places, not twelve.
    #[test]
    fn the_runs_are_bounded_by_the_grids_and_not_by_their_lcm() {
        let r = CommonRefinement::of(3, 1, 4, 1).expect("extents stand");
        assert_eq!(r.lcm_h, 12);
        let runs = CommonRefinement::runs(12, r.scale_in.0, r.scale_out.0);
        assert!(
            runs.len() <= 3 + 4,
            "at most one run per subdivision boundary of either grid, got {}",
            runs.len()
        );
        // And they tile the axis exactly — no coordinate is skipped or
        // counted twice.
        let total: usize = runs.iter().map(|&(_, n, _, _)| n).sum();
        assert_eq!(total, 12, "the runs cover K_lcm exactly once");
        for &(at, n, i_in, i_out) in &runs {
            assert_eq!(at / r.scale_in.0, i_in, "the run's input cell is constant on it");
            assert_eq!(at / r.scale_out.0, i_out, "and so is its output cell");
            assert_eq!(
                (at + n - 1) / r.scale_in.0,
                i_in,
                "constant at the run's far end too"
            );
        }
    }

    /// An extent of zero embeds in nothing, and a refinement of it would be a
    /// complex invented for a scene that is not there (rule 11).
    #[test]
    fn an_empty_extent_has_no_refinement() {
        assert!(CommonRefinement::of(0, 2, 2, 2).is_none());
        assert!(CommonRefinement::of(2, 0, 2, 2).is_none());
        assert!(CommonRefinement::of(2, 2, 0, 2).is_none());
        assert!(CommonRefinement::of(2, 2, 2, 0).is_none());
        assert!(CommonRefinement::ratio(0, 2, 2, 2).is_none());
    }

    /// A ragged grid is not the extent it claims, and reading a hue past its
    /// end would be inventing one.
    #[test]
    fn a_ragged_grid_is_refused_rather_than_padded() {
        let r = CommonRefinement::of(2, 2, 2, 2).expect("extents stand");
        let a: Grid = vec![vec![1, 1], vec![1]];
        let b: Grid = vec![vec![2, 2], vec![2, 2]];
        assert!(r.bipartite_cochain(&a, &b).is_none());
    }
}
