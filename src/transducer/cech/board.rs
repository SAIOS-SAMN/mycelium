// 78.99 (3), 62.4, 63.2 — THE BOARD: a complex's coordinates on the three
// pivot axes, and the locality theorem that makes a walk along them exact in
// any order where its pivots share no vertex.
//
//! THE BOARD IS THE CONSTRUCTION'S COORDINATES, read off the complex and
//! never a frame laid on it: on the axis `ascend / descend`, the sealed cells
//! by key and fill (and the cells standing open); on `condense / drain`, the
//! tethers by key; on `puncture / seal`, the holes by key and the class each
//! keeps. Two walks are CONGRUENT where they leave one board.
//!
//! THE LOCALITY THEOREM. A pivot writes only its own cell and its vertices'
//! stars (`remember` / `forget`), and reads only cells holding its vertices
//! (the pin's keyed meet) or lying among them (`ascend`'s faces). Two pivots
//! on cells sharing no vertex therefore touch disjoint carriers and commute
//! exactly — the same board, the same outflow as a multiset — in any order,
//! with nothing enumerated ([`independent`]). Where two pivots share a vertex
//! the order is content: the boards differ by the coordinate that moved, and
//! a pivot one order refuses is a reading of that order, never an error.

use super::*;

// THE CONSTRUCTION'S PROCESS-WIDE TALLY WAS HERE AND IS DELETED (2026-10-11):
// a static counted every mind's punctures, caps and climbs into one number
// per slot, kept for a console to watch — a watch-tally across fields that
// are each a mind's own (MU-CLIENT §4.2). What a pivot did is its outcome,
// returned whole to the field that applied it.

/// ONE PIVOT AT ITS COORDINATE: the cell (or hole, or pair) it acts on.
#[derive(Clone, Debug)]
pub enum Pivot {
    Ascend(Vec<Vec<u8>>),
    /// Algebraic and Clifford descent: the exact conjugate dual of
    /// `Ascend` — the chamber projected back to its ring — never numerical
    /// gradient optimization (`Complex::descend`).
    Descend(Vec<u8>),
    Condense(Vec<u8>, Vec<u8>, Box<Rotor>, Versor),
    Drain(Vec<u8>),
    Puncture(Vec<u8>),
    Seal(Vec<u8>),
}

/// WHAT ONE PIVOT DID, each pivot's own outcome kept whole.
#[derive(Clone, Debug)]
pub enum Applied {
    Ascended(Ascended),
    Descended(Descended),
    Condensed(Condensed),
    Drained(Drained),
    Punctured(Punctured),
    Capped(Capped),
}

/// A COMPLEX'S COORDINATES ON THE THREE AXES, each in key order (law 2).
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    /// `ascend / descend`: the cells of degree two or more that recorded a
    /// reading, with it.
    pub sealed: Vec<(Vec<u8>, Fill)>,
    /// The cells of degree two or more standing unsettled.
    pub open: Vec<Vec<u8>>,
    /// `condense / drain`: the tethers.
    pub tethers: Vec<Vec<u8>>,
    /// `puncture / seal`: the holes, with the class each keeps.
    pub holes: Vec<(Vec<u8>, Fill)>,
    /// `puncture / cauterize`: the scars — holes closed by heat whose class
    /// could not be read back.
    pub scars: u64,
    /// `puncture / freeze`: the classes the vacuum holds, frozen in place.
    pub frozen: Vec<(Vec<u8>, Fill)>,
}

impl Pivot {
    /// The vertices of the cell this pivot acts at, read on `cx` — a key
    /// names a cell or a hole there; `None` where it names neither.
    pub fn vertices(&self, cx: &Complex) -> Option<Vec<Vec<u8>>> {
        match self {
            Pivot::Ascend(vs) => Some(sorted_verts(vs.clone())),
            Pivot::Condense(a, b, _, _) => Some(sorted_verts([a.as_slice(), b.as_slice()])),
            Pivot::Descend(k) | Pivot::Drain(k) | Pivot::Puncture(k) | Pivot::Seal(k) => cx.vertices_at(k),
        }
    }
}

/// THE LOCALITY THEOREM'S CONDITION: two pivots whose cells share no vertex
/// are independent and commute exactly.
pub fn independent(x: &[Vec<u8>], y: &[Vec<u8>]) -> bool {
    x.iter().all(|v| !y.contains(v))
}

impl Complex {
    /// The vertices of the cell or hole at `key`.
    pub fn vertices_at(&self, key: &[u8]) -> Option<Vec<Vec<u8>>> {
        match self.simplex(key) {
            Some(s) => Some(s.vertices.clone()),
            None => self.holes.get(key).map(|h| h.vertices.clone()),
        }
    }

    /// THE BOARD: this complex's coordinates on the three axes.
    pub fn board(&self) -> Board {
        let mut b = Board {
            sealed: Vec::new(),
            open: Vec::new(),
            tethers: Vec::new(),
            holes: Vec::new(),
            scars: self.scars,
            frozen: self.frozen.scoped().map(|(k, s)| (k, s.fill.clone())).collect(),
        };
        for (k, d) in self.by_key.scoped() {
            let Some(s) = self.degrees.get(*d).and_then(|l| l.get(&k)) else { continue };
            match (*d, s.fill.unsettled()) {
                (0, _) => {}
                (1, _) => b.tethers.push(k),
                (_, true) => b.open.push(k),
                (_, false) => b.sealed.push((k, s.fill.clone())),
            }
        }
        for (k, h) in self.holes.scoped() {
            b.holes.push((k, h.fill.clone()));
        }
        b
    }

    /// ONE PIVOT APPLIED AT ITS COORDINATE; `volume_of` reads a cell's volume
    /// where the pivot mints one (`ascend`, `seal`), a tether's volume is the
    /// condensing caller's.
    pub fn apply(&mut self, pivot: &Pivot, volume_of: impl FnOnce(&[Vec<u8>]) -> [u64; 4], tether: [u64; 4]) -> Applied {
        match pivot {
            Pivot::Ascend(vs) => Applied::Ascended(self.ascend(vs, volume_of)),
            Pivot::Descend(k) => Applied::Descended(self.descend(k)),
            Pivot::Condense(a, b, g, c) => Applied::Condensed(self.condense(a, b, g.as_ref().clone(), c.clone(), tether)),
            Pivot::Drain(k) => Applied::Drained(self.drain(k)),
            Pivot::Puncture(k) => Applied::Punctured(self.puncture(k)),
            Pivot::Seal(k) => Applied::Capped(self.seal(k, volume_of)),
        }
    }
}
