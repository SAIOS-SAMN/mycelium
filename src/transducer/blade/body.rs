// 16, 20 — THE BLADE: a body lifted into the conformal algebra, and the
// invariants read off it.
//
//! A component of one hue becomes a blade; its moduli, its turns and its
//! topology are read from the algebra rather than from a bounding box. The
//! invariant signature is what two congruent bodies share.

use super::*;

/// A polytonic volumetric body. The four strata are ORTHOGONAL lanes (never
/// summed, law 5); only the SPATIAL lane is a Cl(3,1)
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
    /// 𝒮 refinement — the exact fourth body moments about the centroid: the
    /// isotropic part `∫(u²+v²)²` and the fourth harmonic
    /// `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))`, which turns at four times the
    /// body's angle. It orients a body whose inertia is isotropic.
    pub quartic: [BigRational; 3],
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
            quartic: self.quartic.clone(),
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

    /// The second harmonic of the body: the anisotropy `(Ixx − Iyy, 2 Ixy)`,
    /// a vector that turns at twice the body's angle and vanishes for an
    /// isotropic body. `mirrored` reads it for the body reflected across
    /// the row axis (the off-diagonal moment changes sign).
    pub(crate) fn harmonic2(&self, mirrored: bool) -> [BigRational; 2] {
        let (ixx, iyy, ixy) = (&self.inertia[0], &self.inertia[1], &self.inertia[2]);
        let cross = ixy * q(2);
        [ixx - iyy, if mirrored { -cross } else { cross }]
    }

    /// The fourth harmonic of the body, turning at four times its angle.
    pub(crate) fn harmonic4(&self, mirrored: bool) -> [BigRational; 2] {
        let (hx, hy) = (&self.quartic[1], &self.quartic[2]);
        [hx.clone(), if mirrored { -hy } else { hy.clone() }]
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

    // Moments about the centroid (exact ℚ) — of the BODY: the union of unit
    // squares, so each cell contributes the integral of the monomial over
    // its own square (the parallel-axis law at every order), never the
    // moments of cell centers as points. This is what makes the moments
    // scale as a body does under dilation (ρ² per cell, ρ⁴ in the second
    // moment, ρ⁶ in the fourth).
    let (mut ixx, mut iyy, mut ixy) = (q(0), q(0), q(0));
    let (mut q_iso, mut h4x, mut h4y) = (q(0), q(0), q(0));
    for (r, c, _) in comp {
        let dr = q(*r) - &c1;
        let dc = q(*c) - &c2;
        let (dr2, dc2) = (&dr * &dr, &dc * &dc);
        ixx += &dr2 + qn(1, 12);
        iyy += &dc2 + qn(1, 12);
        ixy += &dr * &dc;
        // ∫u⁴ = d⁴ + d²/2 + 1/80; ∫u² = d² + 1/12; ∫u³ = d³ + d/4; ∫u = d,
        // each over the unit interval centered at the offset d.
        let u4 = &dr2 * &dr2 + &dr2 * qn(1, 2) + qn(1, 80);
        let v4 = &dc2 * &dc2 + &dc2 * qn(1, 2) + qn(1, 80);
        let u2v2 = (&dr2 + qn(1, 12)) * (&dc2 + qn(1, 12));
        let u3v = (&dr2 * &dr + &dr * qn(1, 4)) * &dc;
        let uv3 = &dr * (&dc2 * &dc + &dc * qn(1, 4));
        q_iso += &u4 + &u2v2 * q(2) + &v4;
        h4x += &u4 - &u2v2 * q(6) + &v4;
        h4y += (&u3v - &uv3) * q(4);
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
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: the tally is keyed by hue, and the hue is the key"
    )]
    gauge.sort_by_key(|(h, _)| *h);

    let mut cells: Vec<(i64, i64)> = comp.iter().map(|(r, c, _)| (*r, *c)).collect();
    cells.sort_unstable();
    cells.dedup();

    Blade {
        spatial,
        inertia: [ixx, iyy, ixy],
        quartic: [q_iso, h4x, h4y],
        gauge,
        temporal: q(time_index),
        shell: n_q,
        cells,
    }
}

/// 4-connected components of a scene, as `Vec`-of-`Vec` (no maps/sets — Law II).
/// A BODY IS ONE HUE (chapter 20, ruled 2026-09-09): two cells join iff
/// they are orthogonally adjacent and carry the same hue. Two touching
/// bodies of different hues are two bodies sharing a boundary, which the
/// pair form carries as a contact face with its edge transport (50.3); a
/// component of every non-zero cell merged them into one blob no single
/// versor carries, so every such scene became a deformation demand and no
/// pair form ever had a contact face.
pub fn components(cells: &[(i64, i64, u8)]) -> Vec<Vec<(i64, i64, u8)>> {
    // The support in row order; its 1-skeleton read off the ordering
    // (sentence::incidence_of), never a distance test against every cell;
    // an edge between two hues is a boundary, not an adjacency.
    let mut sorted: Vec<(i64, i64, u8)> = cells.to_vec();
    sorted.sort_unstable();
    sorted.dedup_by_key(|c| (c.0, c.1));
    let positions: Vec<(i64, i64)> = sorted.iter().map(|c| (c.0, c.1)).collect();
    let edges: Vec<(usize, usize)> = crate::transducer::sentence::incidence_of(&positions)
        .into_iter()
        .filter(|&(i, j)| sorted[i].2 == sorted[j].2)
        .collect();
    let roots = crate::transducer::sentence::components(sorted.len(), &edges);
    let mut out: Vec<(usize, Vec<Painted>)> = Vec::new();
    for (i, cell) in sorted.into_iter().enumerate() {
        match out.iter_mut().find(|(r, _)| *r == roots[i]) {
            Some((_, comp)) => comp.push(cell),
            None => out.push((roots[i], vec![cell])),
        }
    }
    out.into_iter().map(|(_, comp)| comp).collect()
}

/// Transduce a whole scene: segment into components, lift each to a blade at
/// `time_index`. The transducer output for one scene state.
pub fn transduce(cells: &[(i64, i64, u8)], time_index: i64) -> Vec<Blade> {
    components(cells).iter().map(|comp| lift(comp, time_index)).collect()
}

/// The boundary of a support: every unit edge with the support on exactly
/// one side. Returns (horizontal edges from (r,c) to (r,c+1), vertical
/// edges from (r,c) to (r+1,c)), each as its start vertex. The boundary is
/// the coboundary of the support's 2-chain; its length is a measurement of
/// the body, not a coordinate of it.
/// A CELL AS THE SENSOR HANDS IT OVER: its position and the hue it carries.
pub(crate) type Painted = (i64, i64, u8);

/// THE OPEN FACES OF A BODY, each named by the cell it opens on: the
/// horizontal edges from (r,c) to (r,c+1), then the vertical ones from
/// (r,c) to (r+1,c). This is the coboundary of the support's 2-chain.
pub(crate) type OpenFaces = (Vec<(i64, i64)>, Vec<(i64, i64)>);

pub(crate) fn boundary_edges(cells: &[(i64, i64)]) -> OpenFaces {
    // Which of a cell's four faces are shared is read off the support's
    // 1-skeleton (sentence::incidence_of): a row edge (i, j) closes i's
    // right face and j's left face; a column edge closes i's lower face
    // and j's upper face. Every face left open is boundary.
    let mut open = vec![[true; 4]; cells.len()]; // up, down, left, right
    for (i, j) in crate::transducer::sentence::incidence_of(cells) {
        if cells[i].0 == cells[j].0 {
            open[i][3] = false;
            open[j][2] = false;
        } else {
            open[i][1] = false;
            open[j][0] = false;
        }
    }
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for (i, &(r, c)) in cells.iter().enumerate() {
        if open[i][0] {
            h.push((r, c));
        }
        if open[i][1] {
            h.push((r + 1, c));
        }
        if open[i][2] {
            v.push((r, c));
        }
        if open[i][3] {
            v.push((r, c + 1));
        }
    }
    h.sort_unstable();
    v.sort_unstable();
    (h, v)
}

/// The turning count of the boundary: the vertices where a horizontal and
/// a vertical boundary edge meet. A similarity invariant of the body (a
/// dilated body turns the same number of times).
pub fn turns(cells: &[(i64, i64)]) -> usize {
    let (h, v) = boundary_edges(cells);
    let mut vertices: Vec<(i64, i64)> = Vec::new();
    for &(r, c) in &h {
        vertices.push((r, c));
        vertices.push((r, c + 1));
    }
    vertices.sort_unstable();
    vertices.dedup();
    vertices
        .into_iter()
        .filter(|&(r, c)| {
            let has_h = h.binary_search(&(r, c)).is_ok() || h.binary_search(&(r, c - 1)).is_ok();
            let has_v = v.binary_search(&(r, c)).is_ok() || v.binary_search(&(r - 1, c)).is_ok();
            has_h && has_v
        })
        .count()
}

/// The topology of a support as the union of its closed unit cells:
/// `(β₀, β₁)` — components and holes. Euler characteristic
/// `χ = V − E + F` on the cell complex, with `β₁ = β₀ − χ` (a planar
/// complex has no β₂). Vertices are counted through their incidence: each
/// cell corner contributes `1/k` for the `k` cells meeting there, so the
/// count is exact without a vertex set.
pub fn topology(cells: &[(i64, i64)]) -> (u64, u64) {
    if cells.is_empty() {
        return (0, 0);
    }
    let held = |r: i64, c: i64| cells.binary_search(&(r, c)).is_ok();
    let n = cells.len() as i64;
    let mut adjacent = 0i64;
    let mut vertices = q(0);
    for &(r, c) in cells {
        adjacent += held(r + 1, c) as i64 + held(r, c + 1) as i64;
        for (x, y) in [(r, c), (r, c + 1), (r + 1, c), (r + 1, c + 1)] {
            let k = held(x - 1, y - 1) as i64 + held(x - 1, y) as i64 + held(x, y - 1) as i64 + held(x, y) as i64;
            if k > 0 {
                vertices += qn(1, k);
            }
        }
    }
    let edges = 4 * n - adjacent;
    // χ is exact on the cell complex. A χ that does not fit an i64 is a
    // support wider than this reader measures, and zero would be a
    // topological claim about it (rule 11): the body reads as one
    // component with no hole only when χ says so.
    let Ok(chi) = i64::try_from((vertices - q(edges) + q(n)).to_integer()) else {
        return (0, 0);
    };
    let comps: Vec<(i64, i64, u8)> = cells.iter().map(|&(r, c)| (r, c, 0u8)).collect();
    let b0 = components(&comps).len() as i64;
    (b0 as u64, if b0 >= chi { (b0 - chi) as u64 } else { 0 })
}

/// A6: two's-complement numerator and positive denominator. Refuses when
/// either limb overflows; nothing saturates, the sign is kept.
pub(crate) fn ratio_axes(x: &BigRational) -> Option<[u64; 2]> {
    let n: i64 = x.numer().try_into().ok()?;
    let d: u64 = x.denom().try_into().ok()?;
    Some([n as u64, d])
}

/// The boundary's length: the count of unit edges with the support on
/// exactly one side.
pub fn boundary_length(cells: &[(i64, i64)]) -> usize {
    let (h, v) = boundary_edges(cells);
    h.len() + v.len()
}

/// A rational as two exact axis limbs, or none when the ratio exceeds ℤ₆₄.
pub fn axes_of(x: &BigRational) -> Option<[u64; 2]> {
    ratio_axes(x)
}

/// THE MODULI of a body — its coordinates on the base. Empty on A6
/// refusal: novelty, never a saturated MAX axis.
pub fn moduli(b: &Blade) -> Vec<u64> {
    // A6 refusing is NOVELTY, and the empty axis list is what novelty
    // reads as here — a measurement of this body, not a default put in
    // place of one (rule 11).
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; empty axes is A6's refusal read out"
    )]
    match crate::transducer::blade::harmonics::Harmonics::of(b).axes() {
        Some(axes) => axes,
        None => Vec::new(),
    }
}

/// A body as the carrier a transport is solved on and verified against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub blade: Blade,
}

/// Lift a support (cells without hue) to a shape.
pub fn shape_of(cells: &[(i64, i64)]) -> Shape {
    let comp: Vec<(i64, i64, u8)> = cells.iter().map(|&(r, c)| (r, c, 0u8)).collect();
    Shape { blade: lift(&comp, 0) }
}
