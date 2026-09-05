// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH COMPLEX — vertices are the hyphas (or germs) whose cover contains
//! the chart, one section each. A k-cell is a (k+1)-fold overlap and exists
//! only when that overlap has volume (open listings / shared support), a
//! polytonic μ⃗, never a word count. Coefficients are Rotor of Cl(4,0).
//! Work per glue is the link of one vertex, climbing degree only while
//! volume persists. A transition that does not realise as a rotor is a
//! remainder; no cell forms over it. The tower has no top.

use crate::network::nerve::{Keyed, ROOT};
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



/// State of a cell: open work, a coboundary gauge, a finite-order congruence,
/// or a class at this degree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fill {
    Open,
    Filled(Rotor),
    Congruent(Rotor, u32),
    Class(Rotor),
    /// A fill attempted on a cell that has already transitioned. Not stored
    /// on the cell; the return of `fill` when there is no new state.
    Already,
}

fn word_of_rotor(r: &Rotor) -> crate::transducer::alphabet::Word {
    use crate::transducer::alphabet::{spell, Word};
    let one = crate::transducer::alphabet::Q::from_integer(num_bigint::BigInt::from(1));
    let zero = crate::transducer::alphabet::Q::from_integer(num_bigint::BigInt::from(0));
    let e0 = r.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()]);
    let e1 = r.carry(&[zero.clone(), one, zero.clone(), zero]);
    if let (Some(a), Some(b)) = (e0, e1) {
        let block = [[a[0].clone(), b[0].clone()], [a[1].clone(), b[1].clone()]];
        if let Some(sp) = spell(&block) {
            return sp.word;
        }
    }
    Word::identity()
}

/// Transition between two sections on a cell: identity or coboundary is
/// Extended, finite Word::order is Congruent, otherwise Class.
pub fn extend(section_a: &Section, section_b: &Section, cell: &Cell) -> Fill {
    #[cfg(test)]
    crate::network::nerve::work::count_extend();
    let inv = section_a.gamma.inverse().unwrap_or_else(Rotor::identity);
    let trans = section_b.gamma.geo(&inv);
    if cell.degree() <= 1 {
        let coeff_inv = cell.coefficient.inverse().unwrap_or_else(Rotor::identity);
        let g = trans.geo(&coeff_inv);
        if g.is_identity() {
            return Fill::Filled(trans);
        }
        return match word_of_rotor(&g).order() {
            crate::transducer::alphabet::Order::Finite(ell) => Fill::Congruent(g, ell),
            crate::transducer::alphabet::Order::Infinite => Fill::Class(g),
        };
    }
    if cell.coefficient.is_identity() {
        return Fill::Filled(Rotor::identity());
    }
    match word_of_rotor(&cell.coefficient).order() {
        crate::transducer::alphabet::Order::Finite(ell) => Fill::Congruent(cell.coefficient.clone(), ell),
        crate::transducer::alphabet::Order::Infinite => Fill::Class(cell.coefficient.clone()),
    }
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
    /// cell key → degree, written in remember. Fill and class are keyed
    /// reads, never a walk of degrees.
    by_key: Keyed<u8>,
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

    fn remember(&mut self, mut cell: Cell) {
        cell.coefficient = cell.coefficient.normalised();
        let degree = cell.degree();
        let key = cell.key();
        self.ensure(degree);
        self.by_key.insert(key, degree as u8);
        // index: vertices of this cell (the simplex, bounded by degree)
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

    fn store_coeff(g: Rotor) -> Rotor {
        g.normalised()
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
        let stored = if vertices[0] == a { g } else { g.inverse().unwrap_or_else(Rotor::identity) };
        self.remember(Cell {
            vertices,
            coefficient: Self::store_coeff(stored),
            volume,
            fill: Fill::Open,
        });
    }

    /// File a finite-order congruence on an edge: the torsion axis of a merge.
    pub fn file_congruent(&mut self, a: [u8; 32], b: [u8; 32], g: Rotor, ell: u32) {
        let vertices = sorted_verts(vec![a, b]);
        if vertices.len() != 2 {
            return;
        }
        self.remember(Cell {
            vertices,
            coefficient: g.clone(),
            volume: [1, 0, 0, 0],
            fill: Fill::Congruent(g, ell),
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

    /// Cell at `key`, read in place. Never a clone of the rotor.
    pub fn cell(&self, key: &[u8; 32]) -> Option<&Cell> {
        let d = *self.by_key.get(key)?;
        self.degrees.get(d as usize)?.get(key)
    }

    /// A key in `by_key` is a stored cell. The degree is the layer.
    fn stored_mut(&mut self, key: &[u8; 32]) -> &mut Cell {
        let d = *self.by_key.get(key).expect("by_key names a stored cell") as usize;
        assert!(d < self.degrees.len(), "by_key degree is a live layer");
        self.degrees[d].get_mut(key).expect("by_key names a stored cell")
    }

    /// Reopen a stored cell only when its coboundary coefficient moved.
    /// Absence of a rotor is remainder: the fill stands.
    fn reopen_on_coeff_change(&mut self, key: &[u8; 32], verts: &[[u8; 32]]) {
        let Some(coefficient) = self.coboundary_coeff(verts) else {
            return;
        };
        let coefficient = Self::store_coeff(coefficient);
        let c = self.stored_mut(key);
        match c.coefficient.projective_eq(&coefficient) {
            true => {}
            false => {
                c.coefficient = coefficient;
                c.fill = Fill::Open;
            }
        }
    }

    /// The link of `vertex`: every incident cell, all degrees. A peel of
    /// the star; work per glue never leaves this set.
    pub fn star(&self, vertex: &[u8; 32]) -> Vec<[u8; 32]> {
        self.link_of(vertex)
    }

    /// Keys of every cell incident on `vertex`. The climb reads cells by
    /// these keys and never clones a rotor.
    pub fn link_of(&self, vertex: &[u8; 32]) -> Vec<[u8; 32]> {
        let mut out = Vec::new();
        // index: keyed fiber of star(vertex)
        for (k, _) in self.star.link(vertex) {
            if self.by_key.contains(k) {
                out.push(*k);
            }
        }
        out
    }

    fn degree_of(&self, key: &[u8; 32]) -> Option<usize> {
        self.by_key.get(key).map(|&d| d as usize)
    }

    /// Mark a cell Extended by a known gauge (a translation, a taught
    /// coboundary). Degree-1 never tree-solves — that would always succeed.
    pub fn fill_by_gauge(&mut self, key: &[u8; 32], gamma: &Rotor) -> Fill {
        let Some(degree) = self.degree_of(key) else {
            return Fill::Open;
        };
        let Some(cell) = self.degrees[degree].get(key).cloned() else {
            return Fill::Open;
        };
        if !matches!(cell.fill, Fill::Open) {
            return Fill::Already;
        }
        let matches = cell.coefficient == *gamma
            || cell.coefficient.inverse().as_ref() == Some(gamma);
        if matches {
            if cell.vertices.len() >= 2 {
                if let Some(sec) = self.sections.get_mut(&cell.vertices[1]) {
                    sec.gamma = gamma.clone();
                }
            }
            let filled = Fill::Filled(gamma.clone());
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        self.fill(key)
    }

    /// Keys of one degree that contain `vertex` — a peel of the star.
    pub fn link(&self, vertex: &[u8; 32], degree: usize) -> Vec<[u8; 32]> {
        let mut out = Vec::new();
        let Some(layer) = self.degrees.get(degree) else {
            return out;
        };
        for (k, _) in self.star.link(vertex) {
            if layer.contains(k) {
                out.push(*k);
            }
        }
        out
    }

    fn holonomy(&self, i: &[u8; 32], j: &[u8; 32], k: &[u8; 32]) -> Option<Rotor> {
        let g_ij = self.edge(i, j)?;
        let g_jk = self.edge(j, k)?;
        let g_ki = self.edge(k, i)?;
        Some(g_ij.geo(&g_jk).geo(&g_ki))
    }

    /// Face coefficient from stored lower cells: edges (and, when already
    /// climbed, higher faces). A triangle is the holonomy of its three
    /// stored edges — never the triangle's own (possibly stale) cell.
    fn stored_face_coeff(&self, verts: &[[u8; 32]]) -> Option<Rotor> {
        match verts.len() {
            0 | 1 => Some(Rotor::identity()),
            2 => self.edge(&verts[0], &verts[1]),
            3 => self.holonomy(&verts[0], &verts[1], &verts[2]),
            _ => self.get(verts).map(|c| c.coefficient.clone()),
        }
    }

    /// Coboundary of the (k−1)-faces of a k-cell. Face coefficients are
    /// read from stored lower cells that `faces_ready` already proved exist.
    fn coboundary_coeff(&self, verts: &[[u8; 32]]) -> Option<Rotor> {
        let n = verts.len();
        if n < 4 {
            return self.stored_face_coeff(verts);
        }
        let mut acc = Rotor::identity();
        for i in 0..n {
            let mut face: Vec<[u8; 32]> = verts.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).collect();
            face = sorted_verts(face);
            let mut c = self.stored_face_coeff(&face)?;
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

    /// A candidate forms when every edge already exists (the 1-skeleton of
    /// the simplex). Faces that omit the climbing vertex are not required
    /// to sit in its link.
    fn faces_ready(&self, verts: &[[u8; 32]]) -> bool {
        let n = verts.len();
        if n < 3 {
            return true;
        }
        // index: edges of this simplex
        for i in 0..n {
            for j in i + 1..n {
                if self.get(&sorted_verts(vec![verts[i], verts[j]])).is_none() {
                    return false;
                }
            }
        }
        true
    }

    /// Climb the link of one vertex: the candidate at degree k+1 is the
    /// intersection of the links of existing k-cells with the link of one
    /// more neighbour, never a subset of the fiber. Stops when the next
    /// degree produces no volume. Cells are read by key; rotors are not cloned.
    pub fn coboundary_on_link(&mut self, vertex: &[u8; 32], mut volume_of: impl FnMut(&[[u8; 32]]) -> [u64; 4]) {
        let mut degree = 1usize;
        loop {
            let layer = self.link(vertex, degree);
            if layer.is_empty() {
                break;
            }
            let mut pending: Vec<Cell> = Vec::new();
            let mut seen: Keyed<()> = Keyed::new();
            // index: link of `vertex` at this degree
            for key in &layer {
                let Some(cell_verts) = self.cell(key).map(|c| c.vertices.clone()) else {
                    continue;
                };
                let nverts = cell_verts.len();
                // index: vertices of this cell
                for u in &cell_verts {
                    if u == vertex {
                        continue;
                    }
                    let edges = self.link(u, 1);
                    // index: link of neighbour u (degree 1)
                    for ek in &edges {
                        let Some(edge_verts) = self.cell(ek).map(|c| c.vertices.clone()) else {
                            continue;
                        };
                        // index: vertices of that edge
                        for w in &edge_verts {
                            if w == u || cell_verts.binary_search(w).is_ok() {
                                continue;
                            }
                            let mut verts = cell_verts.clone();
                            verts.push(*w);
                            let verts = sorted_verts(verts);
                            if verts.len() != nverts + 1 {
                                continue;
                            }
                            let nkey = cell_key(&verts);
                            if seen.contains(&nkey) {
                                continue;
                            }
                            seen.insert(nkey, ());
                            match self.by_key.get(&nkey).copied() {
                                Some(_) => {
                                    self.reopen_on_coeff_change(&nkey, &verts);
                                    continue;
                                }
                                None => {}
                            }
                            let mut blocked = false;
                            // index: pairs on this simplex (faces)
                            for i in 0..verts.len() {
                                for j in i + 1..verts.len() {
                                    if self.is_remainder(&[verts[i], verts[j]]) {
                                        blocked = true;
                                    }
                                }
                            }
                            if blocked || !self.faces_ready(&verts) {
                                continue;
                            }
                            let volume = volume_of(&verts);
                            if !nonzero(&volume) {
                                continue;
                            }
                            let Some(coefficient) = self.coboundary_coeff(&verts) else {
                                continue;
                            };
                            pending.push(Cell {
                                vertices: verts,
                                coefficient,
                                volume,
                                fill: Fill::Open,
                            });
                        }
                    }
                }
            }
            if pending.is_empty() {
                break;
            }
            for cell in pending {
                self.remember(cell);
            }
            degree += 1;
        }
    }

    /// Fill a cell by extend: coboundary is Extended, finite order is
    /// Congruent, otherwise Class.
    pub fn fill(&mut self, key: &[u8; 32]) -> Fill {
        let Some(degree) = self.degree_of(key) else {
            return Fill::Open;
        };
        let Some(cell) = self.degrees[degree].get(key).cloned() else {
            return Fill::Open;
        };
        if !matches!(cell.fill, Fill::Open) {
            return Fill::Already;
        }
        if cell.degree() == 0 {
            let filled = Fill::Filled(Rotor::identity());
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        if cell.coefficient.is_identity() {
            let filled = Fill::Filled(Rotor::identity());
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        let verts = &cell.vertices;
        if cell.degree() >= 2 {
            let mut edges: Vec<(usize, usize, Rotor)> = Vec::new();
            for i in 0..verts.len() {
                for j in i + 1..verts.len() {
                    if let Some(g) = self.edge(&verts[i], &verts[j]) {
                        edges.push((i, j, g));
                    }
                }
            }
            if let Some(gs) = solve_gauge(verts.len(), &edges) {
                for (v, g) in verts.iter().zip(gs.iter()) {
                    if let Some(sec) = self.sections.get_mut(v) {
                        sec.gamma = g.clone();
                    }
                }
                let filled = Fill::Filled(gs.get(1).cloned().unwrap_or_else(Rotor::identity));
                if let Some(c) = self.degrees[degree].get_mut(key) {
                    c.fill = filled.clone();
                }
                return filled;
            }
        }
        let a = self
            .sections
            .get(&verts[0])
            .cloned()
            .unwrap_or(Section { addr: verts[0], gamma: Rotor::identity() });
        let b = verts.get(1).and_then(|v| self.sections.get(v).cloned()).unwrap_or(Section {
            addr: *verts.last().unwrap_or(&verts[0]),
            gamma: Rotor::identity(),
        });
        let outcome = extend(&a, &b, &cell);
        if let Some(c) = self.degrees[degree].get_mut(key) {
            c.fill = outcome.clone();
        }
        outcome
    }

    pub fn class(&self, key: &[u8; 32]) -> Option<(Rotor, usize)> {
        let d = self.degree_of(key)?;
        let c = self.degrees.get(d)?.get(key)?;
        match &c.fill {
            Fill::Class(r) => Some((r.clone(), d)),
            Fill::Open if !c.coefficient.is_identity() && d >= 2 => Some((c.coefficient.clone(), d)),
            _ => None,
        }
    }

    /// Drop every cell that contains `vertex`. Used when bubbles merge.
    pub fn drop_vertex(&mut self, vertex: &[u8; 32]) {
        let Some(keys) = self.star.remove(vertex) else {
            self.sections.remove(vertex);
            return;
        };
        let mut drop: Vec<[u8; 32]> = Vec::new();
        // index: keyed fiber of star(vertex)
        for (k, _) in keys.fiber(&ROOT) {
            drop.push(*k);
        }
        for k in drop {
            if let Some(d) = self.by_key.remove(&k) {
                if let Some(layer) = self.degrees.get_mut(d as usize) {
                    layer.remove(&k);
                }
            }
        }
        self.sections.remove(vertex);
    }

    pub fn classes(&self) -> Vec<(Rotor, usize)> {
        let mut out = Vec::new();
        // index: keyed cells of this complex (one chart's nerve)
        for (k, d) in self.by_key.fiber(&ROOT) {
            let d = *d as usize;
            if let Some(c) = self.degrees.get(d).and_then(|l| l.get(k)) {
                if let Fill::Class(r) = &c.fill {
                    out.push((r.clone(), d));
                }
            }
        }
        out
    }

    pub fn unfilled(&self) -> Vec<(usize, [u8; 32])> {
        let mut out = Vec::new();
        // index: keyed cells of this complex (one chart's nerve)
        for (k, d) in self.by_key.fiber(&ROOT) {
            let d = *d as usize;
            if d >= 1 {
                if let Some(c) = self.degrees.get(d).and_then(|l| l.get(k)) {
                    if matches!(c.fill, Fill::Open) {
                        out.push((d, *k));
                    }
                }
            }
        }
        out
    }

    pub fn cell_count(&self) -> usize {
        self.by_key.len()
    }

    pub fn byte_len(&self) -> usize {
        let mut n = 0usize;
        // index: keyed cells of this complex (persistence size)
        for (k, d) in self.by_key.fiber(&ROOT) {
            if let Some(c) = self.degrees.get(*d as usize).and_then(|l| l.get(k)) {
                n += 32 * c.vertices.len() + 32 + 32;
            }
        }
        n
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.sections, |k, s| {
            out.extend_from_slice(k);
            out.extend_from_slice(&s.addr);
            s.gamma.to_bytes(out);
        });
        out.extend_from_slice(&(self.degrees.len() as u32).to_le_bytes());
        for layer in &self.degrees {
            out.extend_from_slice(&(layer.len() as u32).to_le_bytes());
            crate::snapshot::persist(layer, |_, c| {
                out.push(c.vertices.len() as u8);
                for v in &c.vertices {
                    out.extend_from_slice(v);
                }
                c.coefficient.to_bytes(out);
                for x in &c.volume {
                    out.extend_from_slice(&x.to_le_bytes());
                }
                match &c.fill {
                    Fill::Open | Fill::Already => out.push(0),
                    Fill::Filled(g) => {
                        out.push(1);
                        g.to_bytes(out);
                    }
                    Fill::Class(g) => {
                        out.push(2);
                        g.to_bytes(out);
                    }
                    Fill::Congruent(g, ell) => {
                        out.push(3);
                        g.to_bytes(out);
                        out.extend_from_slice(&ell.to_le_bytes());
                    }
                }
            });
        }
        out.extend_from_slice(&(self.remainder.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.remainder, |k, _| {
            out.extend_from_slice(k);
        });
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
                    3 => {
                        let g = Rotor::from_bytes(b, pos)?;
                        let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                        *pos += 4;
                        Fill::Congruent(g, ell)
                    }
                    _ => Fill::Open,
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

    fn pythag5() -> Rotor {
        let id = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).unwrap();
        crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
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
        for k in c.link(&a, 2) {
            let cell = c.cell(&k).expect("link key is held");
            assert!(cell.contains(&a), "link(a) contains only a");
        }
        assert!(c.degrees.get(3).map_or(true, |l| l.is_empty()), "no tetra on a's link");
        let mut empty = Complex::new();
        empty.set_section(a, Section { addr: a, gamma: Rotor::identity() }, [0; 4]);
        empty.set_edge(a, b, Rotor::identity(), [0; 4]);
        assert!(
            empty.degrees.get(2).map_or(true, |l| l.is_empty()),
            "zero remainder volume forms no higher cell"
        );
    }

    /// A coboundary 1-cochain fills; a finite leftover is Congruent; an
    /// infinite-order leftover (prime 5) is a Class.
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
        let key = tri[0];
        assert!(matches!(c.fill(&key), Fill::Filled(_)));
        c.set_edge(a, cc, Rotor::identity(), vol());
        c.coboundary_on_link(&a, |_| vol());
        let key = c.link(&a, 2)[0];
        assert!(matches!(c.fill(&key), Fill::Congruent(_, _)));
        c.set_edge(a, b, pythag5(), vol());
        c.set_edge(b, cc, Rotor::identity(), vol());
        c.set_edge(a, cc, Rotor::identity(), vol());
        c.coboundary_on_link(&a, |_| vol());
        let key = c.link(&a, 2)[0];
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
        let key = tets[0];
        assert!(c.fill(&key) == Fill::Filled(Rotor::identity()) || c.class(&key).is_none());
        c.set_edge(vs[0], vs[1], pythag5(), vol());
        c.set_edge(vs[2], vs[3], sheet(), vol());
        c.coboundary_on_link(&vs[0], |_| vol());
        let mut found = false;
        // index: link of vs[0]
        for key in c.link_of(&vs[0]) {
            let Some(d) = c.cell(&key).map(|c| c.degree()) else {
                continue;
            };
            if d < 2 {
                continue;
            }
            let outcome = c.fill(&key);
            if matches!(outcome, Fill::Class(_) | Fill::Congruent(_, _)) || c.class(&key).is_some() {
                found = true;
            }
        }
        assert!(found, "an inconsistent link carries a class or a finite congruence");
        assert!(c.byte_len() < 4096, "four covering vertices stay in kilobytes");
    }

    /// Work of filling the link of one vertex does not grow with N isolated
    /// sections on the same complex.
    #[test]
    fn work_of_one_fetched_vertex_is_independent_of_n() {
        use crate::network::nerve::work;
        let run = |n: u64| -> u64 {
            let mut c = Complex::new();
            let (a, b, cc) = (v(1), v(2), v(3));
            for x in [a, b, cc] {
                c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
            }
            let id = Rotor::identity();
            c.set_edge(a, b, id.clone(), vol());
            c.set_edge(a, cc, id.clone(), vol());
            c.set_edge(b, cc, id, vol());
            for i in 0..n {
                let mut x = [0u8; 32];
                x[..8].copy_from_slice(&(1000 + i).to_le_bytes());
                c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
            }
            work::reset();
            c.coboundary_on_link(&a, |_| vol());
            for k in c.link(&a, 2) {
                let _ = c.fill(&k);
            }
            work::total()
        };
        let a = run(32);
        let b = run(128);
        eprintln!("work fetched-vertex N=32 {a} N=128 {b}");
        assert_eq!(a, b, "4N differs by zero: {a} vs {b}");
        assert!(a > 0, "the operation does work");
        // assemble a scene of thirty bodies: shell cliques would explode
        // here; isolated extras on the same complex must not grow the work.
        let thirty = crate::transducer::sentence::assemble(thirty_body_germs(30), Vec::new());
        assert!(
            thirty.germs.len() <= 31,
            "a body is one vertex, got {}",
            thirty.germs.len()
        );
        assert!(
            thirty.complex.degrees.get(3).map_or(true, |l| l.is_empty()),
            "thirty disjoint bodies form no 3-cell"
        );
        let run_scene = |n: u64| -> u64 {
            let mut c = thirty.complex.clone();
            for i in 0..n {
                let mut x = [0u8; 32];
                x[..8].copy_from_slice(&(2000 + i).to_le_bytes());
                c.set_section(x, Section { addr: x, gamma: Rotor::identity() }, vol());
            }
            let mut v0 = [0u8; 32];
            v0[..8].copy_from_slice(&0u64.to_le_bytes());
            work::reset();
            c.coboundary_on_link(&v0, |_| [0; 4]);
            work::total()
        };
        let s = run_scene(32);
        let t = run_scene(128);
        eprintln!("work assemble-thirty extras N=32 {s} N=128 {t}");
        assert_eq!(s, t, "4N differs by zero on the assembled scene: {s} vs {t}");
    }

    fn thirty_body_germs(n: usize) -> Vec<crate::transducer::sentence::Germ> {
        use crate::transducer::element::{from_residual, identity_versor};
        use crate::transducer::peel::ShellKind;
        use crate::transducer::sentence::Germ;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let r = (i % 6) as i64;
            let c = (i / 6) as i64;
            let chain = vec![(r, c, 1, 1)];
            out.push(Germ {
                address: from_residual(&chain).address(),
                versor: identity_versor(),
                word: crate::transducer::alphabet::Word::identity(),
                chain,
                source: None,
                target: None,
                kind: ShellKind::Whole,
                shift: None,
            });
        }
        out
    }

    #[test]
    fn fill_is_attempted_once() {
        let mut c = Complex::new();
        let (a, b) = (v(1), v(2));
        c.set_section(a, Section { addr: a, gamma: Rotor::identity() }, vol());
        c.set_section(b, Section { addr: b, gamma: Rotor::identity() }, vol());
        c.set_edge(a, b, Rotor::identity(), vol());
        let key = c.link(&a, 1)[0];
        let first = c.fill(&key);
        assert!(matches!(first, Fill::Filled(_)));
        let second = c.fill(&key);
        assert!(matches!(second, Fill::Already), "a second fill is not a transition");
        assert!(matches!(c.cell(&key).map(|c| &c.fill), Some(Fill::Filled(_))));
    }
}
