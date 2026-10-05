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
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// 78.99 (2), (4) — THE CONSTRUCTION'S TALLY'S NAMES, in
/// [`construction_tally`]'s order: the holes the pivots opened and capped,
/// and what the connecting map read on each glue. The typed reading the
/// voice's Senses carry.
pub const CONSTRUCTION: [&str; 11] = [
    "punctured",
    "capped",
    "cap refused",
    "glues read",
    "classes made",
    "loops flat",
    "holes capped by a glue",
    "glue: no overlap",
    "glue: inexact",
    "glue: a part curved",
    "glue: unread",
];
pub(crate) const PUNCTURED: usize = 0;
pub(crate) const CAPPED: usize = 1;
pub(crate) const CAP_REFUSED: usize = 2;
pub(crate) const GLUES: usize = 3;
pub(crate) const MADE: usize = 4;
pub(crate) const FLAT: usize = 5;
pub(crate) const GLUE_CAPPED: usize = 6;
pub(crate) const NO_OVERLAP: usize = 7;
pub(crate) const INEXACT: usize = 8;
pub(crate) const PART_CURVED: usize = 9;
pub(crate) const GLUE_UNREAD: usize = 10;

static TALLY: [AtomicU64; CONSTRUCTION.len()] = [const { AtomicU64::new(0) }; CONSTRUCTION.len()];

/// One step of a slot of the construction's tally, refused and counted on
/// the metabolism's ledger where it would pass its carrier.
pub fn note_construction(slot: usize, by: u64) {
    let Some(at) = TALLY.get(slot) else { return };
    if by > 0 && at.fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// The construction's tally since the process began, in [`CONSTRUCTION`]'s
/// order.
pub fn construction_tally() -> [u64; CONSTRUCTION.len()] {
    std::array::from_fn(|i| TALLY[i].load(Relaxed))
}

/// ONE PIVOT AT ITS COORDINATE: the cell (or hole, or pair) it acts on.
#[derive(Clone, Debug)]
pub enum Pivot {
    Ascend(Vec<Vec<u8>>),
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

impl Applied {
    /// Whether the pivot was refused — pinned, at a hole, absent, open where
    /// a seal was asked: a reading of its order, never an error.
    pub fn refused(&self) -> bool {
        match self {
            Applied::Ascended(a) => !matches!(a, Ascended::Sealed(_) | Ascended::Open),
            Applied::Descended(d) => !matches!(d, Descended::Released(_) | Descended::Unsealed),
            Applied::Condensed(c) => !matches!(c, Condensed::Tethered),
            Applied::Drained(d) => !matches!(d, Drained::Vacuum | Drained::Cauterized(_)),
            Applied::Punctured(p) => !matches!(p, Punctured::Opened(_)),
            Applied::Capped(c) => !matches!(c, Capped::Sealed { .. }),
        }
    }
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
        let mut b = Board { sealed: Vec::new(), open: Vec::new(), tethers: Vec::new(), holes: Vec::new() };
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
