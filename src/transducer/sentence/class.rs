// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6, A18 — THE STATE VECTOR OF A HELD ORBIT: its axes, the hue cycle
//! type, the placement and dilation residues, and the rank audit read one
//! orbit at a time.

use super::transport::{gauge_image, gauge_keyed, hue_key};
use crate::core::lattice::z_rank;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Versor};

pub use super::axis::{Axis, Rank, RankAxis, RankMove};

/// The state vector of a held orbit (12.6). Rotor conjugacy invariants (15.3)
/// are read for every holonomy first; the six plane blocks are the sensor
/// section, present only when `Word::form()` exists. Placement and dilation
/// residues are the translator and dilator of the holonomy carriage, never
/// a pair of rationals.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassVector {
    /// Sensor-chart section: `None` when the holonomy moves the plane.
    plane: Option<[(bool, u32, i64, i64); alphabet::PAIR_PLANES]>,
    /// Holonomy carriage after the tree gauge (A4).
    pub(crate) carriage: Versor,
    /// Composed gauge around the orbit (A4).
    pub(crate) hue: CycleType,
    /// Rotor conjugacy invariants `(a²/w, p²/w, sign p)` (15.3). Always
    /// read; identity is the zero vector. The signs are the RAY's, not a
    /// lift's — see [`crate::transducer::alphabet::Rotor::class_invariants`],
    /// which is why there is one sign byte here and not two.
    offplane: (
        num_rational::BigRational,
        num_rational::BigRational,
        crate::network::nerve::coord::BivectorKind,
    ),
    /// THE BURGERS VECTOR: the slip per circuit — the part of the
    /// carriage's translation that no choice of origin absorbs, lying in
    /// `ker(I − R)`, the plane the rotor leaves POINTWISE FIXED. Read where
    /// the ROTOR is in hand, because `(I − R)p = t` cannot be split without
    /// `R` and this vector does not carry one.
    ///
    /// WHAT AN ORBIT'S TRANSLATION IS, which is not how big it is. A rotation
    /// about `p` carries `T(p − Rp)`: that translation is WHERE THE TURN
    /// STANDS, a choice of origin, and zeroing it moves the frame rather
    /// than discharging anything. A translation with no such `p` is a slip
    /// — the seam does not return — and that is an obstruction.
    ///
    /// `occupies(Axis::Placement)` read `translator_of(carriage) !=
    /// identity` and could not tell the two apart, so every turn about a
    /// point off the origin published a placement demand. `classify` had
    /// already solved `(I − R)p = t` for the same carriage and filed the
    /// orbit `Congruent`: two readers, one carriage, opposite answers
    /// (law 12).
    ///
    /// AND THEN IT WAS A THREE-VALUED ENUM — `Zero | FixedPoint | Residue`
    /// — which is the same collapse one level up (54.4, as ruled
    /// 2026-09-20). A slip HAS a magnitude and a direction. Reducing it to
    /// the word "Residue" is a measured vector written down as a scalar
    /// status: rule 4 and law 7 together, and the reason a screw filed as
    /// an undifferentiated free generator with nothing said about how far
    /// it slips.
    ///
    /// The field carries the vector now, `element::screw_of` splits it off
    /// the centre exactly over ℚ, and ZERO IS A TURN THAT RETURNS —
    /// including one standing away from the origin, since `T(p − Rp)·R` is
    /// conjugate to `R`. Those were two key values for one conjugacy class.
    placement: [num_rational::BigRational; 3],
}

/// The hue cycle type of an orbit: sorted cycle lengths of the composed
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
    /// THE VECTOR OF A HOLONOMY WORD AT THE IDENTITY ASSIGNMENT: its rotor's
    /// coordinates and nothing else.
    ///
    /// `Placement`, `Dilation` and `Hue` are read off the carriage and the
    /// gauge, so a vector built here occupies NONE of the three whatever
    /// the word is. That is the truth about a word with no assignment
    /// beside it, and it was the wrong reader for a word that has one: the
    /// axis index filed every admitted word through this, so three of the
    /// nine coordinates of 12.6 held nothing the pivot could be offered.
    /// A caller with a carriage and a gauge calls [`ClassVector::read`].
    pub fn of_word(h: &Word) -> Option<ClassVector> {
        Self::read(h, element::identity_versor(), CycleType::Identity)
    }

    pub(crate) fn read(h: &Word, carriage: Versor, hue: CycleType) -> Option<ClassVector> {
        let offplane = if h.is_identity() {
            Self::offplane_zero()
        } else {
            h.rotor().class_invariants()?
        };
        let plane = if h.is_identity() {
            Some([(false, 0, 0, 0); alphabet::PAIR_PLANES])
        } else {
            h.form().map(|pf| pf.coordinates())
        };
        let placement = Self::twist_of(&h.rotor(), &carriage)?;
        Some(ClassVector { plane, carriage, hue, offplane, placement })
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
        let placement = Self::twist_of(r, &carriage)?;
        if r.is_identity() {
            return Some(ClassVector {
                plane: Some([(false, 0, 0, 0); alphabet::PAIR_PLANES]),
                carriage,
                hue,
                offplane: Self::offplane_zero(),
                placement,
            });
        }
        Some(ClassVector {
            plane: r.spelled_word().and_then(|w| w.form()).map(|pf| pf.coordinates()),
            carriage,
            hue,
            offplane: r.class_invariants()?,
            placement,
        })
    }

    /// THE SLIP PER CIRCUIT, read against the rotation that carries it.
    /// The dilator is stripped first: a dilation is its own coordinate, and
    /// a turn about a point composed with a scale still turns about that
    /// point.
    ///
    /// `None` is a reading that CANNOT BE TAKEN — a carriage that is not a
    /// conformal versor, or a turn that carries the base off itself — and
    /// it refuses the whole class vector rather than being written down as
    /// a slip of zero or as an undifferentiated residue (rule 11). That
    /// second case is `Axis::OffPlane`'s and is carried there.
    fn twist_of(
        r: &crate::transducer::alphabet::Rotor,
        carriage: &Versor,
    ) -> Option<[num_rational::BigRational; 3]> {
        let t = element::translator_of(carriage)?;
        if element::same_versor(&t, &element::identity_versor()) {
            return Some(Self::no_twist());
        }
        Some(element::screw_of(r, &t)?.twist)
    }

    fn no_twist() -> [num_rational::BigRational; 3] {
        std::array::from_fn(|_| Self::zero_q())
    }

    /// THE SLIP THIS CLASS STANDS AT, as the translator that is exactly it.
    ///
    /// `Axis::Placement`'s 3.9 move offered `translator_of(&kappa.carriage)`
    /// — the WHOLE translation `t`. The split says `t = t_im ⊕ τ`, and
    /// `t_im = (I − R)p` is the part a choice of origin absorbs: it is
    /// WHERE THE TURN STANDS, and writing it onto a face drags the centre
    /// of rotation across the chart along with the slip. Only `τ` is the
    /// obstruction, so only `τ` is offered — it acts on `ker(I − R)`, which
    /// leaves the centre and the invariant plane exactly where they are.
    ///
    /// `None` where the class does not slip: there is no placement move to
    /// make, and `occupies(Axis::Placement)` already says so.
    pub(crate) fn slip_translator(&self) -> Option<Versor> {
        if !self.occupies(Axis::Placement) {
            return None;
        }
        Some(crate::transducer::cga::translator_at(&self.placement))
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
            plane: Some([(false, 0, 0, 0); alphabet::PAIR_PLANES]),
            carriage: element::identity_versor(),
            hue: CycleType::Identity,
            offplane: Self::offplane_zero(),
            placement: Self::no_twist(),
        }
    }

    /// Coordinates written out in 12.6 axis order. Rotor conjugacy first so
    /// Canonical coordinates written sequentially (Rule 6).
    pub fn put_canonical(&self, buf: &mut Vec<u8>) {
        alphabet::put_q(buf, &self.offplane.0);
        alphabet::put_q(buf, &self.offplane.1);
        buf.push(self.offplane.2.as_u8());
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
        self.hue.to_bytes(buf);
        // THE CARRIAGE IS WRITTEN CANONICALLY (A4).
        self.carriage.put_canonical(buf);
        // WHAT THE TRANSLATION IS, appended LAST so the conjugacy stratum
        // stays the depth-one prefix every court asks by. It is a
        // coordinate of 12.6 the vector did not carry: a base point and a
        // slip read identically on every other axis.
        // THE SLIP, WRITTEN OUT (rule 6, law 4): three exact rationals,
        // never a status byte. A turn about a point off the origin is
        // conjugate to the same turn about the origin, so both write the
        // zero vector here and reach ONE class — which the three-valued
        // byte split into two.
        for q in &self.placement {
            alphabet::put_q(buf, q);
        }
    }

    /// The conjugacy invariants and the sensor coordinates together (A21):
    /// a prefix of the key is the rotor's conjugacy stratum. Closed is the
    /// zero key.
    pub fn class_key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_CLASS];
        self.put_canonical(&mut buf);
        crate::transducer::address::Address::new(buf)
    }

    /// THE COORDINATE SYSTEM THIS KEY IS WRITTEN IN (rule 6), declared so a
    /// reader asks for a stratum by DEPTH and never by a byte count.
    ///
    /// The four coordinates are the four `class_key` writes, in that order,
    /// and the first of them is exactly what [`Rotor::class_key`] writes —
    /// tag, the two invariants, the ray's sign byte. That is why a rotor's
    /// conjugacy class is the depth-one projection of an orbit's state vector
    /// and not a second key that happens to agree: `class_schema` parses
    /// the boundary out of the bytes the writer wrote, and
    /// `the_rotor_class_is_the_first_stratum_of_the_class_vector` asserts
    /// the two coincide rather than trusting it.
    pub fn class_schema() -> &'static crate::network::nerve::Schema {
        &CLASS_SCHEMA
    }

    pub fn from_key(k: &[u8]) -> Option<ClassVector> {
        if crate::transducer::address::tag_of(k) != Some(crate::transducer::address::TAG_CLASS) {
            return None;
        }
        let mut p = 1usize;
        let x = alphabet::take_q(k, &mut p)?;
        let y = alphabet::take_q(k, &mut p)?;
        // 54.7 — THE BYTE NAMES A BIVECTOR READING OR THE VECTOR DOES NOT
        // DECODE. It was carried as a raw `u8`, so a tape could spell a
        // type that is not one of the four and be believed (rule 11).
        let sp = crate::network::nerve::coord::BivectorKind::from_u8(*k.get(p)?)?;
        p += 1;
        let plane = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let mut pl = [(false, 0u32, 0i64, 0i64); alphabet::PAIR_PLANES];
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
        let mut placement = Self::no_twist();
        for q in &mut placement {
            *q = alphabet::take_q(k, &mut p)?;
        }
        Some(ClassVector { plane, carriage, hue, offplane: (x, y, sp), placement })
    }

    /// The coordinate an axis names, as "nonzero". `Open` is never a
    /// coordinate of the vector: it is a move on the form's cover, and the
    /// form's `axes` carrier says whether it was taken — an orbit whose class
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
            // A BASE POINT IS NOT AN OBSTRUCTION. Only a translation no
            // `p` accounts for occupies this axis; `T(p − Rp)` is where the
            // turn stands, and 3.9's move for Placement — the inverse
            // translator — would move the frame and close nothing.
            Axis::Placement => self.placement.iter().any(|q| !num_traits::Zero::is_zero(q)),
            Axis::Dilation => element::dilator_of(&self.carriage)
                .is_some_and(|d| !element::same_versor(&d, &element::identity_versor())),
            Axis::OffPlane => self.plane.is_none(),
            Axis::Open => false,
        }
    }

    /// The occupied coordinates in the fixed order of 12.6.
    ///
    /// THE PLANE SECTION IS READ, NOT INTERROGATED. This built twenty-four
    /// `Axis` values and asked `occupies` about each, and `occupies` answers
    /// by going back to `plane_slot(p)` for the very block it is standing
    /// in — twenty-four bounds-checked re-reads of a fixed-width array to
    /// recover what one pass over it already holds. The exponents ARE the
    /// coordinates: a non-zero entry is an occupied axis and there is
    /// nothing to test.
    ///
    /// Reading the block also keeps the plane's `None` honest. `plane` is
    /// the SENSOR SECTION and is absent exactly when the holonomy moves the
    /// plane, which is `Axis::OffPlane` — so no plane coordinate is
    /// produced there and `OffPlane` is, without either being a fallback
    /// for the other.
    ///
    /// The four that are not plane exponents keep `occupies`: `Hue` reads
    /// the cycle type, `Placement` and `Dilation` the carriage's translator
    /// and dilator, `OffPlane` the section's absence. They are four
    /// different readings of four different carriers and there is no block
    /// to walk.
    ///
    /// The index is the PLANE, which is content and not a position: `p` is
    /// which of the six bivector planes the exponent stands on, and 12.6
    /// fixes the order the coordinates are written in (rule 15 — the order
    /// here is content, and the walk selects nothing).
    pub fn coordinates(&self) -> Vec<Axis> {
        let mut out = Vec::new();
        if let Some(plane) = self.plane {
            for (p, &(orientation, unit, m5, m13)) in plane.iter().enumerate() {
                let p = p as u8;
                if orientation {
                    out.push(Axis::Orientation(p));
                }
                if unit != 0 {
                    out.push(Axis::Unit(p));
                }
                if m5 != 0 {
                    out.push(Axis::Prime5(p));
                }
                if m13 != 0 {
                    out.push(Axis::Prime13(p));
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
    /// orbit is held at all (the one move that is not a coordinate).
    pub fn occupied(&self) -> Vec<Axis> {
        let mut out = self.coordinates();
        if !out.is_empty() {
            out.push(Axis::Open);
        }
        out
    }

    /// LAW 10's DISCHARGE GUARD: did any coordinate but `axis` RISE?
    ///
    /// `self` is the reading AFTER the move, `before` the reading before
    /// it. True when some other coordinate grew — which refuses the move —
    /// and false when every one of them held or fell.
    ///
    /// `agrees_except` stood here and demanded IDENTITY on every
    /// non-target coordinate: it cleared the target from both sides and
    /// compared `plane`, `carriage`, `hue` and `offplane` for equality. Its
    /// own doc line said the opposite — "a coordinate may fall to zero,
    /// none may rise or move" — and law 10 says the summand becomes zero
    /// "with no other rising". A pivot writes a face carrier that every
    /// orbit through that face reads, so a move that zeroes its target
    /// routinely moves other coordinates too, and every one that FELL was
    /// refused by the identity test. One predicate, one carrier (law 12):
    /// that reader is deleted rather than kept beside this one.
    ///
    /// Every comparison is on ONE coordinate's own exact value and no two
    /// are combined — law 5's covers are never summed, and rule 4 refuses
    /// choosing among CANDIDATES by a scalar, not reading whether one
    /// coordinate grew.
    ///
    /// A reading that cannot be taken is a rise: the move is refused
    /// rather than certified on an unread coordinate (rule 11).
    pub fn rises_against(&self, before: &ClassVector, axis: Axis) -> bool {
        let (mut after, mut before) = (self.clone(), before.clone());
        after.clear(axis);
        before.clear(axis);
        let mag = |q: &num_rational::BigRational| -> num_rational::BigRational {
            if q < &Self::zero_q() { -q.clone() } else { q.clone() }
        };
        // THE PLANE BLOCKS, slot by slot: an orientation character
        // appearing, a unit turn count growing, a prime exponent growing
        // in magnitude. A section that was there and is gone has fallen;
        // one that appears where there was none is the holonomy moving the
        // plane, which is `OffPlane` and is read there.
        let plane_rose = match (before.plane, after.plane) {
            (_, None) => false,
            (None, Some(pl)) => pl.iter().any(|s| s.0 || s.1 != 0 || s.2 != 0 || s.3 != 0),
            (Some(b), Some(a)) => b.iter().zip(a.iter()).any(|(b, a)| {
                (a.0 && !b.0)
                    || a.1 > b.1
                    || a.2.unsigned_abs() > b.2.unsigned_abs()
                    || a.3.unsigned_abs() > b.3.unsigned_abs()
            }),
        };
        // THE CARRIAGE, componentwise. The placement is three coordinates
        // and each is compared alone: a norm would sum them, which is
        // exactly what law 5 refuses of the covers.
        let placement = |c: &Versor| -> Option<(num_rational::BigRational, num_rational::BigRational, num_rational::BigRational)> {
            // The class's placement is the alphabet's three base axes (its
            // key's Burgers segment is three rationals, 51.2). It is read on
            // EVERY axis the carriage has, and a component past the third
            // REFUSES — which reads here as a rise, so the move is refused —
            // rather than being cut off unseen (audit N-4; the key's own
            // recut is BQ5b's).
            let rho = element::rho_of(c)?;
            let at = usize::from(c.n()).max(3);
            let img = crate::transducer::cga::base_at(&c.sandwich(&crate::transducer::cga::point_at(&[])), at)?;
            match img.as_slice() {
                [x, y, z, rest @ ..] if rest.iter().all(num_traits::Zero::is_zero) => {
                    Some((x / &rho, y / &rho, z / &rho))
                }
                _ => None,
            }
        };
        // ONE FIBER OR TWO. The placements compatible with a rotation `R`
        // are constrained by `R` — `T(p − Rp)` is a base point under one
        // rotor and an unremoved slip under another — so comparing two
        // translations by magnitude is only meaningful when the ROTATION
        // held. It compared them always, which read a vector standing in
        // one fiber against a vector standing in another and called the
        // difference a rise.
        //
        // Same base: the fiber did not move and componentwise comparison
        // is a reading of one space. Base moved: no magnitude comparison
        // exists, and what is readable is the discrete transition — a
        // base point becoming a residue is a rise, a residue becoming a
        // base point is a fall, and like for like is neither.
        let same_base = before.plane == after.plane && before.offplane == after.offplane;
        let carriage_rose = match same_base {
            true => match (placement(&before.carriage), placement(&after.carriage)) {
                (Some(b), Some(a)) => {
                    mag(&a.0) > mag(&b.0) || mag(&a.1) > mag(&b.1) || mag(&a.2) > mag(&b.2)
                }
                _ => true,
            },
            // The base moved, so no componentwise magnitude reading of the
            // carriage exists. What is readable is the SLIP: a turn that
            // returned and now slips is a rise, the reverse is a fall, and
            // a slip that stays a slip is compared by its own magnitude —
            // lane by lane, never summed (law 5).
            false => {
                let b = &before.placement;
                let a = &after.placement;
                let slips = |v: &[num_rational::BigRational; 3]| {
                    v.iter().any(|q| !num_traits::Zero::is_zero(q))
                };
                match (slips(b), slips(a)) {
                    (false, true) => true,
                    (true, true) => (0..3).any(|i| mag(&a[i]) > mag(&b[i])),
                    _ => false,
                }
            }
        };
        // The dilation's own coordinate: displacement from unity.
        let one: num_rational::BigRational = num_traits::One::one();
        let dilation_rose = match (element::rho_of(&before.carriage), element::rho_of(&after.carriage)) {
            (Some(b), Some(a)) => mag(&(a - &one)) > mag(&(b - &one)),
            _ => true,
        };
        // THE HUE PARTITION, sub-multiset bounded (ruled 2026-09-18): each
        // cycle after must be bounded by a DISTINCT cycle before of equal
        // or greater length. `CycleType::of` sorts ascending, so pairing
        // largest-first is the only matching that can succeed and a
        // mismatch at any position is a rise. Condensation toward the
        // identity falls ([2,3] → [2], → nothing); merging two cycles into
        // one longer one raises the permutation's period and is refused
        // ([2,3] → [5]).
        let cycles = |h: &CycleType| -> Vec<u8> {
            match h {
                CycleType::Identity => Vec::new(),
                CycleType::Type(v) => v.clone(),
            }
        };
        let (cb, ca) = (cycles(&before.hue), cycles(&after.hue));
        let hue_rose =
            ca.len() > cb.len() || ca.iter().rev().zip(cb.iter().rev()).any(|(a, b)| a > b);
        // The off-plane invariants, each on its own: zero to non-zero, or
        // growing. A sign is not a magnitude and a sign change is not a
        // rise.
        let offplane_rose = mag(&after.offplane.0) > mag(&before.offplane.0)
            || mag(&after.offplane.1) > mag(&before.offplane.1);
        plane_rose || carriage_rose || dilation_rose || hue_rose || offplane_rose
    }

    /// THE IDENTITY'S OFF-PLANE READING IS TRIVIAL, NOT PARABOLIC (54.7).
    /// `B = 0` turns nothing; a parabolic has `B ≠ 0` and `B² = 0` and
    /// displaces along its own null direction. They were one byte.
    fn offplane_zero() -> (
        num_rational::BigRational,
        num_rational::BigRational,
        crate::network::nerve::coord::BivectorKind,
    ) {
        (
            Self::zero_q(),
            Self::zero_q(),
            crate::network::nerve::coord::BivectorKind::Trivial,
        )
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
                self.placement = Self::no_twist();
            }
            Axis::Dilation => {
                if let Some(t) = element::translator_of(&self.carriage) {
                    self.carriage = t;
                }
            }
            Axis::OffPlane => {
                self.offplane = Self::offplane_zero();
                self.plane = Some([(false, 0, 0, 0); alphabet::PAIR_PLANES]);
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
    /// order). An orbit with a free part is a demand, never filed torsion.
    pub fn free_part(&self) -> bool {
        self.occupies(Axis::OffPlane)
            || self
                .plane
                .is_some_and(|pl| pl.iter().any(|s| s.2 != 0 || s.3 != 0))
    }
}

// THE FOUR COORDINATES OF A CLASS KEY, each reading the framing
// `class_key` wrote for it. Nothing below is a remembered width: a
// rational's two integers carry their own lengths, the plane block is
// present or absent by its own tag, the hue cycle carries its count, and a
// versor's sixteen coefficients carry theirs. So a key of another system
// fails at the coordinate it fails at, and the depth-one boundary is the
// rotor's conjugacy stratum by parsing rather than by agreement.

use crate::network::nerve::{skip_len_u32, skip_n, skip_q, take_u8, Schema, Segment};

/// Tag, the two conjugacy invariants, the ray's sign byte — exactly what
/// [`crate::transducer::alphabet::Rotor::class_key`] writes.
struct Conjugacy;

impl Segment for Conjugacy {
    fn name(&self) -> &'static str {
        "conjugacy invariants"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        if take_u8(key, pos)? != crate::transducer::address::TAG_CLASS {
            return None;
        }
        skip_q(key, pos)?;
        skip_q(key, pos)?;
        take_u8(key, pos).map(|_| ())
    }
}

/// The sensor section: a presence byte, and where present six plane slots
/// of `(orientation u8, unit u32, m5 i64, m13 i64)`.
struct PlaneSection;

impl Segment for PlaneSection {
    fn name(&self) -> &'static str {
        "plane section"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        match take_u8(key, pos)? {
            0 => Some(()),
            1 => skip_n(key, pos, 6 * (1 + 4 + 8 + 8)),
            _ => None,
        }
    }
}

/// The hue cycle type: a count, and that many sorted cycle lengths.
struct HueCycle;

impl Segment for HueCycle {
    fn name(&self) -> &'static str {
        "hue cycle type"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        let n = take_u8(key, pos)? as usize;
        skip_n(key, pos, n)
    }
}

/// The A4 carriage as `Mv::put_canonical` writes it: its `n`, then one
/// coefficient per slot of that algebra, each a sign byte and two
/// length-framed integers.
struct Carriage;

impl Segment for Carriage {
    fn name(&self) -> &'static str {
        "carriage"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        let n = take_u8(key, pos)?;
        for _ in 0..crate::transducer::cga::slots(n) {
            take_u8(key, pos)?;
            skip_len_u32(key, pos)?;
            skip_len_u32(key, pos)?;
        }
        Some(())
    }
}

/// THE SLIP PER CIRCUIT: the Burgers vector, three exact rationals, each a
/// length-framed numerator and denominator as `put_q` writes them.
///
/// It was ONE BYTE — a `Zero | FixedPoint | Residue` status — and the
/// schema is the writer's own declaration of what it wrote, so it moves
/// with it (rule 6). A reader asking this segment for a stratum now gets
/// the vector's three coordinates and not a word standing for them.
struct Placement;

impl Segment for Placement {
    fn name(&self) -> &'static str {
        "burgers vector"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        for _ in 0..3 {
            skip_len_u32(key, pos)?;
            skip_len_u32(key, pos)?;
        }
        Some(())
    }
}

static CONJUGACY: Conjugacy = Conjugacy;
static PLANE_SECTION: PlaneSection = PlaneSection;
static HUE_CYCLE: HueCycle = HueCycle;
static CARRIAGE: Carriage = Carriage;
static PLACEMENT: Placement = Placement;

static CLASS_SEGMENTS: [&dyn Segment; 5] =
    [&CONJUGACY, &PLANE_SECTION, &HUE_CYCLE, &CARRIAGE, &PLACEMENT];

static CLASS_SCHEMA: Schema = Schema {
    system: "class vector",
    segments: &CLASS_SEGMENTS,
};

/// THE WIDTH OF THE FREE ROW: two prime exponents on every pair plane. It
/// was `12`, which is `PAIR_PLANES` doubled and written out.
pub const FREE_ROW: usize = alphabet::PAIR_PLANES * 2;

/// The free part of a holonomy as one row over ℤ: (m5, m13) per plane.
///
/// The columns are named by the LETTERS that carry them rather than by
/// `7 + p` and `13 + p`, which is the alphabet's layout restated beside
/// the table that defines it, and the `p < 4` that guarded the second is
/// the question `pythagorean_13` answers.
fn free_row(h: &Word) -> Option<[i64; FREE_ROW]> {
    let rep = h.form()?.conjugacy_rep();
    let mut row = [0i64; FREE_ROW];
    let exp = |id: Option<alphabet::LetterId>| {
        match id.and_then(|l| rep.rot.get(l.raw() as usize)) {
            Some(&n) => n,
            // A plane the alphabet carries no such letter on contributes
            // no exponent, which is zero by the form's own shape.
            None => 0,
        }
    };
    for (p, plane) in alphabet::SLOT_PLANES_SIX.iter().enumerate() {
        if let Some(c) = row.get_mut(2 * p) {
            *c = exp(alphabet::LetterId::pythagorean_5(*plane));
        }
        if let Some(c) = row.get_mut(2 * p + 1) {
            *c = exp(alphabet::LetterId::pythagorean_13(*plane));
        }
    }
    Some(row)
}

/// What one orbit contributes to rank (A18): read once per orbit, so a trial
/// that changes one orbit's holonomy swaps one row instead of re-reading the
/// nerve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrbitRow {
    Closed,
    /// The orbit's finite-order class. CARRIED, because the audit counts
    /// torsion by class and not by orbit — see [`rank_of_rows`].
    Torsion(crate::network::nerve::coord::Holonomy),
    Free([i64; FREE_ROW]),
    OffPlane(crate::network::nerve::coord::Holonomy),
    /// The orbit's class is a chart axis the form opened (I6, 15.5): its
    /// holonomy is the transition on that axis, so it counts on `axes`
    /// and on no other coordinate. The class is carried for the same
    /// reason `Torsion` carries it: the axis carrier is keyed by class, so
    /// one opening opens every orbit of that class and they are one axis.
    Opened(crate::network::nerve::coord::Holonomy),
}

/// `None` when the holonomy is not a versor: it has no class (15.3), so
/// it has no row in the audit and the rank is unmeasured. It was keyed by
/// its rotor's bytes in place of the class it lacks, which is 0.12b's
/// refused fallback (rule 11).
pub fn orbit_row(h: &Word) -> Option<OrbitRow> {
    if h.is_identity() {
        return Some(OrbitRow::Closed);
    }
    Some(match free_row(h) {
        // The zero row is FREE_ROW wide, which is the width `free_row`
        // returns. It was written out as `12`, a second statement of
        // `PAIR_PLANES * 2` that a change of rank would have left behind
        // (rule 5).
        // 54.7 R4 — ONE PRODUCER OF A HOLONOMY CLASS. This read
        // `Rotor::class_key`, the `Cl(4,0)` conjugacy coordinates, while
        // the carriers these rows are filed into are read at
        // `holonomy_key` — two producers of one fact (law 12), which is the
        // R1 shape. Both ends ask the same function now.
        Some(row) if row == [0i64; FREE_ROW] => OrbitRow::Torsion(h.rotor().holonomy_key()?),
        Some(row) => OrbitRow::Free(row),
        None => OrbitRow::OffPlane(h.rotor().holonomy_key()?),
    })
}

/// A18 over the rows of a nerve's cycles or a form's orbits.
///
/// EVERY COORDINATE COUNTS BY CLASS. A rank is the dimension of a module —
/// the number of INDEPENDENT generators — never a tally of the orbits that
/// exhibit them, and two of these four already read it that way: `free` is
/// `z_rank`, a matrix rank over the free rows, and `offplane` is a keyed set
/// taking each class once. `torsion` and `axes` counted per orbit.
///
/// Mixing the two breaks the rank delta itself. Two orbits of one finite
/// class are ONE obstruction seen twice; the axis carrier is keyed by class,
/// so opening it opens both, and counted by orbit that single attachment
/// would read `torsion −2, axes +2` while the same object contributes 1 to
/// `offplane`, and T21 (iii) would refuse an elementary attachment as
/// non-elementary. Counted by class the attachment reads `torsion −1, axes
/// +1`, which is `Rank::moved`'s `Discharged { from: Torsion }` and A18's
/// one step.
///
/// A CLASS CANNOT BE BOTH. A key present in `axes` is an opened chart axis
/// and no longer a defect (I6, 15.5), so it is not also counted in torsion;
/// the rows already say which, because `orbit_row_on` answers `Opened` for an
/// orbit whose class the carrier holds and never `Torsion`.
pub fn rank_of_rows(rows: &[OrbitRow]) -> Option<Rank> {
    // A18 COUNTS DISTINCT CLASSES, so each lane is a carrier keyed by the
    // holonomy class itself — the coordinate, not bytes standing for it.
    let mut torsion: Keyed<(), crate::network::nerve::coord::Holonomy> = Keyed::new();
    let mut axes: Keyed<(), crate::network::nerve::coord::Holonomy> = Keyed::new();
    let mut offplane: Keyed<(), crate::network::nerve::coord::Holonomy> = Keyed::new();
    let mut free: Vec<[i64; FREE_ROW]> = Vec::new();
    for r in rows {
        match r {
            OrbitRow::Closed => {}
            OrbitRow::Torsion(k) => {
                torsion.put(k, ());
            }
            OrbitRow::Free(row) => free.push(*row),
            OrbitRow::OffPlane(k) => {
                offplane.put(k, ());
            }
            OrbitRow::Opened(k) => {
                axes.put(k, ());
            }
        }
    }
    Some(Rank {
        torsion: torsion.len() as u32,
        free: z_rank(&free)?,
        offplane: offplane.len() as u32,
        axes: axes.len() as u32,
    })
}
