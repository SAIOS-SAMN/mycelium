// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Class vector, cycle type, rank, and axis.

use super::transport::{gauge_image, gauge_keyed, hue_key};
use crate::core::zlinalg::z_rank;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Versor};

/// declaration order is the walk order and is not a preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Orientation(u8),
    Unit(u8),
    Prime5(u8),
    Prime13(u8),
    Hue,
    Placement,
    Dilation,
    OffPlane,
    Open,
}

impl Axis {
    /// One tape byte: the kind in the high bits, the slot plane in the low.
    pub(crate) fn to_byte(self) -> u8 {
        match self {
            Axis::Orientation(p) => p,
            Axis::Unit(p) => (1 << 3) | p,
            Axis::Prime5(p) => (2 << 3) | p,
            Axis::Prime13(p) => (3 << 3) | p,
            Axis::Hue => 4 << 3,
            Axis::Placement => 5 << 3,
            Axis::Dilation => 6 << 3,
            Axis::OffPlane => 7 << 3,
            Axis::Open => 8 << 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Axis::Orientation(_) => "orientation",
            Axis::Unit(_) => "unit",
            Axis::Prime5(_) => "prime5",
            Axis::Prime13(_) => "prime13",
            Axis::Hue => "hue",
            Axis::Placement => "placement",
            Axis::Dilation => "dilation",
            Axis::OffPlane => "offplane",
            Axis::Open => "open",
        }
    }

    pub(crate) fn from_byte(b: u8) -> Option<Axis> {
        let p = b & 7;
        match (b >> 3, p) {
            (0, p) if p < 6 => Some(Axis::Orientation(p)),
            (1, p) if p < 6 => Some(Axis::Unit(p)),
            (2, p) if p < 6 => Some(Axis::Prime5(p)),
            (3, p) if p < 6 => Some(Axis::Prime13(p)),
            (4, 0) => Some(Axis::Hue),
            (5, 0) => Some(Axis::Placement),
            (6, 0) => Some(Axis::Dilation),
            (7, 0) => Some(Axis::OffPlane),
            (8, 0) => Some(Axis::Open),
            _ => None,
        }
    }
}

/// A18: the dimension audit of a nerve or form, a vector never summed.
/// `torsion` counts the open finite-order loops; `free` is the ℤ-rank of
/// the plane free parts alone; `offplane` is the number of distinct open
/// off-plane rotor classes; `axes` the chart axes opened by discharge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rank {
    pub torsion: u32,
    pub free: u32,
    pub offplane: u32,
    pub axes: u32,
}

/// ONE COORDINATE of the rank audit, by name. A refusal says which
/// coordinate refused it; a number in that sentence would say nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RankAxis {
    Torsion,
    Free,
    OffPlane,
    Axes,
}

/// THE MOVE a pivot made in rank space: the signed difference per
/// coordinate, `after − before`. A18's audit is a vector never summed, and
/// so is the difference of two of them.
///
/// This is a READING. It refuses nothing and admits nothing — 12.6 leaves
/// the refusal with the trial, which is one of the four sites sanctioned to
/// take it, and the reading is what the trial reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RankMove {
    pub torsion: i64,
    pub free: i64,
    pub offplane: i64,
    pub axes: i64,
}

impl RankMove {
    /// The coordinates this move RAISED against 12.6's direction: torsion,
    /// free or offplane climbing, or axes falling. An empty list is a move
    /// that raised nothing — never "the move is admissible", which is the
    /// caller's sentence to say.
    pub fn raised(&self) -> Vec<RankAxis> {
        let mut out = Vec::new();
        if self.torsion > 0 {
            out.push(RankAxis::Torsion);
        }
        if self.free > 0 {
            out.push(RankAxis::Free);
        }
        if self.offplane > 0 {
            out.push(RankAxis::OffPlane);
        }
        if self.axes < 0 {
            out.push(RankAxis::Axes);
        }
        out
    }
}

impl Rank {
    /// What the pivot did to each coordinate (12.6, R2). Signed, per
    /// coordinate, never collapsed: `torsion`, `free` and `offplane` are
    /// read as falling and `axes` as rising, and nothing here decides.
    pub fn moved(before: Rank, after: Rank) -> RankMove {
        let d = |a: u32, b: u32| a as i64 - b as i64;
        RankMove {
            torsion: d(after.torsion, before.torsion),
            free: d(after.free, before.free),
            offplane: d(after.offplane, before.offplane),
            axes: d(after.axes, before.axes),
        }
    }

    pub(crate) fn to_bytes(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.torsion.to_le_bytes());
        out.extend_from_slice(&self.free.to_le_bytes());
        out.extend_from_slice(&self.offplane.to_le_bytes());
        out.extend_from_slice(&self.axes.to_le_bytes());
    }

    pub(crate) fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rank> {
        let mut f = [0u32; 4];
        for x in &mut f {
            *x = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
        }
        Some(Rank {
            torsion: f[0],
            free: f[1],
            offplane: f[2],
            axes: f[3],
        })
    }
}

/// The state vector of a held loop (12.6). Rotor conjugacy invariants (15.3)
/// are read for every holonomy first; the six plane blocks are the sensor
/// section, present only when `Word::form()` exists. Placement and dilation
/// residues are the translator and dilator of the holonomy carriage, never
/// a pair of rationals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassVector {
    /// Sensor-chart section: `None` when the holonomy moves the plane.
    plane: Option<[(bool, u32, i64, i64); 6]>,
    /// Holonomy carriage after the tree gauge (A4).
    pub(crate) carriage: Versor,
    /// Composed gauge around the loop (A4).
    pub(crate) hue: CycleType,
    /// Rotor conjugacy invariants `(a²/w, p²/w, sign a, sign p)` (15.3).
    /// Always read; identity is the zero vector.
    offplane: (
        num_rational::BigRational,
        num_rational::BigRational,
        u8,
        u8,
    ),
}

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

    fn from_bytes(b: &[u8], pos: &mut usize) -> Option<CycleType> {
        let n = *b.get(*pos)? as usize;
        *pos += 1;
        if n == 0 {
            return Some(CycleType::Identity);
        }
        let lens = b.get(*pos..*pos + n)?.to_vec();
        *pos += n;
        Some(CycleType::Type(lens))
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
    /// The vector of a holonomy word alone (identity carriage).
    pub fn of_word(h: &Word) -> Option<ClassVector> {
        Self::read(h, element::identity_versor(), CycleType::Identity)
    }

    pub(crate) fn read(h: &Word, carriage: Versor, hue: CycleType) -> Option<ClassVector> {
        let offplane = if h.is_identity() {
            (Self::zero_q(), Self::zero_q(), 0, 0)
        } else {
            h.rotor().class_invariants()?
        };
        let plane = if h.is_identity() {
            Some([(false, 0, 0, 0); 6])
        } else {
            h.form().map(|pf| pf.coordinates())
        };
        Some(ClassVector {
            plane,
            carriage,
            hue,
            offplane,
        })
    }

    fn zero_q() -> num_rational::BigRational {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(0))
    }

    fn plane_slot(&self, p: u8) -> Option<(bool, u32, i64, i64)> {
        self.plane.and_then(|pl| pl.get(p as usize).copied())
    }

    /// The closed class: every coordinate is zero.
    pub fn zero() -> ClassVector {
        ClassVector {
            plane: Some([(false, 0, 0, 0); 6]),
            carriage: element::identity_versor(),
            hue: CycleType::Identity,
            offplane: (Self::zero_q(), Self::zero_q(), 0, 0),
        }
    }

    /// Coordinates written out in 12.6 axis order. Rotor conjugacy first so
    /// a prefix of the key is the rotor's conjugacy stratum. Closed is the
    /// zero key.
    pub fn class_key(&self) -> crate::transducer::a21::Address {
        let mut buf = vec![crate::transducer::a21::TAG_CLASS];
        alphabet::put_q(&mut buf, &self.offplane.0);
        alphabet::put_q(&mut buf, &self.offplane.1);
        buf.push(self.offplane.2);
        buf.push(self.offplane.3);
        match self.plane {
            None => buf.push(0),
            Some(pl) => {
                buf.push(1);
                for &(orientation, unit, m5, m13) in &pl {
                    buf.push(u8::from(orientation));
                    buf.extend_from_slice(&unit.to_le_bytes());
                    buf.extend_from_slice(&m5.to_le_bytes());
                    buf.extend_from_slice(&m13.to_le_bytes());
                }
            }
        }
        self.hue.to_bytes(&mut buf);
        self.carriage.to_bytes(&mut buf);
        crate::transducer::a21::Address::new(buf)
    }

    pub fn from_key(k: &[u8]) -> Option<ClassVector> {
        if k.first() != Some(&crate::transducer::a21::TAG_CLASS) {
            return None;
        }
        let mut p = 1usize;
        let x = alphabet::take_q(k, &mut p)?;
        let y = alphabet::take_q(k, &mut p)?;
        let sa = *k.get(p)?;
        p += 1;
        let sp = *k.get(p)?;
        p += 1;
        let plane = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let mut pl = [(false, 0u32, 0i64, 0i64); 6];
                for slot in &mut pl {
                    let orientation = *k.get(p)? != 0;
                    p += 1;
                    let unit = u32::from_le_bytes(k.get(p..p + 4)?.try_into().ok()?);
                    p += 4;
                    let m5 = i64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
                    p += 8;
                    let m13 = i64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
                    p += 8;
                    *slot = (orientation, unit, m5, m13);
                }
                Some(pl)
            }
            _ => return None,
        };
        let hue = CycleType::from_bytes(k, &mut p)?;
        let carriage = Versor::from_bytes(k, &mut p)?;
        Some(ClassVector {
            plane,
            carriage,
            hue,
            offplane: (x, y, sa, sp),
        })
    }

    /// The coordinate an axis names, as "nonzero". `Open` is never a
    /// coordinate: it is the last move, admitted when no coordinate's own
    /// pivot discharged. Off-plane is occupied when the holonomy has no
    /// plane section, not as a fallback from a missing form.
    pub fn occupies(&self, axis: Axis) -> bool {
        match axis {
            Axis::Orientation(p) => self.plane_slot(p).is_some_and(|s| s.0),
            Axis::Unit(p) => self.plane_slot(p).is_some_and(|s| s.1 != 0),
            Axis::Prime5(p) => self.plane_slot(p).is_some_and(|s| s.2 != 0),
            Axis::Prime13(p) => self.plane_slot(p).is_some_and(|s| s.3 != 0),
            Axis::Hue => !matches!(self.hue, CycleType::Identity),
            Axis::Placement => element::translator_of(&self.carriage)
                .is_some_and(|t| !element::same_versor(&t, &element::identity_versor())),
            Axis::Dilation => element::dilator_of(&self.carriage)
                .is_some_and(|d| !element::same_versor(&d, &element::identity_versor())),
            Axis::OffPlane => self.plane.is_none(),
            Axis::Open => false,
        }
    }

    /// The occupied coordinates in the fixed order of 12.6.
    pub fn coordinates(&self) -> Vec<Axis> {
        let mut out = Vec::new();
        for p in 0..6u8 {
            for axis in [
                Axis::Orientation(p),
                Axis::Unit(p),
                Axis::Prime5(p),
                Axis::Prime13(p),
            ] {
                if self.occupies(axis) {
                    out.push(axis);
                }
            }
        }
        for axis in [Axis::Hue, Axis::Placement, Axis::Dilation, Axis::OffPlane] {
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
        let plane_ok = a.plane == b.plane;
        let off_ok = a.offplane == b.offplane
            || (!a.occupies(Axis::OffPlane) && !b.occupies(Axis::OffPlane));
        plane_ok
            && element::same_versor(&a.carriage, &b.carriage)
            && a.hue == b.hue
            && off_ok
    }

    fn offplane_zero() -> (
        num_rational::BigRational,
        num_rational::BigRational,
        u8,
        u8,
    ) {
        (Self::zero_q(), Self::zero_q(), 0, 0)
    }

    fn clear(&mut self, axis: Axis) {
        match axis {
            Axis::Orientation(p) => {
                if let Some(pl) = self.plane.as_mut() {
                    if let Some(s) = pl.get_mut(p as usize) {
                        s.0 = false;
                    }
                }
            }
            Axis::Unit(p) => {
                if let Some(pl) = self.plane.as_mut() {
                    if let Some(s) = pl.get_mut(p as usize) {
                        s.1 = 0;
                    }
                }
            }
            Axis::Prime5(p) => {
                if let Some(pl) = self.plane.as_mut() {
                    if let Some(s) = pl.get_mut(p as usize) {
                        s.2 = 0;
                    }
                }
            }
            Axis::Prime13(p) => {
                if let Some(pl) = self.plane.as_mut() {
                    if let Some(s) = pl.get_mut(p as usize) {
                        s.3 = 0;
                    }
                }
            }
            Axis::Placement => {
                if let Some(d) = element::dilator_of(&self.carriage) {
                    self.carriage = d;
                }
            }
            Axis::Dilation => {
                if let Some(t) = element::translator_of(&self.carriage) {
                    self.carriage = t;
                }
            }
            Axis::OffPlane => {
                self.offplane = Self::offplane_zero();
                self.plane = Some([(false, 0, 0, 0); 6]);
            }
            Axis::Hue => self.hue = CycleType::Identity,
            Axis::Open => {}
        }
    }

    /// No coordinate is occupied: the key is the zero key.
    pub fn is_zero(&self) -> bool {
        self.occupied().is_empty()
    }

    /// The free part: a prime exponent, or an off-plane class (infinite
    /// order). A loop with a free part is a demand, never filed torsion.
    pub fn free_part(&self) -> bool {
        self.occupies(Axis::OffPlane)
            || self
                .plane
                .is_some_and(|pl| pl.iter().any(|s| s.2 != 0 || s.3 != 0))
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
    OffPlane(crate::transducer::a21::Address),
}

pub fn loop_row(h: &Word) -> LoopRow {
    if h.is_identity() {
        return LoopRow::Closed;
    }
    match free_row(h) {
        Some(row) if row == [0i64; 12] => LoopRow::Torsion,
        Some(row) => LoopRow::Free(row),
        None => LoopRow::OffPlane(match h.rotor().class_key() {
            Some(k) => k,
            None => h.rotor().key(),
        }),
    }
}

/// A18 over the rows of a nerve's cycles or a form's loops.
pub fn rank_of_rows(rows: &[LoopRow]) -> Option<Rank> {
    let mut torsion = 0u32;
    let mut offplane: Keyed<()> = Keyed::new();
    let mut free: Vec<[i64; 12]> = Vec::new();
    for r in rows {
        match r {
            LoopRow::Closed => {}
            LoopRow::Torsion => torsion += 1,
            LoopRow::Free(row) => free.push(*row),
            LoopRow::OffPlane(k) => {
                offplane.insert(k, ());
            }
        }
    }
    Some(Rank {
        torsion,
        free: z_rank(&free)?,
        offplane: offplane.len() as u32,
        axes: 0,
    })
}

pub fn rank_of_holonomies<'a>(holonomies: impl Iterator<Item = &'a Word>) -> Option<Rank> {
    let rows: Vec<LoopRow> = holonomies.map(loop_row).collect();
    rank_of_rows(&rows)
}
