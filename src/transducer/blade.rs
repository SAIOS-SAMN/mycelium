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
//!
//! The shell is the body's own moments: count, centroid, the second moment
//! (inertia), and the fourth moment, each integrated over the unit cells as
//! a body, never summed over cell centers as points. Orientation is read
//! from the moment harmonics (the anisotropy of the inertia rotates at twice
//! the body's angle, the fourth harmonic at four times), so the transport
//! between two bodies is a quotient of their moments — no corner, no vertex
//! of the sensor's sample enters the algebra. The sample stays on the body
//! only as the support the transport is verified on.

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
    fn harmonic2(&self, mirrored: bool) -> [BigRational; 2] {
        let (ixx, iyy, ixy) = (&self.inertia[0], &self.inertia[1], &self.inertia[2]);
        let cross = ixy * q(2);
        [ixx - iyy, if mirrored { -cross } else { cross }]
    }

    /// The fourth harmonic of the body, turning at four times its angle.
    fn harmonic4(&self, mirrored: bool) -> [BigRational; 2] {
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

// ---- THE SHELL: boundary, topology, the moduli, the transport ----

/// The boundary of a support: every unit edge with the support on exactly
/// one side. Returns (horizontal edges from (r,c) to (r,c+1), vertical
/// edges from (r,c) to (r+1,c)), each as its start vertex. The boundary is
/// the coboundary of the support's 2-chain; its length is a measurement of
/// the body, not a coordinate of it.
fn boundary_edges(cells: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let held = |r: i64, c: i64| cells.binary_search(&(r, c)).is_ok();
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for &(r, c) in cells {
        if !held(r - 1, c) {
            h.push((r, c));
        }
        if !held(r + 1, c) {
            h.push((r + 1, c));
        }
        if !held(r, c - 1) {
            v.push((r, c));
        }
        if !held(r, c + 1) {
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
            vertices += qn(1, k.max(1));
        }
    }
    let edges = 4 * n - adjacent;
    let chi: i64 = (vertices - q(edges) + q(n)).to_integer().try_into().unwrap_or(0);
    let comps: Vec<(i64, i64, u8)> = cells.iter().map(|&(r, c)| (r, c, 0u8)).collect();
    let b0 = components(&comps).len() as i64;
    (b0 as u64, (b0 - chi).max(0) as u64)
}

/// A rational as two saturated u64 coordinates (numerator, denominator) on
/// the base; the coordinates narrow, the transport decides.
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

/// The boundary's length: the count of unit edges with the support on
/// exactly one side.
pub fn boundary_length(cells: &[(i64, i64)]) -> usize {
    let (h, v) = boundary_edges(cells);
    h.len() + v.len()
}

/// A rational as two saturated coordinates on the base (public for the
/// ear, which lays the harmonics out as axes).
pub fn axes_of(x: &BigRational) -> [u64; 2] {
    ratio_axes(x)
}

/// THE MODULI of a body — its coordinates on the base: the ear's
/// harmonics laid out as axes. See [`crate::perception::ear::Harmonics`].
pub fn moduli(b: &Blade) -> Vec<u64> {
    crate::perception::ear::Harmonics::of(b).axes()
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

/// Apply a versor to a point of the sensor's vertex coordinates and read
/// the base back; `None` at infinity.
pub fn carry(v: &Mv, r: i64, c: i64) -> Option<(BigRational, BigRational)> {
    cga::base(&v.sandwich(&cga::point(&q(r), &q(c))))
}

/// The unnormalized rotor carrying direction `a` onto direction `b` in the
/// e₁e₂ plane: `|a||b| + b·a + (b∧a) e₁₂`. Exact when `|a||b|` is rational
/// (the two lengths' product is a rational square), else refused.
fn rotor_between(a: &[BigRational; 2], b: &[BigRational; 2]) -> Option<(BigRational, BigRational)> {
    let aa = &a[0] * &a[0] + &a[1] * &a[1];
    let bb = &b[0] * &b[0] + &b[1] * &b[1];
    let ab = rational_sqrt(&(&aa * &bb))?;
    let s = &ab + &b[0] * &a[0] + &b[1] * &a[1];
    let bv = &b[0] * &a[1] - &b[1] * &a[0];
    Some((s, bv))
}

/// The rotor of half the angle of `s + b e₁₂`: `(|r| + s) + b e₁₂`, exact
/// when `|r|` is rational. A half turn (`s < 0`, `b = 0`) halves to the
/// quarter turn `1 + e₁₂`... whose own half-angle form is `e₁₂` itself when
/// the rotor is the pure bivector.
fn half_rotor(s: &BigRational, b: &BigRational) -> Option<(BigRational, BigRational)> {
    let m2 = s * s + b * b;
    if m2.is_zero() {
        return None;
    }
    let m = rational_sqrt(&m2)?;
    let s2 = &m + s;
    match (s2.is_zero(), b.is_zero()) {
        (true, true) => Some((q(0), q(1))), // the half of a half turn: e₁₂
        _ => Some((s2, b.clone())),
    }
}

/// THE TRANSPORT BETWEEN TWO BODIES, in closed form from their moments.
/// Translation from the body centroids; the dilation ρ from the per-cell
/// spread (a rational square, or the bodies are not similar in ℚ — a
/// refusal, never a rounding); the rotation from the moment harmonics: the
/// inertia's anisotropy turns at twice the body's angle, so the rotor
/// carrying `a`'s anisotropy onto `b`'s is halved once; an isotropic body
/// is oriented by its fourth harmonic, halved twice. Each harmonic leaves a
/// finite ambiguity — the body's own stabilizer (a half turn for the
/// second harmonic, quarter turns for the fourth) — and a reflection is
/// tried as the odd versor; every candidate is verified by carrying every
/// cell center of `a` into a cell of `b` and every cell center of `b` back
/// into a cell of `a`. `None` when no rational versor carries `a` onto `b`.
pub fn versor_between(a: &Shape, b: &Shape) -> Option<Mv> {
    if a.blade.cells.is_empty() || b.blade.cells.is_empty() {
        return None;
    }
    // Body centroids in the sensor's vertex coordinates: the cell-center
    // centroid plus one half on each axis.
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
    if b.blade.shell != &a.blade.shell * &rho * &rho {
        return None; // the cell counts do not scale as ρ²: not similar
    }
    let to_origin = cga::translator(&(-&ca1), &(-&ca2));
    let to_target = cga::translator(&cb1, &cb2);
    let dil = cga::dilator(&rho);
    let quarter = cga::rotator(&q(1), &q(1)); // the quarter turn, unnormalized
    let half_turn = cga::rotator(&q(0), &q(1)); // e₁₂
    let a_inv = a.blade.cells.clone();
    let b_cells = b.blade.cells.clone();
    let lands = |v: &Mv, r: i64, c: i64, held: &[(i64, i64)]| -> bool {
        match crate::transducer::element::carry_center(v, r, c) {
            Some(p) => held.binary_search(&crate::transducer::element::cell_of(&p)).is_ok(),
            None => false,
        }
    };
    let verify = |v: &Mv| -> bool {
        let Some(inv) = v.versor_inverse() else { return false };
        a_inv.iter().all(|&(r, c)| lands(v, r, c, &b_cells))
            && b_cells.iter().all(|&(r, c)| lands(&inv, r, c, &a_inv))
    };
    for reflect in [false, true] {
        let a2 = a.blade.harmonic2(reflect);
        let b2 = b.blade.harmonic2(false);
        let iso_a = a2[0].is_zero() && a2[1].is_zero();
        let iso_b = b2[0].is_zero() && b2[1].is_zero();
        // The rotation candidates: the rotor of the body's angle and its
        // stabilizer under the harmonic that oriented it.
        let candidates: Vec<Mv> = match (iso_a, iso_b) {
            (false, false) => {
                let Some((s, bv)) = rotor_between(&a2, &b2) else { continue };
                let (s, bv) = match (s.is_zero(), bv.is_zero()) {
                    (true, true) => (q(0), q(1)), // antiparallel anisotropy: a half turn of the harmonic
                    _ => (s, bv),
                };
                let Some((hs, hb)) = half_rotor(&s, &bv) else { continue };
                let r = cga::rotator(&hs, &hb);
                vec![r.clone(), r.geo(&half_turn)]
            }
            (true, true) => {
                let a4 = a.blade.harmonic4(reflect);
                let b4 = b.blade.harmonic4(false);
                let iso4 = (a4[0].is_zero() && a4[1].is_zero()) || (b4[0].is_zero() && b4[1].is_zero());
                match iso4 {
                    true => vec![Mv::scalar(q(1))],
                    false => {
                        let Some((s, bv)) = rotor_between(&a4, &b4) else { continue };
                        let (s, bv) = match (s.is_zero(), bv.is_zero()) {
                            (true, true) => (q(0), q(1)),
                            _ => (s, bv),
                        };
                        let Some((s1, b1)) = half_rotor(&s, &bv) else { continue };
                        let Some((s2, b2)) = half_rotor(&s1, &b1) else { continue };
                        let r = cga::rotator(&s2, &b2);
                        let r90 = r.geo(&quarter);
                        let r180 = r90.geo(&quarter);
                        let r270 = r180.geo(&quarter);
                        vec![r, r90, r180, r270]
                    }
                }
            }
            _ => continue, // one body isotropic, the other not: not similar
        };
        for rotor in candidates {
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

    fn dilate(cells: &[(i64, i64)], k: i64) -> Vec<(i64, i64)> {
        let mut v: Vec<(i64, i64)> = cells
            .iter()
            .flat_map(|&(r, c)| (0..k).flat_map(move |i| (0..k).map(move |j| (k * r + i, k * c + j))))
            .collect();
        v.sort_unstable();
        v
    }

    /// The shell: the moduli agree across translation, rotation, reflection,
    /// and integer dilation; the transport between two similar bodies is
    /// solved from their moments and verified on their supports, while
    /// dissimilar bodies refuse.
    #[test]
    fn moduli_and_transport_are_exact_and_similarity_invariant() {
        let bar = shape_of(&rect(0, 0, 2, 3));
        assert_eq!(turns(&bar.blade.cells), 4);
        let l: Vec<(i64, i64)> = vec![(0, 0), (1, 0), (2, 0), (2, 1)];
        let l_shape = shape_of(&l);
        assert_eq!(turns(&l_shape.blade.cells), 6);
        // Rotated (rows↔cols) and translated bar: same moduli, a rigid versor.
        let bar_rot = shape_of(&rect(5, 7, 3, 2));
        assert_eq!(moduli(&bar.blade), moduli(&bar_rot.blade));
        let v = versor_between(&bar, &bar_rot).expect("a rotation+translation");
        assert!(cga::is_versor(&v).is_some());
        // Dilated bar (2×): same moduli, a versor with ρ = 2 carrying the
        // body's vertex (0,0) to (0,0) and (2,3) to (4,6).
        let bar2 = shape_of(&rect(0, 0, 4, 6));
        assert_eq!(moduli(&bar.blade), moduli(&bar2.blade));
        let v2 = versor_between(&bar, &bar2).expect("a dilation");
        assert_eq!(carry(&v2, 0, 0), Some((q(0), q(0))));
        assert_eq!(carry(&v2, 2, 3), Some((q(4), q(6))));
        // The mirrored L needs the reflection; a rotated L is a rotation; a
        // bar and an L are not similar.
        let l_mirror: Vec<(i64, i64)> = vec![(0, 1), (1, 1), (2, 1), (2, 0)];
        assert!(versor_between(&l_shape, &shape_of(&l_mirror)).is_some(), "reflection found");
        let l_rot: Vec<(i64, i64)> = vec![(7, -3), (7, -2), (7, -1), (6, -1)];
        assert!(versor_between(&l_shape, &shape_of(&l_rot)).is_some(), "rotation found");
        assert_eq!(moduli(&l_shape.blade), moduli(&shape_of(&l_rot).blade));
        assert!(versor_between(&bar, &l_shape).is_none(), "dissimilar bodies refuse");
        // A 3×3 against a 2×2: ρ² = 9/4, rational; a 3×3 against a 2×3 refuses.
        assert!(versor_between(&shape_of(&rect(0, 0, 2, 2)), &shape_of(&rect(1, 1, 3, 3))).is_some());
        assert!(versor_between(&shape_of(&rect(0, 0, 3, 3)), &shape_of(&rect(0, 0, 2, 3))).is_none());
        // An isotropic body (a square) is oriented by its stabilizer: a
        // translated, dilated square carries onto itself.
        let sq = shape_of(&rect(0, 0, 2, 2));
        let sq3 = shape_of(&rect(4, 4, 6, 6));
        assert_eq!(moduli(&sq.blade), moduli(&sq3.blade));
        assert!(versor_between(&sq, &sq3).is_some(), "ρ = 3 on an isotropic body");
        // An isotropic body with an orientation (a pinwheel of four arms)
        // is oriented by the fourth harmonic: its quarter-turn image carries.
        let pin: Vec<(i64, i64)> = vec![(1, 1), (1, 2), (2, 2), (0, 1), (1, 0), (2, 1), (1, 3), (3, 2), (0, 3)];
        let pin_shape = shape_of(&pin);
        let pin_rot: Vec<(i64, i64)> = pin.iter().map(|&(r, c)| (c, -r)).collect();
        assert_eq!(moduli(&pin_shape.blade), moduli(&shape_of(&pin_rot).blade));
        assert!(versor_between(&pin_shape, &shape_of(&pin_rot)).is_some(), "a quarter turn found");
    }

    /// Topology on the closed-cell complex: a square has no hole, a ring
    /// one hole, two apart bars two components; the moduli carry both
    /// numbers first, and a dilation keeps them.
    #[test]
    fn topology_counts_components_and_holes_exactly() {
        assert_eq!(topology(&rect(0, 0, 2, 2)), (1, 0));
        let ring: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)];
        assert_eq!(topology(&ring), (1, 1));
        let two: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (5, 5), (5, 6)];
        assert_eq!(topology(&two), (2, 0));
        let m = moduli(&shape_of(&ring).blade);
        assert_eq!((m[0], m[1], m[2]), (1, 1, 8));
        assert_eq!(moduli(&shape_of(&dilate(&ring, 2)).blade), m, "a dilated ring reads the same moduli");
        // The fourth-order axis separates bodies the inertia cannot: an
        // L-tetromino and a T-tetromino... share a cell count; the moduli
        // differ.
        let l4: Vec<(i64, i64)> = vec![(0, 0), (1, 0), (2, 0), (2, 1)];
        let s4: Vec<(i64, i64)> = vec![(0, 1), (0, 2), (1, 0), (1, 1)];
        assert_ne!(moduli(&shape_of(&l4).blade), moduli(&shape_of(&s4).blade));
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
