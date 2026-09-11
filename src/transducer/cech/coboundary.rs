// 13, 14 — THE COBOUNDARY: which cells are ready to close, what closes
// them, and the class that remains when nothing does.
//
//! `coboundary_on_link` walks the link of a vertex and closes what its
//! faces already determine; a cell whose faces are not ready is left open
//! rather than guessed at. `solve_gauge` is the tree walk of 12.2.

use super::*;

/// Tree-plus-chord solver for δγ = ω on a transition's BOTH halves — the
/// rotor in `Cl(4,0)` and the carriage in `Cl(3,1)` (L11e). A forest
/// 1-cochain is exact; a chord that does not match the tree path is a
/// withheld class.
///
/// THIS IS THE GAUGE THAT LETS A TURN ABOUT A POINT FILE. A rotation about
/// `p` is `T_p R T_p⁻¹ = T(p − Rp)·R`, which `classify` reads as a standing
/// class because it carries a placement residue. Once the tree has placed
/// the sections, the base-point translation is conjugated away and the same
/// transition classifies as `Congruent` at `R`'s order. Ungauged it stands,
/// which is the honest reading of a transition with no frame and not a
/// missed fill. Solving the rotor half alone could not do this: the
/// placement is the half that has to move.
pub fn solve_gauge(
    n: usize,
    edges: &[(usize, usize, Rotor, Versor)],
) -> Option<Vec<(Rotor, Versor)>> {
    use crate::transducer::element::{identity_versor, same_versor};
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
    let mut tree: Vec<(usize, usize, Rotor, Versor)> = Vec::new();
    let mut chords: Vec<(usize, usize, Rotor, Versor)> = Vec::new();
    for (i, j, g, c) in edges {
        if *i >= n || *j >= n {
            continue;
        }
        let (a, b) = (find(&mut parent, *i), find(&mut parent, *j));
        if a != b {
            parent[a] = b;
            tree.push((*i, *j, g.clone(), c.clone()));
        } else {
            chords.push((*i, *j, g.clone(), c.clone()));
        }
    }
    let mut gamma = vec![Rotor::identity(); n];
    let mut carriage = vec![identity_versor(); n];
    let mut placed = vec![false; n];
    placed[0] = true;
    let mut progressed = true;
    while progressed {
        progressed = false;
        for (i, j, g, c) in &tree {
            match (placed[*i], placed[*j]) {
                (true, false) => {
                    gamma[*j] = g.geo(&gamma[*i]);
                    carriage[*j] = c.geo(&carriage[*i]);
                    placed[*j] = true;
                    progressed = true;
                }
                (false, true) => {
                    let (gi, ci) = (g.inverse()?, c.versor_inverse()?);
                    gamma[*i] = gi.geo(&gamma[*j]);
                    carriage[*i] = ci.geo(&carriage[*j]);
                    placed[*i] = true;
                    progressed = true;
                }
                _ => {}
            }
        }
    }
    for (i, j, g, c) in &chords {
        let (gi, ci) = (gamma[*i].inverse()?, carriage[*i].versor_inverse()?);
        // A chord must agree on BOTH halves. Agreeing on the rotor alone
        // would call a tree exact when its placements disagree, which is
        // the class the gauge exists to withhold.
        if gamma[*j].geo(&gi) != *g || !same_versor(&carriage[*j].geo(&ci), c) {
            return None;
        }
    }
    Some(gamma.into_iter().zip(carriage).collect())
}

impl Complex {
    /// Mark a cell Extended by a known gauge (a translation, a taught
    /// coboundary). Degree-1 never tree-solves — that would always succeed.
    pub fn fill_by_gauge(&mut self, key: &[u8], gamma: &Rotor, carriage: &Versor) -> Fill {
        use crate::transducer::element::same_versor;
        let Some(degree) = self.degree_of(key) else {
            return Fill::Open;
        };
        let Some(cell) = self.degrees[degree].get(key).cloned() else {
            return Fill::Open;
        };
        if !matches!(cell.fill, Fill::Open) {
            return Fill::Already;
        }
        // The gauge must match on BOTH halves, forward or reversed. A
        // rotor-only match called a cell closed by a gauge whose placement
        // differed from the one the cell expected.
        let rotor_hit =
            cell.coefficient == *gamma || cell.coefficient.inverse().as_ref() == Some(gamma);
        let carriage_hit = same_versor(&cell.carriage, carriage)
            || cell.carriage.versor_inverse().is_some_and(|i| same_versor(&i, carriage));
        if rotor_hit && carriage_hit {
            if cell.vertices.len() >= 2 {
                if let Some(sec) = self.sections.get_mut(&cell.vertices[1]) {
                    sec.gamma = gamma.clone();
                    sec.carriage = carriage.clone();
                }
            }
            let filled = Fill::Filled(gamma.clone(), Box::new(carriage.clone()));
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        self.fill(key)
    }

    /// Face coefficient from stored lower cells: edges (and, when already
    /// climbed, higher faces). A triangle is the holonomy of its three
    /// stored edges — never the triangle's own (possibly stale) cell.
    pub(crate) fn stored_face_coeff(&self, verts: &[Vec<u8>]) -> Option<(Rotor, Versor)> {
        match verts.len() {
            0 | 1 => Some((Rotor::identity(), crate::transducer::element::identity_versor())),
            2 => self.edge(&verts[0], &verts[1]),
            3 => self.holonomy(&verts[0], &verts[1], &verts[2]),
            _ => self.get(verts).map(|c| (c.coefficient.clone(), c.carriage.clone())),
        }
    }

    /// Coboundary of the (k−1)-faces of a k-cell. Face coefficients are
    /// read from stored lower cells that `faces_ready` already proved exist.
    pub(crate) fn coboundary_coeff(&self, verts: &[Vec<u8>]) -> Option<(Rotor, Versor)> {
        use crate::transducer::element::{identity_versor, versor_conjugate};
        let n = verts.len();
        if n < 4 {
            return self.stored_face_coeff(verts);
        }
        let (mut acc_r, mut acc_c) = (Rotor::identity(), identity_versor());
        for i in 0..n {
            let mut face: Vec<Vec<u8>> = verts.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| v.clone()).collect();
            face = sorted_verts(face);
            let (mut r, mut c) = self.stored_face_coeff(&face)?;
            if i % 2 == 1 {
                if let Some((gr, gc)) = self.edge(&verts[0], &verts[i]) {
                    r = r.conjugate(&gr);
                    c = versor_conjugate(&c, &gc);
                }
                r = r.inverse()?;
                c = c.versor_inverse()?;
            }
            acc_r = acc_r.geo(&r);
            acc_c = acc_c.geo(&c);
        }
        Some((acc_r, acc_c))
    }

    /// A candidate forms when every edge already exists (the 1-skeleton of
    /// the simplex). Faces that omit the climbing vertex are not required
    /// to sit in its link.
    pub(crate) fn faces_ready(&self, verts: &[Vec<u8>]) -> bool {
        let n = verts.len();
        if n < 3 {
            return true;
        }
        // index: edges of this simplex
        for i in 0..n {
            for j in i + 1..n {
                if self.get(&sorted_verts([&verts[i], &verts[j]])).is_none() {
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
    pub fn coboundary_on_link(&mut self, vertex: &[u8], mut volume_of: impl FnMut(&[Vec<u8>]) -> [u64; 4]) {
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
                            verts.push(w.clone());
                            let verts = sorted_verts(verts);
                            if verts.len() != nverts + 1 {
                                continue;
                            }
                            let nkey = cell_key(&verts);
                            if seen.contains(&nkey) {
                                continue;
                            }
                            seen.insert(&nkey, ());
                            if self.by_key.get(nkey.as_bytes()).is_some() {
                                self.reopen_on_coeff_change(&nkey, &verts);
                                continue;
                            }
                            let mut blocked = false;
                            // index: pairs on this simplex (faces)
                            for i in 0..verts.len() {
                                for j in i + 1..verts.len() {
                                    if self.is_remainder(&sorted_verts([&verts[i], &verts[j]])) {
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
                            let Some((coefficient, carriage)) = self.coboundary_coeff(&verts)
                            else {
                                continue;
                            };
                            pending.push(Cell {
                                vertices: verts,
                                coefficient,
                                carriage,
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
    pub fn fill(&mut self, key: &[u8]) -> Fill {
        let Some(degree) = self.degree_of(key) else {
            return Fill::Open;
        };
        let Some(cell) = self.degrees[degree].get(key).cloned() else {
            return Fill::Open;
        };
        if !matches!(cell.fill, Fill::Open) {
            return Fill::Already;
        }
        let flat = crate::transducer::element::same_versor(
            &cell.carriage,
            &crate::transducer::element::identity_versor(),
        );
        if cell.degree() == 0 || (cell.coefficient.is_identity() && flat) {
            let filled = Fill::Filled(
                Rotor::identity(),
                Box::new(crate::transducer::element::identity_versor()),
            );
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return filled;
        }
        let verts = &cell.vertices;
        if cell.degree() >= 2 {
            let mut edges: Vec<(usize, usize, Rotor, Versor)> = Vec::new();
            for i in 0..verts.len() {
                for j in i + 1..verts.len() {
                    if let Some((g, c)) = self.edge(&verts[i], &verts[j]) {
                        edges.push((i, j, g, c));
                    }
                }
            }
            if let Some(gs) = solve_gauge(verts.len(), &edges) {
                for (v, (g, c)) in verts.iter().zip(gs.iter()) {
                    if let Some(sec) = self.sections.get_mut(v) {
                        sec.gamma = g.clone();
                        sec.carriage = c.clone();
                    }
                }
                let Some((g1, c1)) = gs.get(1).cloned() else {
                    return Fill::Open;
                };
                let filled = Fill::Filled(g1, Box::new(c1));
                if let Some(c) = self.degrees[degree].get_mut(key) {
                    c.fill = filled.clone();
                }
                return filled;
            }
        }
        let Some(a) = self.sections.get(&verts[0]).cloned() else {
            return Fill::Open;
        };
        let Some(b) = verts.get(1).and_then(|v| self.sections.get(v).cloned()) else {
            return Fill::Open;
        };
        let outcome = extend(&a, &b, &cell);
        self.index_congruent(&outcome);
        if let Some(c) = self.degrees[degree].get_mut(key) {
            c.fill = outcome.clone();
        }
        outcome
    }

    /// The class a cell carries and the degree it carries it at: the class
    /// VECTOR of 12.6 and the rotor it was read from. An open cell of
    /// degree 2 or more whose coefficient is not the identity carries its
    /// coefficient's class, read the same way, so a cell that never filled
    /// still names what stands on it.
    pub fn class(
        &self,
        key: &[u8],
    ) -> Option<(crate::transducer::sentence::ClassVector, Rotor, usize)> {
        let d = self.degree_of(key)?;
        let c = self.degrees.get(d)?.get(key)?;
        match &c.fill {
            Fill::Class(k, r) => Some((k.as_ref().clone(), r.as_ref().clone(), d)),
            Fill::Open if !c.coefficient.is_identity() && d >= 2 => {
                match classify(
                    &c.coefficient,
                    &c.carriage,
                    crate::transducer::sentence::CycleType::of(&[]),
                ) {
                    Fill::Class(k, r) => Some((*k, *r, d)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Every standing class this complex holds. Both objects: the vector
    /// holds conjugacy invariants and the residues, the rotor is the
    /// element — readers that seed a pivot need the second, readers that
    /// compare classes need the first.
    pub fn classes(&self) -> Vec<(crate::transducer::sentence::ClassVector, Rotor, usize)> {
        let mut out = Vec::new();
        // index: keyed cells of this complex (one chart's nerve)
        for (k, d) in self.by_key.fiber(ROOT) {
            let d = *d as usize;
            if let Some(c) = self.degrees.get(d).and_then(|l| l.get(&k)) {
                if let Fill::Class(kappa, r) = &c.fill {
                    out.push((kappa.as_ref().clone(), r.as_ref().clone(), d));
                }
            }
        }
        out
    }

    pub fn unfilled(&self) -> Vec<(usize, Vec<u8>)> {
        let mut out = Vec::new();
        // index: keyed cells of this complex (one chart's nerve)
        for (k, d) in self.by_key.fiber(ROOT) {
            let d = *d as usize;
            if d >= 1 {
                if let Some(c) = self.degrees.get(d).and_then(|l| l.get(&k)) {
                    if matches!(c.fill, Fill::Open) {
                        out.push((d, k));
                    }
                }
            }
        }
        out
    }

}
