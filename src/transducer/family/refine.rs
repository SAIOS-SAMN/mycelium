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
//!
//! NO CAP ON THE LCM (78.43(2), MU-DERIVATIONS D3.2): `29 → 30` is the exact
//! ratio `30/29` and has a finite refinement. What scales with the lcm is
//! what is WRITTEN, so the builders spend one read per run pair compared and
//! one per cell written, and the stroke a refinement needs is read off the
//! four extents before any cell ([`need`]), as a construction's is read off
//! its residue (78.111).

use crate::transducer::family::perceive::Grid;

/// WHY A PAIR HAS NO COMMON REFINEMENT, named. A refusal returns no chain
/// and no grid: a refinement is atomic by rollback of its result (the rule
/// 10 note in `physics/budget.rs`), never a truncated chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrefined {
    /// A grid with no rows or no columns embeds in nothing.
    Empty,
    /// A grid whose rows disagree in length is not the extent it claims.
    Ragged,
    /// The stroke ended before the refinement did; `need` is the stroke the
    /// four extents name ([`need`]).
    Spent { need: usize },
    /// The refinement's lcm or its need passes `usize`: no stroke this
    /// carrier writes can name it.
    Unbounded,
}

/// THE RUNS OF ONE AXIS, counted: the subdivision boundaries of both grids
/// in `(0, L)` are the multiples of `L/a` and of `L/b`, `(a − 1) + (b − 1)`
/// of them less the `gcd(a, b) − 1` they share, and the runs are one more:
/// `R(a, b) = a + b − gcd(a, b)`. `None` past `usize`.
pub fn runs_count(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)?.checked_sub(num_integer::gcd(a, b))
}

/// `lcm(a, b)`, checked: `None` on a zero extent or past `usize`.
fn lcm_of(a: usize, b: usize) -> Option<usize> {
    if a == 0 || b == 0 {
        return None;
    }
    a.checked_div(num_integer::gcd(a, b))?.checked_mul(b)
}

/// THE STROKE A REFINEMENT NEEDS, read off the four extents before any cell
/// (MU-DERIVATIONS D3.2): `R_h·R_w` run pairs compared, and at most
/// `L_h·L_w` cells written by the cochain and `L_h·L_w` by the pulled-back
/// grid, so `need = R_h·R_w + 2·L_h·L_w` bounds every spend a refinement
/// makes. `None` on a zero extent (no refinement, nothing to need) or past
/// `usize`.
pub fn need(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Option<usize> {
    let sites = lcm_of(h_in, h_out)?.checked_mul(lcm_of(w_in, w_out)?)?;
    let compared = runs_count(h_in, h_out)?.checked_mul(runs_count(w_in, w_out)?)?;
    compared.checked_add(sites.checked_mul(2)?)
}

/// THE NEED OF A PAIR OF GRIDS: [`need`] on the extents both grids claim.
/// `None` where either has no extent (empty or ragged): the refinement
/// refuses it by name before any read.
pub fn need_of(grid_in: &Grid, grid_out: &Grid) -> Option<usize> {
    let (h_in, w_in) = extent(grid_in)?;
    let (h_out, w_out) = extent(grid_out)?;
    need(h_in, w_in, h_out, w_out)
}

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
    /// The stroke this refinement needs ([`need`]), read once at `of`.
    pub need: usize,
}

impl CommonRefinement {
    /// `Empty` on an empty extent: a grid with no rows or no columns embeds
    /// in nothing, and a refinement of it would be a complex invented for a
    /// scene that is not there (rule 11). `Unbounded` where the lcm or the
    /// need passes `usize`. No other refusal: the lcm is not capped.
    pub fn of(h_in: usize, w_in: usize, h_out: usize, w_out: usize) -> Result<CommonRefinement, Unrefined> {
        if h_in == 0 || w_in == 0 || h_out == 0 || w_out == 0 {
            return Err(Unrefined::Empty);
        }
        let (Some(lcm_h), Some(lcm_w), Some(need)) =
            (lcm_of(h_in, h_out), lcm_of(w_in, w_out), need(h_in, w_in, h_out, w_out))
        else {
            return Err(Unrefined::Unbounded);
        };
        Ok(CommonRefinement {
            lcm_h,
            lcm_w,
            scale_in: (lcm_h / h_in, lcm_w / w_in),
            scale_out: (lcm_h / h_out, lcm_w / w_out),
            need,
        })
    }

    /// The refusal of a spent stroke, naming the stroke the extents need.
    fn spent(&self) -> Unrefined {
        Unrefined::Spent { need: self.need }
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
        Self::of(h_in, w_in, h_out, w_out).ok()?;
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
    /// `Ragged` where either grid is ragged: a row shorter than the frame
    /// says the grid is not the extent it claims, and reading a hue past it
    /// would be inventing one (rule 11).
    ///
    /// METERED, NEVER CAPPED (rule 10): one read per run pair compared and
    /// one per cell written, at most `R_h·R_w + L_h·L_w`; a stroke that ends
    /// first refuses `Spent { need }` and returns no chain.
    pub fn bipartite_cochain(
        &self,
        grid_in: &Grid,
        grid_out: &Grid,
        budget: &mut crate::physics::Budget,
    ) -> Result<Vec<(i64, i64, u8, u8)>, Unrefined> {
        let (s_r_in, s_c_in) = self.scale_in;
        let (s_r_out, s_c_out) = self.scale_out;
        let rows = Self::runs(self.lcm_h, s_r_in, s_r_out);
        let cols = Self::runs(self.lcm_w, s_c_in, s_c_out);
        let mut out = Vec::new();
        for &(r0, rn, r_in, r_out) in &rows {
            let before_row = grid_in.get(r_in).ok_or(Unrefined::Ragged)?;
            let after_row = grid_out.get(r_out).ok_or(Unrefined::Ragged)?;
            for &(c0, cn, c_in, c_out) in &cols {
                if !budget.spend() {
                    return Err(self.spent());
                }
                let before = *before_row.get(c_in).ok_or(Unrefined::Ragged)?;
                let after = *after_row.get(c_out).ok_or(Unrefined::Ragged)?;
                if before == after {
                    continue;
                }
                // The run is constant, so every coordinate in it carries the
                // same transition. They are written out because a chain is
                // links: the run is how the answer was COMPUTED, not a
                // compression of what it says.
                for r in r0..r0 + rn {
                    for c in c0..c0 + cn {
                        if !budget.spend() {
                            return Err(self.spent());
                        }
                        out.push((r as i64, c as i64, before, after));
                    }
                }
            }
        }
        Ok(out)
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
    extent_or_refusal(g).ok()
}

/// [`extent`] with its refusal named: `Ragged` where the rows disagree,
/// `Empty` where there is no row or no column.
fn extent_or_refusal(g: &Grid) -> Result<(usize, usize), Unrefined> {
    let mut w: Option<usize> = None;
    for row in g {
        match w {
            None => w = Some(row.len()),
            Some(n) if n == row.len() => {}
            Some(_) => return Err(Unrefined::Ragged),
        }
    }
    match w {
        Some(w) if w > 0 => Ok((g.len(), w)),
        _ => Err(Unrefined::Empty),
    }
}

/// THE COMMON REFINEMENT OF TWO GRIDS, read off the extents both claim:
/// `Ragged` or `Empty` named before any read.
pub fn of_grids(grid_in: &Grid, grid_out: &Grid) -> Result<CommonRefinement, Unrefined> {
    let (hi, wi) = extent_or_refusal(grid_in)?;
    let (ho, wo) = extent_or_refusal(grid_out)?;
    CommonRefinement::of(hi, wi, ho, wo)
}

impl CommonRefinement {
    /// ONE GRID ON `K_lcm` at its scale, one read per cell written; a stroke
    /// that ends first refuses `Spent { need }` and returns no grid.
    fn lift(
        &self,
        g: &Grid,
        (sr, sc): (usize, usize),
        budget: &mut crate::physics::Budget,
    ) -> Result<Grid, Unrefined> {
        let mut out: Grid = Vec::with_capacity(self.lcm_h);
        for r in 0..self.lcm_h {
            let src = g.get(r / sr).ok_or(Unrefined::Ragged)?;
            let mut row = Vec::with_capacity(self.lcm_w);
            for c in 0..self.lcm_w {
                if !budget.spend() {
                    return Err(self.spent());
                }
                row.push(*src.get(c / sc).ok_or(Unrefined::Ragged)?);
            }
            out.push(row);
        }
        Ok(out)
    }

    /// THE INPUT PULLED BACK ONTO `K_lcm`: [`Self::refined`]'s input half,
    /// `L_h·L_w` reads.
    pub fn pulled_back(&self, grid_in: &Grid, budget: &mut crate::physics::Budget) -> Result<Grid, Unrefined> {
        self.lift(grid_in, self.scale_in, budget)
    }

    /// BOTH GRIDS ON `K_lcm`, by exact block replication: an input site
    /// covers `s_in` rows and columns of the refinement and an output site
    /// covers `s_out`, so every coordinate of `K_lcm` takes the hue of the
    /// site whose block contains it. Nothing is sampled, averaged or
    /// rounded — the division is exact by construction. One read per cell
    /// written, `2·L_h·L_w`.
    ///
    /// This is what lets the refinement change nothing downstream: the pair
    /// that comes back is site-for-site, so the scene reading and everything
    /// after it read an ordinary equal-extent pair.
    pub fn refined(
        &self,
        grid_in: &Grid,
        grid_out: &Grid,
        budget: &mut crate::physics::Budget,
    ) -> Result<(Grid, Grid), Unrefined> {
        Ok((self.lift(grid_in, self.scale_in, budget)?, self.lift(grid_out, self.scale_out, budget)?))
    }
}

/// THE EXACT COBOUNDARY OF A PAIR: both grids embedded in the grids' common
/// refinement K_lcm without sampling, rounding or interpolation, under the
/// caller's stroke (sized by [`need_of`]).
pub fn refine_delta(
    grid_in: &Grid,
    grid_out: &Grid,
    budget: &mut crate::physics::Budget,
) -> Result<Vec<(i64, i64, u8, u8)>, Unrefined> {
    of_grids(grid_in, grid_out)?.bipartite_cochain(grid_in, grid_out, budget)
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
