// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6, A18 — THE STATE VECTOR OF A HELD LOOP: its axes, the hue cycle
//! type, the placement and dilation residues, and the rank audit read one
//! loop at a time.

use super::transport::{gauge_image, gauge_keyed, hue_key};
use crate::core::lattice::z_rank;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Versor};

pub use super::axis::{Axis, Rank, RankAxis, RankMove};

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

    /// THE VECTOR OF A ROTOR, on any of the six planes (12.6, 15.3).
    ///
    /// `read` takes a WORD, and the only route from a rotor to a word runs
    /// through the sensor plane — `Word::form` is `plane_block`, which is
    /// `rotor_block_on` at `(PLANE_ROW, PLANE_COL)`. A rotor acting on
    /// `e₁∧e₂` maps `e₁` off the plane, has no block there, and so has no
    /// word: read through `read` it would have no class at all, though it
    /// is a letter of the seventeen and its conjugacy invariants exist.
    ///
    /// The invariants are the rotor's own (15.3) and are defined on every
    /// plane. The plane slots are the SENSOR SECTION and are `None` where
    /// the holonomy moves the plane — which the carrier already allows and
    /// says so at its own field. So an off-plane rotor carries its
    /// conjugacy class, its residues and its hue, and carries no sensor
    /// section, which is the truth about it rather than an absence of one.
    pub(crate) fn of_rotor(
        r: &crate::transducer::alphabet::Rotor,
        carriage: Versor,
        hue: CycleType,
    ) -> Option<ClassVector> {
        if r.is_identity() {
            return Some(ClassVector {
                plane: Some([(false, 0, 0, 0); 6]),
                carriage,
                hue,
                offplane: (Self::zero_q(), Self::zero_q(), 0, 0),
            });
        }
        Some(ClassVector {
            plane: r.spelled_word().and_then(|w| w.form()).map(|pf| pf.coordinates()),
            carriage,
            hue,
            offplane: r.class_invariants()?,
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
    pub fn class_key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_CLASS];
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
        crate::transducer::address::Address::new(buf)
    }

    pub fn from_key(k: &[u8]) -> Option<ClassVector> {
        if crate::transducer::address::tag_of(k) != Some(crate::transducer::address::TAG_CLASS) {
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
    /// coordinate of the vector: it is a move on the form's cover, and the
    /// form's `axes` carrier says whether it was taken — a loop whose class
    /// is an opened axis reads as the zero vector (`Form::class_vector`,
    /// I6). Off-plane is occupied when the holonomy has no plane section,
    /// not as a fallback from a missing form.
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

    /// THE LANES THIS CLASS OCCUPIES (42.6 as amended, law 5): one flag
    /// per lane, read from the occupied coordinates through `Axis::lane`.
    ///
    /// Four flags and never their sum. A class occupying S and 𝒽 is not
    /// "two lanes of content": it is content on S and content on 𝒽, and
    /// the two are independent covers that a reduction must not mix.
    pub fn lanes(&self) -> [bool; 4] {
        let mut out = [false; 4];
        for axis in self.coordinates() {
            if let Some(slot) = out.get_mut(axis.lane()) {
                *slot = true;
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
    OffPlane(crate::transducer::address::Address),
    /// The loop's class is a chart axis the form opened (I6, 15.5): its
    /// holonomy is the transition on that axis, so it counts on `axes`
    /// and on no other coordinate.
    Opened,
}

/// `None` when the holonomy is not a versor: it has no class (15.3), so
/// it has no row in the audit and the rank is unmeasured. It was keyed by
/// its rotor's bytes in place of the class it lacks, which is 0.12b's
/// refused fallback (rule 11).
pub fn loop_row(h: &Word) -> Option<LoopRow> {
    if h.is_identity() {
        return Some(LoopRow::Closed);
    }
    Some(match free_row(h) {
        Some(row) if row == [0i64; 12] => LoopRow::Torsion,
        Some(row) => LoopRow::Free(row),
        None => LoopRow::OffPlane(h.rotor().class_key()?),
    })
}

/// A18 over the rows of a nerve's cycles or a form's loops.
pub fn rank_of_rows(rows: &[LoopRow]) -> Option<Rank> {
    let mut torsion = 0u32;
    let mut axes = 0u32;
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
            LoopRow::Opened => axes += 1,
        }
    }
    Some(Rank {
        torsion,
        free: z_rank(&free)?,
        offplane: offplane.len() as u32,
        axes,
    })
}
