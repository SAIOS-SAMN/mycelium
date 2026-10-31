// 78.99 (2), 62.1, 63.3 — THE PIVOTS COMPLETED ON THE COMPLEX: `condense`,
// `drain`'s inverse on content; `puncture`, a sealed cell opened into a HOLE
// that keeps its class; `seal`, the hole capped.
//
//! THE LADDER, WHOLE (63.3's table, every row built):
//!
//! ```text
//!     dim 2   ascend   ───►  seal a cycle with a cell          coboundary.rs
//!             descend  ◄───  the cross: through the puncture   coboundary.rs
//!     dim 1   condense ───►  remember a tether                 here
//!             drain    ◄───  dissolve it, cauterized OUT       coboundary.rs
//!     hole    puncture ───►  open a cell, the class kept IN    here
//!             seal     ◄───  cap the hole, the class killed    here
//!             cauterize ◄──  close it by heat, the scar kept   here
//!             freeze   ◄───  close it by cold, the class held  here
//!             thaw     ───►  the frozen class re-sealed        here
//! ```
//!
//! HEAT AND COLD, THE THROAT'S TWO FACES (ruled
//! 2026-10-21): drain is heat, the continuum; condense is cold, the vacuum.
//! Where a vertex's word has left, its boundary drains with it and the hole
//! is cauterized — the class burned, its scar counted. Where the read-back
//! cannot be made while its vertices stand, its boundary condensed, the hole
//! is frozen — the class held whole by the vacuum, which may hold what the
//! continuum may not, and thawed where its boundary closes again.
//!
//! CAUTERIZE (the eight-component cross, ruled 2026-10-20): where a hole's
//! class cannot be read back — its seal does not close, or a vertex's word
//! has left — the hole is closed by heat, unpinning its boundary, its scar
//! recorded; never left standing as a wound that pins for ever.
//!
//! THE CROSS (ruled 2026-10-20): no descend without its
//! dipole ascend, its antipoles condense and drain, and the puncture. A
//! descend opens the boundary cycle THROUGH a puncture — the class kept in
//! the complex as a hole the boundary winds about, the chamber's content
//! handed down to the field — so no class is ever handed out alone. Only
//! `seal`, ascend's read-back, caps a hole and returns the class to a
//! chamber. A HOLE PINS ITS BOUNDARY — it carries a read class, as a read
//! coface does (63.2): its tethers stay condensed, and drain only where no
//! class stands over them; at a hole `descend`, `ascend` and `puncture`
//! refuse by name (78.99 (2)).
//!
//! HOLES ARE VOLATILE: no persisted complex punctures yet, so the tries
//! (`persist.rs`) carry none; the first caller on a persisted complex brings
//! its own trie with it.

use super::*;

/// A frozen class's entry on one of its vertices: the vertex, length-prefixed
/// so no vertex is another's prefix, then the class's key.
fn frozen_on_key(v: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, v);
    k.extend_from_slice(key);
    k
}

/// WHAT A `condense` DID — [`Drained`]'s inverse — and, where a tether
/// already stands on the pair, the composite `C = old⁻¹ · new` the arriving
/// transport is to the standing one (MU-CLIENT §4.5). What `C` does to the
/// FIBRE — field, torsion, lift (78.74) — is read against the fibre's pivots
/// by whoever holds them (`Contemplation::tether`); the complex holds none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condensed {
    /// The tether is remembered with its transition, standing open.
    Tethered,
    /// A tether already stands carrying this transport: nothing changed.
    AlreadyTethered,
    /// THE FIBRE FLEXED by `C`: the tether now carries the arriving
    /// transport (`old · C`) and every cell over it re-reads.
    Flexed(Box<crate::transducer::cga::Mv>),
    /// A HOLE over the tether — a question the mind still carries — holds
    /// it: it does not flex.
    Pinned(Box<crate::transducer::cga::Mv>),
    /// The pair is a remainder — a transition that does not realise — and
    /// no tether condenses there.
    Remainder,
    /// Not two distinct vertices.
    Absent,
}

/// WHAT A `puncture` DID.
#[derive(Clone, Debug)]
pub enum Punctured {
    /// The cell is opened into a hole carrying this fill: the class stays in.
    Opened(Fill),
    /// The cell stood unsettled — it recorded no class, so there is none to
    /// keep and nothing changed.
    Unsealed,
    /// A read coface (or a hole) above holds it: it must go first.
    Pinned,
    /// A hole already stands here; its conjugate is `seal`.
    Hole,
    /// No cell of degree two or more at this key.
    Absent,
}

/// WHAT A `cauterize` DID.
#[derive(Clone, Debug)]
pub enum Cauterized {
    /// The hole is closed and its class burned: this fill, the scar.
    Scarred(Fill),
    /// No hole at this key.
    Absent,
}

/// WHAT A `freeze` DID.
#[derive(Clone, Debug)]
pub enum Frozen {
    /// The hole is closed by cold and its class held in the vacuum, whole.
    Held(Fill),
    /// No hole at this key.
    Absent,
}

/// WHAT A `thaw` DID.
#[derive(Clone, Debug)]
pub enum Thawed {
    /// The frozen class's boundary closes again: the chamber returns with
    /// this fill and the vacuum lets it go.
    Sealed(Fill),
    /// The boundary still does not close: the class stays frozen.
    Held,
    /// The vacuum holds no class at this key.
    Absent,
}

/// WHAT A `seal` DID.
#[derive(Clone, Debug)]
pub enum Capped {
    /// The cap seals with `cap`; the hole and the class it carried, `killed`,
    /// are gone.
    Sealed { cap: Fill, killed: Fill },
    /// The boundary was read and does not close: no cap, the hole stands.
    Open,
    /// No hole at this key, or its boundary no longer stands: the hole, where
    /// there was one, stands.
    Absent,
}

impl Complex {
    /// A CELL THAT HOLDS ANOTHER FROM ABOVE: a coface whose fill has been
    /// read, or a hole — both carry a class, so nothing beneath either may
    /// drain, descend or be punctured (63.2, 78.99 (2)).
    pub(crate) fn holds_from_above(&self, k: &[u8]) -> bool {
        self.holes.contains(k)
            || self.degree_of(k).and_then(|d| self.degrees.get(d)).and_then(|layer| layer.get(k)).is_some_and(|s| !s.fill.unsettled())
    }

    /// The keys of the cells and holes holding every vertex of `verts` — the
    /// meet of the vertices' stars, never a read of the degrees above (rule 1).
    pub(crate) fn cofaces_of(&self, verts: &[Vec<u8>]) -> Option<Keyed<()>> {
        let mut stars = verts.iter().map(|v| self.star.get(v));
        let first = stars.next().flatten()?;
        stars.try_fold(first.clone(), |acc, s| s.map(|s| acc.meet(s)))
    }

    /// Whether a cell standing on `verts` is pinned from above by some other
    /// cell or hole.
    pub(crate) fn pinned_above(&self, key: &[u8], verts: &[Vec<u8>]) -> bool {
        self.cofaces_of(verts).is_some_and(|cofaces| cofaces.scoped().any(|(k, ())| k != key && self.holds_from_above(&k)))
    }

    /// Whether a hole stands at `key`.
    pub fn is_hole(&self, key: &[u8]) -> bool {
        self.holes.contains(key)
    }

    /// The holes standing, each with the fill it keeps.
    pub fn holes(&self) -> impl Iterator<Item = (Vec<u8>, &Fill)> {
        self.holes.scoped().map(|(k, s)| (k, &s.fill))
    }

    /// `condense` — `drain`'s INVERSE ON CONTENT (62.2): a tether between two
    /// vertices remembered with its transition, standing open, exactly as
    /// `drain` found it before it dissolved it. No strain gate stands here:
    /// whether two bodies touch is topological and the caller's reading
    /// (72.1, A.14), and the tether is what a touching pair carries. A
    /// remainder pair refuses — its transition does not realise.
    pub fn condense(&mut self, a: &[u8], b: &[u8], g: Rotor, carriage: Versor, volume: [u64; 4]) -> Condensed {
        let vertices = sorted_verts([a, b]);
        if vertices.len() != 2 {
            return Condensed::Absent;
        }
        if self.is_remainder(&vertices) {
            return Condensed::Remainder;
        }
        if let Some(standing) = self.get(&vertices).map(|s| s.motion()) {
            return self.collide(&vertices, a, g, carriage, &standing);
        }
        self.set_edge(a, b, g, carriage, volume);
        // THE TETHER'S STRAIN WINDS INTO THE FIELD as it condenses (A.8): the
        // life of a tether balances as a membrane's does — condense winds,
        // drain unwinds — and its line threads what its modes land on.
        let Some(motion) = self.get(&vertices).map(|s| s.motion()) else {
            return Condensed::Absent;
        };
        self.flow_out(Flow { motion, line: Thread::Wound(simplex_key(&vertices).as_bytes().to_vec()) });
        Condensed::Tethered
    }

    /// TWO TRANSPORTS MEET ON ONE PAIR (MU-CLIENT §4.5): the arriving one
    /// oriented as the standing tether is stored (from its lesser vertex) and
    /// read as a motion as the tether's is, `C = old⁻¹ · new` (the reverse is
    /// the inverse up to its weight, and a motion is read projectively). The
    /// fibre flexes to the arriving transport unless a hole pins it; `C` is
    /// answered for its holder to read against the fibre's pivots.
    fn collide(&mut self, vertices: &[Vec<u8>], a: &[u8], g: Rotor, carriage: Versor, standing: &crate::transducer::cga::Mv) -> Condensed {
        let (r, c) = match vertices {
            [lesser, _] if lesser.as_slice() == a => (g, carriage),
            _ => (g.reverse(), carriage.reverse()),
        };
        let arriving = c.geo(&Self::store_coeff(r.clone()).lift());
        let composite = standing.reverse().geo(&arriving);
        let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        if composite.projective_eq(&one) {
            return Condensed::AlreadyTethered;
        }
        let key = simplex_key(vertices).as_bytes().to_vec();
        let cells: Vec<Vec<u8>> = match self.cofaces_of(vertices) {
            Some(c) => c.scoped().map(|(k, ())| k).filter(|k| *k != key).collect(),
            None => Vec::new(),
        };
        if cells.iter().any(|k| self.is_hole(k)) {
            return Condensed::Pinned(Box::new(composite));
        }
        // THE FLEX: the fibre carries the arriving transport, and every
        // cell sealed over it re-reads its holonomy — the membrane flexes
        // with it, its curvature changed as content.
        let Some(t) = self.stored_mut(&key) else { return Condensed::Absent };
        t.coefficient = Self::store_coeff(r);
        t.carriage = c;
        for k in cells {
            if let Some(verts) = self.vertices_at(&k) {
                let _ = self.reopen_on_coeff_change(&k, &verts);
            }
        }
        Condensed::Flexed(Box::new(composite))
    }

    /// FROZEN-IN FLUX (MU-CLIENT §4.5): the tether at `key` is CARRIED by
    /// the field — an admission moved the orbit it is wound on by
    /// `increment`, and the fibre moves with it: its transport becomes
    /// `old · increment`, split back into rotor and carriage as a residue is,
    /// and every cell over it re-reads. No strain is wound for it: the field
    /// moved the fibre, the fibre did not move the field. A hole over it pins
    /// it; a key that is no tether, or a motion that does not split, moves
    /// nothing. Answers whether it moved.
    pub fn carry(&mut self, key: &[u8], increment: &crate::transducer::cga::Mv) -> bool {
        use crate::transducer::element::{conformal_factors, rotor_of_factors};
        if self.degree_of(key) != Some(1) {
            return false;
        }
        let Some(vertices) = self.vertices_at(key) else { return false };
        let cells: Vec<Vec<u8>> = match self.cofaces_of(&vertices) {
            Some(c) => c.scoped().map(|(k, ())| k).filter(|k| k.as_slice() != key).collect(),
            None => Vec::new(),
        };
        if cells.iter().any(|k| self.is_hole(k)) {
            return false;
        }
        let Some(moved) = self.simplex(key).map(|s| s.motion().geo(increment)) else { return false };
        let Ok(f) = conformal_factors(&moved) else { return false };
        let Ok(rotor) = rotor_of_factors(&f) else { return false };
        let Some(t) = self.stored_mut(key) else { return false };
        t.coefficient = Self::store_coeff(rotor);
        t.carriage = f.carriage();
        for k in cells {
            if let Some(verts) = self.vertices_at(&k) {
                let _ = self.reopen_on_coeff_change(&k, &verts);
            }
        }
        true
    }

    /// WHETHER THE TETHER STANDING BETWEEN `a` AND `b` CARRIES THIS TRANSPORT:
    /// the rotor and carriage oriented as a condensed tether stores them,
    /// read projectively against the stored pair. `false` where no tether
    /// stands or the pair is no two vertices.
    pub fn tethered_as(&self, a: &[u8], b: &[u8], g: &Rotor, carriage: &Versor) -> bool {
        let vertices = sorted_verts([a, b]);
        let Some(s) = self.get(&vertices) else { return false };
        // The tether is stored oriented from its lesser vertex, the order
        // its key is written in.
        let (r, c) = match vertices.as_slice() {
            [lesser, _] if lesser.as_slice() == a => (g.clone(), carriage.clone()),
            _ => (g.reverse(), carriage.reverse()),
        };
        s.coefficient.projective_eq(&Self::store_coeff(r)) && crate::transducer::element::same_versor(&s.carriage, &c)
    }

    /// `puncture` — A SEALED CELL OPENED INTO A HOLE (62.1): the cell leaves
    /// the cells as `descend` releases it — its boundary standing with its
    /// transports — but its fill is KEPT, in the complex, as the hole's
    /// class: injected, never handed out, so nothing flows out. The hole
    /// stands in its vertices' stars and pins its boundary.
    pub fn puncture(&mut self, key: &[u8]) -> Punctured {
        if self.holes.contains(key) {
            return Punctured::Hole;
        }
        let Some(degree) = self.degree_of(key).filter(|d| *d >= 2) else {
            return Punctured::Absent;
        };
        let Some(simplex) = self.degrees[degree].get(key).cloned() else {
            return Punctured::Absent;
        };
        if self.pinned_above(key, &simplex.vertices) {
            return Punctured::Pinned;
        }
        // AN UNSETTLED CELL RECORDED NO CLASS — open, or its reading unread —
        // so there is none to keep (rule 11): nothing is injected.
        if simplex.fill.unsettled() {
            return Punctured::Unsealed;
        }
        let fill = simplex.fill.clone();
        self.forget(key);
        for v in &simplex.vertices {
            if let Some(star) = self.star.get_mut(v) {
                star.insert(key, ());
            }
        }
        self.holes.insert(key, simplex);
        Punctured::Opened(fill)
    }

    /// `seal` — THE HOLE CAPPED (62.1, 63.3), `puncture`'s inverse on
    /// content: the cell re-ascended over the hole's standing boundary by
    /// the one `ascend` (its volume read by `volume_of`, never written).
    /// Where the cap seals, the hole and its class go and the cap's fill is
    /// recorded; where the boundary does not close, or no longer stands,
    /// the seal refuses and the hole stands as it was.
    pub fn seal(&mut self, key: &[u8], volume_of: impl FnOnce(&[Vec<u8>]) -> [u64; 4]) -> Capped {
        let Some(hole) = self.holes.remove(key) else {
            return Capped::Absent;
        };
        for v in &hole.vertices {
            if let Some(star) = self.star.get_mut(v) {
                star.remove(key);
            }
        }
        let outcome = self.ascend(&hole.vertices, volume_of);
        let restore = |cx: &mut Complex, hole: Simplex| {
            for v in &hole.vertices {
                if let Some(star) = cx.star.get_mut(v) {
                    star.insert(key, ());
                }
            }
            cx.holes.insert(key, hole);
        };
        match outcome {
            Ascended::Sealed(cap) => {
                // D2 — ENTROPY AT DIMENSION 2: the class the hole kept is
                // killed here and nowhere else (the cross keeps it), counted
                // apart from the dimension-1 cauterization.
                crate::physics::metabolism::note_killed();
                Capped::Sealed { cap, killed: hole.fill }
            }
            Ascended::Open => {
                // The cap was minted and did not close: it goes, and the hole
                // stands as it was.
                self.forget(key);
                restore(self, hole);
                Capped::Open
            }
            Ascended::AlreadySealed | Ascended::Absent | Ascended::Hole => {
                restore(self, hole);
                Capped::Absent
            }
        }
    }

    /// The holes standing on `vertex` — a peel of its star, never a read of
    /// every hole (rule 1).
    pub fn holes_on(&self, vertex: &[u8]) -> Vec<Vec<u8>> {
        self.star.link(vertex).filter(|(k, _)| self.holes.contains(k)).map(|(k, _)| k).collect()
    }

    /// The volume a hole was read with when its cell was minted — what its
    /// cap is re-ascended at (read, never written); `None` where no hole
    /// stands at `key`.
    pub fn hole_volume(&self, key: &[u8]) -> Option<[u64; 4]> {
        self.holes.get(key).map(|h| h.volume)
    }

    /// `cauterize` — THE HOLE CLOSED WHERE ITS CLASS CANNOT BE READ BACK (the
    /// eight-component cross, ruled 2026-10-20): `seal`'s conjugate by heat.
    /// The content was handed down at the throat, so the field already
    /// carries the strain and dissipates what never returns (its beat, the
    /// reservoir's heat); here the class is burned — a cauterization at
    /// dimension two, counted as `drain`'s is at dimension one — the hole
    /// leaves its vertices' stars, unpinning its boundary and its vertices,
    /// and the scar is recorded on this complex. Never silently capped:
    /// the burned class is answered to the caller.
    pub fn cauterize(&mut self, key: &[u8]) -> Cauterized {
        let Some(hole) = self.holes.remove(key) else {
            return Cauterized::Absent;
        };
        for v in &hole.vertices {
            if let Some(star) = self.star.get_mut(v) {
                star.remove(key);
            }
        }
        crate::physics::metabolism::note_cauterized();
        self.scars += 1;
        Cauterized::Scarred(hole.fill)
    }

    /// `freeze` — THE SEAL BY COLD, the vacuum's face of the throat
    /// (ruled 2026-10-21: heat belongs to the continuum and
    /// its inverse is the vacuum's cold; the vacuum holds the frozen
    /// class). Where a hole's class cannot be read back while its vertices
    /// still stand — its boundary condensed, no longer closing as one — the
    /// class is FROZEN IN PLACE: the hole leaves its vertices' stars,
    /// unpinning its boundary, and the vacuum holds the class whole at its
    /// coordinate. Nothing is burned, so no entropy is booked; what is held
    /// may return (`thaw`).
    pub fn freeze(&mut self, key: &[u8]) -> Frozen {
        let Some(hole) = self.holes.remove(key) else {
            return Frozen::Absent;
        };
        for v in &hole.vertices {
            if let Some(star) = self.star.get_mut(v) {
                star.remove(key);
            }
        }
        let fill = hole.fill.clone();
        for v in &hole.vertices {
            self.frozen_on.insert(frozen_on_key(v, key), ());
        }
        self.frozen.insert(key, hole);
        Frozen::Held(fill)
    }

    /// The frozen classes standing on `v`: one keyed fibre, never a read of
    /// the vacuum.
    pub fn frozen_on(&self, v: &[u8]) -> Vec<Vec<u8>> {
        let mut prefix = Vec::new();
        crate::transducer::address::put_key_bytes(&mut prefix, v);
        match self.frozen_on.fibre_at(&prefix) {
            Some(f) => f.filter_map(|(k, ())| k.get(prefix.len()..).map(<[u8]>::to_vec)).collect(),
            None => Vec::new(),
        }
    }

    /// A frozen class leaves the vacuum: its entry and its vertices' index.
    fn unfreeze(&mut self, key: &[u8]) -> Option<Simplex> {
        let held = self.frozen.remove(key)?;
        for v in &held.vertices {
            self.frozen_on.remove(&frozen_on_key(v, key));
        }
        Some(held)
    }

    /// FROM COLD TO HEAT (ruled 2026-10-21): a frozen class
    /// one of whose vertices is shed can no longer thaw, so it is
    /// cauterized — the class burned, booked as a cauterization, its scar
    /// counted — and the vacuum lets it go. The vacuum is thereby bounded by
    /// the field: it holds a class only while every vertex it names stands.
    pub fn cauterize_frozen(&mut self, key: &[u8]) -> Cauterized {
        let Some(held) = self.unfreeze(key) else {
            return Cauterized::Absent;
        };
        crate::physics::metabolism::note_cauterized();
        self.scars += 1;
        Cauterized::Scarred(held.fill)
    }

    /// `thaw` — THE COLD SEAL'S INVERSE: a frozen class re-ascended over its
    /// standing boundary by the one `ascend`, at the volume it was read with
    /// (read, never written). Where the boundary closes again the chamber
    /// returns and the vacuum lets the class go; where it does not, the class
    /// stays frozen and nothing is minted.
    pub fn thaw(&mut self, key: &[u8]) -> Thawed {
        let Some(held) = self.frozen.get(key).map(|s| (s.vertices.clone(), s.volume)) else {
            return Thawed::Absent;
        };
        let (vertices, volume) = held;
        match self.ascend(&vertices, |_| volume) {
            Ascended::Sealed(fill) => {
                let _ = self.unfreeze(key);
                Thawed::Sealed(fill)
            }
            Ascended::Open => {
                // The cap was minted and did not close: it goes.
                self.forget(key);
                Thawed::Held
            }
            Ascended::AlreadySealed | Ascended::Absent | Ascended::Hole => Thawed::Held,
        }
    }
}
