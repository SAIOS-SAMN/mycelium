// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! #97, law 5, law 11 — THE COMMON REFINEMENT COMPLEX: two grids of unequal
//! extent read on one cube complex, exactly, so a reshaped pair is a chain
//! and not a bucket of points.
//!
//! When `h_in × w_in ≠ h_out × w_out` the pairing has no site-for-site
//! correspondence to read a transition on, and perception falls to
//! `Demand::TopologicalDeformation` — a set of links with the relation
//! between them dropped. The relation is not missing; it is a RATIO, and a
//! ratio of integers is exact.
//!
//! `K_lcm = lcm(h_in, h_out) × lcm(w_in, w_out)` is the coarsest complex both
//! grids embed in without rounding: each input site is an `s_in` block of it
//! and each output site an `s_out` block, both integer. On `K_lcm` the two
//! grids are site-for-site and the transition is the ordinary chain the rest
//! of the engine already reads.

use crate::transducer::family::perceive::Grid;

/// The embedding of two grids in the grids' common refinement. All four scales
/// are exact integers — `lcm(a, b) / a` divides by construction — so nothing
/// here rounds, samples or interpolates.
#[derive(Clone, Copy, Debug)]
pub struct CommonRefinement {
    pub lcm_h: usize,
    pub lcm_w: usize,
    /// Rows and columns of `K_lcm` per input site.
    pub scale_in: (usize, usize),
    /// Rows and columns of `K_lcm` per output site.
    pub scale_out: (usize, usize),
}

/// THE MAXIMUM SITES IN A COMMON REFINEMENT COMPLEX (rule 10).
/// Grids in the ARC sensor plane are at most 30×30. Real geometric
/// homotheties (e.g. 2:3, 3:4, 1:2) have lcm ≤ 60 and |K_lcm| ≤ 3600.
/// Incommensurate frame transitions (e.g. 29×30 with lcm = 870 and
/// 756,900 coordinates) are not refinements on this base and refuse
/// (rule 11) rather than exploding memory and economic pay.
pub const MAX_LCM_AXIS: usize = 60;
pub const MAX_LCM_SITES: usize = 3600;

impl CommonRefinement {
    /// `None` on an empty extent: a grid with no rows or no columns embeds in
    /// nothing, and a refinement of it would be a complex invented for a
    /// scene that is not there (rule 11).
    ///
    /// Also `None` when the least common multiple exceeds `MAX_LCM_AXIS` or
    /// `MAX_LCM_SITES`: incommensurate grids have no finite common refinement
    /// on this base and refuse under rule 10.
    pub fn of(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Option<CommonRefinement> {
        if h_in == 0 || w_in == 0 || h_out == 0 || w_out == 0 {
            return None;
        }
        let lcm_h = num_integer::lcm(h_in, h_out);
        let lcm_w = num_integer::lcm(w_in, w_out);
        if lcm_h > MAX_LCM_AXIS || lcm_w > MAX_LCM_AXIS || lcm_h * lcm_w > MAX_LCM_SITES {
            return None;
        }
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
        Self::of(h_in, w_in, h_out, w_out)?;
        let q = |n: usize, d: usize| {
            num_rational::BigRational::new(
                num_bigint::BigInt::from(n),
                num_bigint::BigInt::from(d),
            )
        };
        Some((q(h_out, h_in), q(w_out, w_in)))
    }

    /// Is this pair already site-for-site? Then the refinement is the grids
    /// themselves and there is nothing for this organ to do.
    pub fn is_trivial(&self) -> bool {
        self.scale_in == (1, 1) && self.scale_out == (1, 1)
    }

    /// THE RUNS OF ONE AXIS: the intervals of `K_lcm` on which BOTH source
    /// sites are constant, as `(start, len, i_in, i_out)`.
    ///
    /// READ AS RUNS, NOT AS SITES, and this is the difference between an
    /// organ that is bounded by its objects and one that is bounded by the objects'
    /// least common multiple. `lcm(29, 30)` is 870 for two grids of thirty
    /// rows; over both axes that is three quarters of a million coordinates
    /// to visit to answer a question about at most sixty boundaries.
    ///
    /// The pair `(i_in, i_out)` changes only where a subdivision boundary
    /// falls, and the boundaries are the multiples of `s_in` and of `s_out` —
    /// at most `h_in + h_out` of them. So the runs ARE the two grids' own
    /// boundaries, which is what rule 10 allows a loop to be bounded by, and
    /// the answer is identical because the pair is constant between them.
    pub(super) fn runs(n: usize, s_in: usize, s_out: usize) -> Vec<(usize, usize, usize, usize)> {
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

    /// THE BIPARTITE COCHAIN ∂₁Q over `K_lcm`: one link per coordinate where
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
                // links: the run is how the answer was COMPUTED, not a
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

/// THE EXTENT A GRID CLAIMS, refusing a ragged one. Rows are counted and
/// every row's width is read: a grid whose rows disagree is not a rectangle
/// and has no extent, which is a refusal and not a widest row (rule 11).
///
/// The width is read from the rows themselves rather than from one of them —
/// a row taken by position is a choice about arrival order (rule 15), and
/// here the question is whether they AGREE.
pub fn extent(g: &Grid) -> Option<(usize, usize)> {
    let mut w: Option<usize> = None;
    for row in g {
        match w {
            None => w = Some(row.len()),
            Some(n) if n == row.len() => {}
            Some(_) => return None,
        }
    }
    let w = w?;
    (!g.is_empty() && w > 0).then_some((g.len(), w))
}

impl CommonRefinement {
    /// BOTH GRIDS ON `K_lcm`, by exact block replication: an input site
    /// covers `s_in` rows and columns of the refinement and an output site
    /// covers `s_out`, so every coordinate of `K_lcm` takes the hue of the
    /// site whose block contains it. Nothing is sampled, averaged or
    /// rounded — the division is exact by construction.
    ///
    /// This is what lets the refinement change nothing downstream: the pair
    /// that comes back is site-for-site, so the scene reading and everything
    /// after it read an ordinary equal-extent pair.
    pub fn refined(&self, grid_in: &Grid, grid_out: &Grid) -> Option<(Grid, Grid)> {
        let lift = |g: &Grid, (sr, sc): (usize, usize)| -> Option<Grid> {
            let mut out: Grid = Vec::with_capacity(self.lcm_h);
            for r in 0..self.lcm_h {
                let src = g.get(r / sr)?;
                let mut row = Vec::with_capacity(self.lcm_w);
                for c in 0..self.lcm_w {
                    row.push(*src.get(c / sc)?);
                }
                out.push(row);
            }
            Some(out)
        };
        Some((lift(grid_in, self.scale_in)?, lift(grid_out, self.scale_out)?))
    }
}

/// THE EXACT COBOUNDARY OF A PAIR: both grids embedded in the grids' common
/// refinement K_lcm without sampling, rounding or interpolation.
pub fn refine_delta(grid_in: &Grid, grid_out: &Grid) -> Option<Vec<(i64, i64, u8, u8)>> {
    let (hi, wi) = extent(grid_in)?;
    let (ho, wo) = extent(grid_out)?;
    let refn = CommonRefinement::of(hi, wi, ho, wo)?;
    refn.bipartite_cochain(grid_in, grid_out)
}

/// THE FAMILY FRAME RATIO: the pair of exact rational scales (rh, rw) that
/// every pair of the family agrees on, measured on the common refinement.
pub fn family_ratio(family: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in family {
        let (hi, wi) = extent(a)?;
        let (ho, wo) = extent(b)?;
        let (rh, rw) = CommonRefinement::ratio(hi, wi, ho, wo)?;
        let r = (
            (usize::try_from(rh.numer()).ok()?, usize::try_from(rh.denom()).ok()?),
            (usize::try_from(rw.numer()).ok()?, usize::try_from(rw.denom()).ok()?),
        );
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}
