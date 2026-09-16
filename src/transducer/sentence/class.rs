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
#[derive(Clone, Debug)]
pub struct ClassVector {
    /// Sensor-chart section: `None` when the holonomy moves the plane.
    plane: Option<[(bool, u32, i64, i64); alphabet::PAIR_PLANES]>,
    /// Holonomy carriage after the tree gauge (A4).
    pub(crate) carriage: Versor,
    /// Composed gauge around the loop (A4).
    pub(crate) hue: CycleType,
    /// Rotor conjugacy invariants `(a²/w, p²/w, sign p)` (15.3). Always
    /// read; identity is the zero vector. The signs are the RAY's, not a
    /// lift's — see [`crate::transducer::alphabet::Rotor::class_invariants`],
    /// which is why there is one sign byte here and not two.
    offplane: (num_rational::BigRational, num_rational::BigRational, u8),
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
                plane: Some([(false, 0, 0, 0); alphabet::PAIR_PLANES]),
                carriage,
                hue,
                offplane: Self::offplane_zero(),
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
            plane: Some([(false, 0, 0, 0); alphabet::PAIR_PLANES]),
            carriage: element::identity_versor(),
            hue: CycleType::Identity,
            offplane: Self::offplane_zero(),
        }
    }

    /// Coordinates written out in 12.6 axis order. Rotor conjugacy first so
    /// Canonical coordinates written sequentially (Rule 6).
    pub fn put_canonical(&self, buf: &mut Vec<u8>) {
        alphabet::put_q(buf, &self.offplane.0);
        alphabet::put_q(buf, &self.offplane.1);
        buf.push(self.offplane.2);
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
    /// conjugacy class is the depth-one projection of a loop's state vector
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
        let sp = *k.get(p)?;
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
        Some(ClassVector {
            plane,
            carriage,
            hue,
            offplane: (x, y, sp),
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

    fn offplane_zero() -> (num_rational::BigRational, num_rational::BigRational, u8) {
        (Self::zero_q(), Self::zero_q(), 0)
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
    /// order). A loop with a free part is a demand, never filed torsion.
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

/// The A4 carriage as `Mv::put_canonical` writes it: sixteen coefficients,
/// each a sign byte and two length-framed integers.
struct Carriage;

impl Segment for Carriage {
    fn name(&self) -> &'static str {
        "carriage"
    }

    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()> {
        for _ in 0..16 {
            take_u8(key, pos)?;
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

static CLASS_SEGMENTS: [&dyn Segment; 4] = [&CONJUGACY, &PLANE_SECTION, &HUE_CYCLE, &CARRIAGE];

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

/// What one loop contributes to rank (A18): read once per loop, so a trial
/// that changes one loop's holonomy swaps one row instead of re-reading the
/// nerve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoopRow {
    Closed,
    /// The loop's finite-order class. CARRIED, because the audit counts
    /// torsion by class and not by loop — see [`rank_of_rows`].
    Torsion(crate::transducer::address::Address),
    Free([i64; FREE_ROW]),
    OffPlane(crate::transducer::address::Address),
    /// The loop's class is a chart axis the form opened (I6, 15.5): its
    /// holonomy is the transition on that axis, so it counts on `axes`
    /// and on no other coordinate. The class is carried for the same
    /// reason `Torsion` carries it: the axis carrier is keyed by class, so
    /// one opening opens every loop of that class and they are one axis.
    Opened(crate::transducer::address::Address),
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
        // The zero row is FREE_ROW wide, which is the width `free_row`
        // returns. It was written out as `12`, a second statement of
        // `PAIR_PLANES * 2` that a change of rank would have left behind
        // (rule 5).
        Some(row) if row == [0i64; FREE_ROW] => LoopRow::Torsion(h.rotor().class_key()?),
        Some(row) => LoopRow::Free(row),
        None => LoopRow::OffPlane(h.rotor().class_key()?),
    })
}

/// A18 over the rows of a nerve's cycles or a form's loops.
///
/// EVERY COORDINATE COUNTS BY CLASS. A rank is the dimension of a module —
/// the number of INDEPENDENT generators — never a tally of the loops that
/// exhibit them, and two of these four already read it that way: `free` is
/// `z_rank`, a matrix rank over the free rows, and `offplane` is a keyed set
/// taking each class once. `torsion` and `axes` counted per loop.
///
/// Mixing the two breaks the rank delta itself. Two loops of one finite
/// class are ONE obstruction seen twice; the axis carrier is keyed by class,
/// so opening it opens both, and attaching that single cell moved
/// `torsion −2, axes +2` while the same object contributes 1 to `offplane`.
/// T21 (iii) then refuses an elementary attachment as non-elementary.
/// Measured at 551c609: 11 `axis Open discharged` against 6 `held, the
/// applied pivot did not make the move its certificate claims`, all
/// openings.
///
/// Counted by class the same attachment reads `torsion −1, axes +1`, which
/// is `Rank::moved`'s `Discharged { from: Torsion }` and A18's one step.
///
/// A CLASS CANNOT BE BOTH. A key present in `axes` is an opened chart axis
/// and no longer a defect (I6, 15.5), so it is not also counted in torsion;
/// the rows already say which, because `loop_row_on` answers `Opened` for a
/// loop whose class the carrier holds and never `Torsion`.
pub fn rank_of_rows(rows: &[LoopRow]) -> Option<Rank> {
    let mut torsion: Keyed<()> = Keyed::new();
    let mut axes: Keyed<()> = Keyed::new();
    let mut offplane: Keyed<()> = Keyed::new();
    let mut free: Vec<[i64; FREE_ROW]> = Vec::new();
    for r in rows {
        match r {
            LoopRow::Closed => {}
            LoopRow::Torsion(k) => {
                torsion.insert(k, ());
            }
            LoopRow::Free(row) => free.push(*row),
            LoopRow::OffPlane(k) => {
                offplane.insert(k, ());
            }
            LoopRow::Opened(k) => {
                axes.insert(k, ());
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
