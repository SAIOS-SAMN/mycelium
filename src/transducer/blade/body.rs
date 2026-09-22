// 16, 20, 51.3 — THE BLADE: a body lifted into the conformal algebra, and
// the invariants read off it.
//
//! A component of one hue becomes a blade; its moduli, its turns and its
//! topology are read from the algebra and from the support's own cube
//! complex on the three coordinates of the base, never from a bounding box.
//! The invariant signature is what two congruent bodies share.

use super::*;

/// A lamina as the sensor hands it over: the site it stands at — its three
/// base coordinates — and the hue it carries (16.4, 51.3). The support of
/// a body is a list of these.
pub type Lamina = (i64, i64, i64, u8);

/// A polytonic volumetric body. The four strata are ORTHOGONAL lanes (never
/// summed, law 5); only the SPATIAL lane is a Cl(4,1) multivector
/// (versor-acted). Gauge and topology pass through rigid motion unchanged —
/// that orthogonality IS the polytonic separation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blade {
    /// 𝒮 — the dual sphere `X(c) − ½r²e_∞`, non-null (`S·S = r² ≠ 0`) hence
    /// invertible: the geometric quotient can form.
    pub spatial: Mv,
    /// 𝒮 refinement — the EXACT second-moment (inertia) tensor about the
    /// centroid `(Ixx, Iyy, Ixy, Izz, Ixz, Iyz) ∈ ℚ⁶`, rotation-covariant,
    /// no eigen/roots. A lamina (16.4) has the unit square's own moment on
    /// the two plane axes and none along `e₃`.
    pub inertia: [BigRational; 6],
    /// 𝒮 refinement — the exact fourth body moments about the centroid: the
    /// isotropic part `∫(u²+v²)²` and the fourth harmonic
    /// `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))`, which turns at four times the
    /// body's angle. It orients a body whose inertia is isotropic.
    pub quartic: [BigRational; 3],
    /// 𝒢 — the gauge/hue content as sorted `(hue, count)` relations (Vec, not a
    /// map: cognition obeys the Anti-Flattening Law). Versor-invariant.
    pub gauge: Vec<(u8, BigRational)>,
    /// 𝒽 — the shell topology: the component's lamina count.
    pub shell: BigRational,
    /// The body's support — the `(row, col, frame)` sites its laminae stand
    /// at, sorted (Vec, not a set). The stack, never its shadow (51.3).
    pub support: Vec<(i64, i64, i64)>,
}

impl Blade {
    /// The centroid on the base, read back from the homogeneous sphere —
    /// exact ℚ³, the frame coordinate with the two of the plane.
    pub fn centroid(&self) -> Option<(BigRational, BigRational, BigRational)> {
        cga::base(&self.spatial)
    }

    /// The squared extent `r² = S·S` — always `> 0`, so the body is invertible.
    pub fn radius2(&self) -> BigRational {
        self.spatial.norm2()
    }

    /// THE BODY'S NUCLEUS: its conformal sphere in `ℚ^{4,1}`, centre and
    /// extent in ONE vector (64.1).
    ///
    /// `s = P − ½r²e_∞ + e_o` with `s² = +r²`, so the nucleus is SPACELIKE
    /// — the dual-sphere chart, which is the one where contact is
    /// reachable. On the `v² = −1` hyperboloid `μ = sinh²(d/2) ≥ 0` and
    /// tangency cannot be reached at all (59.2).
    ///
    /// It never pinches to the cone: `radius2` is `≥ 1/6` for the smallest
    /// body the sensor can deliver, because `lift` integrates each lamina's
    /// own unit square rather than taking it as a point mass. That is what
    /// keeps `den2 = 4r_a²r_b²` bounded away from zero and the horizon
    /// DEFINED (64.2).
    ///
    /// ONE PRODUCER (law 12). Every reader of a nucleus comes here rather
    /// than assembling `cga::sphere` from a centroid at its own call site.
    pub fn nucleus(&self) -> Option<crate::transducer::cga::Mv> {
        let (p1, p2, p3) = self.centroid()?;
        Some(crate::transducer::cga::sphere(&p1, &p2, &p3, &self.radius2()))
    }

    /// Every extracted body is invertible by construction (`r² ≥ 1/6 > 0`).
    pub fn is_invertible(&self) -> bool {
        self.spatial.versor_inverse().is_some()
    }

    /// Apply a spatial versor to the SPATIAL lane only — a rigid motion moves the
    /// body coherently (`V σ Ṽ`) while gauge and topology pass through.
    pub fn sandwiched(&self, v: &Mv) -> Blade {
        Blade {
            spatial: v.sandwich(&self.spatial),
            inertia: self.inertia.clone(),
            quartic: self.quartic.clone(),
            gauge: self.gauge.clone(),
            shell: self.shell.clone(),
            support: self.support.clone(),
        }
    }

    /// The rigid-motion invariant key — strictly preserved by translations,
    /// rotations AND reflections, so it correspond-pairs bodies with zero spatial
    /// search: lamina count, `Tr(I)`, `det(I)` (both O(2)-invariant, exact ℚ, no
    /// eigen), and the LABEL-AGNOSTIC hue profile (sorted per-hue counts — a
    /// recolour still matches by shape, the relabel falling out as `Δ𝒢`).
    pub fn invariant(&self) -> InvariantSignature {
        let (ixx, iyy, ixy) = (&self.inertia[0], &self.inertia[1], &self.inertia[2]);
        let mut hue_profile: Vec<BigRational> = self.gauge.iter().map(|(_, c)| c.clone()).collect();
        hue_profile.sort();
        InvariantSignature {
            lamina_count: self.shell.clone(),
            tr_i: ixx + iyy,
            det_i: ixx * iyy - ixy * ixy,
            hue_profile,
        }
    }

    /// THE PLANES A BODY HAS. The base is three coordinates — `e₀, e₁, e₃`
    /// are `r, c, k` (51.2) — so a body's inertia spans three planes and not
    /// the six of `Cl(4,0)`; the three through `e₂` are not the base's.
    ///
    /// `harmonic_on_plane`'s match arms ARE this list, and a caller sweeping
    /// `SLOT_PLANES_SIX` tried six to find three and dropped the rest through
    /// a `continue` — the carrier naming its own planes is the reading, and
    /// the sweep was the question asked of a table that answers it (law 3).
    pub(crate) const BASE_PLANES: [(u8, u8); 3] = [
        (crate::transducer::alphabet::PLANE_ROW, crate::transducer::alphabet::PLANE_COL),
        (crate::transducer::alphabet::PLANE_ROW, 3),
        (crate::transducer::alphabet::PLANE_COL, 3),
    ];

    /// The deviatoric harmonic tensor on a given rotation plane in `BASE_PLANES`.
    /// On plane (0,1) reads `(Ixx - Iyy, 2Ixy)`; on (0,3) reads `(Ixx - Izz, 2Ixz)`;
    /// on (1,3) reads `(Iyy - Izz, 2Iyz)`.
    pub(crate) fn harmonic_on_plane(&self, plane: (u8, u8), mirrored: bool) -> Option<[BigRational; 2]> {
        use crate::transducer::alphabet::{PLANE_COL, PLANE_ROW};
        match plane {
            (PLANE_ROW, PLANE_COL) => {
                let (ixx, iyy, ixy) = (&self.inertia[0], &self.inertia[1], &self.inertia[2]);
                let cross = ixy * q(2);
                Some([ixx - iyy, if mirrored { -cross } else { cross }])
            }
            (PLANE_ROW, 3) => {
                let (ixx, izz, ixz) = (&self.inertia[0], &self.inertia[3], &self.inertia[4]);
                let cross = ixz * q(2);
                Some([ixx - izz, if mirrored { -cross } else { cross }])
            }
            (PLANE_COL, 3) => {
                let (iyy, izz, iyz) = (&self.inertia[1], &self.inertia[3], &self.inertia[5]);
                let cross = iyz * q(2);
                Some([iyy - izz, if mirrored { -cross } else { cross }])
            }
            _ => None,
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
    pub lamina_count: BigRational,
    pub tr_i: BigRational,
    pub det_i: BigRational,
    pub hue_profile: Vec<BigRational>,
}

/// Lift one connected component `(row, col, frame, hue)` into a volumetric
/// blade — exact ℚ throughout. The centroid is in ℚ³ and the moments are the
/// full six-component inertia on `{e₀, e₁, e₃}`. Each element is a lamina
/// (16.4): the unit square standing in its frame, so it contributes the
/// square's own moment on the two plane axes and only its offset along
/// `e₃`. The component's laminae need not be sorted.
pub fn lift(comp: &[Lamina]) -> Blade {
    let n = comp.len().max(1) as i64;
    let n_q = q(n);

    // Centroid c = (Σrow, Σcol, Σframe) / n ∈ ℚ³.
    let sr: i64 = comp.iter().map(|(r, _, _, _)| *r).sum();
    let sc: i64 = comp.iter().map(|(_, c, _, _)| *c).sum();
    let sk: i64 = comp.iter().map(|(_, _, k, _)| *k).sum();
    let c1 = BigRational::new(BigInt::from(sr), BigInt::from(n));
    let c2 = BigRational::new(BigInt::from(sc), BigInt::from(n));
    let c3 = BigRational::new(BigInt::from(sk), BigInt::from(n));

    // Moments about the centroid (exact ℚ) — of the BODY: the union of unit
    // squares, so each lamina contributes the integral of the monomial over
    // its own square (the parallel-axis law at every order) on the plane,
    // and its offset alone along `e₃`.
    let (mut ixx, mut iyy, mut ixy) = (q(0), q(0), q(0));
    let (mut izz, mut ixz, mut iyz) = (q(0), q(0), q(0));
    let (mut q_iso, mut h4x, mut h4y) = (q(0), q(0), q(0));
    for (r, c, k, _) in comp {
        let dr = q(*r) - &c1;
        let dc = q(*c) - &c2;
        let dk = q(*k) - &c3;
        let (dr2, dc2, dk2) = (&dr * &dr, &dc * &dc, &dk * &dk);
        ixx += &dr2 + qn(1, 12);
        iyy += &dc2 + qn(1, 12);
        izz += &dk2;
        ixy += &dr * &dc;
        ixz += &dr * &dk;
        iyz += &dc * &dk;

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

    // Extent r² = per-unit-mass polar 2nd moment + the unit square's own
    // polar moment (1/6), so r² > 0 even for a single lamina — off the null
    // cone, invertible.
    let r2 = (&ixx + &iyy) / &n_q;
    let spatial = cga::sphere(&c1, &c2, &c3, &r2);

    // Gauge: the hue multiset as sorted (hue, count) relations — Vec, not a map.
    let mut gauge: Vec<(u8, BigRational)> = Vec::new();
    for (_, _, _, h) in comp {
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

    let mut support: Vec<(i64, i64, i64)> = comp.iter().map(|(r, c, k, _)| (*r, *c, *k)).collect();
    support.sort_unstable();
    support.dedup();

    Blade {
        spatial,
        inertia: [ixx, iyy, ixy, izz, ixz, iyz],
        quartic: [q_iso, h4x, h4y],
        gauge,
        shell: n_q,
        support,
    }
}

/// The connected components of a scene on the three coordinates of the
/// base, as `Vec`-of-`Vec` (no maps/sets — Law II). A BODY IS ONE HUE
/// (chapter 20, ruled 2026-09-09): two laminae join iff they are adjacent
/// — 4-adjacent within a frame or one frame apart at one site (51.3) —
/// and carry the same hue. Two touching bodies of different hues are two
/// bodies sharing a boundary, which the pair form carries as a contact
/// face with its edge transport (50.3).
pub fn components(laminae: &[Lamina]) -> Vec<Vec<Lamina>> {
    // The support in canonical order; its 1-skeleton read off the ordering
    // (sentence::incidence_of_3d), never a distance test against every
    // lamina; an edge between two hues is a boundary, not an adjacency.
    let mut sorted: Vec<Lamina> = laminae.to_vec();
    sorted.sort_unstable();
    sorted.dedup_by_key(|c| (c.0, c.1, c.2));
    let positions: Vec<(i64, i64, i64)> = sorted.iter().map(|c| (c.0, c.1, c.2)).collect();
    let edges: Vec<(usize, usize)> = crate::transducer::sentence::incidence_of_3d(&positions)
        .into_iter()
        .filter(|&(i, j)| sorted[i].3 == sorted[j].3)
        .collect();
    let roots = crate::transducer::sentence::components(sorted.len(), &edges);
    let mut out: Vec<(usize, Vec<Lamina>)> = Vec::new();
    for (i, lamina) in sorted.into_iter().enumerate() {
        match out.iter_mut().find(|(r, _)| *r == roots[i]) {
            Some((_, comp)) => comp.push(lamina),
            None => out.push((roots[i], vec![lamina])),
        }
    }
    out.into_iter().map(|(_, comp)| comp).collect()
}

/// THE SECTIONS OF A SUPPORT: its sites grouped by frame, each frame's
/// sites in row order — the laminae of one frame, the sections of the
/// stack along `e₃` (16.5). The in-plane readings (a section's turns and
/// perimeter) are taken here and summed over the stack; the topology is
/// not, because adjacency along `e₃` joins the sections (51.3).
pub fn sections(support: &[(i64, i64, i64)]) -> Vec<(i64, Vec<(i64, i64)>)> {
    let mut by_frame: Vec<(i64, i64, i64)> = support.to_vec();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: frame first, then the site's own row and column"
    )]
    by_frame.sort_unstable_by_key(|&(r, c, k)| (k, r, c));
    by_frame.dedup();
    let mut out: Vec<(i64, Vec<(i64, i64)>)> = Vec::new();
    for (r, c, k) in by_frame {
        match out.last_mut() {
            Some((frame, section)) if *frame == k => section.push((r, c)),
            _ => out.push((k, vec![(r, c)])),
        }
    }
    out
}

/// THE OPEN FACES OF A SECTION, each named by the site it opens on: the
/// horizontal edges from (r,c) to (r,c+1), then the vertical ones from
/// (r,c) to (r+1,c). This is the coboundary of the support's 2-chain
/// within one frame.
pub(crate) type OpenFaces = (Vec<(i64, i64)>, Vec<(i64, i64)>);

pub(crate) fn boundary_edges(sites: &[(i64, i64)]) -> OpenFaces {
    // Which of a lamina's four faces are shared is read off the support's
    // 1-skeleton (sentence::incidence_of): a row edge (i, j) closes i's
    // right face and j's left face; a column edge closes i's lower face
    // and j's upper face. Every face left open is boundary.
    let mut open = vec![[true; 4]; sites.len()]; // up, down, left, right
    for (i, j) in crate::transducer::sentence::incidence_of(sites) {
        if sites[i].0 == sites[j].0 {
            open[i][3] = false;
            open[j][2] = false;
        } else {
            open[i][1] = false;
            open[j][0] = false;
        }
    }
    let (mut h, mut v) = (Vec::new(), Vec::new());
    for (i, &(r, c)) in sites.iter().enumerate() {
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

/// The turning count of one section's boundary: the vertices where a
/// horizontal and a vertical boundary edge meet.
fn turns_of_section(sites: &[(i64, i64)]) -> usize {
    let (h, v) = boundary_edges(sites);
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

/// The turning count of a body: the sum over its sections of the vertices
/// where the section's boundary turns. A similarity invariant of the body
/// (a dilated body turns the same number of times) and a translation
/// along `e₃` moves no section's boundary.
pub fn turns(support: &[(i64, i64, i64)]) -> usize {
    sections(support).iter().map(|(_, section)| turns_of_section(section)).sum()
}

/// The in-plane boundary's length: the count of unit edges with a lamina
/// on exactly one side, summed over the sections.
pub fn boundary_length(support: &[(i64, i64, i64)]) -> usize {
    sections(support)
        .iter()
        .map(|(_, section)| {
            let (h, v) = boundary_edges(section);
            h.len() + v.len()
        })
        .sum()
}

/// THE FRAME BOUNDARY of a body: its sides open along `e₃` — a lamina's
/// face toward the frame before it and toward the frame after it, each
/// counted where no lamina of the body stands there (51.3). Two for every
/// lamina of a body of one frame; fewer for a stack.
pub fn frame_faces(support: &[(i64, i64, i64)]) -> usize {
    let mut sorted: Vec<(i64, i64, i64)> = support.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let held = |p: (i64, i64, i64)| sorted.binary_search(&p).is_ok();
    sorted
        .iter()
        .map(|&(r, c, k)| usize::from(!held((r, c, k - 1))) + usize::from(!held((r, c, k + 1))))
        .sum()
}

/// The topology of a support as the union of its closed unit cubes on the
/// 6-adjacency complex (51.3): `(β₀, β₁, β₂)` — components, tunnels and
/// cavities. The Euler characteristic `χ = V − E + F − C` is counted
/// through incidence, each corner contributing `1/k` for the `k` cubes
/// meeting there and each unit segment `1/m` for the `m` cubes sharing it,
/// so the count is exact without a vertex set; `β₂` is the number of
/// enclosed components of the complement, and `β₁ = β₀ + β₂ − χ`. A body
/// of one frame reads its planar components and holes, and no cavity.
pub fn topology(support: &[(i64, i64, i64)]) -> (u64, u64, u64) {
    let mut sorted: Vec<(i64, i64, i64)> = support.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.is_empty() {
        return (0, 0, 0);
    }
    let held = |r: i64, c: i64, k: i64| sorted.binary_search(&(r, c, k)).is_ok();
    let edges = crate::transducer::sentence::incidence_of_3d(&sorted);
    let mut roots = crate::transducer::sentence::components(sorted.len(), &edges);
    roots.sort_unstable();
    roots.dedup();
    let b0 = roots.len() as i64;
    let n = sorted.len() as i64;
    let faces = 6 * n - edges.len() as i64;
    let mut vertices = q(0);
    let mut segments = q(0);
    for &(r, c, k) in &sorted {
        // The eight corners `(x, y, z)`, each shared by the cubes whose
        // origin is one step below it on any axis.
        for dx in 0..2 {
            for dy in 0..2 {
                for dz in 0..2 {
                    let (x, y, z) = (r + dx, c + dy, k + dz);
                    let mut m = 0i64;
                    for ax in [x - 1, x] {
                        for ay in [y - 1, y] {
                            for az in [z - 1, z] {
                                m += i64::from(held(ax, ay, az));
                            }
                        }
                    }
                    vertices += qn(1, m);
                }
            }
        }
        // The twelve unit segments: four along each axis, each shared by
        // the cubes around it in the two other axes.
        for dy in 0..2 {
            for dz in 0..2 {
                let (y, z) = (c + dy, k + dz);
                let mut m = 0i64;
                for ay in [y - 1, y] {
                    for az in [z - 1, z] {
                        m += i64::from(held(r, ay, az));
                    }
                }
                segments += qn(1, m);
            }
        }
        for dx in 0..2 {
            for dz in 0..2 {
                let (x, z) = (r + dx, k + dz);
                let mut m = 0i64;
                for ax in [x - 1, x] {
                    for az in [z - 1, z] {
                        m += i64::from(held(ax, c, az));
                    }
                }
                segments += qn(1, m);
            }
        }
        for dx in 0..2 {
            for dy in 0..2 {
                let (x, y) = (r + dx, c + dy);
                let mut m = 0i64;
                for ax in [x - 1, x] {
                    for ay in [y - 1, y] {
                        m += i64::from(held(ax, ay, k));
                    }
                }
                segments += qn(1, m);
            }
        }
    }
    // χ is exact on the cube complex. A χ that does not fit an i64 is a
    // support wider than this reader measures, and zero would be a
    // topological claim about it (rule 11): the body reads as one
    // component with no tunnel and no cavity only when χ says so.
    let Ok(chi) = i64::try_from((vertices - segments + q(faces) - q(n)).to_integer()) else {
        return (0, 0, 0);
    };
    let b2 = cavities(&sorted);
    let b1 = b0 + b2 - chi;
    (b0 as u64, if b1 >= 0 { b1 as u64 } else { 0 }, b2 as u64)
}

/// THE CAVITIES OF A SUPPORT: the components of its complement, within the
/// box one site wider than the support on every side, that do not reach
/// that box's border. A cavity needs a lamina of the body before it and
/// after it along `e₃`, so a body of fewer than three frames has none and
/// nothing is walked for it.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the bounding box is the extent of the support, content: the complement is walked inside it and its border is where a component reaches outside"
)]
fn cavities(sorted: &[(i64, i64, i64)]) -> i64 {
    let (Some(r0), Some(r1), Some(c0), Some(c1), Some(k0), Some(k1)) = (
        sorted.iter().map(|p| p.0).min(),
        sorted.iter().map(|p| p.0).max(),
        sorted.iter().map(|p| p.1).min(),
        sorted.iter().map(|p| p.1).max(),
        sorted.iter().map(|p| p.2).min(),
        sorted.iter().map(|p| p.2).max(),
    ) else {
        return 0;
    };
    if k1 - k0 < 2 {
        return 0;
    }
    let held = |p: (i64, i64, i64)| sorted.binary_search(&p).is_ok();
    let inside = |(r, c, k): (i64, i64, i64)| {
        (r0 - 1..=r1 + 1).contains(&r) && (c0 - 1..=c1 + 1).contains(&c) && (k0 - 1..=k1 + 1).contains(&k)
    };
    let on_border = |(r, c, k): (i64, i64, i64)| {
        r == r0 - 1 || r == r1 + 1 || c == c0 - 1 || c == c1 + 1 || k == k0 - 1 || k == k1 + 1
    };
    let six = |(r, c, k): (i64, i64, i64)| {
        [(r - 1, c, k), (r + 1, c, k), (r, c - 1, k), (r, c + 1, k), (r, c, k - 1), (r, c, k + 1)]
    };
    let key = |(r, c, k): (i64, i64, i64)| {
        let mut b = [0u8; 24];
        b[..8].copy_from_slice(&r.to_le_bytes());
        b[8..16].copy_from_slice(&c.to_le_bytes());
        b[16..].copy_from_slice(&k.to_le_bytes());
        b
    };
    let mut visited: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut enclosed = 0i64;
    // Every complement component that touches the body is entered from a
    // lamina of the body; a component touching no lamina of it is outside.
    for &p in sorted {
        for start in six(p) {
            if held(start) || visited.contains(&key(start)) {
                continue;
            }
            visited.insert(key(start), ());
            let mut stack = vec![start];
            let mut reaches_border = false;
            while let Some(at) = stack.pop() {
                if on_border(at) {
                    reaches_border = true;
                }
                for next in six(at) {
                    if !inside(next) || held(next) || visited.contains(&key(next)) {
                        continue;
                    }
                    visited.insert(key(next), ());
                    stack.push(next);
                }
            }
            if !reaches_border {
                enclosed += 1;
            }
        }
    }
    enclosed
}

/// THE MODULI of a body — its coordinates on the base, one `ChartAxis`
/// each (A21 as ruled 2026-09-18). Total: a body that has a blade has
/// harmonics, every harmonic is a whole coordinate carrying its own exact
/// value, and there is no width left to read as absence. A body with
/// support is never charted at the root.
pub fn moduli(b: &Blade) -> Vec<crate::transducer::stalk::ChartAxis> {
    crate::transducer::blade::harmonics::Harmonics::of(b).axes()
}

/// A body as the carrier a transport is solved on and verified against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub blade: Blade,
}

/// THE DEVIATORIC VECTOR OF A BODY'S SECOND MOMENT — its ORIENTATION,
/// exact in ℚ and taken with no root (55.3, ruled 2026-09-22).
///
/// `inertia` holds the whole tensor; `r2` already takes its TRACE and uses
/// it where a radius belongs. The trace is an `SO(2)` invariant, so it
/// says how big a body is and nothing about which way it faces — a body
/// read through its trace alone is an ISOTROPIC POINT MASS, two of them
/// have no relative angle, and the only relation left between them is the
/// geodesic through their centres:
///
/// ```text
///     R = 1 − v_b v_a,  ⟨R⟩₂ = v_a ∧ v_b,  (v_a∧v_b)∧(v_a∧v_b) = 0
/// ```
///
/// a simple blade, `I₂ ≡ 0`, `ℓ = 1`. What orients the body is the
/// ANISOTROPIC remainder, `M₂ − ½tr(M₂)I`, which in `Cl(2,0)` is a vector:
///
/// ```text
///     u = ((I_xx − I_yy)/2) e₁ + I_xy e₂
/// ```
///
/// A rank-2 symmetric tensor turns at TWICE the body's angle, so `u` turns
/// at `2φ`. `None` where `u = 0`: a body whose inertia is isotropic — a
/// single lamina, a square, a cross — has no principal axis, and refusing
/// is the honest answer rather than an arbitrary axis (rule 11). Such a
/// body is oriented by [`quartic_harmonic`] instead, which turns at `4φ`.
/// A.12 — THE ZEROTH MOMENT, and the one place the tower is normalised.
///
/// `M₀ = ∫ 1 dA` is the body's own area, which on this base is its lamina
/// count. Every central moment divides by it, so a moment of harmonic order
/// `m` carries homogeneous degree `m` and nothing downstream needs a table
/// of per-carrier weights.
///
/// `None` on an empty support: a body with no laminae has no moments to
/// normalise, and dividing by zero is not a reading (rule 11).
fn zeroth_moment(blade: &Blade) -> Option<BigRational> {
    match blade.support.len() {
        0 => None,
        n => Some(BigRational::from_integer(BigInt::from(n))),
    }
}

pub fn deviatoric(blade: &Blade) -> Option<[BigRational; 2]> {
    let two = BigRational::from_integer(BigInt::from(2));
    // A.12 — NORMALISED BY `M₀`, so the tower speaks one dimension.
    //
    // MEASURED BEFORE THIS (65.6): `radius2()` is `Tr(I)/M₀`, a length²
    // scaling as `ρ²`, while this returned RAW `inertia` combinations,
    // which carry the area's own weight — `∫x² ~ ρ²·ρ² = ρ⁴` — so
    // `‖D‖²` scaled as `ρ⁸` where the harmonic order `m = 2` predicts
    // `ρ⁴`. One tower at two normalisations is law 12 inside a single
    // carrier, and it forced every consumer to hold a private table of
    // degrees: `{r² ↦ 2, D ↦ 4, Q ↦ 6}`, a lookup masquerading as
    // geometry.
    //
    // Normalised, a moment of azimuthal order `m` carries degree `m`, and
    // the scale certificate reads its exponent straight off the gauge
    // order with no table at all:
    //
    // ```text
    //     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
    // ```
    let m0 = zeroth_moment(blade)?;
    let u1 = (&blade.inertia[0] - &blade.inertia[1]) / &two / &m0;
    let u2 = &blade.inertia[2] / &m0;
    match u1.is_zero() && u2.is_zero() {
        true => None,
        false => Some([u1, u2]),
    }
}

/// THE FOURTH HARMONIC OF A BODY — the orientation of a shape whose
/// inertia is isotropic, turning at FOUR times the body's angle.
///
/// `quartic` already holds `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))` beside the
/// isotropic part. A square's second moment cannot tell it from a disc;
/// its fourth moment can, and on the square base that is exactly the `D₄`
/// discriminator. `None` where this vanishes too, which is a body with no
/// orientation at either order.
pub fn quartic_harmonic(blade: &Blade) -> Option<[BigRational; 2]> {
    // A.12 — normalised by `M₀`, as the deviatoric is. Raw, this carried
    // `∫x⁴ ~ ρ⁶` and `‖Q‖²` scaled as `ρ¹²`; normalised it is degree 4,
    // the harmonic order it is named for.
    let m0 = zeroth_moment(blade)?;
    let (h4x, h4y) = (&blade.quartic[1] / &m0, &blade.quartic[2] / &m0);
    match h4x.is_zero() && h4y.is_zero() {
        true => None,
        false => Some([h4x, h4y]),
    }
}

/// T2 — THE INVERSIVE STRAIN BETWEEN TWO BODIES, CARRIED AS A RATIO AND
/// NEVER EVALUATED (59.3, Axiom C).
///
/// Two bodies with centroid `P` and radius of gyration `r` are related by
/// the inversive distance of their spheres,
///
/// ```text
///     δ = (‖P_a − P_b‖² − r_a² − r_b²) / (2 r_a r_b)
/// ```
///
/// and the kinematic charge of the chord between them is `μ = (δ − 1)/2`,
/// which is `⟨R⟩₀²/⟨RR̃⟩₀ − 1` for `R = 1 − v_b v_a`. `μ = 0` is exact
/// tangency, `μ < 0` interpenetration, `μ > 0` separation.
///
/// `μ` IS NOT RATIONAL AND NEVER REACHES THE WIRE. The denominator wants
/// `r_a r_b = √(r_a² r_b²)`, and [`Blade::radius2`] carries `r²`, so law 1
/// would refuse the root as inexact on nearly every body. What IS rational
/// is the pair: the numerator, and the denominator SQUARED. Every question
/// the engine asks of the strain is a comparison, and a comparison of two
/// ratios is a comparison of cross products — which is the projective
/// discipline stated once and obeyed here.
///
/// `num`'S SIGN IS ORTHOGONALITY, NOT TANGENCY. A first reading of this
/// had `num = 0` as the contact locus; it is not. `num = 0` is `δ = 0`,
/// two spheres meeting at RIGHT ANGLES, while tangency is `δ = 1`, which
/// is `num = √den2`. Reading the sign of `num` as the contact side would
/// put the horizon in the wrong place by a whole unit of `δ`, so the side
/// is read by [`Strain::side`] and never off `num` alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Strain {
    /// `‖P_a − P_b‖² − r_a² − r_b²` — the sign is `δ`'s, so it says which
    /// side of ORTHOGONALITY the pair is on.
    pub num: BigRational,
    /// `4 r_a² r_b²`, strictly positive: `Blade::radius2` is `> 0` by
    /// construction, so the pair never carries a zero denominator.
    pub den2: BigRational,
}

/// WHERE TWO BODIES STAND RELATIVE TO CONTACT. The reading `μ` would give
/// if `μ` could be written down, taken without writing it down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    /// `δ > 1` — disjoint exteriors, `d > r_a + r_b`.
    Apart,
    /// `δ = 1` — touching from OUTSIDE, `d = r_a + r_b`. Measure-zero over
    /// ℚ, so it is the locus a threshold is taken about and never the test
    /// itself (59.4).
    TangentExternal,
    /// `|δ| < 1` — secant, `|r_a − r_b| < d < r_a + r_b`. Self-dual under
    /// the antipodal map, with `δ = 0` (inversive orthogonality) as its
    /// fixed point.
    Intersecting,
    /// `δ = −1` — touching from INSIDE, `d = |r_a − r_b|`. A genuine
    /// codimension-1 contact interface, not overlap.
    TangentInternal,
    /// `δ < −1` — strictly nested, `d < |r_a − r_b|`.
    Contained,
}

impl Contact {
    /// THE ANTIPODAL INVOLUTION `σ: s ↦ −s`, which sends `δ ↦ −δ`.
    ///
    /// A conformal sphere and its negation are the same sphere with INSIDE
    /// AND OUTSIDE EXCHANGED, so the involution swaps `Apart` with
    /// `Contained` and the two tangencies with one another, and fixes
    /// `Intersecting`. `σ ∘ σ = id`, asserted in the axioms.
    pub fn antipodal(self) -> Contact {
        match self {
            Contact::Apart => Contact::Contained,
            Contact::TangentExternal => Contact::TangentInternal,
            Contact::Intersecting => Contact::Intersecting,
            Contact::TangentInternal => Contact::TangentExternal,
            Contact::Contained => Contact::Apart,
        }
    }

    /// IS THIS A CONTACT INTERFACE? Both tangencies are, and treating one
    /// as contact and the other as penetration is what lost the boundary's
    /// symmetry.
    pub fn is_tangent(self) -> bool {
        matches!(self, Contact::TangentExternal | Contact::TangentInternal)
    }
}

/// T4 — THE INTERFACE BETWEEN TWO NUCLEI, WHOLE (64, ruled 2026-09-22).
///
/// SCALAR COLLAPSE IS DISALLOWED. The geometric product of two nuclei is
///
/// ```text
///     s_a s_b = s_a · s_b  +  s_a ∧ s_b
/// ```
///
/// and a tether that carries only the first term has thrown the interaction
/// away. `s_a · s_b` is ONE rational: it says how close, and nothing about
/// where, in what plane, or through which pencil. `s_a ∧ s_b` is a 2-blade
/// of `⋀²(ℚ^{4,1})` — TEN independent rational coordinates — carrying the
/// radical hyperplane, the orientation of the interface, and the coaxal
/// pencil of spheres through the two bodies' common boundary.
///
/// So the scalar GATES and the blade IS THE INTERFACE. `condense` asks the
/// first whether a tether forms and hands the second to the 1-cell, which
/// enters `C¹` with its whole geometric orientation rather than as an
/// unoriented link.
///
/// A LONE TETHER IS ALWAYS SIMPLE, and that is a theorem rather than a
/// limitation: `(s_a ∧ s_b) ∧ (s_a ∧ s_b) = 0` identically, because a
/// repeated factor kills the wedge. So `I₂ = 0` on every isolated
/// interface, and the luminal screw can only appear where the NETWORK
/// composes — a relative motor coupling a turn with an off-plane
/// displacement, two tethers spanning four independent directions, or a
/// closed circuit accumulating non-coplanar bivectors (59.7, 60.4). The
/// blade is the local interface; the screw is the global torsion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interface {
    /// Grade 0. The gate: `within(μ_c)` decides whether a tether forms.
    pub strain: Strain,
    /// Grade 2. The coaxal pencil's own blade — the interface the
    /// transition refracts across, carried into `C¹` entire.
    pub blade: crate::transducer::cga::Mv,
}

impl Interface {
    /// The whole product of two nuclei, split by grade. `None` where a
    /// centroid does not read back (rule 11).
    pub fn between(a: &Blade, b: &Blade) -> Option<Interface> {
        let product = a.nucleus()?.geo(&b.nucleus()?);
        Some(Interface { strain: Strain::between(a, b)?, blade: product.grade(2) })
    }

    /// Is this interface inside the horizon? The SCALAR gates; the blade is
    /// what the tether then carries.
    pub fn within(&self, mu_c: &BigRational) -> bool {
        self.strain.within(mu_c)
    }
}

/// A.12 — IS THIS PAIR RELATED BY A DILATION, OR BY A SHEAR?
///
/// A trace ratio cannot tell: a 2×2 square and a 1×4 bar of equal `Tr(I)`
/// have the same `r²` and are not dilates of one another. The whole
/// normalised tower must correspond, and with the tower at one
/// normalisation a moment of azimuthal order `m` carries degree `m`, so
/// the certificate reads its exponent STRAIGHT OFF THE GAUGE ORDER:
///
/// ```text
///     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
/// ```
///
/// checked at `m = 2` (deviatoric) and `m = 4` (quartic), by exact cross
/// multiplication in ℚ with no root taken and no table of weights.
///
/// A VANISHING HARMONIC IS PART OF THE READING, not a gap in it. A `C₄`
/// body has no deviatoric at all, so a pair where one vanishes and the
/// other does not is REFUSED — they cannot be dilates, since a dilation
/// preserves every harmonic's vanishing. Both vanishing is agreement.
pub fn dilation_certified(a: &Blade, b: &Blade) -> bool {
    let (ra2, rb2) = (a.radius2(), b.radius2());
    let norm2 = |v: [BigRational; 2]| &v[0] * &v[0] + &v[1] * &v[1];
    let zero = BigRational::from_integer(BigInt::from(0));
    let order = |ma: Option<[BigRational; 2]>, mb: Option<[BigRational; 2]>, m: usize| {
        match (ma, mb) {
            // Both isotropic at this order: they agree, and nothing is
            // asserted about a harmonic neither body carries.
            (None, None) => true,
            // One carries the harmonic and the other does not. A dilation
            // preserves vanishing, so this is a shape difference (rule 11:
            // the refusal is the reading).
            (None, Some(_)) | (Some(_), None) => false,
            (Some(x), Some(y)) => {
                norm2(y) * ra2.pow(m as i32) == norm2(x) * rb2.pow(m as i32)
            }
        }
    };
    let _ = zero;
    order(deviatoric(a), deviatoric(b), 2)
        && order(quartic_harmonic(a), quartic_harmonic(b), 4)
}

/// T4 — THE PAIRWISE STRAIN FIELD, EVALUATED EXACTLY.
///
/// THIS IS A FIELD, NOT A SEARCH. A search explores a space of
/// possibilities for one that fits an ad-hoc score; this computes the
/// interaction between every pair of bodies present, the way an N-body
/// force calculation does, and every value it produces is an exact rational
/// the physics defines. Rule 1's "never a pair loop over the pieces" was
/// written for INCIDENCE BY SHARED SITE, which a keyed occupancy answers in
/// one linear pass — there is no keyed occupancy that answers a continuous
/// strain, because the relation is not "share a cell".
///
/// The cost does not bind: the corpus's largest cover holds 37 charts, so
/// the field is 666 unordered pairs of exact rational products. Building a
/// tree, a bucket or a lattice to avoid them would cost a standing
/// constraint to optimise an arithmetic cost that does not exist.
///
/// Ordered `i < j` so each interface is computed once and the reading does
/// not depend on the order the bodies were offered.
pub fn condense(bodies: &[Blade], mu_c: &BigRational) -> Vec<(usize, usize, Interface)> {
    let mut out = Vec::new();
    for i in 0..bodies.len() {
        for j in i + 1..bodies.len() {
            let Some(face) = Interface::between(&bodies[i], &bodies[j]) else {
                continue;
            };
            if face.within(mu_c) {
                out.push((i, j, face));
            }
        }
    }
    out
}

impl Strain {
    /// The strain between two bodies, or `None` where a centroid does not
    /// read back (rule 11: refused, never defaulted).
    pub fn between(a: &Blade, b: &Blade) -> Option<Strain> {
        let (a1, a2, a3) = a.centroid()?;
        let (b1, b2, b3) = b.centroid()?;
        let (d1, d2, d3) = (&a1 - &b1, &a2 - &b2, &a3 - &b3);
        let sep2 = &d1 * &d1 + &d2 * &d2 + &d3 * &d3;
        let (ra2, rb2) = (a.radius2(), b.radius2());
        let four = BigRational::from_integer(BigInt::from(4));
        Some(Strain { num: sep2 - &ra2 - &rb2, den2: four * ra2 * rb2 })
    }

    /// `δ` COMPARED AGAINST A RATIONAL, EXACTLY AND WITHOUT A ROOT.
    ///
    /// `δ ≤ k` is `num ≤ k·√den2`. Both sides are compared by SIGN first
    /// and only then by squaring, because squaring loses the sign and a
    /// test that squares first answers the wrong question on a negative
    /// numerator. `√den2 > 0` always, which is what makes the case split
    /// total.
    fn delta_at_most(&self, k: &BigRational) -> bool {
        let zero = BigRational::from_integer(BigInt::from(0));
        match (self.num > zero, *k > zero) {
            // Both positive: `num ≤ k√den2 ⟺ num² ≤ k²·den2`.
            (true, true) => &self.num * &self.num <= k * k * &self.den2,
            // A positive numerator is never below a non-positive bound.
            (true, false) => false,
            // A non-positive numerator is always below a positive bound.
            (false, true) => true,
            // Both non-positive: the inequality REVERSES under squaring.
            (false, false) => &self.num * &self.num >= k * k * &self.den2,
        }
    }

    /// Is this pair inside the horizon `μ ≤ μ_c`? `μ = (δ−1)/2`, so the
    /// bound on `δ` is `2μ_c + 1` and the comparison is the one above.
    pub fn within(&self, mu_c: &BigRational) -> bool {
        let one = BigRational::from_integer(BigInt::from(1));
        let two = BigRational::from_integer(BigInt::from(2));
        self.delta_at_most(&(two * mu_c + one))
    }

    /// WHERE THE PAIR STANDS ON THE INVERSIVE SPECTRUM — five regimes, read
    /// exactly and without a root.
    ///
    /// ```text
    ///  Contained   TangentInt     Intersecting    TangentExt    Apart
    ///   δ < −1       δ = −1        |δ| < 1         δ = 1        δ > 1
    ///  ─────────]──────|──────[───────●───────]──────|──────[─────────
    ///                              δ = 0
    ///                           (orthogonal)
    /// ```
    ///
    /// A FIRST CUT HAD THREE ARMS AND COLLAPSED A CONTACT LOCUS. It read
    /// `Tangent` only for `δ = 1` and folded everything below into
    /// `Through`, so `δ = −1` — INTERNAL TANGENCY, where the spheres touch
    /// at one point from inside, `d = |r_a − r_b|` — was reported as
    /// interpenetration. That is a scalar collapse committed in an enum
    /// rather than in a metric, and it is exactly half the contact boundary
    /// mislabelled.
    ///
    /// The fixture hid it: a body against ITSELF sits at `δ = −1`, where
    /// "touching from inside" and "fully overlapping" are both defensible,
    /// so the assertion passed while testing what was written rather than
    /// what is true. For two DIFFERENT bodies at `d = |r_a − r_b|` they are
    /// not the same statement at all.
    ///
    /// THE COMPARISONS ARE `δ` AGAINST `±1`, and both go through
    /// `delta_at_most`, which splits on sign before squaring — so the
    /// spectrum is read with no root anywhere and no sign lost.
    pub fn side(&self) -> Contact {
        let one = BigRational::from_integer(BigInt::from(1));
        let zero = BigRational::from_integer(BigInt::from(0));
        // `|δ| = 1` exactly: `num² = den2`. The SIGN of `num` then says
        // which of the two tangencies it is, because `√den2 > 0` always.
        if &self.num * &self.num == self.den2 {
            return match self.num > zero {
                true => Contact::TangentExternal,
                false => Contact::TangentInternal,
            };
        }
        match (self.delta_at_most(&one), self.delta_at_most(&-one)) {
            // δ ≤ −1 and not equal to it, so δ < −1.
            (_, true) => Contact::Contained,
            // −1 < δ ≤ 1 and not equal to it, so |δ| < 1.
            (true, false) => Contact::Intersecting,
            // δ > 1.
            (false, false) => Contact::Apart,
        }
    }
}

/// THE DOUBLE ANGLE BETWEEN TWO BODIES' PRINCIPAL AXES, exact in ℚ and
/// with no root taken (55.3).
///
/// Diagonalising one tensor needs the roots of its characteristic
/// quadratic, which lie in `ℚ(√Δ)` — refused by rule 4. The RELATIVE
/// angle needs neither diagonalised. `u` turns at `2φ`, so for congruent
/// bodies `‖u_a‖² = ‖u_b‖² = Δ` and
///
/// ```text
///     cos 2φ = (u_a · u_b) / Δ,    sin 2φ = (u_a ∧ u_b) / Δ
/// ```
///
/// are both rational outright. On a square base congruent bodies are
/// related by `D₄`, so `φ ∈ {0, π/2, π, 3π/2}`, `2φ ∈ {0, π, 2π, 3π}` and
/// these take values in `{−1, 0, 1}` only.
///
/// `None` where either body is isotropic (no principal axis) or where the
/// two anisotropies DIFFER. That is not a policy: `cos 2φ = (u_a·u_b) /
/// (‖u_a‖‖u_b‖)` needs the PRODUCT of the norms, and only when
/// `‖u_a‖² = ‖u_b‖²` does that product equal a rational `Δ`. For unequal
/// norms the value is irrational in general and rule 4 refuses it.
///
/// Dropping the gate would not loosen this reading, it would BREAK it:
/// `(u_a·u_b)/‖u_a‖²` with unequal norms is `s⁴ cos 2φ`, a dilated
/// projection that can exceed `±1` and is not an angle.
///
/// [`relative_rotor`] is therefore NOT the same reading with a weaker
/// gate — the two live on different layers:
///
/// ```text
///   METRIC        relative_double_angle   O(2), an isometry invariant
///                 needs ‖u_a‖² = ‖u_b‖², answers an exact cosine
///   PROJECTIVE    relative_rotor          Spin(2)/ℤ₂ on ℙ¹(ℚ), a
///                 similitude invariant; never forms cos 2φ at all
/// ```
///
/// `relative_rotor` asks only WHICH of the four turns applies, which the
/// SIGNS of `u_a·u_b` and `u_a∧u_b` decide, and those survive any
/// dilation because `u ↦ s⁴u` with `s⁴ > 0`. One is the metric
/// restriction of the other's projective ray action. Exact angle values
/// need congruence; the turn itself needs only similarity.
///
/// IT DETERMINES `φ` ONLY MOD `π`. A rank-2 symmetric tensor is invariant
/// under a half turn, so `u` cannot tell a body from its own 180°
/// rotation: `φ = 0` and `φ = π` both give `2φ ≡ 0`. The two differ as
/// rotors — `1` against `e₁₂` — and are NOT projectively equal, so this
/// returns the double angle and does not mint a rotor from it. Resolving
/// the residual half turn is a question about the SUPPORT, answered by
/// carrying a candidate through [`versor_coset_ordered`], which tests it
/// against the sites rather than guessing.
pub fn relative_double_angle(
    ua: &[BigRational; 2],
    ub: &[BigRational; 2],
) -> Option<(BigRational, BigRational)> {
    let da = &ua[0] * &ua[0] + &ua[1] * &ua[1];
    let db = &ub[0] * &ub[0] + &ub[1] * &ub[1];
    if da.is_zero() || da != db {
        return None;
    }
    let dot = &ua[0] * &ub[0] + &ua[1] * &ub[1];
    let wedge = &ua[0] * &ub[1] - &ua[1] * &ub[0];
    Some((dot / &da, wedge / &da))
}

/// THE FOUR PROPER ROTATIONS OF THE SQUARE BASE, as PROJECTIVE rotors in
/// `Spin(2)(ℚ)` — no root taken and no field extension (56.4).
///
/// ```text
///     φ      R = cos(φ/2) + sin(φ/2) e₁₂     projective        ℓ
///     0      1                               1                 1
///     π/2    (1 + e₁₂)/√2                    1 + e₁₂           4
///     π      e₁₂                             e₁₂               2
///     3π/2   (−1 + e₁₂)/√2                   −1 + e₁₂          4
/// ```
///
/// A versor is projective, so `Ω` and `λΩ` are one transport and the
/// `1/√2` never has to be written: `(1 + e₁₂)(1 − e₁₂) = 1 − e₁₂² = 2 ∈ ℚ`.
///
/// REFLECTIONS ARE NOT HERE. `D₄` has eight elements; the four improper
/// ones lie in `O(2) \ SO(2)` and are not rotors. A reflection between two
/// charts is a different fact from a rotation and is not minted as one.
fn quarter_turns() -> [(i64, i64, Mv); 4] {
    let q = |n: i64| BigRational::from_integer(BigInt::from(n));
    let e12 = cga::E1 | cga::E2;
    let rotor = |s: i64, b: i64| Mv::scalar(q(s)).add(&Mv::blade(e12, q(b)));
    // `(cos φ, sin φ)` beside the rotor that enacts it, so the test below
    // turns the support by the SAME angle the rotor carries.
    [
        (1, 0, rotor(1, 0)),
        (0, 1, rotor(1, 1)),
        (-1, 0, rotor(0, 1)),
        (0, -1, rotor(-1, 1)),
    ]
}

/// A CHART'S FRAME, and whether its half turn is settled (56.5).
///
/// `Turn` is a rotor the SUPPORT confirmed. `HalfOpen` is one the even
/// moments fixed only mod `π`: `u` and `h₄` satisfy `u(−x) = u(x)`, so no
/// even-rank moment distinguishes `φ` from `φ + π`, and where the two
/// bodies differ in cardinality — a transmuted carry — no cell test can
/// settle it either. The representative is carried and the openness is
/// carried WITH it, so a reader cannot mistake a projective class for a
/// determined rotor.
///
/// WHY NOT JUST PICK ONE. Choosing a sign per edge injects EXTRINSIC
/// PARITY into the 1-skeleton: three edges of a cycle guessed
/// independently manufacture a `−1` holonomy out of nothing, and the
/// complex then reports a phantom obstruction it invented. Left open, the
/// edge carries a `ℤ₂` gauge freedom and the COBOUNDARY is the arbiter —
/// a 2-cell either locks, in which case `∂σ` fixes the signs globally
/// across it, or it does not, in which case the obstruction is real.
/// Closing it locally instead needs an odd invariant (a third harmonic, a
/// directed contour), which is not carried here.
#[derive(Clone, Debug)]
pub enum Frame {
    Turn(Mv),
    HalfOpen(Mv),
}

impl Frame {
    /// The rotor either way — the caller that only needs to act takes
    /// this; the caller that must not confuse the two matches instead.
    pub fn rotor(&self) -> &Mv {
        match self {
            Frame::Turn(r) | Frame::HalfOpen(r) => r,
        }
    }
}

/// THE RELATIVE FRAME ROTOR BETWEEN TWO BODIES — `θ_ba`, exact in `ℚ`
/// (56.5, ruled 2026-09-22).
///
/// The deviatoric and the quartic say at what DOUBLE or QUADRUPLE angle
/// two bodies differ, but neither fixes `φ` itself: a rank-2 symmetric
/// tensor is invariant under a half turn, so `u` gives `φ` only mod `π`,
/// and `1` against `e₁₂` are not projectively equal. The residue is a
/// question about the SUPPORT, so the support answers it: each of the four
/// proper turns is applied to `a`'s cells about their centroid and
/// compared with `b`'s. This is a finite group's four elements checked
/// exactly, as `versor_coset_ordered` checks its coset — not a search.
///
/// The harmonics are still what REFUSES: bodies whose anisotropies differ
/// are not congruent, and no turn can carry one onto the other, so the
/// cheap exact test runs first and the support comparison never happens
/// for them.
///
/// WHY TWO BODIES CARRY NO DETERMINED FRAME (66, ruled 2026-09-22).
///
/// `relative_rotor` answered `None` for FIVE different facts, and a
/// refusal that does not say which refusal it is is rule 11's defect at
/// the producer rather than at the call site. The census made it concrete:
/// 89 contacting pairs of 118 came back `None`, and one number could not
/// say whether the corpus is full of unspellable turns — 37.7's condition,
/// live — or of bodies that simply do not orient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoFrame {
    /// A body with no support. Nothing to orient.
    Bodiless,
    /// One body has a principal axis and the other does not, so no turn of
    /// one is the other whatever their sizes.
    Isotropy,
    /// Neither body orients at any order this carrier holds — isotropic at
    /// the second harmonic AND at the fourth.
    Unoriented,
    /// THE AXES SIT AT AN ANGLE THE SQUARE BASE DOES NOT TURN BY. The
    /// relative orientation is WELL DEFINED and is not in `D₄`: `2φ = ±π/2`
    /// is the 45° relation whose half-angle `cos(π/8)` is irrational and
    /// refused by rule 4, and anything else is not a lattice turn at all.
    ///
    /// This is NOT a failure to measure. It is 37.7's condition — a
    /// transition the alphabet cannot spell between two bodies that are
    /// genuinely in contact — and it is where an adjoined generator comes
    /// from.
    Oblique,
    /// Same cardinality and neither candidate turn carries the cells: a
    /// reflection, or a genuine difference of shape the moments happened
    /// to agree on. Not a proper rotation.
    Mirrored,
}

pub fn relative_rotor(a: &Blade, b: &Blade) -> Result<Frame, NoFrame> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoFrame::Bodiless);
    }
    // THE ORIENTATION VECTORS. The second harmonic where the body has a
    // principal axis; the fourth where it does not — a square or a cross
    // is isotropic at second order and the quartic still orients it.
    // `None` at both orders is a body with no orientation to compare.
    let (oa, ob) = match (deviatoric(a), deviatoric(b)) {
        (Some(ua), Some(ub)) => (ua, ub),
        (None, None) => match (quartic_harmonic(a), quartic_harmonic(b)) {
            (Some(qa), Some(qb)) => (qa, qb),
            _ => return Err(NoFrame::Unoriented),
        },
        // One has a principal axis and the other does not: no turn of one
        // is the other, whatever their sizes.
        _ => return Err(NoFrame::Isotropy),
    };
    // SIGNS ONLY, WHICH IS WHAT MAKES IT SCALE-FREE. Under a dilation by
    // `s > 0` the moments scale as `s⁴`, so `u ↦ s⁴ · R(2φ) u` and every
    // sign survives. Taking `cos 2φ = (u_a·u_b)/(‖u_a‖‖u_b‖)` would need
    // that product of norms — the one place a root could enter — and it
    // is never formed. Demanding `‖u_a‖² = ‖u_b‖²` instead, as this did,
    // demanded CONGRUENCE: it refused a dilated carry whose rotation is
    // perfectly well defined and often the identity. A transformation
    // here carries a dilator `D(ρ)`, so similarity is the right group and
    // isometry was too small.
    let dot = &oa[0] * &ob[0] + &oa[1] * &ob[1];
    let wedge = &oa[0] * &ob[1] - &oa[1] * &ob[0];
    let zero = BigRational::from_integer(BigInt::from(0));
    let turn_pair = if wedge == zero && dot > zero {
        // 2φ = 0 — the axes agree, so φ is 0 or π.
        [0usize, 2usize]
    } else if wedge == zero && dot < zero {
        // 2φ = π — the axes are at a right angle, so φ is π/2 or 3π/2.
        [1usize, 3usize]
    } else {
        // 2φ = ±π/2 is a 45° relation whose half-angle `cos(π/8)` is
        // irrational: refused rather than approximated (rule 4). Anything
        // else is not a turn of the square base at all — and in BOTH cases
        // the relative orientation exists and is unspellable, which is
        // 37.7's condition and not a measurement failure.
        return Err(NoFrame::Oblique);
    };
    let (Some(ca), Some(cb)) = (a.centroid(), b.centroid()) else {
        return Err(NoFrame::Bodiless);
    };
    // Centred cells, exact over ℚ: the centroid is rational, so a cell's
    // offset is too, and a quarter turn keeps it rational.
    let centred = |s: &[(i64, i64, i64)], c: &(BigRational, BigRational, BigRational)| {
        let mut out: Vec<(BigRational, BigRational, i64)> = s
            .iter()
            .map(|&(r, col, k)| {
                (
                    BigRational::from_integer(BigInt::from(r)) - &c.0,
                    BigRational::from_integer(BigInt::from(col)) - &c.1,
                    k,
                )
            })
            .collect();
        out.sort();
        out
    };
    // THE HALF TURN IS A PARITY OBSTRUCTION, not a gap in this reader.
    // `u` and `h₄` are EVEN-rank moments: `(−x)² = x²` and `(−x)⁴ = x⁴`,
    // so `u(−x) = u(x)` and no even moment can ever tell `φ` from `φ + π`.
    // Only an odd invariant could — a third harmonic, or a directed
    // boundary contour — and neither is carried here.
    //
    // Where the supports have the SAME cardinality the cells themselves
    // settle it: the candidate is applied and compared. Where they differ
    // the body has transmuted and no cell test exists, so the frame is
    // returned mod `π` and SAID to be, rather than one of the two being
    // chosen silently.
    let turns = quarter_turns();
    if a.support.len() == b.support.len() {
        let (pa, pb) = (centred(&a.support, &ca), centred(&b.support, &cb));
        for ix in turn_pair {
            let (cos, sin, rotor) = &turns[ix];
            let (co, si) = (
                BigRational::from_integer(BigInt::from(*cos)),
                BigRational::from_integer(BigInt::from(*sin)),
            );
            let mut turned: Vec<(BigRational, BigRational, i64)> = pa
                .iter()
                .map(|(r, c, k)| (r * &co - c * &si, r * &si + c * &co, *k))
                .collect();
            turned.sort();
            if turned == pb {
                return Ok(Frame::Turn(rotor.clone()));
            }
        }
        // Same cardinality and neither half turn carries it: the two are
        // not related by a proper rotation — a reflection, or a genuine
        // difference of shape the moments happened to agree on.
        return Err(NoFrame::Mirrored);
    }
    Ok(Frame::HalfOpen(turns[turn_pair[0]].2.clone()))
}

/// Lift a support (sites on the three coordinates of the base, no hue) to
/// a shape.
pub fn shape_of(support: &[(i64, i64, i64)]) -> Shape {
    let comp: Vec<Lamina> = support.iter().map(|&(r, c, k)| (r, c, k, 0u8)).collect();
    Shape { blade: lift(&comp) }
}

/// Lift the support of ONE SECTION — a chain's sites, which A5 reads at
/// the sensor as `(r, c)` with the frame on the pose (16.5) — to its shape
/// in the section's own frame.
pub fn shape_of_section(sites: &[(i64, i64)]) -> Shape {
    let comp: Vec<Lamina> = sites.iter().map(|&(r, c)| (r, c, 0, 0u8)).collect();
    Shape { blade: lift(&comp) }
}
