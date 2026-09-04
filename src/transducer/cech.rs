// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH COMPLEX — vertices are the hyphas (or germs) whose cover contains
//! the chart, one section each. A k-cell is a (k+1)-fold overlap and exists
//! only when that overlap has volume (open listings / shared support), a
//! polytonic μ⃗, never a word count. Coefficients are Rotor of Cl(4,0).
//! Work per glue is the link of one vertex, climbing degree only while
//! volume persists. A transition that does not realise as a rotor is a
//! remainder; no cell forms over it. The tower has no top.

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Rotor;
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

/// Realise a group transition as an alphabet rotor. None is a remainder:
/// the edge is honest and no cell is formed over it.
pub fn rotor_of(g: &Element) -> Option<Rotor> {
    if let Some(letters) = g.letters() {
        return Some(letters.spelling.word.rotor());
    }
    let m = g.fiber(crate::transducer::element::FIBER_SPATIAL)?;
    if m.len() < 2 || m[0].len() < 2 {
        return None;
    }
    let block = [
        [m[0][0].clone(), m[0][1].clone()],
        [m[1][0].clone(), m[1][1].clone()],
    ];
    Some(crate::transducer::alphabet::spell(&block)?.word.rotor())
}

fn nonzero(v: &[u64; 4]) -> bool {
    v.iter().any(|&x| x > 0)
}

fn cell_key(verts: &[[u8; 32]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&(verts.len() as u8).to_le_bytes());
    for v in verts {
        h.update(v);
    }
    *h.finalize().as_bytes()
}

fn sorted_verts(mut verts: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    verts.sort_unstable();
    verts.dedup();
    verts
}

/// Combinations of `k` items from the link — C(m−1, k), never a scan of
/// vertices outside the link.
fn each_combination<T: Copy>(items: &[T], k: usize, mut f: impl FnMut(&[T])) {
    if k == 0 || k > items.len() {
        return;
    }
    let mut idx: Vec<usize> = (0..k).collect();
    loop {
        let combo: Vec<T> = idx.iter().map(|&i| items[i]).collect();
        f(&combo);
        let mut i = k;
        let mut advanced = false;
        while i > 0 {
            i -= 1;
            if idx[i] != i + (items.len() - k) {
                idx[i] += 1;
                for j in i + 1..k {
                    idx[j] = idx[j - 1] + 1;
                }
                advanced = true;
                break;
            }
        }
        if !advanced {
            return;
        }
    }
}

/// Fill of a cell: a gauge on its boundary makes the coefficient the
/// identity, or the coefficient is the class at that degree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fill {
    Unfilled,
    Filled(Rotor),
    Class(Rotor),
}

/// One cell of the Čech complex. Keyed by the hash of its sorted vertices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub vertices: Vec<[u8; 32]>,
    pub coefficient: Rotor,
    pub volume: [u64; 4],
    pub fill: Fill,
}

impl Cell {
    pub fn degree(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }

    pub fn key(&self) -> [u8; 32] {
        cell_key(&self.vertices)
    }

    pub fn contains(&self, v: &[u8; 32]) -> bool {
        self.vertices.binary_search(v).is_ok()
    }
}

/// Degree-0 section: the representative word after in-hypha gluing, and its gauge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub addr: [u8; 32],
    pub gamma: Rotor,
}

/// The Čech complex on one chart. One Keyed trie per degree, grown as
/// overlaps demand. No fixed arity.
#[derive(Clone, Debug, Default)]
pub struct Complex {
    pub degrees: Vec<Keyed<Cell>>,
    pub sections: Keyed<Section>,
    star: Keyed<Keyed<()>>,
    remainder: Keyed<()>,
}

impl Complex {
    pub fn new() -> Complex {
        Complex::default()
    }

    fn ensure(&mut self, degree: usize) {
        while self.degrees.len() <= degree {
            self.degrees.push(Keyed::new());
        }
    }

    fn remember(&mut self, cell: Cell) {
        let degree = cell.degree();
        let key = cell.key();
        self.ensure(degree);
        for v in &cell.vertices {
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
        self.degrees[degree].insert(key, cell);
    }

    pub fn set_section(&mut self, vertex: [u8; 32], section: Section, volume: [u64; 4]) {
        self.sections.insert(vertex, section);
        if nonzero(&volume) {
            self.remember(Cell {
                vertices: vec![vertex],
                coefficient: Rotor::identity(),
                volume,
                fill: Fill::Filled(Rotor::identity()),
            });
        }
    }

    /// Degree-1 cell: the transition rotor, or a remainder if it does not
    /// realise. Remainder edges form no higher cells.
    pub fn set_edge(&mut self, a: [u8; 32], b: [u8; 32], g: Rotor, volume: [u64; 4]) {
        let vertices = sorted_verts(vec![a, b]);
        if vertices.len() != 2 {
            return;
        }
        let key = cell_key(&vertices);
        self.remainder.remove(&key);
        if !nonzero(&volume) {
            return;
        }
        let stored = if vertices[0] == a { g } else { g.inverse().unwrap_or_else(Rotor::identity) };
        self.remember(Cell {
            vertices,
            coefficient: stored,
            volume,
            fill: Fill::Unfilled,
        });
    }

    pub fn set_remainder_edge(&mut self, a: [u8; 32], b: [u8; 32]) {
        let vertices = sorted_verts(vec![a, b]);
        if vertices.len() != 2 {
            return;
        }
        let key = cell_key(&vertices);
        if self.degrees.get(1).is_some() {
            self.degrees[1].remove(&key);
        }
        self.remainder.insert(key, ());
    }

    fn is_remainder(&self, verts: &[[u8; 32]]) -> bool {
        let v = sorted_verts(verts.to_vec());
        self.remainder.contains(&cell_key(&v))
    }

    pub fn get(&self, verts: &[[u8; 32]]) -> Option<&Cell> {
        let d = verts.len().saturating_sub(1);
        self.degrees.get(d)?.get(&cell_key(verts))
    }

    /// Oriented edge rotor from a to b.
    pub fn edge(&self, a: &[u8; 32], b: &[u8; 32]) -> Option<Rotor> {
        let vertices = sorted_verts(vec![*a, *b]);
        let cell = self.get(&vertices)?;
        if vertices[0] == *a {
            Some(cell.coefficient.clone())
        } else {
            cell.coefficient.inverse()
        }
    }

    /// Cells of one degree that contain `vertex` — a peel of the star.
    pub fn link(&self, vertex: &[u8; 32], degree: usize) -> Vec<Cell> {
        let mut out = Vec::new();
        let Some(keys) = self.star.get(vertex) else {
            return out;
        };
        let Some(layer) = self.degrees.get(degree) else {
            return out;
        };
        keys.for_each(|k, _| {
            if let Some(c) = layer.get(k) {
                out.push(c.clone());
            }
        });
        out
    }

    fn neighbors(&self, vertex: &[u8; 32]) -> Vec<[u8; 32]> {
        let mut out = Vec::new();
        for c in self.link(vertex, 1) {
            for u in &c.vertices {
                if u != vertex {
                    out.push(*u);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    fn holonomy(&self, i: &[u8; 32], j: &[u8; 32], k: &[u8; 32]) -> Option<Rotor> {
        let g_ij = self.edge(i, j)?;
        let g_jk = self.edge(j, k)?;
        let g_ki = self.edge(k, i)?;
        Some(g_ij.geo(&g_jk).geo(&g_ki))
    }

    fn face_coeff(&self, verts: &[[u8; 32]]) -> Option<Rotor> {
        if let Some(c) = self.get(verts) {
            return Some(c.coefficient.clone());
        }
        match verts.len() {
            2 => self.edge(&verts[0], &verts[1]),
            3 => self.holonomy(&verts[0], &verts[1], &verts[2]),
            n if n >= 4 => self.coboundary_coeff(verts),
            _ => Some(Rotor::identity()),
        }
    }

    /// Coboundary of the (k−1)-faces of a k-cell. Middle terms are
    /// transported by the edge they share.
    fn coboundary_coeff(&self, verts: &[[u8; 32]]) -> Option<Rotor> {
        let n = verts.len();
        if n < 4 {
            return self.face_coeff(verts);
        }
        let mut acc = Rotor::identity();
        for i in 0..n {
            let mut face: Vec<[u8; 32]> = verts.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).collect();
            face = sorted_verts(face);
            let mut c = self.face_coeff(&face)?;
            if i % 2 == 1 {
                if let Some(g) = self.edge(&verts[0], &verts[i]) {
                    c = c.conjugate(&g);
                }
                c = c.inverse()?;
            }
            acc = acc.geo(&c);
        }
        Some(acc)
    }

    /// Climb degrees on the link of one vertex, forming a k-cell only when
    /// the overlap has volume. Stops when a degree produces no cells.
    pub fn coboundary_on_link(&mut self, vertex: &[u8; 32], mut volume_of: impl FnMut(&[[u8; 32]]) -> [u64; 4]) {
        let nbs = self.neighbors(vertex);
        if nbs.is_empty() {
            return;
        }
        for take in 2..=nbs.len() {
            let mut pending: Vec<Cell> = Vec::new();
            each_combination(&nbs, take, |combo| {
                let mut verts = Vec::with_capacity(take + 1);
                verts.push(*vertex);
                verts.extend_from_slice(combo);
                let verts = sorted_verts(verts);
                let mut blocked = false;
                for i in 0..verts.len() {
                    for j in i + 1..verts.len() {
                        if self.is_remainder(&[verts[i], verts[j]]) {
                            blocked = true;
                        }
                    }
                }
                if blocked {
                    return;
                }
                let volume = volume_of(&verts);
                if !nonzero(&volume) {
                    return;
                }
                let coefficient = if verts.len() == 3 {
                    self.holonomy(&verts[0], &verts[1], &verts[2])
                } else {
                    self.coboundary_coeff(&verts)
                };
                let Some(coefficient) = coefficient else {
                    return;
                };
                pending.push(Cell {
                    vertices: verts,
                    coefficient,
                    volume,
                    fill: Fill::Unfilled,
                });
            });
            if pending.is_empty() {
                break;
            }
            for cell in pending {
                self.remember(cell);
            }
        }
    }

    /// Solve the gauge equation on the cell's boundary. Identity coefficient
    /// fills; otherwise the tree-plus-chord solver on the 1-skeleton; a
    /// leftover coefficient is the class at this degree.
    pub fn fill(&mut self, key: &[u8; 32]) -> Fill {
        let Some(degree) = self.degrees.iter().position(|l| l.contains(key)) else {
            return Fill::Unfilled;
        };
        let Some(cell) = self.degrees[degree].get(key).cloned() else {
            return Fill::Unfilled;
        };
        if cell.coefficient.is_identity() {
            let filled = Fill::Filled(Rotor::identity());
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        let verts = &cell.vertices;
        let mut edges: Vec<(usize, usize, Rotor)> = Vec::new();
        for i in 0..verts.len() {
            for j in i + 1..verts.len() {
                if let Some(g) = self.edge(&verts[i], &verts[j]) {
                    edges.push((i, j, g));
                }
            }
        }
        let outcome = match solve_gauge(verts.len(), &edges) {
            Some(gs) => {
                for (v, g) in verts.iter().zip(gs.iter()) {
                    if let Some(sec) = self.sections.get_mut(v) {
                        sec.gamma = g.clone();
                    }
                }
                Fill::Filled(gs.get(1).cloned().unwrap_or_else(Rotor::identity))
            }
            None => Fill::Class(cell.coefficient.clone()),
        };
        if let Some(c) = self.degrees[degree].get_mut(key) {
            c.fill = outcome.clone();
        }
        outcome
    }

    pub fn class(&self, key: &[u8; 32]) -> Option<(Rotor, usize)> {
        for (d, layer) in self.degrees.iter().enumerate() {
            if let Some(c) = layer.get(key) {
                return match &c.fill {
                    Fill::Class(r) => Some((r.clone(), d)),
                    Fill::Unfilled if !c.coefficient.is_identity() && d >= 2 => {
                        Some((c.coefficient.clone(), d))
                    }
                    _ => None,
                };
            }
        }
        None
    }

    pub fn unfilled(&self) -> Vec<(usize, [u8; 32])> {
        let mut out = Vec::new();
        for (d, layer) in self.degrees.iter().enumerate() {
            layer.for_each(|k, c| {
                if matches!(c.fill, Fill::Unfilled) && d >= 1 {
                    out.push((d, *k));
                }
            });
        }
        out
    }

    pub fn cell_count(&self) -> usize {
        self.degrees.iter().map(|l| l.len()).sum()
    }

    pub fn byte_len(&self) -> usize {
        let mut n = 0usize;
        for layer in &self.degrees {
            layer.for_each(|_, c| {
                n += 32 * c.vertices.len() + 32 + 32; // verts + volume + fill tag
            });
        }
        n
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for (k, s) in self.sections.entries() {
            out.extend_from_slice(k);
            out.extend_from_slice(&s.addr);
            s.gamma.to_bytes(out);
        }
        out.extend_from_slice(&(self.degrees.len() as u32).to_le_bytes());
        for layer in &self.degrees {
            out.extend_from_slice(&(layer.len() as u32).to_le_bytes());
            for (_, c) in layer.entries() {
                out.push(c.vertices.len() as u8);
                for v in &c.vertices {
                    out.extend_from_slice(v);
                }
                c.coefficient.to_bytes(out);
                for x in &c.volume {
                    out.extend_from_slice(&x.to_le_bytes());
                }
                match &c.fill {
                    Fill::Unfilled => out.push(0),
                    Fill::Filled(g) => {
                        out.push(1);
                        g.to_bytes(out);
                    }
                    Fill::Class(g) => {
                        out.push(2);
                        g.to_bytes(out);
                    }
                }
            }
        }
        out.extend_from_slice(&(self.remainder.len() as u32).to_le_bytes());
        for (k, _) in self.remainder.entries() {
            out.extend_from_slice(k);
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Complex> {
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
        let mut c = Complex::new();
        let ns = take_u32(b, pos)? as usize;
        for _ in 0..ns.min(1 << 16) {
            let v = take_id(b, pos)?;
            let addr = take_id(b, pos)?;
            let gamma = Rotor::from_bytes(b, pos)?;
            c.sections.insert(v, Section { addr, gamma });
        }
        let nd = take_u32(b, pos)? as usize;
        for _ in 0..nd.min(64) {
            let n = take_u32(b, pos)? as usize;
            for _ in 0..n.min(1 << 16) {
                let nv = *b.get(*pos)? as usize;
                *pos += 1;
                let mut verts = Vec::with_capacity(nv.min(32));
                for _ in 0..nv.min(32) {
                    verts.push(take_id(b, pos)?);
                }
                let coefficient = Rotor::from_bytes(b, pos)?;
                let mut volume = [0u64; 4];
                for x in &mut volume {
                    *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                let tag = *b.get(*pos)?;
                *pos += 1;
                let fill = match tag {
                    1 => Fill::Filled(Rotor::from_bytes(b, pos)?),
                    2 => Fill::Class(Rotor::from_bytes(b, pos)?),
                    _ => Fill::Unfilled,
                };
                c.remember(Cell {
                    vertices: sorted_verts(verts),
                    coefficient,
                    volume,
                    fill,
                });
            }
        }
        let nr = take_u32(b, pos).unwrap_or(0) as usize;
        for _ in 0..nr.min(1 << 16) {
            let k = take_id(b, pos)?;
            c.remainder.insert(k, ());
        }
        Some(c)
    }
}

/// Tree-plus-chord solver for δγ = ω on Rotor coefficients. A forest
/// 1-cochain is exact; a chord that does not match the tree path is a
/// withheld class.
pub fn solve_gauge(n: usize, edges: &[(usize, usize, Rotor)]) -> Option<Vec<Rotor>> {
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
    let mut tree: Vec<(usize, usize, Rotor)> = Vec::new();
    let mut chords: Vec<(usize, usize, Rotor)> = Vec::new();
    for (i, j, g) in edges {
        if *i >= n || *j >= n {
            continue;
        }
        let (a, b) = (find(&mut parent, *i), find(&mut parent, *j));
        if a != b {
            parent[a] = b;
            tree.push((*i, *j, g.clone()));
        } else {
            chords.push((*i, *j, g.clone()));
        }
    }
    let mut gamma = vec![Rotor::identity(); n];
    let mut placed = vec![false; n];
    placed[0] = true;
    let mut progressed = true;
    while progressed {
        progressed = false;
        for (i, j, g) in &tree {
            match (placed[*i], placed[*j]) {
                (true, false) => {
                    gamma[*j] = g.geo(&gamma[*i]);
                    placed[*j] = true;
                    progressed = true;
                }
                (false, true) => {
                    let Some(inv) = g.inverse() else {
                        return None;
                    };
                    gamma[*i] = inv.geo(&gamma[*j]);
                    placed[*i] = true;
                    progressed = true;
                }
                _ => {}
            }
        }
    }
    for (i, j, g) in &chords {
        let Some(inv) = gamma[*i].inverse() else {
            return None;
        };
        let expect = gamma[*j].geo(&inv);
        if expect != *g {
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

    fn vol() -> [u64; 4] {
        [1, 0, 0, 0]
    }

    fn quarter() -> Rotor {
        let id = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
    }

    fn sheet() -> Rotor {
        let id = crate::transducer::alphabet::LetterId::quarter((1, 2)).unwrap();
        crate::transducer::alphabet::Word::from_letters(&[(id, false), (id, false)]).rotor()
    }

    /// A TRANSITION is antisymmetric: `g_XY ∘ g_YX = 1` (`g_YX = g_XY⁻¹`).
    #[test]
    fn a_transition_and_its_reverse_cancel() {
        let (gx, gy) = (motion(3), motion(6));
        let f = transition(&gx, &gy).unwrap();
        let r = transition(&gy, &gx).unwrap();
        assert!(f.compose(&r).is_identity(), "g_XY ∘ g_YX = 1 — asymmetric coboundary");
    }

    #[test]
    fn consilient_charts_glue() {
        let g = motion(3);
        let id = transition(&g, &g).unwrap();
        assert!(id.is_identity());
        assert!(glues(&id, &id, &id), "identical views → one sheaf");
    }

    #[test]
    fn a_global_section_always_glues() {
        let (ga, gb, gc) = (motion(3), motion(6), motion(9));
        let g_ab = transition(&ga, &gb).unwrap();
        let g_bc = transition(&gb, &gc).unwrap();
        let g_ca = transition(&gc, &ga).unwrap();
        assert!(glues(&g_ab, &g_bc, &g_ca), "a coboundary always satisfies δδ = 0");
    }

    #[test]
    fn an_equivocation_shows_up_as_curvature() {
        let g = motion(3);
        let id = transition(&g, &g).unwrap();
        let disagree = transition(&motion(3), &motion(6)).unwrap();
        assert!(!disagree.is_identity(), "the disagreeing edge is nontrivial");
        assert!(!glues(&id, &id, &disagree), "a lone dialect edge → curvature (δσ ≠ 0)");
        assert!(!holonomy(&id, &id, &disagree).is_identity(), "the holonomy is the exact obstruction");
    }

    /// The link of one vertex is only the incident cells; a tetra not on
    /// the link is not formed. Volume-zero overlaps form nothing.
    #[test]
    fn the_link_is_the_star_and_volume_gates_cells() {
        let mut c = Complex::new();
        let (a, b, cc, d) = (v(1), v(2), v(3), v(4));
        for x in [a, b, cc, d] {
            c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
        }
        let id = Rotor::identity();
        c.set_edge(a, b, id.clone(), vol());
        c.set_edge(a, cc, id.clone(), vol());
        c.set_edge(b, cc, id.clone(), vol());
        c.set_edge(cc, d, id, vol());
        c.coboundary_on_link(&a, |_| vol());
        for cell in c.link(&a, 2) {
            assert!(cell.contains(&a), "link(a) contains only a");
        }
        assert!(c.degrees.get(3).map_or(true, |l| l.is_empty()), "no tetra on a's link");
        let mut empty = Complex::new();
        empty.set_section(a, Section { addr: a, gamma: Rotor::identity() }, [0; 4]);
        empty.set_edge(a, b, Rotor::identity(), [0; 4]);
        assert_eq!(empty.cell_count(), 0, "zero volume forms no cell");
    }

    /// A coboundary 1-cochain fills; a leftover holonomy is the class.
    #[test]
    fn fill_is_gauge_or_class() {
        let mut c = Complex::new();
        let (a, b, cc) = (v(1), v(2), v(3));
        for x in [a, b, cc] {
            c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
        }
        let q = quarter();
        c.set_edge(a, b, q.clone(), vol());
        c.set_edge(b, cc, q.clone(), vol());
        c.set_edge(a, cc, q.geo(&q), vol());
        c.coboundary_on_link(&a, |_| vol());
        let tri = c.link(&a, 2);
        assert_eq!(tri.len(), 1);
        let key = tri[0].key();
        assert!(matches!(c.fill(&key), Fill::Filled(_)));
        c.set_edge(a, cc, Rotor::identity(), vol());
        c.coboundary_on_link(&a, |_| vol());
        let key = c.link(&a, 2)[0].key();
        assert!(matches!(c.fill(&key), Fill::Class(_)));
    }

    /// Four vertices with a coboundary 1-cochain have δh = 1; a rotation on
    /// one pair and a sheet on the opposite edge is a class at degree 3.
    #[test]
    fn a_class_at_degree_three_is_the_bianchi_form() {
        let mut c = Complex::new();
        let vs = [v(1), v(2), v(3), v(4)];
        for x in vs {
            c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
        }
        let id = Rotor::identity();
        for i in 0..4 {
            for j in i + 1..4 {
                c.set_edge(vs[i], vs[j], id.clone(), vol());
            }
        }
        c.coboundary_on_link(&vs[0], |_| vol());
        let tets = c.link(&vs[0], 3);
        assert_eq!(tets.len(), 1, "the link of one vertex of K4 is one tetra");
        let key = tets[0].key();
        assert!(c.fill(&key) == Fill::Filled(Rotor::identity()) || c.class(&key).is_none());
        c.set_edge(vs[0], vs[1], quarter(), vol());
        c.set_edge(vs[2], vs[3], sheet(), vol());
        c.coboundary_on_link(&vs[0], |_| vol());
        let key = c.link(&vs[0], 3)[0].key();
        let _ = c.fill(&key);
        assert!(c.class(&key).is_some(), "an inconsistent 3-cell is a class");
        assert!(c.byte_len() < 4096, "four covering vertices stay in kilobytes");
    }
}
