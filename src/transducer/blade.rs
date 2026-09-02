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
use num_traits::Zero;

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

    // Second moments about the centroid (exact ℚ) — the inertia tensor of the
    // BODY: the union of unit squares, so each cell carries its own moment
    // 1/12 about its center on each axis (the parallel-axis law), never the
    // point moments of cell centers. This is what makes the moments scale as
    // a body does under dilation (ρ² per cell, ρ⁴ in total).
    let (mut ixx, mut iyy, mut ixy) = (q(0), q(0), q(0));
    for (r, c, _) in comp {
        let dr = q(*r) - &c1;
        let dc = q(*c) - &c2;
        ixx += &dr * &dr + qn(1, 12);
        iyy += &dc * &dc + qn(1, 12);
        ixy += &dr * &dc;
    }

    // Extent r² = per-unit-mass polar 2nd moment + the unit cell's own polar
    // moment (1/6), so r² > 0 even for a single cell — off the null cone,
    // invertible.
    let r2 = (&ixx + &iyy) / &n_q; // the body's own moment is already inside
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

// ---- THE SHELL: boundary corners, the similarity-invariant chart, versors ----

/// The boundary of a cell set on the vertex lattice: every unit edge with
/// the set on exactly one side. Returns (horizontal edges from (r,c) to
/// (r,c+1), vertical edges from (r,c) to (r+1,c)), each as its start vertex.
fn boundary_edges(cells: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let held = |r: i64, c: i64| cells.binary_search(&(r, c)).is_ok();
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for &(r, c) in cells {
        // Top edge (r, c)-(r, c+1): boundary iff the cell above is absent.
        if !held(r - 1, c) {
            h.push((r, c));
        }
        // Bottom edge (r+1, c)-(r+1, c+1).
        if !held(r + 1, c) {
            h.push((r + 1, c));
        }
        // Left edge (r, c)-(r+1, c).
        if !held(r, c - 1) {
            v.push((r, c));
        }
        // Right edge (r, c+1)-(r+1, c+1).
        if !held(r, c + 1) {
            v.push((r, c + 1));
        }
    }
    h.sort_unstable();
    v.sort_unstable();
    (h, v)
}

/// THE CORNERS of a cell set: boundary vertices where a horizontal and a
/// vertical boundary edge meet — the polygon's turning points, the null
/// points the boundary blades are built on. Exact on the vertex lattice;
/// resolution-free for rectilinear shapes (a dilated shape has the same
/// corners at dilated positions).
pub fn corners(cells: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let (h, v) = boundary_edges(cells);
    let mut out: Vec<(i64, i64)> = Vec::new();
    // A vertex (r, c) touches horizontal edges starting at (r, c) or (r, c−1)
    // and vertical edges starting at (r, c) or (r−1, c).
    let mut vertices: Vec<(i64, i64)> = Vec::new();
    for &(r, c) in &h {
        vertices.push((r, c));
        vertices.push((r, c + 1));
    }
    vertices.sort_unstable();
    vertices.dedup();
    for (r, c) in vertices {
        let has_h = h.binary_search(&(r, c)).is_ok() || h.binary_search(&(r, c - 1)).is_ok();
        let has_v = v.binary_search(&(r, c)).is_ok() || v.binary_search(&(r - 1, c)).is_ok();
        if has_h && has_v {
            out.push((r, c));
        }
    }
    out
}

/// A rational as two saturated u64 coordinates (numerator, denominator) for
/// the chart; the chart only narrows, the versor decides.
fn ratio_axes(x: &BigRational) -> [u64; 2] {
    let sat = |b: &BigInt| -> u64 {
        let (_, bytes) = b.to_bytes_le();
        if bytes.len() > 8 {
            return u64::MAX;
        }
        let mut buf = [0u8; 8];
        buf[..bytes.len()].copy_from_slice(&bytes);
        u64::from_le_bytes(buf)
    };
    [sat(x.numer()), sat(x.denom())]
}

/// THE SIMILARITY CHART of a body — invariant under translation, rotation,
/// reflection, and exact dilation (for lattice shapes, under integer block
/// dilation exactly): corner count; boundary length² over cell count;
/// det(I)/tr(I)² (anisotropy); tr(I)/n² (spread per area); the hue
/// fractions sorted. All exact rationals, flattened as (numerator,
/// denominator) axes. A chart, never a classifier.
pub fn similarity_chart(b: &Blade) -> Vec<u64> {
    let (h, v) = boundary_edges(&b.cells);
    let corners_n = corners(&b.cells).len() as u64;
    let n = b.shell.clone();
    let edges = q((h.len() + v.len()) as i64);
    let (ixx, iyy, ixy) = (&b.inertia[0], &b.inertia[1], &b.inertia[2]);
    let tr = ixx + iyy;
    let det = ixx * iyy - ixy * ixy;
    let mut axes = vec![corners_n];
    axes.extend(ratio_axes(&(&edges * &edges / &n)));
    axes.extend(ratio_axes(&match tr.is_zero() {
        true => q(0),
        false => &det / (&tr * &tr),
    }));
    axes.extend(ratio_axes(&(&tr / (&n * &n))));
    let mut fractions: Vec<BigRational> = b.gauge.iter().map(|(_, c)| c / &n).collect();
    fractions.sort();
    for f in fractions {
        axes.extend(ratio_axes(&f));
    }
    axes
}

/// A body with its boundary corners as the null points a versor is solved
/// against and verified on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub blade: Blade,
    pub corners: Vec<(i64, i64)>,
}

/// Lift a support (cells without hue) to a shape.
pub fn shape_of(cells: &[(i64, i64)]) -> Shape {
    let comp: Vec<(i64, i64, u8)> = cells.iter().map(|&(r, c)| (r, c, 0u8)).collect();
    let blade = lift(&comp, 0);
    let corners = corners(&blade.cells);
    Shape { blade, corners }
}

/// The exact square root of a rational, if it is a rational square.
fn rational_sqrt(x: &BigRational) -> Option<BigRational> {
    if x.numer().sign() == num_bigint::Sign::Minus {
        return None;
    }
    let (n, d) = (x.numer().sqrt(), x.denom().sqrt());
    if &(&n * &n) == x.numer() && &(&d * &d) == x.denom() {
        Some(BigRational::new(n, d))
    } else {
        None
    }
}

/// Apply a versor to a lattice point and read the base back; `None` at
/// infinity.
pub fn carry(v: &Mv, r: i64, c: i64) -> Option<(BigRational, BigRational)> {
    cga::base(&v.sandwich(&cga::point(&q(r), &q(c))))
}

/// THE VERSOR BETWEEN TWO SHAPES, in closed form. Translation from the
/// centroids; the dilation ρ from the per-cell spread (a rational square, or
/// the shapes are not similar in ℚ — a refusal, never a rounding); the rotor
/// from one corresponding corner pair, `R = (ρ|d|² + d'·d) + (d'∧d) e₁₂`,
/// exact and unnormalized; a reflection is tried as the odd versor. The
/// corner correspondence is chart-directed: the corner farthest from the
/// centroid, its ties being the shape's own symmetry (the stabilizer), not a
/// scan. Verified by carrying every corner of `a` onto a corner of `b` —
/// projective equality of the null-point sets. `None` when no rational
/// versor carries `a` onto `b`.
pub fn versor_between(a: &Shape, b: &Shape) -> Option<Mv> {
    if a.corners.len() != b.corners.len() || a.corners.is_empty() {
        return None;
    }
    // Corners live on the vertex lattice; the body's centroid in that frame
    // is the cell-center centroid plus one half on each axis.
    let (ca1, ca2) = a.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let (cb1, cb2) = b.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let spread = |s: &Shape| (&s.blade.inertia[0] + &s.blade.inertia[1]) / &s.blade.shell;
    let (sa, sb) = (spread(a), spread(b));
    let rho = match sa.is_zero() {
        true => match sb.is_zero() {
            true => q(1),
            false => return None,
        },
        false => rational_sqrt(&(&sb / &sa))?,
    };
    if &b.blade.shell != &(&a.blade.shell * &rho * &rho) {
        return None; // the cell counts do not scale as ρ²: not similar
    }
    let dist2 = |k: &(i64, i64), c1: &BigRational, c2: &BigRational| {
        let (dr, dc) = (q(k.0) - c1, q(k.1) - c2);
        (&dr * &dr + &dc * &dc, dr, dc)
    };
    let far_a: Vec<(BigRational, BigRational, BigRational)> =
        a.corners.iter().map(|k| dist2(k, &ca1, &ca2)).collect();
    let max_a = far_a.iter().map(|(d, _, _)| d.clone()).max()?;
    let want_b = &max_a * &rho * &rho;
    let far_b: Vec<(BigRational, BigRational, BigRational)> =
        b.corners.iter().map(|k| dist2(k, &cb1, &cb2)).collect();
    let to_origin = cga::translator(&(-&ca1), &(-&ca2));
    let to_target = cga::translator(&cb1, &cb2);
    let dil = cga::dilator(&rho);
    let corners_b: Vec<(i64, i64)> = {
        let mut k = b.corners.clone();
        k.sort_unstable();
        k
    };
    let verify = |v: &Mv| -> bool {
        a.corners.iter().all(|&(r, c)| match carry(v, r, c) {
            Some((x, y)) => match (x.is_integer(), y.is_integer()) {
                (true, true) => {
                    let xi: i64 = x.to_integer().try_into().unwrap_or(i64::MAX);
                    let yi: i64 = y.to_integer().try_into().unwrap_or(i64::MAX);
                    corners_b.binary_search(&(xi, yi)).is_ok()
                }
                _ => false,
            },
            None => false,
        })
    };
    for (da, dar, dac) in far_a.iter().filter(|(d, _, _)| *d == max_a) {
        for (_, dbr, dbc) in far_b.iter().filter(|(d, _, _)| *d == want_b) {
            for reflect in [false, true] {
                // The source direction, reflected across the e2 axis when asked.
                let (ar, ac) = match reflect {
                    false => (dar.clone(), dac.clone()),
                    true => (-dar, dac.clone()),
                };
                let s = &rho * da + (dbr * &ar + dbc * &ac);
                let bv = dbr * &ac - dbc * &ar;
                // Antiparallel directions: the half-angle form vanishes and
                // the half turn is the bivector e₁₂ itself.
                let rotor = match (s.is_zero(), bv.is_zero()) {
                    (true, true) => cga::rotator(&q(0), &q(1)),
                    _ => cga::rotator(&s, &bv),
                };
                if cga::is_versor(&rotor).is_none() {
                    continue;
                }
                let mut v = rotor.geo(&to_origin);
                if reflect {
                    v = rotor.geo(&cga::reflector()).geo(&to_origin);
                }
                v = to_target.geo(&dil).geo(&v);
                if verify(&v) {
                    return Some(v);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(r0: i64, c0: i64, h: i64, w: i64) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = (0..h).flat_map(|r| (0..w).map(move |c| (r0 + r, c0 + c))).collect();
        v.sort_unstable();
        v
    }

    /// The shell: corners are the polygon's turning points; the similarity
    /// chart agrees across translation, rotation, reflection, and integer
    /// dilation; and the versor between two similar shapes is found in
    /// closed form and verified, while dissimilar shapes refuse.
    #[test]
    fn corners_chart_and_versor_are_exact_and_similarity_invariant() {
        let bar = shape_of(&rect(0, 0, 2, 3));
        assert_eq!(bar.corners.len(), 4);
        let l: Vec<(i64, i64)> = vec![(0, 0), (1, 0), (2, 0), (2, 1)];
        let l_shape = shape_of(&l);
        assert_eq!(l_shape.corners.len(), 6);
        // Rotated (rows↔cols) and translated bar: same chart, a rigid versor.
        let bar_rot = shape_of(&rect(5, 7, 3, 2));
        assert_eq!(similarity_chart(&bar.blade), similarity_chart(&bar_rot.blade));
        let v = versor_between(&bar, &bar_rot).expect("a rotation+translation");
        assert!(cga::is_versor(&v).is_some());
        // Dilated bar (2×): same chart, a versor with ρ = 2.
        let bar2 = shape_of(&rect(0, 0, 4, 6));
        assert_eq!(similarity_chart(&bar.blade), similarity_chart(&bar2.blade));
        let v2 = versor_between(&bar, &bar2).expect("a dilation");
        // Corners carry onto corners: (0,0) → (0,0) and (2,3) → (4,6).
        assert_eq!(carry(&v2, 0, 0), Some((q(0), q(0))));
        assert_eq!(carry(&v2, 2, 3), Some((q(4), q(6))));
        // The mirrored L needs the reflection; a bar and an L are not similar.
        let l_mirror: Vec<(i64, i64)> = vec![(0, 1), (1, 1), (2, 1), (2, 0)];
        assert!(versor_between(&l_shape, &shape_of(&l_mirror)).is_some(), "reflection found");
        assert!(versor_between(&bar, &l_shape).is_none(), "dissimilar shapes refuse");
        // A 3×3 against a 2×2: ρ² = 9/4, rational; a 3×3 against a 2×3 refuses.
        assert!(versor_between(&shape_of(&rect(0, 0, 2, 2)), &shape_of(&rect(1, 1, 3, 3))).is_some());
        assert!(versor_between(&shape_of(&rect(0, 0, 3, 3)), &shape_of(&rect(0, 0, 2, 3))).is_none());
    }

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
