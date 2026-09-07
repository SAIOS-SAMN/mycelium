// 1.1, 1.2 — THE SEVENTEEN LETTERS and the relators between them.
//
//! One identity, six quarter turns, six Pythagorean (3,4,5) rotors and four
//! (5,12,13) rotors, each the one pair with its carried inverse, exact over
//! the rationals. The relators are the equalities the alphabet satisfies;
//! they are stated, never searched for.

use crate::transducer::alphabet::{Rotor, Word, Q};

use super::*;

/// LAW VIII cardinality — the single declaration site.
pub const CAYLEY_ALPHABET_LEN: usize = 17;

/// Rank of the Euclidean chart the letters occupy (`e0..e3`).
pub const LETTER_RANK: usize = 4;

/// The sensor's plane inside the chart: rows on `e0`, columns on `e1`.
pub const PLANE_ROW: u8 = 0;
pub const PLANE_COL: u8 = 1;

/// Opaque letter identifier. Range `0..17` at the constructor; a wire
/// identifier only — the letter's content is its rotor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LetterId(u8);

/// Six Euclidean pair planes on `e0..e3` (slots 1–6 and 7–12).
pub(crate) const SLOT_PLANES_SIX: [(u8, u8); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
/// The four (5,12,13) planes (slots 13–16).
pub(crate) const SLOT_PLANES_13: [(u8, u8); 4] = [(0, 1), (0, 2), (0, 3), (1, 2)];

impl LetterId {
    pub const IDENTITY: LetterId = LetterId(0);

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

    /// The quarter turn on a pair plane (slots 1–6).
    pub fn quarter(plane: (u8, u8)) -> Option<LetterId> {
        SLOT_PLANES_SIX.iter().position(|p| *p == plane).map(|i| LetterId(1 + i as u8))
    }

    /// The (3,4,5) rotor on a pair plane (slots 7–12).
    pub fn pythagorean_5(plane: (u8, u8)) -> Option<LetterId> {
        SLOT_PLANES_SIX.iter().position(|p| *p == plane).map(|i| LetterId(7 + i as u8))
    }

    /// The (5,12,13) rotor on a pair plane (slots 13–16).
    pub fn pythagorean_13(plane: (u8, u8)) -> Option<LetterId> {
        SLOT_PLANES_13.iter().position(|p| *p == plane).map(|i| LetterId(13 + i as u8))
    }

    /// Every letter, in slot order.
    pub fn all() -> impl Iterator<Item = LetterId> {
        (0..CAYLEY_ALPHABET_LEN as u8).map(LetterId)
    }
}

/// The pair plane of a letter; the identity has none.
pub fn plane_of(id: LetterId) -> Option<(u8, u8)> {
    match id.0 {
        0 => None,
        1..=6 => Some(SLOT_PLANES_SIX[(id.0 - 1) as usize]),
        7..=12 => Some(SLOT_PLANES_SIX[(id.0 - 7) as usize]),
        _ => Some(SLOT_PLANES_13[(id.0 - 13) as usize]),
    }
}

/// The rotor pair `(s, b)` of a letter: `Φ = s + b·e_ij`.
pub fn rotor_pair(id: LetterId) -> (Q, Q) {
    match id.0 {
        0 => (q(1), q(0)),
        1..=6 => (q(1), q(1)),
        7..=12 => (qn(3, 5), qn(4, 5)),
        _ => (qn(5, 13), qn(12, 13)),
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
    matches!(
        (a, b),
        ((0, 1), (2, 3))
            | ((2, 3), (0, 1))
            | ((0, 2), (1, 3))
            | ((1, 3), (0, 2))
            | ((0, 3), (1, 2))
            | ((1, 2), (0, 3))
    )
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
    QuarterOrderFour,
    PlaneCommute,
    SheetConjugate,
    PrimeInfinite,
}

pub(crate) const RELATOR_N: usize = 12;

/// Grammar of the seventeen: quarter turns order four, complementary
/// planes commute, a sheet conjugates a rotation to its inverse, prime
/// letters have infinite order.
pub fn relators() -> [Relator; RELATOR_N] {
    let q = |plane: (u8, u8)| LetterId::quarter(plane).expect("pair plane");
    let mut out: [Relator; RELATOR_N] = std::array::from_fn(|_| Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: None,
        word: Word::identity(),
    });
    let mut i = 0usize;
    for &plane in &SLOT_PLANES_SIX {
        let id = q(plane);
        // q^4 = 1 — stored as the raw tape of four quarters, then reduced
        // to the identity vertex.
        let w = Word::from_letters(&[(id, false), (id, false), (id, false), (id, false)]);
        out[i] = Relator { kind: RelatorKind::QuarterOrderFour, plane: Some(plane), word: w };
        i += 1;
    }
    let complements: [((u8, u8), (u8, u8)); 3] = [((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2))];
    for &(a, b) in &complements {
        let qa = q(a);
        let qb = q(b);
        // aba^{-1}b^{-1} = 1
        let w = Word::from_letters(&[(qa, false), (qb, false), (qa, true), (qb, true)]);
        out[i] = Relator { kind: RelatorKind::PlaneCommute, plane: Some(a), word: w };
        i += 1;
    }
    let sheet = LetterId::quarter((1, 2)).expect("sheet plane");
    // S² = 1: four quarter-turns on the sheet plane reduce to the identity.
    let w = Word::from_letters(&[(sheet, false), (sheet, false), (sheet, false), (sheet, false)]);
    out[i] = Relator {
        kind: RelatorKind::SheetConjugate,
        plane: Some((1, 2)),
        word: w,
    };
    i += 1;
    out[i] = Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: Some((0, 1)),
        word: Word::from_letters(&[(LetterId::pythagorean_5((0, 1)).unwrap(), false)]),
    };
    i += 1;
    out[i] = Relator {
        kind: RelatorKind::PrimeInfinite,
        plane: Some((0, 1)),
        word: Word::from_letters(&[(LetterId::pythagorean_13((0, 1)).unwrap(), false)]),
    };
    out
}
