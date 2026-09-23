// 16, 20, 51.3, 71 — THE BLADE: a body lifted into the conformal algebra,
// and the invariants read off it.
//
//! A hue-blind component becomes a blade (70.4); its moduli, its turns and its
//! topology are read from the algebra and from the support's own cube
//! complex on the sites of the base, never from a bounding box. The
//! invariant signature is what two congruent bodies share.

use super::*;
use crate::transducer::base::{is_grid_axis, Site, GRID_AXES};

/// A lamina as the sensor hands it over: the site it stands at and the hue
/// it carries (16.4, 51.3, 71.2(1)). The support of a body is a list of
/// these. Ordered by site, then hue, as the tuple `(r, c, k, hue)` was.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lamina {
    pub site: Site,
    pub hue: u8,
}

/// A polytonic volumetric body. The four strata are ORTHOGONAL lanes (never
/// summed, law 5); only the SPATIAL lane is a conformal multivector
/// (versor-acted). Gauge and topology pass through rigid motion unchanged —
/// that orthogonality IS the polytonic separation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blade {
    /// 𝒮 — the dual sphere `X(c) − ½r²e_∞`, non-null (`S·S = r² ≠ 0`) hence
    /// invertible: the geometric quotient can form.
    pub spatial: Mv,
    /// 𝒮 refinement — the EXACT second-moment (inertia) tensor about the
    /// centroid, rotation-covariant, no eigen/roots: its upper triangle on
    /// the base's `n` axes, each axis's diagonal followed by its products
    /// with the axes before it, `(I₀₀, I₁₁, I₀₁, I₂₂, I₀₂, I₁₂, …)`. The
    /// order is prefix-stable, so at `n = 3` it is `(Ixx, Iyy, Ixy, Izz,
    /// Ixz, Iyz)` and a reader of the first six reads the same moments at
    /// every `n`. A lamina (16.4) has the unit square's own moment on the
    /// two grid axes and none along any other.
    pub inertia: Vec<BigRational>,
    /// 𝒮 refinement — the exact fourth body moments about the centroid on
    /// the grid plane: the isotropic part `∫(u²+v²)²` and the fourth
    /// harmonic `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))`, which turns at four times
    /// the body's angle. It orients a body whose inertia is isotropic.
    pub quartic: [BigRational; 3],
    /// 𝒢 — the gauge/hue content as sorted `(hue, count)` relations (Vec, not a
    /// map: cognition obeys the Anti-Flattening Law). Versor-invariant.
    pub gauge: Vec<(u8, BigRational)>,
    /// 𝒽 — the shell topology: the component's lamina count.
    pub shell: BigRational,
    /// The body's support — the sites its laminae stand at, sorted (Vec,
    /// not a set). The stack, never its shadow (51.3).
    pub support: Vec<Site>,
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

    /// THE BODY'S NUCLEUS: its conformal sphere, centre and extent in ONE
    /// vector (64.1).
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
        let at = |i: usize| self.inertia.get(i);
        let read = |a: usize, b: usize, cross: usize| -> Option<[BigRational; 2]> {
            let (d1, d2, x) = (at(a)?, at(b)?, at(cross)?);
            let cross = x * q(2);
            Some([d1 - d2, if mirrored { -cross } else { cross }])
        };
        match plane {
            (PLANE_ROW, PLANE_COL) => read(0, 1, 2),
            (PLANE_ROW, 3) => read(0, 3, 4),
            (PLANE_COL, 3) => read(1, 3, 5),
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

/// THE MOMENTS' AXIS PAIRS, in the inertia's own order: each axis's
/// diagonal, then its products with the axes before it. Prefix-stable in
/// `n`, so the first six are the `n = 3` layout at every `n`.
fn inertia_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(n * (n + 1) / 2);
    for j in 0..n {
        out.push((j, j));
        for i in 0..j {
            out.push((i, j));
        }
    }
    out
}

/// Lift one connected component into a volumetric blade — exact ℚ
/// throughout, on the base's `n` axes (71.2(3)). The centroid is in ℚⁿ and
/// the moments are the full upper triangle of the second-moment tensor.
/// Each element is a lamina (16.4): the unit square standing on the grid
/// axes, so it contributes the square's own moment on those two axes and
/// only its offset along every other. The component's laminae need not be
/// sorted.
///
/// A COMPONENT HAS ONE `n`, and the lift reads it off its sites. A lamina
/// of a smaller base read beside a larger one stands in the larger by the
/// inclusion ℚᵐ ⊂ ℚⁿ, its missing coordinates zero — the same inclusion the
/// algebra composes values by — and `components` never builds such a
/// component, since two sites of different `n` are never adjacent.
pub fn lift(comp: &[Lamina]) -> Blade {
    let count = comp.len().max(1) as i64;
    let n_q = q(count);
    let dim = comp.iter().map(|l| l.site.n()).fold(0, usize::max);
    let coord = |l: &Lamina, i: usize| -> i64 {
        #[expect(
            clippy::manual_unwrap_or_default,
            reason = "the inclusion ℚᵐ ⊂ ℚⁿ (71): an axis a smaller base lacks is zero in the larger, measured and not defaulted"
        )]
        match l.site.axis(i) {
            Some(a) => a,
            // The inclusion: a coordinate the lamina's smaller base does
            // not have is zero in the larger one.
            None => 0,
        }
    };

    // The centroid c = (Σ x_i) / count ∈ ℚⁿ.
    let centroid: Vec<BigRational> = (0..dim)
        .map(|i| BigRational::new(BigInt::from(comp.iter().map(|l| coord(l, i)).sum::<i64>()), BigInt::from(count)))
        .collect();

    // Moments about the centroid (exact ℚ) — of the BODY: the union of unit
    // squares, so each lamina contributes the integral of the monomial over
    // its own square (the parallel-axis law at every order) on the grid
    // axes, and its offset alone along every other.
    let pairs = inertia_pairs(dim);
    let mut inertia: Vec<BigRational> = vec![q(0); pairs.len()];
    let (mut q_iso, mut h4x, mut h4y) = (q(0), q(0), q(0));
    let [ga, gb] = GRID_AXES;
    for l in comp {
        let d: Vec<BigRational> = (0..dim).map(|i| q(coord(l, i)) - &centroid[i]).collect();
        for (slot, &(i, j)) in inertia.iter_mut().zip(&pairs) {
            let mut m = &d[i] * &d[j];
            if i == j && is_grid_axis(i) {
                m += qn(1, 12);
            }
            *slot += m;
        }

        // The fourth moments on the grid plane. ∫u⁴ = d⁴ + d²/2 + 1/80;
        // ∫u² = d² + 1/12; ∫u³ = d³ + d/4; ∫u = d, each over the unit
        // interval centered at the offset d.
        let (dr, dc) = (
            match d.get(ga) {
                Some(x) => x.clone(),
                None => q(0),
            },
            match d.get(gb) {
                Some(x) => x.clone(),
                None => q(0),
            },
        );
        let (dr2, dc2) = (&dr * &dr, &dc * &dc);
        let u4 = &dr2 * &dr2 + &dr2 * qn(1, 2) + qn(1, 80);
        let v4 = &dc2 * &dc2 + &dc2 * qn(1, 2) + qn(1, 80);
        let u2v2 = (&dr2 + qn(1, 12)) * (&dc2 + qn(1, 12));
        let u3v = (&dr2 * &dr + &dr * qn(1, 4)) * &dc;
        let uv3 = &dr * (&dc2 * &dc + &dc * qn(1, 4));
        q_iso += &u4 + &u2v2 * q(2) + &v4;
        h4x += &u4 - &u2v2 * q(6) + &v4;
        h4y += (&u3v - &uv3) * q(4);
    }

    // Extent r² = the per-unit-mass polar second moment on the grid plane,
    // with the unit square's own polar moment (1/6) in it, so r² > 0 even
    // for a single lamina — off the null cone, invertible.
    let diag = |i: usize| -> BigRational {
        match pairs.iter().position(|&p| p == (i, i)) {
            Some(at) => inertia[at].clone(),
            None => q(0),
        }
    };
    let r2 = (diag(ga) + diag(gb)) / &n_q;
    let spatial = cga::sphere_at(&centroid, &r2);

    // Gauge: the hue multiset as sorted (hue, count) relations — Vec, not a map.
    let mut gauge: Vec<(u8, BigRational)> = Vec::new();
    for l in comp {
        match gauge.iter_mut().find(|(hue, _)| *hue == l.hue) {
            Some((_, c)) => *c += q(1),
            None => gauge.push((l.hue, q(1))),
        }
    }
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: the tally is keyed by hue, and the hue is the key"
    )]
    gauge.sort_by_key(|(h, _)| *h);

    let mut support: Vec<Site> = comp.iter().map(|l| l.site.clone()).collect();
    support.sort_unstable();
    support.dedup();

    Blade {
        spatial,
        inertia,
        quartic: [q_iso, h4x, h4y],
        gauge,
        shell: n_q,
        support,
    }
}

/// The connected components of a scene on the sites of the base, as
/// `Vec`-of-`Vec` (no maps/sets — Law II).
///
/// H1, 70.4 — A BODY IS A MAXIMAL HUE-BLIND CONNECTED REGION, and its hue
/// is the gauge section it carries, not its identity. Two laminae join iff
/// their cubes share a face — one coordinate differing by one (51.3) —
/// WHATEVER HUES THEY CARRY.
///
/// THIS SUPERSEDES CHAPTER 20 (ruled 2026-09-09), WHICH MADE PERCEPTION
/// MONOCHROME. "A body is one hue" defines a body by its colour, which is
/// not a body seen in colour — it is a body seen one colour at a time.
/// Measured over 40 tasks and 128 scenes under the old rule:
///
/// ```text
///     732 hue-blind regions → 1646 bodies      2.25 bodies per region
///     single-cell bodies    1053 of 1646       64.0%
///     134 polychrome objects → 1048 bodies, 7.8 fragments each
/// ```
///
/// And the tell was that the discarded cross-hue edges formed a graph
/// whose COMPONENT COUNT was exactly 732 — the region count — so what
/// looked like a contact graph between bodies was the internal gauge
/// structure of single objects, and its 188 cycles were loops WITHIN
/// polychrome shapes rather than between anything.
///
/// THE DEFECT WAS FIBER DATA IN THE BASE. Everything downstream already
/// treats hue as a gauge the body carries — `Carriage.sigma`, the `𝒢`
/// lane, `CycleType::of(gauge)`, `hue_pair_key` — and `Blade.gauge`
/// already carries a body's hues as sorted `(hue, count)` relations. A
/// body is WHERE MASS IS; its colour is what it CARRIES.
///
/// INTER-BODY CONTACT IS NOT FACE-ADJACENCY ANY MORE, and cannot be: two
/// distinct bodies sharing a face would be ONE body by maximality.
/// [`crate::transducer::sentence::incidence_diagonal_3d`] carries it
/// instead (70.6) — exact, integer, and in this same idiom.
pub fn components(laminae: &[Lamina]) -> Vec<Vec<Lamina>> {
    // The support in canonical order; its 1-skeleton read off the ordering
    // (sentence::incidence_of_3d), never a distance test against every
    // lamina. Every adjacency joins, because hue no longer decides
    // identity.
    let mut sorted: Vec<Lamina> = laminae.to_vec();
    sorted.sort_unstable();
    sorted.dedup_by(|a, b| a.site == b.site);
    let positions: Vec<Site> = sorted.iter().map(|l| l.site.clone()).collect();
    let edges: Vec<(usize, usize)> =
        crate::transducer::sentence::incidence_of_3d(&positions);
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

/// A site's coordinates off the grid plane: the frame, and every further
/// base axis. The key a section is filed under.
fn off_grid(s: &Site) -> Vec<i64> {
    s.axes().iter().enumerate().filter(|(i, _)| !is_grid_axis(*i)).map(|(_, a)| *a).collect()
}

/// A site's two grid coordinates: row and column.
fn on_grid(s: &Site) -> (i64, i64) {
    let [ga, gb] = GRID_AXES;
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "the inclusion ℚᵐ ⊂ ℚⁿ (71), as `lift` reads it: a grid axis a smaller base lacks is zero"
    )]
    let at = |i: usize| match s.axis(i) {
        Some(a) => a,
        None => 0,
    };
    (at(ga), at(gb))
}

/// ONE SECTION OF A STACK: its coordinates off the grid plane, and its
/// sites on the plane in row order.
pub type Section = (Vec<i64>, Vec<(i64, i64)>);

/// THE SECTIONS OF A SUPPORT: its sites grouped by their coordinates off
/// the grid plane — the frame, and any further axis — each section's sites
/// in row order: the laminae standing on one grid plane, the sections of
/// the stack (16.5). The in-plane readings (a section's turns and
/// perimeter) are taken here and summed over the stack; the topology is
/// not, because face adjacency off the plane joins the sections (51.3).
pub fn sections(support: &[Site]) -> Vec<Section> {
    let mut by_section: Vec<(Vec<i64>, (i64, i64))> =
        support.iter().map(|s| (off_grid(s), on_grid(s))).collect();
    by_section.sort_unstable();
    by_section.dedup();
    let mut out: Vec<Section> = Vec::new();
    for (key, rc) in by_section {
        match out.last_mut() {
            Some((k, section)) if *k == key => section.push(rc),
            _ => out.push((key, vec![rc])),
        }
    }
    out
}

/// THE OPEN FACES OF A SECTION, each named by the site it opens on: the
/// horizontal edges from (r,c) to (r,c+1), then the vertical ones from
/// (r,c) to (r+1,c). This is the coboundary of the support's 2-chain
/// within one section.
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
/// (a dilated body turns the same number of times) and a translation off
/// the grid plane moves no section's boundary.
pub fn turns(support: &[Site]) -> usize {
    sections(support).iter().map(|(_, section)| turns_of_section(section)).sum()
}

/// The in-plane boundary's length: the count of unit edges with a lamina
/// on exactly one side, summed over the sections.
pub fn boundary_length(support: &[Site]) -> usize {
    sections(support)
        .iter()
        .map(|(_, section)| {
            let (h, v) = boundary_edges(section);
            h.len() + v.len()
        })
        .sum()
}

/// THE BOUNDARY OFF THE GRID PLANE of a body: its sides open along every
/// axis the lamina has no extent on — a lamina's face toward the site
/// before it and toward the site after it on that axis, each counted where
/// no lamina of the body stands there (51.3). At the corpus's `n = 3` that
/// axis is the frame: two for every lamina of a body of one frame; fewer
/// for a stack.
pub fn frame_faces(support: &[Site]) -> usize {
    let mut sorted: Vec<Site> = support.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let held = |p: &Site| sorted.binary_search(p).is_ok();
    let mut open = 0usize;
    for s in &sorted {
        for axis in (0..s.n()).filter(|i| !is_grid_axis(*i)) {
            for step in [-1i64, 1] {
                let mut d = vec![0i64; s.n()];
                d[axis] = step;
                match s.offset(&d) {
                    Some(next) if held(&next) => {}
                    _ => open += 1,
                }
            }
        }
    }
    open
}

/// The three coordinates of a site of the corpus's `n = 3`, for the one
/// reading below that is taken at three and no other (38.2).
fn at_three(s: &Site) -> Option<[i64; 3]> {
    <[i64; 3]>::try_from(s.axes()).ok()
}

/// The topology of a support as the union of its closed unit cubes on the
/// face-adjacency complex (51.3): `(β₀, β₁, β₂)` — components, tunnels and
/// cavities. The Euler characteristic `χ = V − E + F − C` is counted
/// through incidence, each corner contributing `1/k` for the `k` cubes
/// meeting there and each unit segment `1/m` for the `m` cubes sharing it,
/// so the count is exact without a vertex set; `β₂` is the number of
/// enclosed components of the complement, and `β₁ = β₀ + β₂ − χ`. A body
/// of one frame reads its planar components and holes, and no cavity.
///
/// AT `n = 3` AND NO OTHER, and it says so by refusing. `β₁ = β₀ + β₂ − χ`
/// holds on a cube complex of three dimensions; past three there are Betti
/// numbers χ does not separate, and 38.2 reads them by exact integer linear
/// algebra over ℤ, which is BQ4's. `None` for a support of another `n`.
pub fn topology(support: &[Site]) -> Option<(u64, u64, u64)> {
    let mut sites: Vec<Site> = support.to_vec();
    sites.sort_unstable();
    sites.dedup();
    let sorted: Vec<[i64; 3]> = sites.iter().map(at_three).collect::<Option<Vec<_>>>()?;
    Some(topology_at_three(&sites, &sorted))
}

/// The topology of a support already read at three: `sites` sorted and
/// unique, `sorted` their three coordinates in the same order.
fn topology_at_three(sites: &[Site], sorted: &[[i64; 3]]) -> (u64, u64, u64) {
    if sorted.is_empty() {
        return (0, 0, 0);
    }
    let held = |r: i64, c: i64, k: i64| sorted.binary_search(&[r, c, k]).is_ok();
    let edges = crate::transducer::sentence::incidence_of_3d(sites);
    let mut roots = crate::transducer::sentence::components(sites.len(), &edges);
    roots.sort_unstable();
    roots.dedup();
    let b0 = roots.len() as i64;
    let n = sorted.len() as i64;
    let faces = 6 * n - edges.len() as i64;
    let mut vertices = q(0);
    let mut segments = q(0);
    for &[r, c, k] in sorted {
        // The eight corners `(x, y, z)`, each shared by the cubes whose
        // origin is one below it on any axis.
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
    let b2 = cavities(sorted);
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
fn cavities(sorted: &[[i64; 3]]) -> i64 {
    let (Some(r0), Some(r1), Some(c0), Some(c1), Some(k0), Some(k1)) = (
        sorted.iter().map(|p| p[0]).min(),
        sorted.iter().map(|p| p[0]).max(),
        sorted.iter().map(|p| p[1]).min(),
        sorted.iter().map(|p| p[1]).max(),
        sorted.iter().map(|p| p[2]).min(),
        sorted.iter().map(|p| p[2]).max(),
    ) else {
        return 0;
    };
    if k1 - k0 < 2 {
        return 0;
    }
    let held = |p: [i64; 3]| sorted.binary_search(&p).is_ok();
    let inside = |[r, c, k]: [i64; 3]| {
        (r0 - 1..=r1 + 1).contains(&r) && (c0 - 1..=c1 + 1).contains(&c) && (k0 - 1..=k1 + 1).contains(&k)
    };
    let on_border = |[r, c, k]: [i64; 3]| {
        r == r0 - 1 || r == r1 + 1 || c == c0 - 1 || c == c1 + 1 || k == k0 - 1 || k == k1 + 1
    };
    let six = |[r, c, k]: [i64; 3]| {
        [[r - 1, c, k], [r + 1, c, k], [r, c - 1, k], [r, c + 1, k], [r, c, k - 1], [r, c, k + 1]]
    };
    let key = |[r, c, k]: [i64; 3]| {
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
/// each (A21 as ruled 2026-09-18). Every harmonic is a whole coordinate
/// carrying its own exact value, and there is no width left to read as
/// absence. A body with support is never charted at the root.
///
/// `None` where the body's harmonics are not read at its `n` (38.2, BQ4):
/// a chart is not invented for it.
pub fn moduli(b: &Blade) -> Option<Vec<crate::transducer::stalk::ChartAxis>> {
    Some(crate::transducer::blade::harmonics::Harmonics::of(b)?.axes())
}

/// THE MODULI OF ONE SECTION — a chain's sites on the grid axes, the frame
/// on the pose (16.5). A section stands at `n = 3` by construction, so its
/// topology is read at three with nothing to refuse, and the chart is
/// total: the same reading [`moduli`] gives the section's shape.
pub fn section_moduli(sites: &[(i64, i64)]) -> Vec<crate::transducer::stalk::ChartAxis> {
    let shape = shape_of_section(sites);
    let mut triples: Vec<[i64; 3]> = sites.iter().map(|&(r, c)| [r, c, 0]).collect();
    triples.sort_unstable();
    triples.dedup();
    let at: Vec<Site> = triples.iter().map(|t| Site::new(t.to_vec())).collect();
    let t = topology_at_three(&at, &triples);
    crate::transducer::blade::harmonics::Harmonics::with_topology(&shape.blade, t).axes()
}

/// A body as the carrier a transport is solved on and verified against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub blade: Blade,
}

/// Lift a support (sites of the base, no hue) to a shape. The sites may be
/// handed over as anything the sensor reads as a site.
pub fn shape_of<S: Clone + Into<Site>>(support: &[S]) -> Shape {
    let comp: Vec<Lamina> = support.iter().map(|s| Lamina { site: s.clone().into(), hue: 0 }).collect();
    Shape { blade: lift(&comp) }
}

/// Lift the support of ONE SECTION — a chain's sites, which A5 reads at
/// the sensor as `(r, c)` with the frame on the pose (16.5) — to its shape
/// in the section's own frame.
pub fn shape_of_section(sites: &[(i64, i64)]) -> Shape {
    let comp: Vec<Lamina> = sites.iter().map(|&(r, c)| Lamina::from((r, c, 0, 0u8))).collect();
    Shape { blade: lift(&comp) }
}
