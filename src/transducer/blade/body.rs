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
    /// 𝒮 refinement — the exact fourth body moments about the centroid, ONE
    /// TRIPLE PER PLANE generated from `n` ([`crate::transducer::base::planes`]
    /// order, BQ4): on the plane `(i, j)` with `u, v` the offsets along its
    /// two axes, the isotropic part `∫(u²+v²)²` and the fourth harmonic
    /// `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))`, which turns at four times the
    /// body's angle in that plane. It orients a body whose inertia is
    /// isotropic there.
    pub quartic: Vec<[BigRational; 3]>,
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
    /// The base's `n` this body stands on: its sites' count of coordinates
    /// (71.2(1)). A body with no support stands on no base, `n = 0`.
    pub fn n(&self) -> usize {
        self.support.iter().map(|s| s.n()).fold(0, usize::max)
    }

    /// The centroid on the base, read back from the homogeneous sphere —
    /// exact ℚⁿ at the body's own `n` (PH2: no fixed arity).
    pub fn centroid(&self) -> Option<Vec<BigRational>> {
        cga::base_at(&self.spatial, self.n())
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
        Some(crate::transducer::cga::sphere_at(&self.centroid()?, &self.radius2()))
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

    /// THE PLANES A BODY HAS: every pair of its base's axes, generated from
    /// its `n` (71.2(3), BQ4). The body's harmonics are read on each.
    pub fn planes(&self) -> Vec<(usize, usize)> {
        crate::transducer::base::planes(self.n())
    }

    /// The second moment `I_ij` about the centroid, `i ≤ j` or either
    /// order. `None` for an axis the body's base does not have.
    pub fn moment2(&self, i: usize, j: usize) -> Option<&BigRational> {
        let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
        inertia_pairs(hi + 1).iter().position(|&p| p == (lo, hi)).and_then(|at| self.inertia.get(at))
    }

    /// The fourth-moment triple on a plane `(i, j)`, `i < j`. `None` for a
    /// plane the body's base does not have.
    pub fn quartic_on(&self, plane: (usize, usize)) -> Option<&[BigRational; 3]> {
        self.planes().iter().position(|&p| p == plane).and_then(|at| self.quartic.get(at))
    }

    /// THE SECOND HARMONIC ON A PLANE `(i, j)`: `(I_ii − I_jj, 2 I_ij)`, a
    /// vector that turns at twice the body's angle in that plane and
    /// vanishes where the body is isotropic there. `mirrored` reads it for
    /// the body reflected across the plane's first axis (the off-diagonal
    /// moment changes sign). `None` for a plane the body's base does not
    /// have.
    pub fn harmonic_on_plane(&self, plane: (usize, usize), mirrored: bool) -> Option<[BigRational; 2]> {
        let (i, j) = plane;
        let (di, dj, x) = (self.moment2(i, i)?, self.moment2(j, j)?, self.moment2(i, j)?);
        let cross = x * q(2);
        Some([di - dj, if mirrored { -cross } else { cross }])
    }

    /// THE FOURTH HARMONIC ON A PLANE, turning at four times the body's
    /// angle there. `None` for a plane the body's base does not have.
    pub fn harmonic4_on(&self, plane: (usize, usize), mirrored: bool) -> Option<[BigRational; 2]> {
        let [_, hx, hy] = self.quartic_on(plane)?;
        Some([hx.clone(), if mirrored { -hy } else { hy.clone() }])
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

/// THE LAMINA'S OWN MOMENTS ALONG ONE AXIS, `∫uᵖ` for p = 1…4, at offset
/// `d` from the centroid (16.4). On a grid axis the lamina is the unit
/// interval centred at `d`: ∫u = d, ∫u² = d² + 1/12, ∫u³ = d³ + d/4,
/// ∫u⁴ = d⁴ + d²/2 + 1/80. On every other axis it is the point `d`. The
/// ONE producer of a lamina's moments, read by the lift and by the tower's
/// odd harmonic (law 12).
pub(crate) fn lamina_moments(d: &BigRational, grid: bool) -> [BigRational; 4] {
    let x2 = d * d;
    match grid {
        true => [d.clone(), &x2 + qn(1, 12), &x2 * d + d * qn(1, 4), &x2 * &x2 + &x2 * qn(1, 2) + qn(1, 80)],
        false => [d.clone(), x2.clone(), &x2 * d, &x2 * &x2],
    }
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
    let planes = crate::transducer::base::planes(dim);
    let mut quartic: Vec<[BigRational; 3]> = vec![[q(0), q(0), q(0)]; planes.len()];
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

        // The axes are independent, so a moment on a plane is the product
        // of the lamina's own moments along its two axes.
        let along = |i: usize| lamina_moments(&d[i], is_grid_axis(i));
        // THE FOURTH MOMENTS ON EVERY PLANE generated from `n` (BQ4).
        for (slot, &(i, j)) in quartic.iter_mut().zip(&planes) {
            let ([u1, u2, u3, u4], [v1, v2, v3, v4]) = (along(i), along(j));
            let u2v2 = &u2 * &v2;
            let (u3v, uv3) = (&u3 * &v1, &u1 * &v3);
            slot[0] += &u4 + &u2v2 * q(2) + &v4;
            slot[1] += &u4 - &u2v2 * q(6) + &v4;
            slot[2] += (&u3v - &uv3) * q(4);
        }
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
        quartic,
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
/// [`crate::transducer::sentence::contact_cells`] carries it instead
/// (70.6) — the cells of lower grade, off the same reader.
pub fn components(laminae: &[Lamina]) -> Vec<Vec<Lamina>> {
    // The support in canonical order; its 1-skeleton read off the ordering
    // (sentence::face_incidence), never a distance test against every
    // lamina. Every adjacency joins, because hue no longer decides
    // identity.
    let mut sorted: Vec<Lamina> = laminae.to_vec();
    sorted.sort_unstable();
    sorted.dedup_by(|a, b| a.site == b.site);
    let positions: Vec<Site> = sorted.iter().map(|l| l.site.clone()).collect();
    let edges: Vec<(usize, usize)> =
        crate::transducer::sentence::face_incidence(&positions);
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

/// A CELL OF THE CUBICAL COMPLEX: the axes it has extent along (a bit per
/// base axis) and its lowest corner. Its dimension is the number of axes.
type Cell = (u64, Vec<i64>);

/// THE TOPOLOGY OF A SUPPORT (38.2, 71.2(6), BQ4): the Betti numbers
/// `β₀ … β_{n−1}` of the CLOSED UNION of its unit cubes on ℤⁿ, at any `n`.
///
/// The complex is enumerated whole — each cube `[s, s+1]ⁿ` contributes its
/// `3ⁿ` closed cells, shared cells counted once — and every Betti number is
/// read from the ranks of the boundary maps over ℤ,
///
/// ```text
///     βₖ = Cₖ − rank ∂ₖ − rank ∂ₖ₊₁
/// ```
///
/// by [`crate::core::lattice::sparse_rank`], exact. Two facts of the
/// complex are theorems and are not eliminated: `rank ∂₀ = 0`, and `∂ₙ` is
/// injective (`Hₙ = 0` for any finite cubical complex in ℝⁿ), so
/// `rank ∂ₙ = Cₙ`.
///
/// THE COMPLEX IS THE UNION'S, AND A BODY IS STILL FACE-ADJACENCY. Bodies
/// are the face-connected components ([`components`]); the chart's topology
/// is the union's, so two parts of a support that meet only at an edge or a
/// corner are one component here (71.2(6)). The `n = 3` formula this
/// replaces took `β₀` from face adjacency and `χ`, `β₂` from the union,
/// which is two complexes in one reading.
///
/// `None` where the sites stand on bases of different `n`, or where the
/// elimination overflows `i128` (refused, never wrapped).
pub fn topology(support: &[Site]) -> Option<Vec<u64>> {
    let mut sites: Vec<Site> = support.to_vec();
    sites.sort_unstable();
    sites.dedup();
    let n = match sites.as_slice() {
        [] => return Some(Vec::new()),
        [first, rest @ ..] => {
            if rest.iter().any(|s| s.n() != first.n()) {
                return None;
            }
            first.n()
        }
    };
    // THE CELLS BY DIMENSION, each sorted and unique: the index of a cell is
    // its place in its dimension's list, read by binary search.
    let mut by_dim: Vec<Vec<Cell>> = vec![Vec::new(); n + 1];
    for s in &sites {
        for dirs in 0..(1u64 << n) {
            // Every corner offset along the axes the cell has no extent on.
            let fixed: Vec<usize> = (0..n).filter(|i| dirs & (1 << i) == 0).collect();
            for eps in 0..(1u64 << fixed.len()) {
                let mut anchor = s.axes().to_vec();
                for (b, &i) in fixed.iter().enumerate() {
                    if eps & (1 << b) != 0 {
                        anchor[i] += 1;
                    }
                }
                by_dim[dirs.count_ones() as usize].push((dirs, anchor));
            }
        }
    }
    for cells in by_dim.iter_mut() {
        cells.sort_unstable();
        cells.dedup();
    }
    // `rank ∂ₖ` for 1 ≤ k < n by elimination; `rank ∂ₙ = Cₙ`.
    let mut rank = vec![0usize; n + 2];
    for k in 1..n {
        let faces = &by_dim[k - 1];
        let rows: Vec<crate::core::lattice::SparseRow> = by_dim[k]
            .iter()
            .map(|(dirs, anchor)| {
                let mut row: crate::core::lattice::SparseRow = Vec::with_capacity(2 * k);
                for (p, i) in (0..n).filter(|i| dirs & (1 << i) != 0).enumerate() {
                    let sign: i64 = if p % 2 == 0 { 1 } else { -1 };
                    let face_dirs = dirs & !(1 << i);
                    let mut upper = anchor.clone();
                    upper[i] += 1;
                    for (corner, s) in [(anchor.clone(), -sign), (upper, sign)] {
                        if let Ok(at) = faces.binary_search(&(face_dirs, corner)) {
                            row.push((at, s));
                        }
                    }
                }
                row.sort_unstable();
                row
            })
            .collect();
        rank[k] = crate::core::lattice::sparse_rank(&rows, faces.len())?;
    }
    rank[n] = by_dim[n].len();
    // `βₖ ≥ 0` is a theorem; a negative difference is a broken count and
    // refuses rather than wrapping (rule 11).
    (0..n)
        .map(|k| by_dim[k].len().checked_sub(rank[k])?.checked_sub(rank[k + 1]).map(|b| b as u64))
        .collect()
}

/// THE MODULI of a body — its coordinates on the base, one `ChartAxis`
/// each (A21 as ruled 2026-09-18). Every harmonic is a whole coordinate
/// carrying its own exact value, and there is no width left to read as
/// absence. A body with support is never charted at the root.
///
/// `None` where the body's topology is refused ([`topology`]): a chart is
/// not invented for it.
pub fn moduli(b: &Blade) -> Option<Vec<crate::transducer::stalk::ChartAxis>> {
    Some(crate::transducer::blade::harmonics::Harmonics::of(b)?.axes())
}

/// THE MODULI OF ONE SECTION — a chain's sites on the grid axes, the frame
/// on the pose (16.5): the same reading [`moduli`] gives the section's
/// shape.
///
/// A CHART ONLY LOCATES (71.2(6)), so where the section's topology is
/// refused the section is filed at the coarsest stratum it honestly has —
/// its base's `n`, the chart's first axis — and nothing below it is
/// invented. The physics decides what the section is; the chart only finds
/// the candidates.
pub fn section_moduli(sites: &[(i64, i64)]) -> Vec<crate::transducer::stalk::ChartAxis> {
    let shape = shape_of_section(sites);
    match moduli(&shape.blade) {
        Some(axes) => axes,
        None => vec![crate::transducer::stalk::ChartAxis::Count(shape.blade.n() as u64)],
    }
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
