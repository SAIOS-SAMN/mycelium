// 13, 14 — THE COBOUNDARY: which cells are ready to close, what closes
// them, and the class that remains when nothing does.
//
//! `coboundary_on_link` walks the link of a vertex and closes what its
//! faces already determine; a cell whose faces are not ready is left open
//! rather than guessed at. `solve_gauge` is the spanning-forest solve of
//! 12.2 with its documented sign convention.

use super::*;

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

impl Complex {
    /// Mark a cell Extended by a known gauge (a translation, a taught
    /// coboundary). Degree-1 never tree-solves — that would always succeed.
    pub fn fill_by_gauge(&mut self, key: &[u8], gamma: &Rotor) -> Fill {
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

    /// Face coefficient from stored lower cells: edges (and, when already
    /// climbed, higher faces). A triangle is the holonomy of its three
    /// stored edges — never the triangle's own (possibly stale) cell.
    pub(crate) fn stored_face_coeff(&self, verts: &[Vec<u8>]) -> Option<Rotor> {
        match verts.len() {
            0 | 1 => Some(Rotor::identity()),
            2 => self.edge(&verts[0], &verts[1]),
            3 => self.holonomy(&verts[0], &verts[1], &verts[2]),
            _ => self.get(verts).map(|c| c.coefficient.clone()),
        }
    }

    /// Coboundary of the (k−1)-faces of a k-cell. Face coefficients are
    /// read from stored lower cells that `faces_ready` already proved exist.
    pub(crate) fn coboundary_coeff(&self, verts: &[Vec<u8>]) -> Option<Rotor> {
        let n = verts.len();
        if n < 4 {
            return self.stored_face_coeff(verts);
        }
        let mut acc = Rotor::identity();
        for i in 0..n {
            let mut face: Vec<Vec<u8>> = verts.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| v.clone()).collect();
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
                let Some(g1) = gs.get(1).cloned() else {
                    return Fill::Open;
                };
                let filled = Fill::Filled(g1);
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

    pub fn class(&self, key: &[u8]) -> Option<(Rotor, usize)> {
        let d = self.degree_of(key)?;
        let c = self.degrees.get(d)?.get(key)?;
        match &c.fill {
            Fill::Class(r) => Some((r.clone(), d)),
            Fill::Open if !c.coefficient.is_identity() && d >= 2 => Some((c.coefficient.clone(), d)),
            _ => None,
        }
    }

    pub fn classes(&self) -> Vec<(Rotor, usize)> {
        let mut out = Vec::new();
        // index: keyed cells of this complex (one chart's nerve)
        for (k, d) in self.by_key.fiber(ROOT) {
            let d = *d as usize;
            if let Some(c) = self.degrees.get(d).and_then(|l| l.get(&k)) {
                if let Fill::Class(r) = &c.fill {
                    out.push((r.clone(), d));
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
