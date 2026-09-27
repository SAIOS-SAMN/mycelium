// 13 — THE COMPLEX: every simplex a mind holds, and the reads that cross it.
//
//! The star, the link and the degree are reads of structure. Setting an
//! edge or a section reopens exactly the simplices whose coefficients
//! changed, so nothing is recomputed that did not move.

use super::*;

/// The Čech complex on one chart. One Keyed trie per degree, grown as
/// overlaps demand. No fixed arity.
#[derive(Clone, Debug, Default)]
pub struct Complex {
    pub degrees: Vec<Keyed<Simplex>>,
    /// simplex key → degree, written in remember. Fill and class are keyed
    /// reads, never a traversal of degrees.
    pub(crate) by_key: Keyed<usize>,
    pub sections: Keyed<Section>,
    pub(crate) star: Keyed<Keyed<()>>,
    pub(crate) remainder: Keyed<()>,
    /// Congruent fills keyed by the orbit's whole class key (15.4, rule 6).
    /// Court torsion reads this by the rotor's conjugacy key, which is that
    /// key's depth-one stratum, so the read is a fiber; remember and fill
    /// write it.
    pub(crate) congruent: Keyed<(Rotor, u32)>,
    /// THE ORBITS OF EACH SIMPLEX, one per chord, each at its OWN
    /// coordinate: `simplex_key ++ vertex_i ++ vertex_j` (law 2, A21), so
    /// a simplex's orbits are the fiber under its key and a reader moves to
    /// them rather than reading a list through.
    ///
    /// One `Ω` per chord and never their product. 11.3 holds each `H¹`
    /// orbit "with its class vector (12.6)", and composing the orbits of one
    /// simplex destroys that: two chords of opposite class multiply to the
    /// identity, so two real classes annihilate into "nothing curved" and
    /// the axes 3.9 would have read its candidate pivots off are gone.
    pub(crate) orbits: Keyed<Fill>,
    /// RM1 — RE-FILLS: a simplex standing `Congruent` written `Filled`,
    /// counted where a fill is written (78.37(4)). Zero until the named door
    /// of phase (c) lands; volatile.
    pub(crate) refilled: u64,
}

/// WHAT A REOPEN CUT (78.26(6)): a ring closed at holonomy 1, or one closed at
/// its period — a remainder, coherent up to `ℓ` (78.37(4)). Counted apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cut {
    Filled,
    Congruent,
}

/// The coordinate of ONE orbit: the simplex it closes on, then the two
/// vertices its chord runs between, each written out (A21). A prefix of it
/// is the simplex, which is what makes `orbits_of` a fiber read.
pub(crate) fn orbit_key(simplex: &[u8], i: &[u8], j: &[u8]) -> crate::transducer::address::Address {
    let mut buf = simplex.to_vec();
    crate::transducer::address::put_key_bytes(&mut buf, i);
    crate::transducer::address::put_key_bytes(&mut buf, j);
    crate::transducer::address::Address::new(buf)
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

    pub(crate) fn remember(&mut self, mut simplex: Simplex) {
        // A SIMPLEX ON NO VERTICES IS NO SIMPLEX. It is not one of degree
        // zero — that is a point — so it is not remembered as one, and
        // every reader of `degree` may then take `n − 1` whole.
        if simplex.vertices.is_empty() {
            return;
        }
        simplex.coefficient = simplex.coefficient.normalised();
        let degree = simplex.degree();
        let key = simplex.key();
        self.ensure(degree);
        // The degree in its own count: a simplex's arity is the charts
        // standing together, and `as u8` read a 257-vertex simplex back at
        // another degree (STD1 3c-5).
        self.by_key.insert(&key, degree);
        // index: vertices of this simplex (bounded by degree)
        for v in &simplex.vertices {
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
        self.index_congruent(&simplex.fill);
        self.degrees[degree].insert(&key, simplex);
    }

    /// FILED AT ITS FULL COORDINATES (rule 6). `ClassVector::class_key`
    /// writes the rotor's conjugacy invariants FIRST — proven, not
    /// assumed, by `the_rotor_class_is_the_first_stratum_of_the_class_vector`
    /// — so the rotor class a court asks by is this key's depth-one
    /// stratum and the ask is a fiber read under it.
    ///
    /// It filed at the rotor class alone. Every congruent orbit of one
    /// conjugacy class landed on ONE key, so the last writer won and every
    /// finer coordinate of the orbits before it was unreachable: a whole
    /// fiber stored as its most recent member.
    pub(crate) fn index_congruent(&mut self, fill: &Fill) {
        if let Fill::Congruent(kappa, g, ell) = fill {
            self.congruent.insert(kappa.class_key(), (g.as_ref().clone(), *ell));
        }
    }

    /// THE CONGRUENT ORBITS UNDER A CLASS COORDINATE — a fiber read, not a
    /// lookup. `class` may be a rotor's conjugacy key (the depth-one
    /// stratum) or a whole class key; either way the answer is every filed
    /// orbit standing at or below that coordinate, at whatever finer
    /// coordinates they hold. Empty when the chart holds none.
    pub fn congruent(&self, class: &[u8]) -> Vec<(Rotor, u32)> {
        match self.congruent.fiber_at(class) {
            Some(fiber) => fiber.map(|(_, v)| v.clone()).collect(),
            None => Vec::new(),
        }
    }

    /// THE ORBITS ON ONE SIMPLEX — the fiber under its key. Each is one
    /// chord's `Ω`, classified on its own, so a simplex whose 1-skeleton
    /// closes two independent orbits answers with two classes and not with
    /// one composite that is the holonomy of neither.
    pub fn orbits_of(&self, simplex: &[u8]) -> Vec<Fill> {
        match self.orbits.fiber_at(simplex) {
            Some(fiber) => fiber.map(|(_, f)| f.clone()).collect(),
            None => Vec::new(),
        }
    }

    pub(crate) fn store_coeff(g: Rotor) -> Rotor {
        g.normalised()
    }

    pub fn set_section(&mut self, vertex: impl AsRef<[u8]>, section: Section, volume: [u64; 4]) {
        let vertex = vertex.as_ref().to_vec();
        self.sections.insert(&vertex, section);
        if nonzero(&volume) {
            self.remember(Simplex {
                vertices: vec![vertex],
                coefficient: Rotor::identity(),
                carriage: crate::transducer::element::identity_versor(),
                volume,
                fill: Fill::Filled(
                    Box::new(Rotor::identity()),
                    Box::new(crate::transducer::element::identity_versor()),
                ),
            });
        }
    }

    /// Degree-1 simplex: the transition rotor, or a remainder if it does
    /// not realise. Remainder edges form no higher simplices.
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
        let key = simplex_key(&vertices);
        self.remainder.remove(&key);
        // Both halves are oriented together: an edge stored against its
        // given direction inverts the rotor AND the carriage, or it is not
        // stored at all.
        let (stored_r, stored_c) = if vertices[0].as_slice() == a {
            (g, carriage)
        } else {
            // Both halves reversed: rays (78.32(1)).
            if !g.is_versor() {
                return;
            }
            (g.reverse(), carriage.reverse())
        };
        self.remember(Simplex {
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
        let carriage = crate::transducer::element::identity_versor();
        // The class vector is read through the SAME reader every other
        // congruence is read through, so a filed torsion axis carries the
        // coordinates 3.9 instantiates candidate moves from. A rotor with
        // no conjugacy invariants is not a versor (15.3): nothing is filed
        // for it rather than a period filed with no coordinates (rule 11).
        let Some(kappa) = crate::transducer::sentence::ClassVector::of_rotor(
            &g,
            carriage.clone(),
            crate::transducer::sentence::CycleType::Identity,
        ) else {
            return;
        };
        self.remember(Simplex {
            vertices,
            coefficient: g.clone(),
            // A filed congruence is residue-free by construction: it is a
            // finite-order rotation about a point, which `classify` returns
            // only where both residues are the identity.
            carriage,
            volume: [1, 0, 0, 0],
            fill: Fill::Congruent(Box::new(kappa), Box::new(g), ell),
        });
    }

    pub fn set_remainder_edge(&mut self, a: &[u8], b: &[u8]) {
        let vertices = sorted_verts(vec![a, b]);
        if vertices.len() != 2 {
            return;
        }
        let key = simplex_key(&vertices);
        if self.degrees.get(1).is_some() {
            self.degrees[1].remove(&key);
        }
        self.remainder.insert(key, ());
    }

    pub(crate) fn is_remainder(&self, verts: &[Vec<u8>]) -> bool {
        let v = sorted_verts(verts.to_vec());
        self.remainder.contains(&simplex_key(&v))
    }

    pub fn get(&self, verts: &[Vec<u8>]) -> Option<&Simplex> {
        // No vertices name no simplex.
        let d = verts.len().checked_sub(1)?;
        self.degrees.get(d)?.get(&simplex_key(verts))
    }

    /// Oriented edge rotor from a to b.
    pub fn edge(&self, a: &[u8], b: &[u8]) -> Option<(Rotor, Versor)> {
        let vertices = sorted_verts([a, b]);
        let s = self.get(&vertices)?;
        if vertices[0] == *a {
            Some((s.coefficient.clone(), s.carriage.clone()))
        } else {
            Some((s.coefficient.is_versor().then(|| s.coefficient.reverse())?, s.carriage.reverse()))
        }
    }

    /// Simplex at `key`, read in place. Never a clone of the rotor.
    pub fn simplex(&self, key: &[u8]) -> Option<&Simplex> {
        let d = *self.by_key.get(key)?;
        self.degrees.get(d)?.get(key)
    }

    /// A key in `by_key` names a stored simplex in the layer of its
    /// degree. `None` when it names none: a key with no simplex is not
    /// reopened (§2.4).
    pub(crate) fn stored_mut(&mut self, key: &[u8]) -> Option<&mut Simplex> {
        let d = *self.by_key.get(key)?;
        self.degrees.get_mut(d)?.get_mut(key)
    }

    /// Reopen a stored simplex only when its coboundary coefficient moved.
    /// Absence of a rotor is remainder: the fill stands. Answers what a
    /// reopen CUT, where it cut a closed ring — 78.26(6)'s decoherence at its
    /// site: a ring filled at holonomy 1, or one closed at its period
    /// (Congruent, a remainder coherent up to ℓ). Anything else reopened cut
    /// no coherence.
    pub(crate) fn reopen_on_coeff_change(&mut self, key: &[u8], verts: &[Vec<u8>]) -> Option<Cut> {
        let (coefficient, carriage) = self.coboundary_coeff(verts)?;
        let coefficient = Self::store_coeff(coefficient);
        let c = self.stored_mut(key)?;
        // EITHER half moving reopens the simplex. One whose rotor is
        // unchanged and whose placement moved is a different transition,
        // and leaving it filled would keep a gauge that no longer closes it.
        let same = c.coefficient.projective_eq(&coefficient)
            && crate::transducer::element::same_versor(&c.carriage, &carriage);
        if same {
            return None;
        }
        let cut = match c.fill {
            Fill::Filled(..) => Some(Cut::Filled),
            Fill::Congruent(..) => Some(Cut::Congruent),
            Fill::Class(..) | Fill::Unspellable(_) | Fill::Unread(_) | Fill::Open => None,
        };
        c.coefficient = coefficient;
        c.carriage = carriage;
        c.fill = Fill::Open;
        cut
    }

    /// The link of `vertex`: every incident simplex, all degrees. A peel
    /// of the star; work per glue never leaves this set.
    pub fn star(&self, vertex: &[u8]) -> Vec<Vec<u8>> {
        self.link_of(vertex)
    }

    /// Keys of every simplex incident on `vertex`. The climb reads
    /// simplices by these keys and never clones a rotor.
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
        self.by_key.get(key).copied()
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

    /// Drop every simplex that contains `vertex`. Used when bubbles merge.
    pub fn drop_vertex(&mut self, vertex: &[u8]) {
        let Some(keys) = self.star.remove(vertex) else {
            self.sections.remove(vertex);
            return;
        };
        let mut drop: Vec<Vec<u8>> = Vec::new();
        // THE STAR OF ONE VERTEX: `star.remove(vertex)` is the one keyed read, so
        // these are the simplices containing it and every one of them
        // drops.
        for (k, _) in keys.scoped() {
            drop.push(k);
        }
        for k in drop {
            if let Some(d) = self.by_key.remove(&k) {
                if let Some(layer) = self.degrees.get_mut(d) {
                    layer.remove(&k);
                }
            }
            // A dropped simplex's orbits leave with it: they stand at
            // coordinates its key prefixes, and nothing else reads them.
            let orbits: Vec<Vec<u8>> = match self.orbits.fiber_at(&k) {
                Some(fiber) => fiber.map(|(ok, _)| ok).collect(),
                None => Vec::new(),
            };
            for ok in orbits {
                self.orbits.remove(&ok);
            }
        }
        self.sections.remove(vertex);
    }

    pub fn simplex_count(&self) -> usize {
        self.by_key.len()
    }

}
