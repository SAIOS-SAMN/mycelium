// 13 — THE COMPLEX: every cell a mind holds, and the walks that read it.
//
//! The star, the link and the degree are reads of structure. Setting an
//! edge or a section reopens exactly the cells whose coefficients changed,
//! so nothing is recomputed that did not move.

use super::*;

/// The Čech complex on one chart. One Keyed trie per degree, grown as
/// overlaps demand. No fixed arity.
#[derive(Clone, Debug, Default)]
pub struct Complex {
    pub degrees: Vec<Keyed<Cell>>,
    /// cell key → degree, written in remember. Fill and class are keyed
    /// reads, never a walk of degrees.
    pub(crate) by_key: Keyed<u8>,
    pub sections: Keyed<Section>,
    pub(crate) star: Keyed<Keyed<()>>,
    pub(crate) remainder: Keyed<()>,
    /// Congruent fills keyed by the rotor's class key (15.4). Court torsion
    /// reads this; remember and fill write it.
    pub(crate) congruent: Keyed<(Rotor, u32)>,
}

impl Complex {
    pub fn new() -> Complex {
        Complex::default()
    }

    pub(crate) fn ensure(&mut self, degree: usize) {
        while self.degrees.len() <= degree {
            self.degrees.push(Keyed::new());
        }
    }

    pub(crate) fn remember(&mut self, mut cell: Cell) {
        cell.coefficient = cell.coefficient.normalised();
        let degree = cell.degree();
        let key = cell.key();
        self.ensure(degree);
        self.by_key.insert(&key, degree as u8);
        // index: vertices of this cell (the simplex, bounded by degree)
        for v in &cell.vertices {
            match self.star.get_mut(v) {
                Some(keys) => {
                    keys.insert(&key, ());
                }
                None => {
                    let mut keys = Keyed::new();
                    keys.insert(&key, ());
                    self.star.insert(v.clone(), keys);
                }
            }
        }
        self.index_congruent(&cell.fill);
        self.degrees[degree].insert(&key, cell);
    }

    pub(crate) fn index_congruent(&mut self, fill: &Fill) {
        if let Fill::Congruent(g, ell) = fill {
            if let Some(ck) = g.class_key() {
                self.congruent.insert(ck, (g.clone(), *ell));
            }
        }
    }

    /// The Congruent cell whose rotor has this class key, if the chart holds one.
    pub fn congruent(&self, class: &[u8]) -> Option<&(Rotor, u32)> {
        self.congruent.get(class)
    }

    pub(crate) fn store_coeff(g: Rotor) -> Rotor {
        g.normalised()
    }

    pub fn set_section(&mut self, vertex: impl AsRef<[u8]>, section: Section, volume: [u64; 4]) {
        let vertex = vertex.as_ref().to_vec();
        self.sections.insert(&vertex, section);
        if nonzero(&volume) {
            self.remember(Cell {
                vertices: vec![vertex],
                coefficient: Rotor::identity(),
                carriage: crate::transducer::element::identity_versor(),
                volume,
                fill: Fill::Filled(
                    Rotor::identity(),
                    Box::new(crate::transducer::element::identity_versor()),
                ),
            });
        }
    }

    /// Degree-1 cell: the transition rotor, or a remainder if it does not
    /// realise. Remainder edges form no higher cells.
    pub fn set_edge(
        &mut self,
        a: impl AsRef<[u8]>,
        b: impl AsRef<[u8]>,
        g: Rotor,
        carriage: Versor,
        volume: [u64; 4],
    ) {
        let a = a.as_ref();
        let b = b.as_ref();
        let vertices = sorted_verts(vec![a, b]);
        if vertices.len() != 2 {
            return;
        }
        let key = cell_key(&vertices);
        self.remainder.remove(&key);
        // Both halves are oriented together: an edge stored against its
        // given direction inverts the rotor AND the carriage, or it is not
        // stored at all.
        let (stored_r, stored_c) = if vertices[0].as_slice() == a {
            (g, carriage)
        } else {
            let (Some(gi), Some(ci)) = (g.inverse(), carriage.versor_inverse()) else {
                return;
            };
            (gi, ci)
        };
        self.remember(Cell {
            vertices,
            coefficient: Self::store_coeff(stored_r),
            carriage: stored_c,
            volume,
            fill: Fill::Open,
        });
    }

    /// File a finite-order congruence on an edge: the torsion axis of a merge.
    pub fn file_congruent(&mut self, a: impl AsRef<[u8]>, b: impl AsRef<[u8]>, g: Rotor, ell: u32) {
        let vertices = sorted_verts([a.as_ref(), b.as_ref()]);
        if vertices.len() != 2 {
            return;
        }
        self.remember(Cell {
            vertices,
            coefficient: g.clone(),
            // A filed congruence is residue-free by construction: it is a
            // finite-order rotation about a point, which `classify` returns
            // only where both residues are the identity.
            carriage: crate::transducer::element::identity_versor(),
            volume: [1, 0, 0, 0],
            fill: Fill::Congruent(g, ell),
        });
    }

    pub fn set_remainder_edge(&mut self, a: &[u8], b: &[u8]) {
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

    pub(crate) fn is_remainder(&self, verts: &[Vec<u8>]) -> bool {
        let v = sorted_verts(verts.to_vec());
        self.remainder.contains(&cell_key(&v))
    }

    pub fn get(&self, verts: &[Vec<u8>]) -> Option<&Cell> {
        // No vertices name no cell.
        let d = verts.len().checked_sub(1)?;
        self.degrees.get(d)?.get(&cell_key(verts))
    }

    /// Oriented edge rotor from a to b.
    pub fn edge(&self, a: &[u8], b: &[u8]) -> Option<(Rotor, Versor)> {
        let vertices = sorted_verts([a, b]);
        let cell = self.get(&vertices)?;
        if vertices[0] == *a {
            Some((cell.coefficient.clone(), cell.carriage.clone()))
        } else {
            Some((cell.coefficient.inverse()?, cell.carriage.versor_inverse()?))
        }
    }

    /// Cell at `key`, read in place. Never a clone of the rotor.
    pub fn cell(&self, key: &[u8]) -> Option<&Cell> {
        let d = *self.by_key.get(key)?;
        self.degrees.get(d as usize)?.get(key)
    }

    /// A key in `by_key` names a stored cell in the layer of its degree.
    /// `None` when it names none: a key with no cell is not reopened, and
    /// was two panics and an assertion (§2.4).
    pub(crate) fn stored_mut(&mut self, key: &[u8]) -> Option<&mut Cell> {
        let d = *self.by_key.get(key)? as usize;
        self.degrees.get_mut(d)?.get_mut(key)
    }

    /// Reopen a stored cell only when its coboundary coefficient moved.
    /// Absence of a rotor is remainder: the fill stands.
    pub(crate) fn reopen_on_coeff_change(&mut self, key: &[u8], verts: &[Vec<u8>]) {
        let Some((coefficient, carriage)) = self.coboundary_coeff(verts) else {
            return;
        };
        let coefficient = Self::store_coeff(coefficient);
        let Some(c) = self.stored_mut(key) else {
            return;
        };
        // EITHER half moving reopens the cell. A cell whose rotor is
        // unchanged and whose placement moved is a different transition,
        // and leaving it filled would keep a gauge that no longer closes it.
        let same = c.coefficient.projective_eq(&coefficient)
            && crate::transducer::element::same_versor(&c.carriage, &carriage);
        if !same {
            c.coefficient = coefficient;
            c.carriage = carriage;
            c.fill = Fill::Open;
        }
    }

    /// The link of `vertex`: every incident cell, all degrees. A peel of
    /// the star; work per glue never leaves this set.
    pub fn star(&self, vertex: &[u8]) -> Vec<Vec<u8>> {
        self.link_of(vertex)
    }

    /// Keys of every cell incident on `vertex`. The climb reads cells by
    /// these keys and never clones a rotor.
    pub fn link_of(&self, vertex: &[u8]) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        // index: keyed fiber of star(vertex)
        for (k, _) in self.star.link(vertex) {
            if self.by_key.contains(&k) {
                out.push(k);
            }
        }
        out
    }

    pub(crate) fn degree_of(&self, key: &[u8]) -> Option<usize> {
        self.by_key.get(key).map(|&d| d as usize)
    }

    /// Keys of one degree that contain `vertex` — a peel of the star.
    pub fn link(&self, vertex: &[u8], degree: usize) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let Some(layer) = self.degrees.get(degree) else {
            return out;
        };
        for (k, _) in self.star.link(vertex) {
            if layer.contains(&k) {
                out.push(k);
            }
        }
        out
    }

    /// The holonomy around a triangle, on both halves. A rotor-only
    /// composite reads the identity for a triangle whose placements do not
    /// close, which is the obstruction the reading exists to find.
    pub(crate) fn holonomy(&self, i: &[u8], j: &[u8], k: &[u8]) -> Option<(Rotor, Versor)> {
        let (r_ij, c_ij) = self.edge(i, j)?;
        let (r_jk, c_jk) = self.edge(j, k)?;
        let (r_ki, c_ki) = self.edge(k, i)?;
        Some((r_ij.geo(&r_jk).geo(&r_ki), c_ij.geo(&c_jk).geo(&c_ki)))
    }

    /// Drop every cell that contains `vertex`. Used when bubbles merge.
    pub fn drop_vertex(&mut self, vertex: &[u8]) {
        let Some(keys) = self.star.remove(vertex) else {
            self.sections.remove(vertex);
            return;
        };
        let mut drop: Vec<Vec<u8>> = Vec::new();
        // index: keyed fiber of star(vertex)
        for (k, _) in keys.fiber(ROOT) {
            drop.push(k);
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

    pub fn cell_count(&self) -> usize {
        self.by_key.len()
    }

}
