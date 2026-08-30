// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE VOLUMETRIC BLADE — perception → graded Cl(3,1) body.
//!
//! A perceived scene segments into connected components; each lifts to a
//! polytonic BLADE: a non-null dual SPHERE (guaranteed invertible, so the
//! geometric quotient forms) carrying four ORTHOGONAL strata — spatial extent
//! (𝒮, a Cl(3,1) multivector), gauge hue (𝒢), temporal index (𝒯), shell
//! topology (𝒽). Ported from the validated mycelium transducer and RE-GROUNDED
//! for the Anti-Flattening Law: gauge and cells are `Vec`-of-relations, never
//! maps — this is cognition, so no `BTreeMap`/`BTreeSet` lives here.

use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_rational::BigRational;

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}
fn qn(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// A polytonic volumetric body. The four strata are ORTHOGONAL lanes (never
/// summed — the Dershowitz–Manna law); only the SPATIAL lane is a Cl(3,1)
/// multivector (versor-acted). Gauge/time/topology pass through rigid motion
/// unchanged — that orthogonality IS the polytonic separation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blade {
    /// 𝒮 — the dual sphere `X(c) − ½r²e_∞`, non-null (`S·S = r² ≠ 0`) hence
    /// invertible: the geometric quotient can form.
    pub spatial: Mv,
    /// 𝒮 refinement — the EXACT second-moment (inertia) tensor about the
    /// centroid `(Ixx, Iyy, Ixy) ∈ ℚ³`, rotation-covariant, no eigen/roots.
    pub inertia: [BigRational; 3],
    /// 𝒢 — the gauge/hue content as sorted `(hue, count)` relations (Vec, not a
    /// map: cognition obeys the Anti-Flattening Law). Versor-invariant.
    pub gauge: Vec<(u8, BigRational)>,
    /// 𝒯 — the temporal sequence index (which scene state raised this body).
    pub temporal: BigRational,
    /// 𝒽 — the shell topology: the component's cell count.
    pub shell: BigRational,
    /// The body's support — occupied `(row, col)` cells, sorted (Vec, not a set).
    pub cells: Vec<(i64, i64)>,
}

impl Blade {
    /// The centroid read back from the homogeneous sphere — the exact ℚ² base.
    pub fn centroid(&self) -> Option<(BigRational, BigRational)> {
        cga::base(&self.spatial)
    }

    /// The squared extent `r² = S·S` — always `> 0`, so the body is invertible.
    pub fn radius2(&self) -> BigRational {
        self.spatial.norm2()
    }

    /// Every extracted body is invertible by construction (`r² ≥ 1/6 > 0`).
    pub fn is_invertible(&self) -> bool {
        self.spatial.versor_inverse().is_some()
    }

    /// The polytonic four-lane grade `(𝒮, 𝒢, 𝒯, 𝒽) ∈ ℚ⁴` — orthogonal
    /// magnitudes, NEVER summed: spatial extent, gauge complexity (distinct
    /// hues), temporal index, shell size.
    pub fn mu4(&self) -> [BigRational; 4] {
        [
            self.radius2(),
            q(self.gauge.len() as i64),
            self.temporal.clone(),
            self.shell.clone(),
        ]
    }

    /// Apply a spatial versor to the SPATIAL lane only — a rigid motion moves the
    /// body coherently (`V σ Ṽ`) while gauge/time/topology pass through.
    pub fn sandwiched(&self, v: &Mv) -> Blade {
        Blade {
            spatial: v.sandwich(&self.spatial),
            inertia: self.inertia.clone(),
            gauge: self.gauge.clone(),
            temporal: self.temporal.clone(),
            shell: self.shell.clone(),
            cells: self.cells.clone(),
        }
    }

    /// The rigid-motion invariant key — strictly preserved by translations,
    /// rotations AND reflections, so it correspond-pairs bodies with zero spatial
    /// search: cell count, `Tr(I)`, `det(I)` (both O(2)-invariant, exact ℚ, no
    /// eigen), and the LABEL-AGNOSTIC hue profile (sorted per-hue counts — a
    /// recolour still matches by shape, the relabel falling out as `Δ𝒢`).
    pub fn invariant(&self) -> InvariantSignature {
        let (ixx, iyy, ixy) = (&self.inertia[0], &self.inertia[1], &self.inertia[2]);
        let mut hue_profile: Vec<BigRational> = self.gauge.iter().map(|(_, c)| c.clone()).collect();
        hue_profile.sort();
        InvariantSignature {
            cell_count: self.shell.clone(),
            tr_i: ixx + iyy,
            det_i: ixx * iyy - ixy * ixy,
            hue_profile,
        }
    }
}

/// The rigid-motion invariant signature of a body (see [`Blade::invariant`]).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvariantSignature {
    pub cell_count: BigRational,
    pub tr_i: BigRational,
    pub det_i: BigRational,
    pub hue_profile: Vec<BigRational>,
}

/// Lift one connected component `(row, col, hue)` into a volumetric blade —
/// exact ℚ throughout. The component's cells need not be sorted.
pub fn lift(comp: &[(i64, i64, u8)], time_index: i64) -> Blade {
    let n = comp.len().max(1) as i64;
    let n_q = q(n);

    // Centroid c = (Σrow, Σcol) / n ∈ ℚ².
    let sr: i64 = comp.iter().map(|(r, _, _)| *r).sum();
    let sc: i64 = comp.iter().map(|(_, c, _)| *c).sum();
    let c1 = BigRational::new(BigInt::from(sr), BigInt::from(n));
    let c2 = BigRational::new(BigInt::from(sc), BigInt::from(n));

    // Second moments about the centroid (exact ℚ) — the inertia tensor.
    let (mut ixx, mut iyy, mut ixy) = (q(0), q(0), q(0));
    for (r, c, _) in comp {
        let dr = q(*r) - &c1;
        let dc = q(*c) - &c2;
        ixx += &dr * &dr;
        iyy += &dc * &dc;
        ixy += &dr * &dc;
    }

    // Extent r² = per-unit-mass polar 2nd moment + the unit cell's own polar
    // moment (1/6), so r² > 0 even for a single cell — off the null cone,
    // invertible.
    let r2 = (&ixx + &iyy) / &n_q + qn(1, 6);
    let spatial = cga::sphere(&c1, &c2, &r2);

    // Gauge: the hue multiset as sorted (hue, count) relations — Vec, not a map.
    let mut gauge: Vec<(u8, BigRational)> = Vec::new();
    for (_, _, h) in comp {
        match gauge.iter_mut().find(|(hue, _)| hue == h) {
            Some((_, count)) => *count += q(1),
            None => gauge.push((*h, q(1))),
        }
    }
    gauge.sort_by_key(|(h, _)| *h);

    let mut cells: Vec<(i64, i64)> = comp.iter().map(|(r, c, _)| (*r, *c)).collect();
    cells.sort_unstable();
    cells.dedup();

    Blade { spatial, inertia: [ixx, iyy, ixy], gauge, temporal: q(time_index), shell: n_q, cells }
}

/// 4-connected components of a scene, as `Vec`-of-`Vec` (no maps/sets — Law II).
/// Two cells join iff they are orthogonally adjacent, regardless of hue (the
/// object is a shape; the relabel is read afterward as `Δ𝒢`).
pub fn components(cells: &[(i64, i64, u8)]) -> Vec<Vec<(i64, i64, u8)>> {
    let n = cells.len();
    let mut visited = vec![false; n];
    let mut out: Vec<Vec<(i64, i64, u8)>> = Vec::new();
    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut comp: Vec<(i64, i64, u8)> = Vec::new();
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(i) = stack.pop() {
            let (r, c, _) = cells[i];
            comp.push(cells[i]);
            for (j, &(rj, cj, _)) in cells.iter().enumerate() {
                if !visited[j] && (r - rj).abs() + (c - cj).abs() == 1 {
                    visited[j] = true;
                    stack.push(j);
                }
            }
        }
        out.push(comp);
    }
    out
}

/// Transduce a whole scene: segment into components, lift each to a blade at
/// `time_index`. The transducer output for one scene state.
pub fn transduce(cells: &[(i64, i64, u8)], time_index: i64) -> Vec<Blade> {
    components(cells).iter().map(|comp| lift(comp, time_index)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(v: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
        v.to_vec()
    }

    /// A 2×2 square lifts to a blade with the exact centroid (½,½), positive
    /// extent, and an invertible spatial sphere — the body HAS volume.
    #[test]
    fn a_square_lifts_to_an_invertible_body() {
        let b = lift(&cells(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]), 0);
        assert_eq!(b.centroid(), Some((qn(1, 2), qn(1, 2))));
        assert!(b.radius2() > q(0), "the body has volume (off the null cone)");
        assert!(b.is_invertible());
        assert_eq!(b.shell, q(4));
        assert_eq!(b.gauge, vec![(1u8, q(4))]);
    }

    /// A lone cell still HAS volume (r² = 1/6) — no body is a null point.
    #[test]
    fn a_lone_cell_has_volume() {
        let b = lift(&cells(&[(3, 5, 7)]), 0);
        assert_eq!(b.radius2(), qn(1, 6));
        assert!(b.is_invertible());
    }

    /// Segmentation is by adjacency, hue-blind: two touching cells of different
    /// hues are ONE body (the recolour is Δ𝒢, read afterward); a gap splits.
    #[test]
    fn components_join_by_adjacency_not_hue() {
        let one = components(&cells(&[(0, 0, 1), (0, 1, 2)]));
        assert_eq!(one.len(), 1, "adjacent cells are one body regardless of hue");
        let two = components(&cells(&[(0, 0, 1), (0, 5, 1)]));
        assert_eq!(two.len(), 2, "a gap separates bodies");
    }

    /// A translated square is the SAME body up to rigid motion: identical
    /// invariant signature (the correspond key), positions aside.
    #[test]
    fn a_translate_shares_the_invariant() {
        let a = lift(&cells(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]), 0);
        let b = lift(&cells(&[(4, 7, 1), (4, 8, 1), (5, 7, 1), (5, 8, 1)]), 1);
        assert_eq!(a.invariant(), b.invariant(), "a translate is the same orbit");
    }
}
