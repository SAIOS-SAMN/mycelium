// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE CAYLEY ALPHABET (LAW VIII) — seventeen immutable letters.
//!
//! The alphabet is exactly seventeen generators on the Euclidean rank-4
//! chart `e0..e3`: one identity, six quarter turns (one per pair plane),
//! six Pythagorean (3,4,5) rotors and four Pythagorean (5,12,13) rotors.
//! Every letter is the one pair `Φ` with its carried `Φ⁻¹`, exact over ℚ:
//!
//! * quarter turn on `e_i∧e_j`:  `Φ = 1 + e_ij`,        `Φ⁻¹ = (1 − e_ij)/2`
//! * (3,4,5):                     `Φ = 3/5 + 4/5·e_ij`,   `Φ⁻¹ = 3/5 − 4/5·e_ij`
//! * (5,12,13):                   `Φ = 5/13 + 12/13·e_ij`, `Φ⁻¹ mirrored`
//!
//! A letter acts by the sandwich `Φ v Φ̃ / (Φ Φ̃)` — the rotor's rotation,
//! which is twice the rotor's own half-angle. No angles, no radians, no
//! roots: exact Pythagorean transports only. The alphabet never grows; the
//! language grows by WORDS — oriented tapes of letters whose product is one
//! rotor — minted on measured obstruction and adjudicated by the court.
//!
//! `rot90`, `rot180`, the flips and the transposition are not letters we
//! add: they are words already in the seventeen. A flip of the sensor's
//! plane is the shadow of a half turn in a plane that uses the third axis,
//! so the alphabet has no odd element and needs none.
//!
//! SPELLING is the peel that reduces a measured similarity of the plane to
//! the alphabet: the orientation sheet is peeled first, then the prime-5
//! letters, then the prime-13 letters, then the unit turns — each shell
//! reads a valuation and removes exactly one syllable, never trying and
//! verifying candidates. A rotation whose Gaussian factors lie outside the
//! alphabet's primes stands as a remainder: honest dialect, not a word.
//!
//! Base geometry: everything above (the conformal versors of `cga`, the
//! lattice isometries, the frame) is realised FROM a word, never beside it.

use crate::transducer::peel::{peel_nest, PeelNest, ShellMotion, ShellStep, ShrinkGovernor};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

pub type Q = BigRational;

/// LAW VIII cardinality — the single declaration site.
pub const CAYLEY_ALPHABET_LEN: usize = 17;

/// Rank of the Euclidean chart the letters occupy (`e0..e3`).
pub const LETTER_RANK: usize = 4;

/// The sensor's plane inside the chart: rows on `e0`, columns on `e1`.
pub const PLANE_ROW: u8 = 0;
pub const PLANE_COL: u8 = 1;

fn q(v: i64) -> Q {
    Q::from_integer(BigInt::from(v))
}
fn qn(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// Opaque letter identifier. Range `0..17` at the constructor; a wire
/// identifier only — the letter's content is its rotor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LetterId(u8);

/// Six Euclidean pair planes on `e0..e3` (slots 1–6 and 7–12).
const SLOT_PLANES_SIX: [(u8, u8); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
/// The four (5,12,13) planes (slots 13–16).
const SLOT_PLANES_13: [(u8, u8); 4] = [(0, 1), (0, 2), (0, 3), (1, 2)];

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

// ---- the exact algebra Cl(4,0) over ℚ ----

/// An exact multivector of `Cl(4,0)`: 16 blade coefficients indexed by a
/// 4-bit mask over `e0..e3`, every generator squaring to +1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotor {
    c: [Q; 16],
}

/// The parity of the transpositions sorting blade `a` before blade `b`.
fn reorder_sign(a: usize, b: usize) -> i64 {
    let mut hi = a >> 1;
    let mut swaps = 0u32;
    while hi != 0 {
        swaps += (hi & b).count_ones();
        hi >>= 1;
    }
    if swaps & 1 == 0 {
        1
    } else {
        -1
    }
}

impl Rotor {
    pub fn zero() -> Rotor {
        Rotor { c: std::array::from_fn(|_| Q::zero()) }
    }

    pub fn scalar(v: Q) -> Rotor {
        let mut r = Rotor::zero();
        r.c[0] = v;
        r
    }

    pub fn identity() -> Rotor {
        Rotor::scalar(Q::one())
    }

    pub fn blade(mask: usize, v: Q) -> Rotor {
        let mut r = Rotor::zero();
        r.c[mask & 15] = v;
        r
    }

    pub fn coeff(&self, mask: usize) -> &Q {
        &self.c[mask & 15]
    }

    pub fn add(&self, o: &Rotor) -> Rotor {
        Rotor { c: std::array::from_fn(|i| &self.c[i] + &o.c[i]) }
    }

    pub fn scale(&self, s: &Q) -> Rotor {
        Rotor { c: std::array::from_fn(|i| &self.c[i] * s) }
    }

    /// The geometric product, Euclidean metric.
    pub fn geo(&self, o: &Rotor) -> Rotor {
        let mut r = Rotor::zero();
        for i in 0..16 {
            if self.c[i].is_zero() {
                continue;
            }
            for j in 0..16 {
                if o.c[j].is_zero() {
                    continue;
                }
                let term = &self.c[i] * &o.c[j];
                let out = i ^ j;
                if reorder_sign(i, j) >= 0 {
                    r.c[out] = &r.c[out] + &term;
                } else {
                    r.c[out] = &r.c[out] - &term;
                }
            }
        }
        r
    }

    /// The reverse `Φ̃`.
    pub fn reverse(&self) -> Rotor {
        Rotor {
            c: std::array::from_fn(|i| {
                let g = (i as u32).count_ones();
                if (g * g.wrapping_sub(1) / 2) & 1 == 0 {
                    self.c[i].clone()
                } else {
                    -&self.c[i]
                }
            }),
        }
    }

    /// `Φ Φ̃` when it is a nonzero scalar — the rotor's own weight; `None`
    /// for anything that is not a versor.
    pub fn weight(&self) -> Option<Q> {
        let p = self.geo(&self.reverse());
        let s = p.c[0].clone();
        if s.is_zero() || (1..16).any(|i| !p.c[i].is_zero()) {
            return None;
        }
        Some(s)
    }

    /// The exact inverse `Φ̃ / (Φ Φ̃)`.
    pub fn inverse(&self) -> Option<Rotor> {
        let w = self.weight()?;
        Some(self.reverse().scale(&(Q::one() / w)))
    }

    /// The sandwich on a vector of the chart, scale divided out exactly:
    /// `Φ v Φ̃ / (Φ Φ̃)`. `None` when the result is not a pure vector (the
    /// element was not a versor).
    pub fn carry(&self, v: &[Q; 4]) -> Option<[Q; 4]> {
        let w = self.weight()?;
        let mut x = Rotor::zero();
        for k in 0..4 {
            x.c[1 << k] = v[k].clone();
        }
        let out = self.geo(&x).geo(&self.reverse());
        if (0..16).any(|i| (i as u32).count_ones() != 1 && !out.c[i].is_zero()) {
            return None;
        }
        Some(std::array::from_fn(|k| &out.c[1 << k] / &w))
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

// ---- words ----

/// A WORD: an oriented tape of letters. `true` reads the letter backwards
/// (its carried `Φ⁻¹`). The word's rotor is the product in tape order, so
/// the LAST letter acts first on a vector.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Word(Vec<(LetterId, bool)>);

impl Word {
    pub fn identity() -> Word {
        Word(Vec::new())
    }

    pub fn from_letters(tape: &[(LetterId, bool)]) -> Word {
        Word(tape.to_vec())
    }

    pub fn letters(&self) -> &[(LetterId, bool)] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, id: LetterId, inverse: bool) {
        self.0.push((id, inverse));
    }

    /// The formed product `γ` of the tape.
    pub fn rotor(&self) -> Rotor {
        self.0.iter().fold(Rotor::identity(), |acc, &(id, inv)| {
            acc.geo(&if inv { letter_inverse(id) } else { letter(id) })
        })
    }

    /// The word read backwards: the inverse transport.
    pub fn inverse(&self) -> Word {
        Word(self.0.iter().rev().map(|&(id, inv)| (id, !inv)).collect())
    }

    /// Concatenate tapes: `self` then `other` (last letter of `other` acts first).
    pub fn compose(&self, other: &Word) -> Word {
        let mut t = self.0.clone();
        t.extend_from_slice(&other.0);
        Word(t)
    }

    /// The plane group is rotations ⋊ sheet parity, not a commuting tape.
    /// `None` when the word does not preserve the sensor plane.
    pub fn form(&self) -> Option<PlaneForm> {
        let b = self.plane_block()?;
        let det = &b[0][0] * &b[1][1] - &b[0][1] * &b[1][0];
        let rho2 = &b[0][0] * &b[0][0] + &b[1][0] * &b[1][0];
        if rho2.is_zero() {
            return None;
        }
        let ori = det / &rho2;
        let parity = if ori.is_one() {
            false
        } else if (-&ori).is_one() {
            true
        } else {
            return None;
        };
        let mut r = b;
        if parity {
            r[0][1] = -&r[0][1];
            r[1][1] = -&r[1][1];
        }
        let sp = spell(&r)?;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        for &(id, inv) in sp.word.letters() {
            rot[id.raw() as usize] += if inv { -1 } else { 1 };
        }
        Some(PlaneForm { parity, rot })
    }

    /// Two words are the same plane element: same sheet parity and the
    /// same rotation, never a commuting exponent sum.
    pub fn same_form(&self, other: &Word) -> bool {
        match (self.form(), other.form()) {
            (Some(a), Some(b)) => a == b,
            (None, None) => self.0 == other.0,
            _ => false,
        }
    }

    /// The plane element is the identity: even sheet, trivial rotation.
    pub fn abelian_identity(&self) -> bool {
        self.form().is_some_and(|f| f.is_identity())
    }

    /// Exact period of the plane element. A sheet has order 2. A rotation
    /// closes iff its prime exponents vanish and the unit (quarter) exponent
    /// is read mod 4. None when the word does not close under repetition.
    pub fn period(&self) -> Option<u32> {
        let f = self.form()?;
        f.period()
    }

    /// The exact 4×4 matrix of the word on the chart, columns the images
    /// of `e0..e3`.
    pub fn matrix(&self) -> Option<[[Q; 4]; 4]> {
        let r = self.rotor();
        let mut cols: Vec<[Q; 4]> = Vec::with_capacity(4);
        for k in 0..4 {
            let e: [Q; 4] = std::array::from_fn(|i| if i == k { Q::one() } else { Q::zero() });
            cols.push(r.carry(&e)?);
        }
        Some(std::array::from_fn(|row| std::array::from_fn(|col| cols[col][row].clone())))
    }

    /// The word's action on the sensor's plane `(e0, e1)` as a 2×2 block —
    /// only when the word preserves that plane (no third axis mixed in).
    pub fn plane_block(&self) -> Option<[[Q; 2]; 2]> {
        let m = self.matrix()?;
        let closed = (0..2).all(|col| (2..4).all(|row| m[row][col].is_zero()));
        if !closed {
            return None;
        }
        Some([[m[0][0].clone(), m[0][1].clone()], [m[1][0].clone(), m[1][1].clone()]])
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.0.len() as u16).to_le_bytes());
        for &(id, inv) in &self.0 {
            out.push(id.raw() | if inv { 0x80 } else { 0 });
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Word> {
        let n = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
        *pos += 2;
        let mut tape = Vec::with_capacity(n);
        for _ in 0..n {
            let byte = *b.get(*pos)?;
            *pos += 1;
            tape.push((LetterId::try_new(byte & 0x7f)?, byte & 0x80 != 0));
        }
        Some(Word(tape))
    }
}

/// The plane group element a word represents: sheet parity and the
/// rotation in (e0, e1). Rotation exponents are those of the unflipped
/// spelling; a sheet conjugates a rotation to its inverse, so this is
/// not the commuting sum of the tape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaneForm {
    pub parity: bool,
    pub rot: [i64; CAYLEY_ALPHABET_LEN],
}

impl PlaneForm {
    pub fn is_identity(&self) -> bool {
        !self.parity && self.rot.iter().all(|&x| x == 0)
    }

    pub fn period(&self) -> Option<u32> {
        if self.is_identity() {
            return Some(1);
        }
        if self.parity {
            return Some(2);
        }
        let q01 = LetterId::quarter((0, 1)).map(|id| id.raw() as usize);
        let p5 = LetterId::pythagorean_5((0, 1)).map(|id| id.raw() as usize);
        let p13 = LetterId::pythagorean_13((0, 1)).map(|id| id.raw() as usize);
        for (i, &n) in self.rot.iter().enumerate() {
            if n == 0 {
                continue;
            }
            if Some(i) == p5 || Some(i) == p13 {
                return None;
            }
            if Some(i) != q01 {
                return None;
            }
        }
        let q = q01.map(|i| self.rot[i]).unwrap_or(0).rem_euclid(4) as u32;
        if q == 0 {
            return Some(1);
        }
        Some(4 / gcd_u32(q, 4))
    }
}

fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// A SPELLING: a rational similarity of the plane reduced to the alphabet —
/// the word and the exact dilation `ρ` it is read at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spelling {
    pub word: Word,
    pub rho: Q,
}

impl Spelling {
    /// The 2×2 block the spelling speaks: `ρ` times the word's plane action.
    pub fn block(&self) -> Option<[[Q; 2]; 2]> {
        let b = self.word.plane_block()?;
        Some(std::array::from_fn(|r| std::array::from_fn(|c| &b[r][c] * &self.rho)))
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        self.word.to_bytes(out);
        put_q(out, &self.rho);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Spelling> {
        let word = Word::from_bytes(b, pos)?;
        let rho = take_q(b, pos)?;
        Some(Spelling { word, rho })
    }
}

pub fn put_q(out: &mut Vec<u8>, v: &Q) {
    let nb = v.numer().to_signed_bytes_le();
    let db = v.denom().to_signed_bytes_le();
    out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
    out.extend_from_slice(&nb);
    out.extend_from_slice(&(db.len() as u32).to_le_bytes());
    out.extend_from_slice(&db);
}

pub fn take_q(b: &[u8], pos: &mut usize) -> Option<Q> {
    let take_int = |pos: &mut usize| -> Option<BigInt> {
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let bytes = b.get(*pos..*pos + n)?;
        *pos += n;
        Some(BigInt::from_signed_bytes_le(bytes))
    };
    let num = take_int(pos)?;
    let den = take_int(pos)?;
    if den.is_zero() {
        return None;
    }
    Some(Q::new(num, den))
}

// ---- the spelling peel ----

fn rational_sqrt(x: &Q) -> Option<Q> {
    if x.is_negative() {
        return None;
    }
    let (n, d) = (x.numer().sqrt(), x.denom().sqrt());
    if &(&n * &n) == x.numer() && &(&d * &d) == x.denom() {
        Some(Q::new(n, d))
    } else {
        None
    }
}

/// A Gaussian integer `p + q·i`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Gauss {
    p: BigInt,
    q: BigInt,
}

impl Gauss {
    fn new(p: i64, q: i64) -> Gauss {
        Gauss { p: BigInt::from(p), q: BigInt::from(q) }
    }
    fn mul(&self, o: &Gauss) -> Gauss {
        Gauss { p: &self.p * &o.p - &self.q * &o.q, q: &self.p * &o.q + &self.q * &o.p }
    }
    fn conj(&self) -> Gauss {
        Gauss { p: self.p.clone(), q: -&self.q }
    }
    fn norm(&self) -> BigInt {
        &self.p * &self.p + &self.q * &self.q
    }
    /// Exact division, or `None` when `o` does not divide `self`.
    fn div_exact(&self, o: &Gauss) -> Option<Gauss> {
        let n = o.norm();
        let num = self.mul(&o.conj());
        let (pd, pr) = num.p.div_rem(&n);
        let (qd, qr) = num.q.div_rem(&n);
        if !pr.is_zero() || !qr.is_zero() {
            return None;
        }
        Some(Gauss { p: pd, q: qd })
    }
    /// The valuation at `pi`: how many times `pi` divides exactly, and
    /// what remains. A peel — each division strictly lowers the norm.
    fn valuation(&self, pi: &Gauss) -> (i64, Gauss) {
        let mut v = 0i64;
        let mut z = self.clone();
        while let Some(next) = z.div_exact(pi) {
            if next.norm() >= z.norm() {
                break; // a unit divisor would loop; the alphabet's primes never are
            }
            z = next;
            v += 1;
        }
        (v, z)
    }
}

/// The residual of the spelling peel: the rotation as a Gaussian integer
/// over its weight, and the orientation sheet still to be peeled.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpellResidual {
    z: Gauss,
    n: BigInt,
    flipped: bool,
}

/// One syllable of the tape.
type Syllable = Vec<(LetterId, bool)>;

/// The alphabet's own actions on the plane, read once from the letters so
/// every orientation convention is settled by the algebra itself.
struct SpellStep {
    quarter: LetterId,
    l5: LetterId,
    l13: LetterId,
    flip: LetterId,
    /// The quarter letter's action on `e0`, as a Gaussian unit.
    quarter_unit: Gauss,
    /// Whether the forward (3,4,5) letter realises `((2+i)/(2−i))²` (true)
    /// or its conjugate.
    l5_forward: bool,
    l13_forward: bool,
    pi5: Gauss,
    pi13: Gauss,
}

fn unit_of_block(b: &[[Q; 2]; 2]) -> Option<Gauss> {
    let as_int = |v: &Q| -> Option<BigInt> {
        if v.denom().is_one() {
            Some(v.numer().clone())
        } else {
            None
        }
    };
    Some(Gauss { p: as_int(&b[0][0])?, q: as_int(&b[1][0])? })
}

impl SpellStep {
    fn new() -> Option<SpellStep> {
        let plane = (PLANE_ROW, PLANE_COL);
        let quarter = LetterId::quarter(plane)?;
        let l5 = LetterId::pythagorean_5(plane)?;
        let l13 = LetterId::pythagorean_13(plane)?;
        let flip = LetterId::quarter((PLANE_COL, 2))?;
        let quarter_unit = unit_of_block(&Word::from_letters(&[(quarter, false)]).plane_block()?)?;
        let pi5 = Gauss::new(2, 1);
        let pi13 = Gauss::new(3, 2);
        // ((2+i)/(2−i))² = (2+i)⁴/25 and ((3+2i)/(3−2i))² = (3+2i)⁴/169:
        // compare with the letters' measured actions.
        let t5 = pi5.mul(&pi5).mul(&pi5).mul(&pi5);
        let t13 = pi13.mul(&pi13).mul(&pi13).mul(&pi13);
        let action = |id: LetterId, weight: i64| -> Option<Gauss> {
            let b = Word::from_letters(&[(id, false)]).plane_block()?;
            let w = q(weight);
            unit_of_block(&[[&b[0][0] * &w, &b[0][1] * &w], [&b[1][0] * &w, &b[1][1] * &w]])
        };
        let a5 = action(l5, 25)?;
        let a13 = action(l13, 169)?;
        let l5_forward = if a5 == t5 {
            true
        } else if a5 == t5.conj() {
            false
        } else {
            return None;
        };
        let l13_forward = if a13 == t13 {
            true
        } else if a13 == t13.conj() {
            false
        } else {
            return None;
        };
        Some(SpellStep { quarter, l5, l13, flip, quarter_unit, l5_forward, l13_forward, pi5, pi13 })
    }

    /// The signed prime exponents `(m5, m13)` and the unit remainder of a
    /// residual — `None` when a prime outside the alphabet remains.
    fn read(&self, r: &SpellResidual) -> Option<(i64, i64, Gauss)> {
        let (v5p, z) = r.z.valuation(&self.pi5);
        let (v5m, z) = z.valuation(&self.pi5.conj());
        let (v13p, z) = z.valuation(&self.pi13);
        let (v13m, rest) = z.valuation(&self.pi13.conj());
        // The weight after the same primes leave it.
        let mut n = r.n.clone();
        for _ in 0..(v5p + v5m) / 2 {
            let (d, m) = n.div_rem(&BigInt::from(5));
            if !m.is_zero() {
                return None;
            }
            n = d;
        }
        for _ in 0..(v13p + v13m) / 2 {
            let (d, m) = n.div_rem(&BigInt::from(13));
            if !m.is_zero() {
                return None;
            }
            n = d;
        }
        if (v5p + v5m) % 2 != 0 || (v13p + v13m) % 2 != 0 {
            return None;
        }
        let unit = rest.div_exact(&Gauss { p: n, q: BigInt::zero() })?;
        if !unit.norm().is_one() {
            return None;
        }
        // On the circle the prime's exponent is half the valuation
        // difference (the weight carries the other half); one letter is
        // two of it, so an odd exponent is a half letter.
        Some(((v5p - v5m) / 2, (v13p - v13m) / 2, unit))
    }

    /// Steps of the quarter letter from a unit back to 1.
    fn unit_steps(&self, u: &Gauss) -> Option<u64> {
        let one = Gauss::new(1, 0);
        let mut cur = one.clone();
        for k in 0..4u64 {
            if &cur == u {
                return Some(k.min(4 - k));
            }
            cur = cur.mul(&self.quarter_unit);
        }
        None
    }

    fn distance(&self, r: &SpellResidual) -> Option<u64> {
        let (m5, m13, unit) = self.read(r)?;
        Some(r.flipped as u64 + m5.unsigned_abs() + m13.unsigned_abs() + self.unit_steps(&unit)?)
    }
}

impl ShellStep for SpellStep {
    type Residual = SpellResidual;
    type Core = Syllable;

    /// Peel one syllable, outer shell first: the orientation sheet, then a
    /// prime-5 letter, then a prime-13 letter, then a unit turn. `None`
    /// when what remains is not a whole letter — the arc rides out.
    fn shell(&self, r: &SpellResidual) -> Option<(Syllable, SpellResidual)> {
        if r.flipped {
            return Some((vec![(self.flip, false), (self.flip, false)], SpellResidual { flipped: false, ..r.clone() }));
        }
        let (m5, m13, unit) = self.read(r)?;
        let remove = |z: &Gauss, pi: &Gauss, forward: bool| -> Option<Gauss> {
            let conj = pi.conj();
            let (out, back): (&Gauss, &Gauss) = if forward { (pi, &conj) } else { (&conj, pi) };
            Some(z.div_exact(out)?.div_exact(out)?.mul(back).mul(back))
        };
        if m5.abs() >= 2 {
            let forward = m5 > 0;
            let z = remove(&r.z, &self.pi5, forward)?;
            return Some((vec![(self.l5, forward != self.l5_forward)], SpellResidual { z, ..r.clone() }));
        }
        if m5 != 0 {
            return None; // a half letter: not in the language
        }
        if m13.abs() >= 2 {
            let forward = m13 > 0;
            let z = remove(&r.z, &self.pi13, forward)?;
            return Some((vec![(self.l13, forward != self.l13_forward)], SpellResidual { z, ..r.clone() }));
        }
        if m13 != 0 {
            return None;
        }
        // The unit: one quarter letter toward 1, oriented by the shorter arc.
        let one = Gauss::new(1, 0);
        if unit == one {
            return None; // identity: the governor reads Closed before this
        }
        let q_inv = self.quarter_unit.conj();
        let forward = unit == self.quarter_unit || unit == self.quarter_unit.mul(&self.quarter_unit);
        let (id_inv, step) = if forward { (false, q_inv) } else { (true, self.quarter_unit.clone()) };
        let z = r.z.mul(&step);
        Some((vec![(self.quarter, id_inv)], SpellResidual { z, ..r.clone() }))
    }
}

struct SpellGovernor<'a> {
    step: &'a SpellStep,
    last: Option<u64>,
}

impl ShrinkGovernor for SpellGovernor<'_> {
    type Residual = SpellResidual;
    fn read(&mut self, r: &SpellResidual) -> ShellMotion {
        if !r.flipped && r.z.q.is_zero() && r.z.p == r.n {
            return ShellMotion::Closed;
        }
        let d = self.step.distance(r);
        let motion = match (d, self.last) {
            (Some(now), Some(before)) if now < before => ShellMotion::Shrinking,
            (Some(_), None) => ShellMotion::Shrinking,
            _ => ShellMotion::Stalled,
        };
        self.last = d;
        motion
    }
}

/// SPELL a rational similarity of the plane (rotation, reflection and
/// dilation; columns the images of the row and column steps) into the
/// alphabet. `None` when the block is not a similarity, its dilation is
/// not rational, or its rotation has a Gaussian factor outside the
/// alphabet's primes — a remainder, never a rounding.
pub fn spell(block: &[[Q; 2]; 2]) -> Option<Spelling> {
    match spell_peel(block)? {
        (PeelNest::Closed { cores }, rho) => {
            // Cores come out outer to inner (the sheet first); the tape's
            // last letter acts first, so the tape is the cores reversed.
            let mut word = Word::identity();
            for syllable in cores.iter().rev() {
                for &(id, inv) in syllable {
                    word.push(id, inv);
                }
            }
            let sp = Spelling { word, rho };
            // The algebra verifies its own spelling: the word's block must
            // be the block that was spelled.
            if sp.block()? != *block {
                return None;
            }
            Some(sp)
        }
        (PeelNest::Standing { .. }, _) => None,
    }
}

/// The spelling peel itself: the nest of syllables and, when the peel
/// stalls, the standing arc (the Gaussian remainder the alphabet cannot
/// speak) — carried out whole for the caller to hold as thought.
pub fn spell_peel(block: &[[Q; 2]; 2]) -> Option<(PeelNest<SpellResidualView, Syllable>, Q)> {
    let (a, b, c, d) = (&block[0][0], &block[0][1], &block[1][0], &block[1][1]);
    let rho2 = a * a + c * c;
    if rho2.is_zero() || &(b * b + d * d) != &rho2 || !(a * b + c * d).is_zero() {
        return None;
    }
    let rho = rational_sqrt(&rho2)?;
    let (r00, r10) = (a / &rho, c / &rho);
    let (r01, r11) = (b / &rho, d / &rho);
    let det = &r00 * &r11 - &r01 * &r10;
    let flipped = if det.is_one() {
        false
    } else if (-&det).is_one() {
        true
    } else {
        return None;
    };
    // Rotation form after the sheet: r11 = r00 and r01 = −r10.
    let (r01, r11) = if flipped { (-r01, -r11) } else { (r01, r11) };
    if r11 != r00 || r01 != -&r10 {
        return None;
    }
    let n = r00.denom().lcm(r10.denom());
    let z = Gauss { p: (&r00 * &n).to_integer(), q: (&r10 * &n).to_integer() };
    if z.norm() != &n * &n {
        return None;
    }
    let step = SpellStep::new()?;
    let mut governor = SpellGovernor { step: &step, last: None };
    let nest = peel_nest(SpellResidual { z, n, flipped }, &step, &mut governor);
    let view = match nest {
        PeelNest::Closed { cores } => PeelNest::Closed { cores },
        PeelNest::Standing { cores, arc } => PeelNest::Standing {
            cores,
            arc: SpellResidualView { p: arc.z.p, q: arc.z.q, weight: arc.n, flipped: arc.flipped },
        },
    };
    Some((view, rho))
}

/// The standing arc of a spelling that did not close: the rotation the
/// alphabet cannot speak, as `(p + q·i)/weight` and the unpeeled sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellResidualView {
    pub p: BigInt,
    pub q: BigInt,
    pub weight: BigInt,
    pub flipped: bool,
}

/// The block of a lattice isometry given by the images of the row step
/// and the column step — the sensor's way of reading a motion, spelled.
pub fn spell_images(row_image: (i64, i64), col_image: (i64, i64)) -> Option<Spelling> {
    spell(&[[q(row_image.0), q(col_image.0)], [q(row_image.1), q(col_image.1)]])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// LAW VIII: seventeen letters, each a versor with its carried inverse
    /// exact — `Φ Φ⁻¹ = 1` for all of them, and no eighteenth slot.
    #[test]
    fn seventeen_letters_each_with_its_inverse() {
        assert_eq!(LetterId::all().count(), 17);
        assert!(LetterId::try_new(17).is_none());
        for id in LetterId::all() {
            let phi = letter(id);
            assert!(phi.weight().is_some(), "letter {} is a versor", id.raw());
            assert_eq!(phi.geo(&letter_inverse(id)), Rotor::identity(), "letter {} carries its inverse", id.raw());
        }
    }

    /// The lattice isometries are WORDS, not letters: rot90 is one quarter
    /// letter, rot180 two, the column flip a half turn through the third
    /// axis, and the transposition their product — every one spelled by
    /// the peel and re-derived exactly from its tape.
    #[test]
    fn the_dihedral_motions_are_words_in_the_alphabet() {
        let rot90 = spell_images((0, -1), (1, 0)).expect("rot90 spells");
        assert_eq!(rot90.word.len(), 1);
        let rot180 = spell_images((-1, 0), (0, -1)).expect("rot180 spells");
        assert_eq!(rot180.word.len(), 2);
        let flip_cols = spell_images((1, 0), (0, -1)).expect("flip spells");
        assert_eq!(flip_cols.word.len(), 2);
        assert!(flip_cols.word.letters().iter().all(|(id, _)| plane_of(*id) == Some((1, 2))));
        let transpose = spell_images((0, 1), (1, 0)).expect("transpose spells");
        assert_eq!(transpose.word.len(), 3);
        let id = spell_images((1, 0), (0, 1)).expect("identity spells");
        assert!(id.word.is_empty());
        // The words re-derive their blocks and round-trip the wire.
        for sp in [&rot90, &rot180, &flip_cols, &transpose] {
            let mut b = Vec::new();
            sp.to_bytes(&mut b);
            let mut pos = 0;
            assert_eq!(Spelling::from_bytes(&b, &mut pos).as_ref(), Some(sp));
            assert_eq!(pos, b.len());
        }
    }

    /// Pythagorean rotations and dilations: the (3,4,5) letter's own
    /// rotation spells as one letter at ρ = 1 and at ρ = 3; a rotation by a
    /// prime outside the alphabet (17) stands as a remainder; a non-
    /// similarity refuses.
    #[test]
    fn pythagorean_words_dilations_and_honest_remainders() {
        let l5 = spell(&[[qn(-7, 25), qn(-24, 25)], [qn(24, 25), qn(-7, 25)]]).expect("(3,4,5)² spells");
        assert_eq!(l5.word.len(), 1);
        assert_eq!(l5.rho, q(1));
        let l5_inv = spell(&[[qn(-7, 25), qn(24, 25)], [qn(-24, 25), qn(-7, 25)]]).expect("inverse spells");
        assert_eq!(l5_inv.word.letters()[0].0, l5.word.letters()[0].0);
        assert_ne!(l5_inv.word.letters()[0].1, l5.word.letters()[0].1);
        let scaled = spell(&[[qn(-21, 25), qn(-72, 25)], [qn(72, 25), qn(-21, 25)]]).expect("ρ = 3");
        assert_eq!(scaled.rho, q(3));
        assert_eq!(scaled.word, l5.word);
        // (3,4,5) itself is a half letter under the sandwich: not a word.
        let (nest, _) = spell_peel(&[[qn(3, 5), qn(-4, 5)], [qn(4, 5), qn(3, 5)]]).expect("a similarity");
        assert!(matches!(nest, PeelNest::Standing { .. }));
        // (8,15,17): a prime the alphabet does not hold.
        assert!(spell(&[[qn(8, 17), qn(-15, 17)], [qn(15, 17), qn(8, 17)]]).is_none());
        // A shear is not a similarity.
        assert!(spell(&[[q(1), q(1)], [q(0), q(1)]]).is_none());
        // (5,12,13)² composed with a quarter turn: two letters.
        let w = Word::from_letters(&[(LetterId::quarter((0, 1)).unwrap(), false), (LetterId::pythagorean_13((0, 1)).unwrap(), true)]);
        let block = w.plane_block().expect("plane preserved");
        let sp = spell(&block).expect("spells back");
        assert_eq!(sp.word.plane_block(), Some(block));
        assert_eq!(sp.word.len(), 2);
    }

    /// Sheet conjugates a rotation to its inverse: S Q S Q⁻¹ is a half
    /// turn, not the identity the commuting sum would claim.
    #[test]
    fn sheet_conjugation_is_not_abelian() {
        let sheet = spell_images((1, 0), (0, -1)).expect("column flip");
        let q = spell_images((0, -1), (1, 0)).expect("rot90");
        let w = sheet.word.compose(&q.word).compose(&sheet.word).compose(&q.word.inverse());
        let f = w.form().expect("the product preserves the plane");
        assert!(!f.is_identity(), "S Q S Q⁻¹ is a half turn, not 1");
        assert_eq!(w.period(), Some(2));
        let half = spell_images((-1, 0), (0, -1)).expect("rot180");
        assert!(w.same_form(&half.word));
    }
}
