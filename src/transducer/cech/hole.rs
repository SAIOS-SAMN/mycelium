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

/// WHAT A `condense` DID — [`Drained`]'s inverse, four facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condensed {
    /// The tether is remembered with its transition, standing open.
    Tethered,
    /// A tether already stands between the two: nothing changed.
    AlreadyTethered,
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
        if self.get(&vertices).is_some() {
            return Condensed::AlreadyTethered;
        }
        self.set_edge(a, b, g, carriage, volume);
        match self.get(&vertices) {
            Some(_) => Condensed::Tethered,
            None => Condensed::Absent,
        }
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
}
