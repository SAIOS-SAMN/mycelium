// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Class vector, cycle type, rank, and axis.

use super::transport::{gauge_image, gauge_keyed, hue_key};
use crate::core::zlinalg::z_rank;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Word};

/// declaration order is the walk order and is not a preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Sheet(u8),
    Unit(u8),
    Prime5(u8),
    Prime13(u8),
    Hue,
    Placement,
    OffPlane,
    Open,
}

impl Axis {
    /// One tape byte: the kind in the high bits, the slot plane in the low.
    pub(crate) fn to_byte(self) -> u8 {
        match self {
            Axis::Sheet(p) => p,
            Axis::Unit(p) => (1 << 3) | p,
            Axis::Prime5(p) => (2 << 3) | p,
            Axis::Prime13(p) => (3 << 3) | p,
            Axis::Hue => 4 << 3,
            Axis::Placement => 5 << 3,
            Axis::OffPlane => 6 << 3,
            Axis::Open => 7 << 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Axis::Sheet(_) => "sheet",
            Axis::Unit(_) => "unit",
            Axis::Prime5(_) => "prime5",
            Axis::Prime13(_) => "prime13",
            Axis::Hue => "hue",
            Axis::Placement => "placement",
            Axis::OffPlane => "offplane",
            Axis::Open => "open",
        }
    }

    pub(crate) fn from_byte(b: u8) -> Option<Axis> {
        let p = b & 7;
        match (b >> 3, p) {
            (0, p) if p < 6 => Some(Axis::Sheet(p)),
            (1, p) if p < 6 => Some(Axis::Unit(p)),
            (2, p) if p < 6 => Some(Axis::Prime5(p)),
            (3, p) if p < 6 => Some(Axis::Prime13(p)),
            (4, 0) => Some(Axis::Hue),
            (5, 0) => Some(Axis::Placement),
            (6, 0) => Some(Axis::OffPlane),
            (7, 0) => Some(Axis::Open),
            _ => None,
        }
    }
}

/// A18: the dimension audit of a nerve or form, a vector never summed.
/// `torsion` counts the open finite-order loops; `free` is the ℤ-rank of
/// the free parts of the open plane loops plus the open off-plane loops;
/// `axes` the chart axes opened by discharge (none on a nerve, whose atlas
/// is the contemplation's).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rank {
    pub torsion: u32,
    pub free: u32,
    pub axes: u32,
}

impl Rank {
    /// A pivot is admissible when it leaves rank equal or lowers `torsion`
    /// or `free` while `axes` does not fall (12.6, R2).
    pub fn admits(before: Rank, after: Rank) -> bool {
        after.torsion <= before.torsion && after.free <= before.free && after.axes >= before.axes
    }

    pub(crate) fn to_bytes(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.torsion.to_le_bytes());
        out.extend_from_slice(&self.free.to_le_bytes());
        out.extend_from_slice(&self.axes.to_le_bytes());
    }

    pub(crate) fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rank> {
        let mut f = [0u32; 3];
        for x in &mut f {
            *x = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
        }
        Some(Rank {
            torsion: f[0],
            free: f[1],
            axes: f[2],
        })
    }
}

/// The state vector of a held loop (12.6), read from the canonical conjugacy
/// representative of its holonomy's plane form (15.2) and from the
/// translation part of its versor. Every coordinate is an exact integer or
/// rational; an axis is occupied when its coordinate is nonzero, and a pivot
/// is tried only on an occupied axis. The hue cycle type is not on a nerve's
/// faces and is read on the form (chapter 30).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassVector {
    /// Per slot plane, in `SLOT_PLANES_SIX` order: sheet parity, unit turns
    /// in Z/4, prime-5 and prime-13 exponents in Z.
    plane: [(bool, u32, i64, i64); 6],
    /// Translation part of the loop's versor after the tree gauge.
    pub(crate) placement: (num_rational::BigRational, num_rational::BigRational),
    /// Composed gauge around the loop (A4).
    pub(crate) hue: CycleType,
    /// The holonomy has no plane form: the class is the rotor's invariants.
    offplane: bool,
    key: [u8; 32],
}

/// The class key of a closed loop: the identity's, written as zero (A20).
pub const CLOSED_KEY: [u8; 32] = [0u8; 32];

/// The hue cycle type of a loop: sorted cycle lengths of the composed
/// permutation around its vertices. The listing itself lives on
/// `PivotBy::Hue`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CycleType {
    Identity,
    Type(Vec<u8>),
}

impl CycleType {
    pub(crate) fn of(g: &[(u8, u8)]) -> CycleType {
        let mut lengths = cycle_lengths(g);
        lengths.sort_unstable();
        if lengths.is_empty() {
            CycleType::Identity
        } else {
            CycleType::Type(lengths)
        }
    }

    fn to_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            CycleType::Identity => buf.push(0),
            CycleType::Type(lens) => {
                buf.push(lens.len() as u8);
                buf.extend_from_slice(lens);
            }
        }
    }
}

/// Cycle lengths greater than 1 of a hue listing. Absence is identity, so
/// a cycle is the orbit under the listed map until it returns.
fn cycle_lengths(listing: &[(u8, u8)]) -> Vec<u8> {
    let keyed = gauge_keyed(listing);
    let mut seen: Keyed<()> = Keyed::new();
    let mut lengths = Vec::new();
    for &(start, _) in listing {
        if seen.contains(&hue_key(start)) {
            continue;
        }
        let mut len = 0u8;
        let mut h = start;
        loop {
            if seen.contains(&hue_key(h)) {
                break;
            }
            seen.insert(hue_key(h), ());
            len = len.saturating_add(1);
            let next = gauge_image(&keyed, h);
            if next == h {
                break;
            }
            h = next;
            if h == start {
                if len > 1 {
                    lengths.push(len);
                }
                break;
            }
        }
    }
    lengths
}

impl ClassVector {
    /// The vector of a holonomy word alone (no versor: zero residue).
    pub fn of_word(h: &Word) -> ClassVector {
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        Self::read(h, (zero.clone(), zero), CycleType::Identity)
    }

    pub(crate) fn read(
        h: &Word,
        placement: (num_rational::BigRational, num_rational::BigRational),
        hue: CycleType,
    ) -> ClassVector {
        let (plane, offplane) = if h.is_identity() {
            ([(false, 0, 0, 0); 6], false)
        } else {
            match h.form() {
                Some(pf) => (pf.coordinates(), false),
                None => ([(false, 0, 0, 0); 6], true),
            }
        };
        let mut cv = ClassVector {
            plane,
            placement,
            hue,
            offplane,
            key: CLOSED_KEY,
        };
        cv.key = cv.compute_key(h);
        cv
    }

    /// `blake3(rotor conjugacy key ‖ cycle type ‖ put_q placement)`.
    /// `CLOSED_KEY` iff every coordinate is zero.
    fn compute_key(&self, h: &Word) -> [u8; 32] {
        if self.coordinates().is_empty() {
            return CLOSED_KEY;
        }
        let mut buf = Vec::new();
        buf.extend_from_slice(&h.rotor().class_key());
        self.hue.to_bytes(&mut buf);
        alphabet::put_q(&mut buf, &self.placement.0);
        alphabet::put_q(&mut buf, &self.placement.1);
        *blake3::hash(&buf).as_bytes()
    }

    /// The coordinate an axis names, as "nonzero". `Open` is never a
    /// coordinate: it is the last move, admitted when no coordinate's own
    /// pivot discharged.
    pub fn occupies(&self, axis: Axis) -> bool {
        match axis {
            Axis::Sheet(p) => self.plane.get(p as usize).is_some_and(|s| s.0),
            Axis::Unit(p) => self.plane.get(p as usize).is_some_and(|s| s.1 != 0),
            Axis::Prime5(p) => self.plane.get(p as usize).is_some_and(|s| s.2 != 0),
            Axis::Prime13(p) => self.plane.get(p as usize).is_some_and(|s| s.3 != 0),
            Axis::Hue => !matches!(self.hue, CycleType::Identity),
            Axis::Placement => {
                let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
                self.placement.0 != zero || self.placement.1 != zero
            }
            Axis::OffPlane => self.offplane,
            Axis::Open => false,
        }
    }

    /// The occupied coordinates in the fixed order of 12.6.
    pub fn coordinates(&self) -> Vec<Axis> {
        let mut out = Vec::new();
        for p in 0..6u8 {
            for axis in [Axis::Sheet(p), Axis::Unit(p), Axis::Prime5(p), Axis::Prime13(p)] {
                if self.occupies(axis) {
                    out.push(axis);
                }
            }
        }
        for axis in [Axis::Hue, Axis::Placement, Axis::OffPlane] {
            if self.occupies(axis) {
                out.push(axis);
            }
        }
        out
    }

    /// The occupied axes: the coordinates, then `Open` last whenever the
    /// loop is held at all (the one move that is not a coordinate).
    pub fn occupied(&self) -> Vec<Axis> {
        let mut out = self.coordinates();
        if !out.is_empty() {
            out.push(Axis::Open);
        }
        out
    }

    /// Every coordinate other than `axis` agrees: the discharge condition of
    /// 12.6 (a coordinate may fall to zero, none may rise or move).
    pub fn agrees_except(&self, other: &ClassVector, axis: Axis) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        a.clear(axis);
        b.clear(axis);
        a.plane == b.plane
            && a.placement == b.placement
            && a.hue == b.hue
            && a.offplane == b.offplane
    }

    fn clear(&mut self, axis: Axis) {
        match axis {
            Axis::Sheet(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.0 = false;
                }
            }
            Axis::Unit(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.1 = 0;
                }
            }
            Axis::Prime5(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.2 = 0;
                }
            }
            Axis::Prime13(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.3 = 0;
                }
            }
            Axis::Placement => {
                let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
                self.placement = (zero.clone(), zero);
            }
            Axis::OffPlane => self.offplane = false,
            Axis::Hue => self.hue = CycleType::Identity,
            Axis::Open => {}
        }
    }

    pub fn class_key(&self) -> [u8; 32] {
        self.key
    }

    /// No coordinate is occupied: the loop is closed.
    pub fn is_zero(&self) -> bool {
        self.occupied().is_empty()
    }

    /// The free part: a prime exponent, or an off-plane class (infinite
    /// order). A loop with a free part is a demand, never filed torsion.
    pub fn free_part(&self) -> bool {
        self.offplane || self.plane.iter().any(|s| s.2 != 0 || s.3 != 0)
    }
}

/// The free part of a holonomy as one row over ℤ: (m5, m13) per plane.
fn free_row(h: &Word) -> Option<[i64; 12]> {
    let rep = h.form()?.conjugacy_rep();
    let mut row = [0i64; 12];
    for p in 0..6 {
        row[2 * p] = rep.rot[7 + p];
        row[2 * p + 1] = match (p < 4, rep.rot.get(13 + p)) {
            (true, Some(&n)) => n,
            _ => 0,
        };
    }
    Some(row)
}

/// What one loop contributes to rank (A18): read once per loop, so a trial
/// that changes one loop's holonomy swaps one row instead of re-reading the
/// nerve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoopRow {
    Closed,
    Torsion,
    Free([i64; 12]),
    OffPlane,
}

pub fn loop_row(h: &Word) -> LoopRow {
    if h.is_identity() {
        return LoopRow::Closed;
    }
    match free_row(h) {
        Some(row) if row == [0i64; 12] => LoopRow::Torsion,
        Some(row) => LoopRow::Free(row),
        None => LoopRow::OffPlane,
    }
}

/// A18 over the rows of a nerve's cycles or a form's loops.
pub fn rank_of_rows(rows: &[LoopRow]) -> Option<Rank> {
    let mut torsion = 0u32;
    let mut offplane = 0u32;
    let mut free: Vec<[i64; 12]> = Vec::new();
    for r in rows {
        match r {
            LoopRow::Closed => {}
            LoopRow::Torsion => torsion += 1,
            LoopRow::Free(row) => free.push(*row),
            LoopRow::OffPlane => offplane += 1,
        }
    }
    Some(Rank {
        torsion,
        free: z_rank(&free)?.saturating_add(offplane),
        axes: 0,
    })
}

pub fn rank_of_holonomies<'a>(holonomies: impl Iterator<Item = &'a Word>) -> Option<Rank> {
    let rows: Vec<LoopRow> = holonomies.map(loop_row).collect();
    rank_of_rows(&rows)
}
