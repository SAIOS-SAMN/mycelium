// 13 — THE COMPLEX: every simplex a mind holds, and the reads that cross it.
//
//! The star, the link and the degree are reads of structure. Setting an
//! edge or a section reopens exactly the simplices whose coefficients
//! changed, so nothing is recomputed that did not move.

use super::*;
pub use super::orbit::Thread;

/// 78.75, MU-CLIENT §4.5 — ONE FLOW A PIVOT HANDS BACK: its motion, and the
/// line it belongs to — WOUND where a tether condenses or a face seals (its
/// strain enters the field and threads the tubes and orbits its modes land
/// on), UNWOUND where it drains or is released (its reverse leaves them), FREE
/// where it belongs to no line of the complex.
#[derive(Clone, Debug)]
pub struct Flow {
    pub motion: crate::transducer::cga::Mv,
    pub line: Thread,
}

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
    /// Congruent fills whose carriage had no canonical ray: not filed.
    pub(crate) congruent_unfiled: u64,
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
    /// 78.74 — THIS COMPLEX'S SEALS, counted where a fill is written: a cell
    /// of degree two or more passing from unsettled to settled. Held on the
    /// complex, not only on a thread's ledger, so a court that glues on many
    /// threads reads every seal it made; volatile.
    pub(crate) sealed: u64,
    /// 78.75 — THE PIVOTS' OUTFLOW: the motion each pivot hands back — a
    /// tether condensed or cauterized, a face sealed or a membrane released —
    /// each with the line it belongs to, held for the owner that armed it, who
    /// reads it into its field (`take_outflow`). `None` while no owner
    /// listens, so a complex nobody reads holds nothing. Volatile.
    pub(crate) outflow: Option<Vec<Flow>>,
    /// 78.99 (2) — THE HOLES: cells `puncture` opened, each keeping its fill
    /// as the class the boundary winds about, standing in its vertices'
    /// stars and pinning its boundary until `seal` caps it. Volatile: no
    /// persisted complex punctures yet (`hole.rs`).
    pub(crate) holes: Keyed<Simplex>,
    /// THE SCARS: holes CAUTERIZED where their class could not be read back
    /// (the eight-component cross, ruled 2026-10-20) — counted on the chart
    /// that bore them, the class itself handed to the caller that burned it,
    /// never held here, so the record keeps its bound. Volatile.
    pub(crate) scars: u64,
    /// THE VACUUM: holes SEALED BY COLD (the operator's ruling, 2026-10-21:
    /// "the vacuum hold the frozen class") — a class whose read-back could
    /// not be made while its vertices stood, frozen in place at its
    /// coordinate, whole: no longer a hole (it pins nothing), never burned,
    /// and thawed back into a chamber where a meet over the same vertices
    /// finds its boundary closing again. The continuum's seal (`cauterize`)
    /// burns and keeps a count; the vacuum holds what the continuum may not,
    /// so this carrier is not bounded by a count. Volatile.
    pub(crate) frozen: Keyed<Simplex>,
    /// 78.112 H7b — THE TORSION HEARING WOUND AT EACH VERTEX: the crossings of
    /// the rings heard in that perception (the light crossing their planes,
    /// the impact), composed as one strain and wound on at each hearing. It
    /// unwinds when the vertex is let go (`vent_wound`). Volatile.
    pub(crate) wound: Keyed<crate::transducer::cga::Mv>,
}

/// 78.123 C — THE MINDS' COMPLEX IS A COCHAIN IN THE VERSOR GROUP: the
/// coefficient crossing an edge is its rotor and carriage, oriented as
/// `Complex::edge` reads them.
impl super::cover::Cochain<super::coefficient::Turned> for Complex {
    fn crossing(&self, a: &[u8], b: &[u8]) -> Option<super::coefficient::Turned> {
        let (rotor, carriage) = self.edge(a, b)?;
        Some(super::coefficient::Turned { rotor, carriage })
    }
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

/// 78.75 — ONE DEGREE OF A NERVE, BY WHAT ITS CELLS HOLD: how many cells,
/// and of them how many stand open (unread), filled (closed under a gauge),
/// congruent, classed, unspellable or unread by refusal — and what each
/// cell's motion IS, read by its modes (MR1) off `Simplex::motion`, the one
/// `Cl(4,1)` element: flat (no mode), and the cells holding a turn, a boost,
/// a null flow (a slide, or a source at a finite point), or a 180° turn (the
/// held `ℤ₂`); a cell holding several is counted on each. A cell whose modes
/// the stroke could not split is unread by name.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DegreeCensus {
    pub cells: u64,
    pub open: u64,
    pub filled: u64,
    pub congruent: u64,
    pub class: u64,
    pub unspellable: u64,
    pub unread: u64,
    pub flat: u64,
    pub turn: u64,
    pub boost: u64,
    pub null: u64,
    pub half: u64,
    pub modes_unread: u64,
}

impl DegreeCensus {
    /// Add another's counts, each past its carrier refused and left.
    pub fn add(&mut self, o: &DegreeCensus) {
        for (s, x) in [
            (&mut self.cells, o.cells),
            (&mut self.open, o.open),
            (&mut self.filled, o.filled),
            (&mut self.congruent, o.congruent),
            (&mut self.class, o.class),
            (&mut self.unspellable, o.unspellable),
            (&mut self.unread, o.unread),
            (&mut self.flat, o.flat),
            (&mut self.turn, o.turn),
            (&mut self.boost, o.boost),
            (&mut self.null, o.null),
            (&mut self.half, o.half),
            (&mut self.modes_unread, o.modes_unread),
        ] {
            let _ = crate::physics::metabolism::step_or_refuse(s, x);
        }
    }

    /// The cells that carry content: a motion with some mode. `None` where
    /// the counts do not partition (they always do: flat and unread are
    /// cells).
    pub fn curved(&self) -> Option<u64> {
        self.cells.checked_sub(self.flat)?.checked_sub(self.modes_unread)
    }
}

impl Complex {
    pub fn new() -> Complex {
        Complex::default()
    }

    /// 78.75 — LISTEN TO THE PIVOTS: from here on every motion a pivot hands
    /// back is held until taken.
    pub fn arm_outflow(&mut self) {
        if self.outflow.is_none() {
            self.outflow = Some(Vec::new());
        }
    }

    /// THE FLOWS THE PIVOTS HANDED BACK since the last take; the complex
    /// keeps listening.
    pub fn take_outflow(&mut self) -> Vec<Flow> {
        match self.outflow.as_mut() {
            Some(held) => std::mem::take(held),
            None => Vec::new(),
        }
    }

    /// 78.112 H7b — WIND TORSION ON A VERTEX: `strain` composed onto what the
    /// vertex already holds, the later after the earlier. The identity winds
    /// nothing.
    pub fn wind(&mut self, v: &[u8], strain: &crate::transducer::cga::Mv) {
        let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        if strain.projective_eq(&one) {
            return;
        }
        let held = match self.wound.get(v) {
            Some(w) => strain.geo(w),
            None => strain.clone(),
        };
        self.wound.insert(v, held);
    }

    /// The torsion wound on a vertex, where any is.
    pub fn wound_at(&self, v: &[u8]) -> Option<&crate::transducer::cga::Mv> {
        self.wound.get(v)
    }

    /// 78.112 H7b — A VERTEX LET GO UNWINDS WHAT HEARING WOUND ON IT: its
    /// strain's reverse flows out (78.106), into the field of an owner that
    /// listens.
    pub(crate) fn vent_wound(&mut self, v: &[u8]) {
        if let Some(w) = self.wound.remove(v) {
            self.flow_out(Flow { motion: w.reverse(), line: Thread::Free });
        }
    }

    /// One flow handed back, held where an owner listens; the identity
    /// carries nothing and is not held.
    pub(crate) fn flow_out(&mut self, flow: Flow) {
        let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        if let (Some(held), false) = (self.outflow.as_mut(), flow.motion.projective_eq(&one)) {
            held.push(flow);
        }
    }

    /// 78.75 — THE NERVE BY DEGREE: per degree, its cells by what they hold,
    /// and each cell's motion by its modes. Does it hold only vertices and
    /// edges, or do faces and volumes stand — and what is the content of each
    /// face: a turn, a boost, a null flow, or nothing? Under the stroke: a
    /// cell the stroke does not reach is unread by name.
    pub fn census(&self, budget: &mut crate::physics::Budget) -> Vec<DegreeCensus> {
        use crate::network::nerve::coord::BivectorKind;
        self.degrees
            .iter()
            .map(|layer| {
                let mut d = DegreeCensus::default();
                for (_, s) in layer.scoped() {
                    d.cells += 1;
                    match &s.fill {
                        Fill::Open => d.open += 1,
                        Fill::Filled(..) => d.filled += 1,
                        Fill::Congruent(..) => d.congruent += 1,
                        Fill::Class(..) => d.class += 1,
                        Fill::Unspellable(..) => d.unspellable += 1,
                        Fill::Unread(_) => d.unread += 1,
                    }
                    let modes = match budget.spend() {
                        true => crate::transducer::cga::motion_modes(&s.motion(), budget).and_then(Result::ok),
                        false => None,
                    };
                    let Some(modes) = modes else {
                        d.modes_unread += 1;
                        continue;
                    };
                    let live: Vec<_> = modes.iter().filter(|m| m.kind != BivectorKind::Trivial).collect();
                    if live.is_empty() {
                        d.flat += 1;
                        continue;
                    }
                    let has = |k: BivectorKind| live.iter().any(|m| m.kind == k);
                    d.turn += u64::from(has(BivectorKind::Elliptic));
                    d.boost += u64::from(has(BivectorKind::Hyperbolic));
                    d.null += u64::from(has(BivectorKind::Parabolic));
                    d.half += u64::from(live.iter().any(|m| m.flow().is_some_and(|p| p.parameter().is_none())));
                }
                d
            })
            .collect()
    }

    /// THE HOLES STANDING: classes the cross kept and questions carried, each
    /// pinning its boundary and its vertices until its seal.
    pub fn holes_standing(&self) -> usize {
        self.holes.len()
    }

    /// THE SCARS: holes cauterized on this complex, each a class burned where
    /// it could not be read back.
    pub fn scars(&self) -> u64 {
        self.scars
    }

    /// THE CLASSES THE VACUUM HOLDS, frozen in place.
    pub fn frozen_held(&self) -> usize {
        self.frozen.len()
    }

    /// Whether the vacuum holds a frozen class at `key`.
    pub fn is_frozen(&self, key: &[u8]) -> bool {
        self.frozen.contains(key)
    }

    /// THE ENTRIES THE COMPLEX HOLDS: its simplices at every degree and
    /// their index, its sections, its stars, its remainder and its
    /// congruent fills.
    pub fn held(&self) -> usize {
        let simplices: usize = self.degrees.iter().map(|d| d.len()).sum();
        let star: usize = self.star.scoped().map(|(_, s)| s.len()).sum();
        simplices + self.by_key.len() + self.sections.len() + star + self.remainder.len() + self.congruent.len() + self.holes.len()
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
    ///
    /// AND WHERE THE ORBIT STANDS UNDER ITS CLASS (78.49(4)): the class key,
    /// then the canonical ray of the class vector's carriage — where the turn
    /// stands. Two turns of one class about different points are one class
    /// and two orbits; keyed by the class alone, the second overwrote the
    /// first. A carriage with no canonical ray is no versor and files
    /// nothing, counted.
    pub(crate) fn index_congruent(&mut self, fill: &Fill) {
        if let Fill::Congruent(kappa, g, ell) = fill {
            let Some(at) = kappa.carriage.canonical() else {
                self.congruent_unfiled += 1;
                return;
            };
            let mut key = kappa.class_key().as_bytes().to_vec();
            let mut ray = Vec::new();
            at.put_canonical(&mut ray);
            crate::transducer::address::put_key_bytes(&mut key, &ray);
            self.congruent.insert(key, (g.as_ref().clone(), *ell));
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
    ///
    /// 78.123 C — READ BY THE ONE HOLONOMY every cover reads its cycles by
    /// (`cover::holonomy`), this complex a cochain in the versor group: the
    /// ledger's lanes and rates and the minds' versors, one machinery.
    pub(crate) fn holonomy(&self, i: &[u8], j: &[u8], k: &[u8]) -> Option<(Rotor, Versor)> {
        let t = super::cover::holonomy(self, &[i, j, k]).ok()?;
        Some((t.rotor, t.carriage))
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
            // A hole on the vertex goes with it, from its other vertices'
            // stars too: nothing names a hole whose boundary is gone.
            if let Some(hole) = self.holes.remove(&k) {
                for v in &hole.vertices {
                    if let Some(star) = self.star.get_mut(v) {
                        star.remove(&k);
                    }
                }
            }
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
