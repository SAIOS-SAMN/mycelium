// 78.99 (2), 62.1, 63.3 — THE PIVOTS COMPLETED ON THE COMPLEX: `condense`,
// `drain`'s inverse on content; `puncture`, a sealed cell opened into a HOLE
// that keeps its class; `seal`, the hole capped.
//
//! THE LADDER, WHOLE (63.3's table, every row built):
//!
//! ```text
//!     dim 2   ascend   ───►  seal a cycle with a cell          coboundary.rs
//!             descend  ◄───  release it, the class OUT         coboundary.rs
//!     dim 1   condense ───►  remember a tether                 here
//!             drain    ◄───  dissolve it, cauterized OUT       coboundary.rs
//!     hole    puncture ───►  open a cell, the class kept IN    here
//!             seal     ◄───  cap the hole, the class killed    here
//! ```
//!
//! `descend` AND `puncture` BOTH OPEN THE BOUNDARY CYCLE, and they are two
//! facts: `descend` hands the class out to the owner's field, `puncture`
//! keeps it in the complex as a hole the boundary winds about. Only `seal`
//! caps a hole. A HOLE PINS ITS BOUNDARY — it carries a read class, as a
//! read coface does (63.2) — and at a hole `descend`, `ascend` and
//! `puncture` refuse by name (78.99 (2)).
//!
//! HOLES ARE VOLATILE: no persisted complex punctures yet, so the tries
//! (`persist.rs`) carry none; the first caller on a persisted complex brings
//! their trie with it.

use super::*;

/// WHAT A `condense` DID — [`Drained`]'s inverse — and, where a tether
/// already stands on the pair, how the FIBRE FLEXED (MU-CLIENT §4.5): the
/// two transports' composite `C = old⁻¹ · new` read by its modes against
/// the tether's own axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condensed {
    /// The tether is remembered with its transition, standing open.
    Tethered,
    /// A tether already stands carrying this transport: nothing changed.
    AlreadyTethered,
    /// THE FIBRE FLEXED by `C`: the tether now carries the arriving
    /// transport (`old · C`), every cell over it re-reads, and `C`'s strain
    /// winds into the field on the tether's own line — where a stretch's
    /// boost never returns and dissipates on a beat, the line is released
    /// and the tether drains: it BREAKS, as the field's event.
    Flexed(Flex),
    /// A HOLE over the tether — a question the mind still carries — holds
    /// it: it does not flex, and `C` goes to the field free.
    Pinned(Flex),
    /// `C`'s modes did not read: the tether stands, nothing goes out.
    CollisionUnread,
    /// The pair is a remainder — a transition that does not realise — and
    /// no tether condenses there.
    Remainder,
    /// Not two distinct vertices.
    Absent,
}

/// HOW A FIBRE FLEXED (MU-CLIENT §4.5): `C`'s modes against the tether's own
/// axis, each counted by what it does to the fibre — STRETCH (a boost, 𝒮),
/// BEND (a turn on a plane off the tether's own), TWIST (a turn commuting
/// with the tether's own plane, about its axis), FOLD (a turn of order two:
/// the fibre turned back on itself), SLIDE (a null mode, 𝒯) — and the
/// period the flex returns at, where every mode turns at a finite order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flex {
    pub stretch: u8,
    pub bend: u8,
    pub twist: u8,
    pub fold: u8,
    pub slide: u8,
    pub period: Option<u64>,
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
    /// meet of their stars, never a read of the degrees above (rule 1).
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
    /// the inverse up to its weight, and a motion is read projectively), and
    /// `C` read by its modes on the stroke they need.
    fn collide(&mut self, vertices: &[Vec<u8>], a: &[u8], g: Rotor, carriage: Versor, standing: &crate::transducer::cga::Mv) -> Condensed {
        use crate::network::nerve::coord::BivectorKind;
        use crate::transducer::cga::{modes_need, motion_modes, period_of, Period};
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
        let Some(need) = modes_need(&composite) else { return Condensed::CollisionUnread };
        let mut stroke = crate::physics::Physics::default().stroke_n(need);
        let Some(Ok(modes)) = motion_modes(&composite, &mut stroke) else { return Condensed::CollisionUnread };
        // Each mode against the tether's own plane: a turn whose plane
        // commutes with it turns about the tether's axis (a twist); any other
        // bends it. An identity tether has no axis, and every turn bends it.
        let axis = standing.grade(2);
        let mut flex = Flex {
            period: match period_of(&modes) {
                Some(Period::Closes(order)) => Some(order),
                _ => None,
            },
            ..Flex::default()
        };
        for m in &modes {
            let count = match m.kind {
                BivectorKind::Hyperbolic => &mut flex.stretch,
                BivectorKind::Parabolic => &mut flex.slide,
                BivectorKind::Trivial => continue,
                BivectorKind::Elliptic if m.order == Some(2) => &mut flex.fold,
                BivectorKind::Elliptic => {
                    let plane = m.ray.grade(2);
                    match !axis.is_zero() && plane.geo(&axis).sub(&axis.geo(&plane)).is_zero() {
                        true => &mut flex.twist,
                        false => &mut flex.bend,
                    }
                }
            };
            if let Some(n) = count.checked_add(1) {
                *count = n;
            }
        }
        let key = simplex_key(vertices).as_bytes().to_vec();
        let cells: Vec<Vec<u8>> = match self.cofaces_of(vertices) {
            Some(c) => c.scoped().map(|(k, ())| k).filter(|k| *k != key).collect(),
            None => Vec::new(),
        };
        if cells.iter().any(|k| self.is_hole(k)) {
            self.flow_out(Flow { motion: composite, line: Thread::Free });
            return Condensed::Pinned(flex);
        }
        // THE FLEX: the fibre carries the arriving transport, and every
        // cell sealed over it re-reads its holonomy — the membrane flexes
        // with it, its curvature changed as content.
        let Some(t) = self.stored_mut(&key) else { return Condensed::CollisionUnread };
        t.coefficient = Self::store_coeff(r);
        t.carriage = c;
        for k in cells {
            if let Some(verts) = self.vertices_at(&k) {
                let _ = self.reopen_on_coeff_change(&k, &verts);
            }
        }
        self.flow_out(Flow { motion: composite, line: Thread::Wound(key) });
        Condensed::Flexed(flex)
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
            Ascended::Sealed(cap) => Capped::Sealed { cap, killed: hole.fill },
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

    /// 78.99 (2) — THE HOLES ON A VERTEX, CAPPED before it is released: each
    /// hole re-ascended over its standing boundary at the volume it was read
    /// with when its cell was minted (read, never written), so the class it
    /// kept returns to a cell and the release hands it out by `descend`. A
    /// hole whose cap does not close stands and pins, and the vertex with it.
    pub(crate) fn seal_holes_on(&mut self, v: &[u8]) {
        let holes: Vec<(Vec<u8>, [u64; 4])> = match self.star.get(v) {
            Some(star) => star.scoped().filter_map(|(k, ())| self.holes.get(&k).map(|h| (k, h.volume))).collect(),
            None => Vec::new(),
        };
        for (k, volume) in holes {
            let _ = self.seal(&k, |_| volume);
        }
    }
}
