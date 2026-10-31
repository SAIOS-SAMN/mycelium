// 1.1, 1.2 — THE SEVENTEEN LETTERS and the relators between them.
//
//! One identity, six 90° turns, six Pythagorean (3,4,5) rotors and four
//! (5,12,13) rotors, each the one pair with its carried inverse, exact over
//! the rationals. The relators are stated, never searched for.

use crate::transducer::alphabet::{Rotor, Word, Q};

use super::*;

/// Rank of the Euclidean chart the letters occupy (`e0..e3`).
pub const LETTER_RANK: usize = 4;

/// The sensor's plane inside the chart: rows on `e0`, columns on `e1`.
pub const PLANE_ROW: u8 = 0;
pub const PLANE_COL: u8 = 1;

/// THE ALPHABET'S SLOT FOR A BASE AXIS (51.2): rows on `e0`, columns on
/// `e1`, the frame on `e3`; `e2` is the orientation character and no base
/// axis. An axis past the frame has no slot: the seventeen act on the
/// planes they were written on, and a plane a larger `n` opens receives a
/// letter by 37.7 or not at all (71.2(3)).
pub fn slot_of_axis(axis: usize) -> Option<u8> {
    match axis {
        0 => Some(PLANE_ROW),
        1 => Some(PLANE_COL),
        2 => Some(3),
        _ => None,
    }
}

/// THE BASE AXIS A SLOT STANDS FOR, the reading [`slot_of_axis`] inverts:
/// `e0` rows, `e1` columns, `e3` the frame. `None` for `e2`, the
/// orientation character, which is no base axis — the chart's off-plane
/// channel, carried by the conformal algebra's `e₊` (65.6). Every slot is
/// its axis's slot and no other (`tests` holds the two against each other).
pub fn axis_of_slot(slot: u8) -> Option<usize> {
    match slot {
        PLANE_ROW => Some(0),
        PLANE_COL => Some(1),
        3 => Some(2),
        _ => None,
    }
}

/// A plane of the base `(i, j)` in the alphabet's slots, where both of its
/// axes have one (BQ4).
pub fn slot_plane(plane: (usize, usize)) -> Option<(u8, u8)> {
    Some((slot_of_axis(plane.0)?, slot_of_axis(plane.1)?))
}

/// THE PAIR PLANES OF THE CHART: every two-subset of `e0..e3`. It is
/// `rank choose 2`, read off the rank rather than written beside it — a
/// count stated twice is a count that can disagree with itself, and the
/// six here were spelled into a table, into its type, into the name
/// `SLOT_PLANES_SIX`, and into every `p < 6` and `1..=6` that read it.
pub const PAIR_PLANES: usize = LETTER_RANK * (LETTER_RANK - 1) / 2;

/// The two-subsets in lexicographic order, which is slot order. Derived,
/// so a chart of another rank moves the alphabet with it instead of
/// leaving a table behind.
const fn pair_planes_of_rank() -> [(u8, u8); PAIR_PLANES] {
    let mut out = [(0u8, 0u8); PAIR_PLANES];
    let mut k = 0usize;
    let mut i = 0u8;
    while (i as usize) < LETTER_RANK {
        let mut j = i + 1;
        while (j as usize) < LETTER_RANK {
            out[k] = (i, j);
            k += 1;
            j += 1;
        }
        i += 1;
    }
    out
}

/// LAW VIII cardinality, DERIVED from what the alphabet holds: the
/// identity, a 90° turn and a (3,4,5) rotor on every pair plane, and a
/// (5,12,13) rotor on the planes that carry one. It was `17` written out,
/// which is the same fact as the three tables below stated a fourth time.
pub const CAYLEY_ALPHABET_LEN: usize = 1 + 2 * PAIR_PLANES + SLOT_PLANES_13.len();

/// Opaque letter identifier. Range `0..17` at the constructor; a wire
/// identifier only — the letter's content is its rotor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LetterId(u8);

/// The pair planes in slot order, carrying the 90° turns and the
/// (3,4,5) rotors.
pub(crate) const SLOT_PLANES_SIX: [(u8, u8); PAIR_PLANES] = pair_planes_of_rank();

/// THE PLANES THAT CARRY A (5,12,13) ROTOR. This is content about the
/// alphabet, not a count: four of the pair planes have such a letter and
/// two do not, and nothing in the rank says which. Its LENGTH is read
/// wherever that number was written out — `CAYLEY_ALPHABET_LEN` above, and
/// `LetterId::pythagorean_13` answers the membership question for every
/// reader that used to test `p < 4`.
pub(crate) const SLOT_PLANES_13: [(u8, u8); 4] = [(0, 1), (0, 2), (0, 3), (1, 2)];

/// THE SENSOR PLANE as a pair, so a reader asking "is this the sensor's"
/// compares planes instead of testing a slot index against zero.
pub const fn sensor_plane() -> (u8, u8) {
    (PLANE_ROW, PLANE_COL)
}

/// THE PLANE WHOSE 180°-TURN IS THE ORIENTATION CHARACTER of `plane`: the
/// pair of `plane.1` with the least axis outside `plane`. `SpellStep`
/// derives its flip letter exactly this way; this is that derivation named
/// once so `LetterId::SHEET` can be checked against it rather than written
/// out beside it.
pub const fn flip_plane(plane: (u8, u8)) -> Option<(u8, u8)> {
    let mut k = 0u8;
    while (k as usize) < LETTER_RANK {
        if k != plane.0 && k != plane.1 {
            return Some(if plane.1 < k { (plane.1, k) } else { (k, plane.1) });
        }
        k += 1;
    }
    None
}

/// Slot bases, each read off what precedes it: the identity, then a
/// 90° turn per pair plane, then a (3,4,5) rotor per pair plane, then
/// the (5,12,13) rotors. They were `1`, `7` and `13` written out at every
/// reader, which is `PAIR_PLANES` restated in three places.
const TURN_90_BASE: u8 = 1;
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the algebra's range: PAIR_PLANES = C(LETTER_RANK = 4, 2) = 6, a const expression"
)]
const PRIME_5_BASE: u8 = TURN_90_BASE + PAIR_PLANES as u8;
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the algebra's range: PAIR_PLANES = C(LETTER_RANK = 4, 2) = 6, a const expression"
)]
const PRIME_13_BASE: u8 = PRIME_5_BASE + PAIR_PLANES as u8;

impl LetterId {
    pub const IDENTITY: LetterId = LetterId(0);
    /// The 90° turn on the sheet plane `(1, 2)` (slot 4): two of them
    /// are the orientation character of the sensor plane; there is no
    /// reflection letter.
    pub const SHEET: LetterId = LetterId(4);
    /// The (3,4,5) rotor on the sensor plane `(0, 1)` (slot 7).
    pub const PRIME_5: LetterId = LetterId(7);
    /// The (5,12,13) rotor on the sensor plane `(0, 1)` (slot 13).
    pub const PRIME_13: LetterId = LetterId(13);

    /// Refuses `id >= 17` typed; the range comparison lives here.
    pub const fn try_new(id: u8) -> Option<LetterId> {
        if (id as usize) < CAYLEY_ALPHABET_LEN {
            Some(LetterId(id))
        } else {
            None
        }
    }

    #[inline]
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// The 90° turn on a pair plane.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the algebra's range: a pair plane's slot in SLOT_PLANES_SIX, one of PAIR_PLANES = 6"
    )]
    pub fn turn_90(plane: (u8, u8)) -> Option<LetterId> {
        Self::slot_of(plane).map(|i| LetterId(TURN_90_BASE + i as u8))
    }

    /// The (3,4,5) rotor on a pair plane.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the algebra's range: a pair plane's slot in SLOT_PLANES_SIX, one of PAIR_PLANES = 6"
    )]
    pub fn pythagorean_5(plane: (u8, u8)) -> Option<LetterId> {
        Self::slot_of(plane).map(|i| LetterId(PRIME_5_BASE + i as u8))
    }

    /// The (5,12,13) rotor on a pair plane. `None` is the reading for a
    /// plane the alphabet carries no such letter on, and it is the question
    /// every `p < 4` was asking without being able to name it.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — a constant table: a position in SLOT_PLANES_13, 4 entries"
    )]
    pub fn pythagorean_13(plane: (u8, u8)) -> Option<LetterId> {
        SLOT_PLANES_13
            .iter()
            .position(|p| *p == plane)
            .map(|i| LetterId(PRIME_13_BASE + i as u8))
    }

    /// The slot a pair plane occupies, which is its position in the
    /// chart's own lexicographic order of two-subsets — content, and the
    /// one construction the three letter readers above share.
    pub fn slot_of(plane: (u8, u8)) -> Option<usize> {
        SLOT_PLANES_SIX.iter().position(|p| *p == plane)
    }

    /// Every letter, in slot order.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the algebra's range: CAYLEY_ALPHABET_LEN = 1 + 2·PAIR_PLANES + 4 = 17 letters, a const expression"
    )]
    pub fn all() -> impl Iterator<Item = LetterId> {
        (0..CAYLEY_ALPHABET_LEN as u8).map(LetterId)
    }
}

/// The pair plane of a letter; the identity has none. The block a slot
/// falls in is read against the bases rather than against `1..=6` and
/// `7..=12`, which are `PAIR_PLANES` written out twice more.
pub fn plane_of(id: LetterId) -> Option<(u8, u8)> {
    let i = id.0;
    if i == LetterId::IDENTITY.0 {
        return None;
    }
    if i < PRIME_5_BASE {
        return SLOT_PLANES_SIX.get((i - TURN_90_BASE) as usize).copied();
    }
    if i < PRIME_13_BASE {
        return SLOT_PLANES_SIX.get((i - PRIME_5_BASE) as usize).copied();
    }
    SLOT_PLANES_13.get((i - PRIME_13_BASE) as usize).copied()
}

/// The rotor pair `(s, b)` of a letter: `Φ = s + b·e_ij`.
pub fn rotor_pair(id: LetterId) -> (Q, Q) {
    let i = id.0;
    if i == LetterId::IDENTITY.0 {
        (q(1), q(0))
    } else if i < PRIME_5_BASE {
        (q(1), q(1))
    } else if i < PRIME_13_BASE {
        (qn(3, 5), qn(4, 5))
    } else {
        (qn(5, 13), qn(12, 13))
    }
}

/// THE PLANE ON THE AXES `plane` DOES NOT USE: its complement in the
/// chart. Two complementary planes share no axis and therefore commute
/// (1.2), so this is the relation `complementary` tests, computed rather
/// than tabulated — the six-case match below was the rank-4 answer written
/// out, and a chart of another rank had no entry in it.
pub const fn complement_plane(plane: (u8, u8)) -> Option<(u8, u8)> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the algebra's range: LETTER_RANK = 4, the letter chart's axis count e0..e3"
    )]
    let none = LETTER_RANK as u8;
    let (mut lo, mut hi) = (none, none);
    let mut k = 0u8;
    while (k as usize) < LETTER_RANK {
        if k != plane.0 && k != plane.1 {
            if lo == none {
                lo = k;
            } else if hi == none {
                hi = k;
            }
        }
        k += 1;
    }
    if hi == none {
        None
    } else {
        Some((lo, hi))
    }
}

/// The rotor `Φ` of a letter.
pub fn letter(id: LetterId) -> Rotor {
    let (s, b) = rotor_pair(id);
    match plane_of(id) {
        None => Rotor::scalar(s),
        Some((i, j)) => Rotor::scalar(s).add(&Rotor::blade((1 << i) | (1 << j), b)),
    }
}

/// The carried inverse `Φ⁻¹` of a letter — exact, never computed by a
/// general inverse at the point of use.
pub fn letter_inverse(id: LetterId) -> Rotor {
    let (s, b) = rotor_pair(id);
    match plane_of(id) {
        None => Rotor::scalar(s),
        Some((i, j)) => {
            let mask = (1 << i) | (1 << j);
            match id.0 {
                1..=6 => Rotor::scalar(qn(1, 2)).add(&Rotor::blade(mask, qn(-1, 2))),
                _ => Rotor::scalar(s).add(&Rotor::blade(mask, -b)),
            }
        }
    }
}

pub(crate) fn complementary(a: (u8, u8), b: (u8, u8)) -> bool {
    complement_plane(a) == Some(b)
}

/// A relation of the Cayley group: a tape that multiplies to the identity,
/// or a prime letter of infinite order (empty tape, kind records it).
#[derive(Debug, Clone)]
pub struct Relator {
    pub kind: RelatorKind,
    pub plane: Option<(u8, u8)>,
    pub word: Word,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelatorKind {
    Turn90OrderFour,
    PlaneCommute,
    SheetConjugate,
    PrimeInfinite,
}

/// The relators, counted from what the grammar states: a 90° turn of
/// order four on every pair plane, one commuting relation per
/// complementary pair, the sheet's square, and the two prime letters of
/// infinite order. It was `12`, which is those four facts added up by hand.
pub(crate) const RELATOR_N: usize = PAIR_PLANES + PAIR_PLANES / 2 + 1 + 2;

/// Grammar of the seventeen: 90° turns order four, complementary
/// planes commute, a sheet conjugates a rotation to its inverse, prime
/// letters have infinite order.
pub fn relators() -> [Relator; RELATOR_N] {
    // The letters are named by slot (law VIII), never searched for by
    // plane: a 90° turn in slot `i` of the six planes is letter `1 + i`.
    // It looked each one up and panicked on a plane it declared itself (§2.4).
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the algebra's range: a 90°-turn slot, one of PAIR_PLANES = 6"
    )]
    let turn_90_in = |slot: usize| LetterId(TURN_90_BASE + slot as u8);
    let mut out: [Relator; RELATOR_N] = std::array::from_fn(|_| Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: None,
        word: Word::identity(),
    });
    let mut i = 0usize;
    for (slot, &plane) in SLOT_PLANES_SIX.iter().enumerate() {
        let id = turn_90_in(slot);
        // q^4 = 1 — stored as the raw tape of four 90° turns, then reduced
        // to the identity vertex.
        let w = Word::from_letters(&[(id, false), (id, false), (id, false), (id, false)]);
        out[i] = Relator { kind: RelatorKind::Turn90OrderFour, plane: Some(plane), word: w };
        i += 1;
    }
    // COMPLEMENTARY PLANES COMMUTE, and each pair is stated once. The
    // slot pairs were written out as `[(0, 5), (1, 4), (2, 3)]`, which is
    // the rank-4 answer tabulated; the complement is a reading of the
    // plane's own axes (`complement_plane`), and the read is over the
    // planes themselves rather than over the planes' product (rule 1). A pair is
    // emitted at its lesser slot, so each relation appears once.
    for (a, &plane) in SLOT_PLANES_SIX.iter().enumerate() {
        let Some(other) = complement_plane(plane) else {
            continue;
        };
        let Some(b) = LetterId::slot_of(other) else {
            continue;
        };
        if b <= a {
            continue;
        }
        let qa = turn_90_in(a);
        let qb = turn_90_in(b);
        // aba^{-1}b^{-1} = 1
        let w = Word::from_letters(&[(qa, false), (qb, false), (qa, true), (qb, true)]);
        out[i] = Relator { kind: RelatorKind::PlaneCommute, plane: Some(plane), word: w };
        i += 1;
    }
    let sheet = LetterId::SHEET;
    // S² = 1: four 90°-turns on the sheet plane reduce to the identity.
    let w = Word::from_letters(&[(sheet, false), (sheet, false), (sheet, false), (sheet, false)]);
    out[i] = Relator {
        kind: RelatorKind::SheetConjugate,
        plane: plane_of(sheet),
        word: w,
    };
    i += 1;
    out[i] = Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: plane_of(LetterId::PRIME_5),
        word: Word::from_letters(&[(LetterId::PRIME_5, false)]),
    };
    i += 1;
    out[i] = Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: plane_of(LetterId::PRIME_13),
        word: Word::from_letters(&[(LetterId::PRIME_13, false)]),
    };
    out
}
