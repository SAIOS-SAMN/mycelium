// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH NERVE — vertices are the hyphas (or germs) whose cover contains the
//! chart, each contributing one section. Words that differ inside one hypha
//! are the stalk over that vertex and never vertices. Coefficients are
//! PlaneForm. Work per glue is the link of one vertex: at most
//! C(m−1,1)+C(m−1,2)+C(m−1,3) products, m the cover multiplicity. No simplex
//! is enumerated outside a link. A transition that does not spell into a
//! PlaneForm is a remainder edge; simplices over it are not formed.

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::PlaneForm;
use crate::transducer::element::Element;

/// The TRANSITION carrying chart X's view to chart Y's, from each chart's germ for
/// the shared object: `g_XY = g_Y ∘ g_X⁻¹` (the change of view). Antisymmetric by
/// construction — `g_YX = g_XY⁻¹`. `None` if X's germ is not invertible (a
/// coboundary/residual chart has no group view-change; that overlap is handled by
/// the additive coboundary, not this group transition).
pub fn transition(g_x: &Element, g_y: &Element) -> Option<Element> {
    Some(g_y.compose(&g_x.solution()?))
}

/// The HOLONOMY around a triangle of charts — the composite transition
/// `g_ab ∘ g_bc ∘ g_ca`. Identity ⇔ the charts GLUE (one sheaf); otherwise it is
/// the exact obstruction (the equivocation) the charts must reconcile.
pub fn holonomy(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> Element {
    g_ab.compose(g_bc).compose(g_ca)
}

/// Does a triangle of charts GLUE — the Čech cocycle `δg = 0`? True iff the
/// holonomy around the overlap is trivial (the transitions are the coboundary of a
/// consistent global assignment). False = CURVATURE (δσ ≠ 0), a dialect.
pub fn glues(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> bool {
    holonomy(g_ab, g_bc, g_ca).is_identity()
}

/// Spell a group transition into the coefficient group. None is a remainder:
/// the edge is honest and no simplex is formed over it.
pub fn form_of(g: &Element) -> Option<PlaneForm> {
    let m = g.fiber(crate::transducer::element::FIBER_SPATIAL)?;
    if m.len() < 2 || m[0].len() < 2 {
        return None;
    }
    let block = [
        [m[0][0].clone(), m[0][1].clone()],
        [m[1][0].clone(), m[1][1].clone()],
    ];
    crate::transducer::alphabet::spell(&block)?.word.form()
}

/// A simplex of the nerve: sorted vertex identities, hashed to one key.
/// Dimension is `verts.len() - 1`, at most 3.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Simplex {
    pub verts: Vec<[u8; 32]>,
}

impl Simplex {
    pub fn new(mut verts: Vec<[u8; 32]>) -> Option<Simplex> {
        if verts.is_empty() || verts.len() > 4 {
            return None;
        }
        verts.sort_unstable();
        verts.dedup();
        if verts.is_empty() || verts.len() > 4 {
            return None;
        }
        Some(Simplex { verts })
    }

    pub fn dim(&self) -> usize {
        self.verts.len().saturating_sub(1)
    }

    pub fn key(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(&[self.dim() as u8]);
        for v in &self.verts {
            h.update(v);
        }
        *h.finalize().as_bytes()
    }

    pub fn contains(&self, v: &[u8; 32]) -> bool {
        self.verts.binary_search(v).is_ok()
    }

    pub fn other<'a>(&'a self, v: &[u8; 32]) -> Option<&'a [u8; 32]> {
        if self.dim() != 1 {
            return None;
        }
        self.verts.iter().find(|u| *u != v)
    }
}

/// c⁰ value: the representative address on this chart and its gauge γ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section {
    pub addr: [u8; 32],
    pub gamma: PlaneForm,
}

/// Fill of a triangle: δγ = h has a solution (the three views are one after
/// re-gauging) or h is the torsion class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fill {
    Filled(PlaneForm),
    Twisted(PlaneForm),
}

/// Cochains on one chart. One Keyed per degree; the star is vertex → simplex
/// keys so `link` is a peel, never a scan of all simplices.
#[derive(Clone, Debug, Default)]
pub struct Cochains {
    pub c0: Keyed<Section>,
    pub c1: Keyed<PlaneForm>,
    pub c2: Keyed<PlaneForm>,
    pub c3: Keyed<PlaneForm>,
    simplices: Keyed<Simplex>,
    star: Keyed<Keyed<()>>,
    remainder: Keyed<()>,
}

impl Cochains {
    pub fn new() -> Cochains {
        Cochains::default()
    }

    fn remember(&mut self, s: Simplex) {
        let key = s.key();
        for v in &s.verts {
            match self.star.get_mut(v) {
                Some(keys) => {
                    keys.insert(key, ());
                }
                None => {
                    let mut keys = Keyed::new();
                    keys.insert(key, ());
                    self.star.insert(*v, keys);
                }
            }
        }
        self.simplices.insert(key, s);
    }

    /// c⁰: the representative and gauge at this vertex.
    pub fn set_vertex(&mut self, vertex: [u8; 32], section: Section) {
        self.c0.insert(vertex, section);
        if let Some(s) = Simplex::new(vec![vertex]) {
            self.remember(s);
        }
    }

    /// c¹: g_ij on the sorted edge, or a remainder if the transition does
    /// not spell. Remainder edges do not form higher simplices.
    pub fn set_edge(&mut self, a: [u8; 32], b: [u8; 32], g: PlaneForm) {
        let Some(s) = Simplex::new(vec![a, b]) else {
            return;
        };
        let key = s.key();
        self.remainder.remove(&key);
        let stored = if s.verts[0] == a { g } else { g.inverse() };
        self.c1.insert(key, stored);
        self.remember(s);
    }

    pub fn set_remainder_edge(&mut self, a: [u8; 32], b: [u8; 32]) {
        let Some(s) = Simplex::new(vec![a, b]) else {
            return;
        };
        let key = s.key();
        self.c1.remove(&key);
        self.remainder.insert(key, ());
    }

    fn is_remainder(&self, a: &[u8; 32], b: &[u8; 32]) -> bool {
        Simplex::new(vec![*a, *b]).is_some_and(|s| self.remainder.contains(&s.key()))
    }

    /// g from a to b, oriented. None if the edge is missing or a remainder.
    pub fn edge(&self, a: &[u8; 32], b: &[u8; 32]) -> Option<PlaneForm> {
        let s = Simplex::new(vec![*a, *b])?;
        let g = *self.c1.get(&s.key())?;
        if s.verts[0] == *a {
            Some(g)
        } else {
            Some(g.inverse())
        }
    }

    /// Simplices that contain `vertex` — a peel of the star, not a scan.
    pub fn link(&self, vertex: &[u8; 32]) -> Vec<Simplex> {
        let mut out = Vec::new();
        let Some(keys) = self.star.get(vertex) else {
            return out;
        };
        keys.for_each(|k, _| {
            if let Some(s) = self.simplices.get(k) {
                out.push(s.clone());
            }
        });
        out
    }

    fn neighbors(&self, vertex: &[u8; 32]) -> Vec<[u8; 32]> {
        let mut out = Vec::new();
        for s in self.link(vertex) {
            if s.dim() == 1 {
                if let Some(u) = s.other(vertex) {
                    out.push(*u);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    fn holonomy_of(&self, i: &[u8; 32], j: &[u8; 32], k: &[u8; 32]) -> Option<PlaneForm> {
        let g_ij = self.edge(i, j)?;
        let g_jk = self.edge(j, k)?;
        let g_ki = self.edge(k, i)?;
        Some(g_ij.compose(&g_jk).compose(&g_ki))
    }

    /// Compute c² and c³ on the link of one vertex only.
    pub fn delta_on_link(&mut self, vertex: &[u8; 32]) {
        let nbs = self.neighbors(vertex);
        for a in 0..nbs.len() {
            for b in a + 1..nbs.len() {
                let (i, j) = (nbs[a], nbs[b]);
                if self.is_remainder(vertex, &i)
                    || self.is_remainder(vertex, &j)
                    || self.is_remainder(&i, &j)
                {
                    continue;
                }
                let Some(s) = Simplex::new(vec![*vertex, i, j]) else {
                    continue;
                };
                let (p, q, r) = (s.verts[0], s.verts[1], s.verts[2]);
                let Some(h) = self.holonomy_of(&p, &q, &r) else {
                    continue;
                };
                let key = s.key();
                self.c2.insert(key, h);
                self.remember(s);
            }
        }
        for a in 0..nbs.len() {
            for b in a + 1..nbs.len() {
                for c in b + 1..nbs.len() {
                    let (i, j, k) = (nbs[a], nbs[b], nbs[c]);
                    let Some(s) = Simplex::new(vec![*vertex, i, j, k]) else {
                        continue;
                    };
                    let Some(dh) = self.delta_h(&s) else {
                        continue;
                    };
                    let key = s.key();
                    self.c3.insert(key, dh);
                    self.remember(s);
                }
            }
        }
    }

    /// δh on a tetrahedron: h_jkl · (h_ikl⁻¹ transported by the shared edge)
    /// · h_ijl · (h_ijk⁻¹ transported by the shared edge).
    fn delta_h(&self, tetra: &Simplex) -> Option<PlaneForm> {
        if tetra.dim() != 3 {
            return None;
        }
        let (i, j, k, l) = (tetra.verts[0], tetra.verts[1], tetra.verts[2], tetra.verts[3]);
        let face = |a: [u8; 32], b: [u8; 32], c: [u8; 32]| {
            Simplex::new(vec![a, b, c])
                .and_then(|s| self.c2.get(&s.key()).copied())
                .or_else(|| self.holonomy_of(&a, &b, &c))
        };
        let h_jkl = face(j, k, l)?;
        let h_ikl = face(i, k, l)?;
        let h_ijl = face(i, j, l)?;
        let h_ijk = face(i, j, k)?;
        let g_ij = self.edge(&i, &j)?;
        let g_kl = self.edge(&k, &l)?;
        let h_ikl_t = h_ikl.conjugate_by(&g_ij);
        let h_ijk_t = h_ijk.conjugate_by(&g_kl);
        Some(
            h_jkl
                .compose(&h_ikl_t.inverse())
                .compose(&h_ijl)
                .compose(&h_ijk_t.inverse()),
        )
    }

    /// Solve δγ = h on the triangle. Identity holonomy fills (tree gauge);
    /// otherwise the holonomy is the class.
    pub fn fill(&mut self, triangle: &Simplex) -> Fill {
        if triangle.dim() != 2 {
            return Fill::Twisted(PlaneForm::identity());
        }
        let (a, b, c) = (triangle.verts[0], triangle.verts[1], triangle.verts[2]);
        let h = self
            .c2
            .get(&triangle.key())
            .copied()
            .or_else(|| self.holonomy_of(&a, &b, &c))
            .unwrap_or_else(PlaneForm::identity);
        let Some(g_ab) = self.edge(&a, &b) else {
            return Fill::Twisted(h);
        };
        let Some(g_bc) = self.edge(&b, &c) else {
            return Fill::Twisted(h);
        };
        let Some(g_ac) = self.edge(&a, &c) else {
            return Fill::Twisted(h);
        };
        match solve_gauge(3, &[(0, 1, g_ab), (1, 2, g_bc), (0, 2, g_ac)]) {
            Some(gs) => {
                for (v, g) in [a, b, c].iter().zip(gs.iter()) {
                    if let Some(sec) = self.c0.get_mut(v) {
                        sec.gamma = *g;
                    }
                }
                Fill::Filled(gs.get(1).copied().unwrap_or_else(PlaneForm::identity))
            }
            None => Fill::Twisted(h),
        }
    }

    /// The H² class on a tetrahedron: Some iff δh ≠ 1.
    pub fn class(&self, tetra: &Simplex) -> Option<PlaneForm> {
        let dh = self
            .c3
            .get(&tetra.key())
            .copied()
            .or_else(|| self.delta_h(tetra))?;
        if dh.is_identity() {
            None
        } else {
            Some(dh)
        }
    }

    pub fn simplex_count(&self) -> usize {
        self.c0.len() + self.c1.len() + self.c2.len() + self.c3.len()
    }

    /// Simplices of one degree with their coefficient. A walk of that
    /// Keyed, never a scan of the other degrees.
    pub fn of_degree(&self, dim: usize) -> Vec<(Simplex, PlaneForm)> {
        let keyed = match dim {
            1 => &self.c1,
            2 => &self.c2,
            3 => &self.c3,
            _ => return Vec::new(),
        };
        let mut out = Vec::new();
        keyed.for_each(|k, g| {
            if let Some(s) = self.simplices.get(k) {
                out.push((s.clone(), *g));
            }
        });
        out
    }

    /// Bytes of the cochain values (one PlaneForm per simplex, plus c⁰ addresses).
    pub fn byte_len(&self) -> usize {
        const FORM: usize = 1 + 8 * 17;
        self.c0.len() * (32 + FORM) + (self.c1.len() + self.c2.len() + self.c3.len()) * FORM
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.c0.len() as u32).to_le_bytes());
        for (k, s) in self.c0.entries() {
            out.extend_from_slice(k);
            out.extend_from_slice(&s.addr);
            s.gamma.to_bytes(out);
        }
        let write_forms = |out: &mut Vec<u8>, keyed: &Keyed<PlaneForm>, dim: usize| {
            out.extend_from_slice(&(keyed.len() as u32).to_le_bytes());
            for (k, g) in keyed.entries() {
                if let Some(s) = self.simplices.get(k) {
                    out.push(s.verts.len() as u8);
                    for v in &s.verts {
                        out.extend_from_slice(v);
                    }
                } else {
                    out.push((dim + 1) as u8);
                    for _ in 0..=dim {
                        out.extend_from_slice(&[0u8; 32]);
                    }
                }
                g.to_bytes(out);
            }
        };
        write_forms(out, &self.c1, 1);
        write_forms(out, &self.c2, 2);
        write_forms(out, &self.c3, 3);
        out.extend_from_slice(&(self.remainder.len() as u32).to_le_bytes());
        for (k, _) in self.remainder.entries() {
            if let Some(s) = self.simplices.get(k) {
                out.push(s.verts.len() as u8);
                for v in &s.verts {
                    out.extend_from_slice(v);
                }
            } else {
                out.push(2);
                out.extend_from_slice(k);
                out.extend_from_slice(&[0u8; 32]);
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Cochains> {
        let take_u32 = |b: &[u8], pos: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
            Some(v)
        };
        let take_id = |b: &[u8], pos: &mut usize| -> Option<[u8; 32]> {
            let v: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            Some(v)
        };
        let mut c = Cochains::new();
        let n0 = take_u32(b, pos)? as usize;
        for _ in 0..n0.min(1 << 16) {
            let v = take_id(b, pos)?;
            let addr = take_id(b, pos)?;
            let gamma = PlaneForm::from_bytes(b, pos)?;
            c.set_vertex(v, Section { addr, gamma });
        }
        for dim in 1..=3u8 {
            let n = take_u32(b, pos)? as usize;
            for _ in 0..n.min(1 << 16) {
                let nv = *b.get(*pos)? as usize;
                *pos += 1;
                let mut verts = Vec::with_capacity(nv.min(4));
                for _ in 0..nv.min(4) {
                    verts.push(take_id(b, pos)?);
                }
                let g = PlaneForm::from_bytes(b, pos)?;
                if let Some(s) = Simplex::new(verts) {
                    match dim {
                        1 if s.verts.len() == 2 => c.set_edge(s.verts[0], s.verts[1], g),
                        2 => {
                            let key = s.key();
                            c.c2.insert(key, g);
                            c.remember(s);
                        }
                        3 => {
                            let key = s.key();
                            c.c3.insert(key, g);
                            c.remember(s);
                        }
                        _ => {}
                    }
                }
            }
        }
        let nr = take_u32(b, pos).unwrap_or(0) as usize;
        for _ in 0..nr.min(1 << 16) {
            let nv = *b.get(*pos)? as usize;
            *pos += 1;
            let mut verts = Vec::with_capacity(nv.min(4));
            for _ in 0..nv.min(4) {
                verts.push(take_id(b, pos)?);
            }
            if verts.len() == 2 {
                c.set_remainder_edge(verts[0], verts[1]);
            }
        }
        Some(c)
    }
}

/// Tree-plus-chord solver for δγ = ω on PlaneForm coefficients (the
/// abelianized-plus-parity gauge: a forest 1-cochain is exact; a chord
/// that does not match the tree path is a withheld class).
pub fn solve_gauge(n: usize, edges: &[(usize, usize, PlaneForm)]) -> Option<Vec<PlaneForm>> {
    if n == 0 {
        return Some(Vec::new());
    }
    let mut parent: Vec<usize> = (0..n).collect();
    let find = |parent: &mut [usize], mut x: usize| {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };
    let mut tree: Vec<(usize, usize, PlaneForm)> = Vec::new();
    let mut chords: Vec<(usize, usize, PlaneForm)> = Vec::new();
    for &(i, j, g) in edges {
        if i >= n || j >= n {
            continue;
        }
        let (a, b) = (find(&mut parent, i), find(&mut parent, j));
        if a != b {
            parent[a] = b;
            tree.push((i, j, g));
        } else {
            chords.push((i, j, g));
        }
    }
    let mut gamma = vec![PlaneForm::identity(); n];
    let mut placed = vec![false; n];
    placed[0] = true;
    let mut progressed = true;
    while progressed {
        progressed = false;
        for &(i, j, g) in &tree {
            match (placed[i], placed[j]) {
                (true, false) => {
                    gamma[j] = g.compose(&gamma[i]);
                    placed[j] = true;
                    progressed = true;
                }
                (false, true) => {
                    gamma[i] = g.inverse().compose(&gamma[j]);
                    placed[i] = true;
                    progressed = true;
                }
                _ => {}
            }
        }
    }
    for &(i, j, g) in &chords {
        let expect = gamma[j].compose(&gamma[i].inverse());
        if expect != g {
            return None;
        }
    }
    Some(gamma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::engine::Q;
    use crate::transducer::element::FIBER_SPATIAL;
    use num_bigint::BigInt;

    fn motion(dc: i64) -> Element {
        let q = |v: i64| Q::from(BigInt::from(v));
        Element::identity(3).with_fiber(
            FIBER_SPATIAL,
            vec![
                vec![q(1), q(0), q(0)],
                vec![q(0), q(1), q(dc)],
                vec![q(0), q(0), q(1)],
            ],
        )
    }

    fn v(n: u8) -> [u8; 32] {
        [n; 32]
    }

    fn quarter() -> PlaneForm {
        let id = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        crate::transducer::alphabet::Word::from_letters(&[(id, false)])
            .form()
            .unwrap()
    }

    /// A TRANSITION is antisymmetric: `g_XY ∘ g_YX = 1` (`g_YX = g_XY⁻¹`).
    #[test]
    fn a_transition_and_its_reverse_cancel() {
        let (gx, gy) = (motion(3), motion(6));
        let f = transition(&gx, &gy).unwrap();
        let r = transition(&gy, &gx).unwrap();
        assert!(f.compose(&r).is_identity(), "g_XY ∘ g_YX = 1 — asymmetric coboundary");
    }

    /// CONSILIENT charts glue trivially: if all three learned the SAME germ, every
    /// transition is the identity and the cocycle closes.
    #[test]
    fn consilient_charts_glue() {
        let g = motion(3);
        let id = transition(&g, &g).unwrap();
        assert!(id.is_identity());
        assert!(glues(&id, &id, &id), "identical views → one sheaf");
    }

    /// A GLOBAL SECTION always glues: transitions built as `g_Y ∘ g_X⁻¹` from ANY
    /// per-chart germs are a coboundary, so δδ = 0 — the cocycle closes regardless
    /// of the germs. This is the consistent-assignment (no-dialect) case.
    #[test]
    fn a_global_section_always_glues() {
        let (ga, gb, gc) = (motion(3), motion(6), motion(9));
        let g_ab = transition(&ga, &gb).unwrap();
        let g_bc = transition(&gb, &gc).unwrap();
        let g_ca = transition(&gc, &ga).unwrap();
        assert!(glues(&g_ab, &g_bc, &g_ca), "a coboundary always satisfies δδ = 0");
    }

    /// An EQUIVOCATION is CURVATURE: transitions that do NOT come from a global
    /// section leave a nonzero holonomy — the cocycle fails, the dialect is caught.
    #[test]
    fn an_equivocation_shows_up_as_curvature() {
        let g = motion(3);
        let id = transition(&g, &g).unwrap();
        let disagree = transition(&motion(3), &motion(6)).unwrap();
        assert!(!disagree.is_identity(), "the disagreeing edge is nontrivial");
        assert!(!glues(&id, &id, &disagree), "a lone dialect edge → curvature (δσ ≠ 0)");
        assert!(!holonomy(&id, &id, &disagree).is_identity(), "the holonomy is the exact obstruction");
    }

    /// Simplex keys are the sorted vertices; the link of one vertex is only
    /// the incident simplices.
    #[test]
    fn the_link_is_the_star_not_the_nerve() {
        let mut c = Cochains::new();
        let (a, b, cc, d) = (v(1), v(2), v(3), v(4));
        for x in [a, b, cc, d] {
            c.set_vertex(x, Section { addr: x, gamma: PlaneForm::identity() });
        }
        let id = PlaneForm::identity();
        c.set_edge(a, b, id);
        c.set_edge(a, cc, id);
        c.set_edge(b, cc, id);
        c.set_edge(cc, d, id);
        c.delta_on_link(&a);
        let link_a: Vec<_> = c.link(&a);
        assert!(link_a.iter().all(|s| s.contains(&a)), "link(a) contains only a");
        assert!(c.class(&Simplex::new(vec![a, b, cc, d]).unwrap()).is_none() || c.c3.is_empty());
        assert!(c.c3.is_empty(), "a tetra not on the link of a is not formed");
        c.delta_on_link(&cc);
        assert_eq!(c.c2.len(), 1, "one triangle on the cover of three");
        assert!(c.c3.is_empty(), "four vertices but d is not a neighbor of a; no tetra on a's leftover");
    }

    /// A coboundary 1-cochain fills; a leftover holonomy is the class.
    #[test]
    fn fill_is_gauge_or_class() {
        let mut c = Cochains::new();
        let (a, b, cc) = (v(1), v(2), v(3));
        for x in [a, b, cc] {
            c.set_vertex(x, Section { addr: x, gamma: PlaneForm::identity() });
        }
        let q = quarter();
        c.set_edge(a, b, q);
        c.set_edge(b, cc, q);
        c.set_edge(a, cc, q.compose(&q));
        c.delta_on_link(&a);
        let tri = Simplex::new(vec![a, b, cc]).unwrap();
        assert!(matches!(c.fill(&tri), Fill::Filled(_)));
        c.set_edge(a, cc, PlaneForm::identity());
        c.delta_on_link(&a);
        assert!(matches!(c.fill(&tri), Fill::Twisted(_)));
    }

    /// Four vertices with a coboundary 1-cochain have δh = 1; a leftover
    /// face class is an H² class.
    #[test]
    fn a_tetrahedron_class_is_the_h2() {
        let mut c = Cochains::new();
        let vs = [v(1), v(2), v(3), v(4)];
        for x in vs {
            c.set_vertex(x, Section { addr: x, gamma: PlaneForm::identity() });
        }
        let id = PlaneForm::identity();
        for i in 0..4 {
            for j in i + 1..4 {
                c.set_edge(vs[i], vs[j], id);
            }
        }
        c.delta_on_link(&vs[0]);
        let tet = Simplex::new(vs.to_vec()).unwrap();
        assert!(c.class(&tet).is_none(), "a coboundary has δh = 1");
        let q = quarter();
        let mut sheet = PlaneForm::identity();
        sheet.parity = true;
        // A rotation on one pair of vertices and a sheet on the opposite
        // edge: the transported faces do not cancel, so δh ≠ 1.
        c.set_edge(vs[0], vs[1], q);
        c.set_edge(vs[2], vs[3], sheet);
        c.delta_on_link(&vs[0]);
        assert!(c.class(&tet).is_some(), "an inconsistent face is an H² class");
        assert!(c.byte_len() < 4096, "four covering vertices stay in kilobytes");
    }
}
