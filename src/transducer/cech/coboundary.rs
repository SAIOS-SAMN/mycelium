// 13, 14 — THE COBOUNDARY: which simplices are ready to close, what closes
// them, and the class that remains when nothing does.
//
//! `coboundary_on_link` walks the link of a vertex and closes what its
//! faces already determine; a simplex whose faces are not ready is left
//! open rather than guessed at. `solve_gauge` is the tree walk of 12.2.

use super::*;

/// What a tree walk produces: the 0-cochain γ that places each vertex, and
/// the 1-cocycle the chords carry that γ cannot place.
///
/// The two are one reading and are returned together. `sections` is exact
/// iff `holonomy` is empty; a non-empty `holonomy` does not make `sections`
/// a gauge, and no caller may treat it as one.
#[derive(Clone, Debug)]
pub struct Gauge {
    /// γ_v for each vertex: the rotor and the carriage that place it,
    /// composed along the spanning forest.
    pub sections: Vec<(Rotor, Versor)>,
    /// Ω on each chord that disagrees: `(i, j, γ_j γ_i⁻¹ g⁻¹, …)` on both
    /// halves. This is H¹ — the curvature of the 1-skeleton, measured on a
    /// closed loop — and it is the content of the disagreement, not a
    /// symptom of one.
    pub holonomy: Vec<(usize, usize, Rotor, Versor)>,
}

/// Tree-plus-chord solver for δγ = ω on a transition's BOTH halves — the
/// rotor in `Cl(4,0)` and the carriage in `Cl(3,1)` (L11e). A forest
/// 1-cochain is exact; a chord that does not match the tree path closes a
/// loop, and what it carries round that loop is returned.
///
/// IT COMPUTED THE HOLONOMY AND THREW IT AWAY. The disagreement test was
/// `γ_j γ_i⁻¹ ≠ g`, which is `Ω = γ_j γ_i⁻¹ g⁻¹ ≠ 1` with `Ω` formed
/// inside the comparison and discarded on the next line by returning
/// `None` for the whole complex — one chord's class erasing every vertex
/// the tree had placed. `Ω` is the Čech 1-cocycle: the obstruction to the
/// sections gluing, and the only object in the tree walk that a spanning
/// forest does not already determine. `None` now means only that a reading
/// has no inverse.
///
/// The note here claimed this gauge conjugates a turn's base-point
/// translation away so the turn can file `Congruent`. It never did: it
/// composes placements along the tree and returns them, and `fill` stores
/// them and answers `Filled`. A turn about a point is read where a turn is
/// read — [`crate::transducer::element::turn_base_point`], through
/// [`classify`].
pub fn solve_gauge(n: usize, edges: &[(usize, usize, Rotor, Versor)]) -> Option<Gauge> {
    use crate::transducer::element::{identity_versor, same_versor};
    if n == 0 {
        return Some(Gauge { sections: Vec::new(), holonomy: Vec::new() });
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
    let mut holonomy: Vec<(usize, usize, Rotor, Versor)> = Vec::new();
    for (i, j, g, c) in &chords {
        let (gi, ci) = (gamma[*i].inverse()?, carriage[*i].versor_inverse()?);
        let (path_r, path_c) = (gamma[*j].geo(&gi), carriage[*j].geo(&ci));
        // A chord must agree on BOTH halves. Agreeing on the rotor alone
        // would call a tree exact when its placements disagree, which is
        // the class the gauge exists to withhold.
        if path_r == *g && same_versor(&path_c, c) {
            continue;
        }
        // `Ω = (tree path) · (chord)⁻¹` on each half: what the loop
        // through the tree and back along the chord carries. Both halves,
        // for the reason the test above is on both.
        let (gc, cc) = (g.inverse()?, c.versor_inverse()?);
        holonomy.push((*i, *j, path_r.geo(&gc), path_c.geo(&cc)));
    }
    Some(Gauge { sections: gamma.into_iter().zip(carriage).collect(), holonomy })
}

impl Complex {
    /// Mark a simplex Extended by a known gauge (a translation, a taught
    /// coboundary). Degree-1 never tree-solves — that would always succeed.
    pub fn fill_by_gauge(
        &mut self,
        key: &[u8],
        gamma: &Rotor,
        carriage: &Versor,
    ) -> CoboundaryOutcome {
        use crate::transducer::element::same_versor;
        let Some(degree) = self.degree_of(key) else {
            return CoboundaryOutcome::Absent;
        };
        let Some(simplex) = self.degrees[degree].get(key).cloned() else {
            return CoboundaryOutcome::Absent;
        };
        if !matches!(simplex.fill, Fill::Open) {
            return CoboundaryOutcome::AlreadyFilled;
        }
        // The gauge must match on BOTH halves, forward or reversed: a
        // rotor-only match would call a simplex closed by a gauge whose
        // placement differs from the one it expects.
        let rotor_hit = simplex.coefficient == *gamma
            || simplex.coefficient.inverse().as_ref() == Some(gamma);
        let carriage_hit = same_versor(&simplex.carriage, carriage)
            || simplex.carriage.versor_inverse().is_some_and(|i| same_versor(&i, carriage));
        if rotor_hit && carriage_hit {
            if simplex.vertices.len() >= 2 {
                if let Some(sec) = self.sections.get_mut(&simplex.vertices[1]) {
                    sec.gamma = gamma.clone();
                    sec.carriage = carriage.clone();
                }
            }
            let filled = Fill::Filled(Box::new(gamma.clone()), Box::new(carriage.clone()));
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return CoboundaryOutcome::Attached(filled);
        }
        self.fill(key)
    }

    /// Face coefficient from stored lower simplices: edges (and, when
    /// already climbed, higher faces). A triangle is the holonomy of its
    /// three stored edges — never the triangle's own (possibly stale)
    /// simplex.
    pub(crate) fn stored_face_coeff(&self, verts: &[Vec<u8>]) -> Option<(Rotor, Versor)> {
        match verts.len() {
            0 | 1 => Some((Rotor::identity(), crate::transducer::element::identity_versor())),
            2 => self.edge(&verts[0], &verts[1]),
            3 => self.holonomy(&verts[0], &verts[1], &verts[2]),
            _ => self.get(verts).map(|c| (c.coefficient.clone(), c.carriage.clone())),
        }
    }

    /// Coboundary of the (k−1)-faces of a k-simplex. Face coefficients are
    /// read from stored lower simplices that `faces_ready` already proved
    /// exist.
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
    /// intersection of the links of existing k-simplices with the link of
    /// one more neighbour, never a subset of the fiber. Stops when the next
    /// degree produces no volume. Simplices are read by key; rotors are
    /// not cloned.
    pub fn coboundary_on_link(&mut self, vertex: &[u8], mut volume_of: impl FnMut(&[Vec<u8>]) -> [u64; 4]) {
        let mut degree = 1usize;
        loop {
            let layer = self.link(vertex, degree);
            if layer.is_empty() {
                break;
            }
            let mut pending: Vec<Simplex> = Vec::new();
            let mut seen: Keyed<()> = Keyed::new();
            // index: link of `vertex` at this degree
            for key in &layer {
                let Some(simplex_verts) = self.simplex(key).map(|c| c.vertices.clone()) else {
                    continue;
                };
                let nverts = simplex_verts.len();
                // index: vertices of this simplex
                for u in &simplex_verts {
                    if u == vertex {
                        continue;
                    }
                    let edges = self.link(u, 1);
                    // index: link of neighbour u (degree 1)
                    for ek in &edges {
                        let Some(edge_verts) = self.simplex(ek).map(|c| c.vertices.clone()) else {
                            continue;
                        };
                        // index: vertices of that edge
                        for w in &edge_verts {
                            if w == u || simplex_verts.binary_search(w).is_ok() {
                                continue;
                            }
                            let mut verts = simplex_verts.clone();
                            verts.push(w.clone());
                            let verts = sorted_verts(verts);
                            if verts.len() != nverts + 1 {
                                continue;
                            }
                            let nkey = simplex_key(&verts);
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
                            pending.push(Simplex {
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
            for simplex in pending {
                self.remember(simplex);
            }
            degree += 1;
        }
    }

    /// Fill a simplex by extend: coboundary is Extended, finite order is
    /// Congruent, otherwise Class.
    pub fn fill(&mut self, key: &[u8]) -> CoboundaryOutcome {
        let Some(degree) = self.degree_of(key) else {
            return CoboundaryOutcome::Absent;
        };
        let Some(simplex) = self.degrees[degree].get(key).cloned() else {
            return CoboundaryOutcome::Absent;
        };
        if !matches!(simplex.fill, Fill::Open) {
            return CoboundaryOutcome::AlreadyFilled;
        }
        let flat = crate::transducer::element::same_versor(
            &simplex.carriage,
            &crate::transducer::element::identity_versor(),
        );
        if simplex.degree() == 0 || (simplex.coefficient.is_identity() && flat) {
            let filled = Fill::Filled(
                Box::new(Rotor::identity()),
                Box::new(crate::transducer::element::identity_versor()),
            );
            if let Some(c) = self.degrees[degree].get_mut(key) {
                c.fill = filled.clone();
            }
            return CoboundaryOutcome::Attached(filled);
        }
        let verts = &simplex.vertices;
        if simplex.degree() >= 2 {
            let mut edges: Vec<(usize, usize, Rotor, Versor)> = Vec::new();
            for i in 0..verts.len() {
                for j in i + 1..verts.len() {
                    if let Some((g, c)) = self.edge(&verts[i], &verts[j]) {
                        edges.push((i, j, g, c));
                    }
                }
            }
            if let Some(gauge) = solve_gauge(verts.len(), &edges) {
                if gauge.holonomy.is_empty() {
                    for (v, (g, c)) in verts.iter().zip(gauge.sections.iter()) {
                        if let Some(sec) = self.sections.get_mut(v) {
                            sec.gamma = g.clone();
                            sec.carriage = c.clone();
                        }
                    }
                    let Some((g1, c1)) = gauge.sections.get(1).cloned() else {
                        // The gauge placed no second section, so there is
                        // no reading to attach — absent, not open.
                        return CoboundaryOutcome::Absent;
                    };
                    let filled = Fill::Filled(Box::new(g1), Box::new(c1));
                    if let Some(c) = self.degrees[degree].get_mut(key) {
                        c.fill = filled.clone();
                    }
                    return CoboundaryOutcome::Attached(filled);
                }
                // THE CHORDS CARRY CLASSES — one per loop, each filed at
                // its own coordinate. The sections are NOT placed: a
                // forest its own chords contradict is not a gauge, and
                // writing it onto the sections would publish a frame the
                // complex does not have.
                //
                // Each `Ω` is classified alone, by the one classifier
                // (L23b), and written at `loop_key`. It COMPOSED them and
                // classified the product: the product of holonomies around
                // different loops is the holonomy of none of them, and two
                // chords of opposite class multiplied to the identity — two
                // real classes reading as "nothing curved". Before that it
                // returned `None` and the loops were not read at all.
                //
                // The simplex's own fill still comes from `extend` below:
                // this adds the loops beside it and takes nothing away.
                for (i, j, gr, gc) in &gauge.holonomy {
                    let (Some(vi), Some(vj)) = (verts.get(*i), verts.get(*j)) else {
                        continue;
                    };
                    let outcome =
                        classify(gr, gc, crate::transducer::sentence::CycleType::of(&[]));
                    if matches!(outcome, Fill::Open) {
                        continue;
                    }
                    self.index_congruent(&outcome);
                    let lk = crate::transducer::cech::complex::loop_key(key, vi, vj);
                    self.loops.insert(lk, outcome);
                }
            }
        }
        // A SECTION THE FILL NEEDS IS NOT THERE. This answered `Open`,
        // which a caller then read as "the simplex stands unfilled" — but
        // nothing was evaluated, and an unread cell is not an open one
        // (rule 11).
        let Some(a) = self.sections.get(&verts[0]).cloned() else {
            return CoboundaryOutcome::Absent;
        };
        let Some(b) = verts.get(1).and_then(|v| self.sections.get(v).cloned()) else {
            return CoboundaryOutcome::Absent;
        };
        let outcome = extend(&a, &b, &simplex);
        self.index_congruent(&outcome);
        if let Some(c) = self.degrees[degree].get_mut(key) {
            c.fill = outcome.clone();
        }
        CoboundaryOutcome::Attached(outcome)
    }

    /// The class a simplex carries and the degree it carries it at: the
    /// class VECTOR of 12.6 and the rotor it was read from. An open simplex
    /// of degree 2 or more whose coefficient is not the identity carries
    /// its coefficient's class, read the same way, so a simplex that never
    /// filled still names what stands on it.
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
        // EVERY standing class, as the name says: the simplices of this
        // complex, which is one chart's nerve and a fiber already.
        for (k, d) in self.by_key.scoped() {
            let d = *d as usize;
            if let Some(c) = self.degrees.get(d).and_then(|l| l.get(&k)) {
                if let Fill::Class(kappa, r) = &c.fill {
                    out.push((kappa.as_ref().clone(), r.as_ref().clone(), d));
                }
            }
            // AND THE LOOPS ON IT, each one its own standing class (11.3:
            // an `H¹` loop is held with its class vector). Read as the
            // fiber under the simplex's key, at the degree the simplex
            // already carries — the loops were stored and unread, which is
            // inert (24), and folding them was worse.
            for fill in self.loops_of(&k) {
                if let Fill::Class(kappa, r) = fill {
                    out.push((*kappa, *r, d));
                }
            }
        }
        out
    }

    pub fn unfilled(&self) -> Vec<(usize, Vec<u8>)> {
        let mut out = Vec::new();
        // As above: this complex's own simplices, all of them, since an
        // unfilled simplex anywhere in the nerve is a demand.
        for (k, d) in self.by_key.scoped() {
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
