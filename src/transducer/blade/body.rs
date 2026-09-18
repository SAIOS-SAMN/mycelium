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

    /// The deviatoric harmonic tensor on a given rotation plane in `SLOT_PLANES_SIX`.
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
