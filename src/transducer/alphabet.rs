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

/// A6: two's-complement numerator and positive denominator, or refuse.
fn pack_axis(x: &Q) -> Option<[u64; 2]> {
    let n: i64 = x.numer().try_into().ok()?;
    let d: u64 = x.denom().try_into().ok()?;
    Some([n as u64, d])
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
/// Vertex equality is [`Rotor::projective_eq`]; coefficient identity is
/// only the normal-form constructor's check.
#[derive(Debug, Clone)]
pub struct Rotor {
    c: [Q; 16],
}

impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        self.projective_eq(other)
    }
}
impl Eq for Rotor {}

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
        #[cfg(test)]
        crate::network::nerve::work::count_geo();
        let mut r = Rotor::zero();
        // index: sixteen blade masks (fixed physical size)
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

    /// Even-grade: only scalar, bivectors and pseudoscalar. An odd blade
    /// means the element is not a rotor.
    fn is_even(&self) -> bool {
        (0..16).all(|i| (i as u32).count_ones() % 2 == 0 || self.c[i].is_zero())
    }

    /// Canonical rotor (A1): divide by |first nonzero|, then negate so that
    /// coefficient is +1. −Φ and Φ are one vertex (the sandwich). Zero is
    /// not a vertex and stays zero.
    pub fn normalised(&self) -> Rotor {
        if !self.is_even() {
            return self.clone();
        }
        let Some(i) = (0..16).find(|&i| !self.c[i].is_zero()) else {
            return Rotor::zero();
        };
        let a = self.c[i].abs();
        let mut r = if a.is_one() {
            self.clone()
        } else {
            self.scale(&(Q::one() / a))
        };
        if r.c[i].is_negative() {
            r = r.scale(&-Q::one());
        }
        r
    }

    /// Coefficient identity: the normal-form constructor's check, never
    /// vertex equality.
    fn coeffs_eq(&self, other: &Rotor) -> bool {
        self.c == other.c
    }

    /// True when `self ∘ other⁻¹` is a nonzero scalar of either sign
    /// (`λ ∈ ℚ×`). The action is `R X R̃ / (R R̃)`, on which −R and R agree.
    pub fn projective_eq(&self, other: &Rotor) -> bool {
        match other.inverse() {
            Some(inv) => {
                let p = self.geo(&inv);
                !p.c[0].is_zero() && (1..16).all(|i| p.c[i].is_zero())
            }
            None => self.normalised().coeffs_eq(&other.normalised()),
        }
    }

    /// Conjugation-and-scale invariants `(a²/w, p²/w, sign a, sign p)` —
    /// exact rationals, never a digest. `a` the scalar, `p` the
    /// pseudoscalar, `w = ΦΦ̃`.
    pub fn class_invariants(&self) -> (Q, Q, u8, u8) {
        let w = self.weight().unwrap_or_else(Q::one);
        let a = self.c[0].clone();
        let p = self.c[15].clone();
        if w.is_zero() {
            (Q::zero(), Q::zero(), 0, 0)
        } else {
            (
                &(&a * &a) / &w,
                &(&p * &p) / &w,
                u8::from(a.is_negative()),
                u8::from(p.is_negative()),
            )
        }
    }

    /// Same conjugacy-and-scale class: the invariants agree as rationals.
    pub fn class_eq(&self, other: &Rotor) -> bool {
        self.class_invariants() == other.class_invariants()
    }

    /// Transport identifier of [`class_invariants`]. Identity of class is
    /// [`class_eq`], never this digest.
    pub fn class_key(&self) -> [u8; 32] {
        let (x, y, sa, sp) = self.class_invariants();
        let mut buf = Vec::new();
        put_q(&mut buf, &x);
        put_q(&mut buf, &y);
        buf.push(sa);
        buf.push(sp);
        *blake3::hash(&buf).as_bytes()
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

    pub fn is_identity(&self) -> bool {
        self.c[0].is_one() && (1..16).all(|i| self.c[i].is_zero())
    }

    /// `by Φ by⁻¹`. A non-invertible conjugator leaves Φ unmoved.
    pub fn conjugate(&self, by: &Rotor) -> Rotor {
        match by.inverse() {
            Some(inv) => by.geo(self).geo(&inv),
            None => self.clone(),
        }
    }

    /// A1 key: blake3 of the canonical coefficients. Identity is
    /// [`projective_eq`], never this digest compared as meaning.
    pub fn key(&self) -> [u8; 32] {
        let n = self.normalised();
        let mut buf = Vec::new();
        n.to_bytes(&mut buf);
        *blake3::hash(&buf).as_bytes()
    }

    /// Refinement axes of this class rotor (A7): conjugacy invariants packed
    /// exactly, never a hash fragment.
    pub fn class_axes(&self) -> Option<Vec<u64>> {
        let n = self.normalised();
        let w = n.weight().unwrap_or_else(Q::one);
        let a = n.c[0].clone();
        let p = n.c[15].clone();
        let mut axes = vec![2u64, u64::from(a.is_negative()), u64::from(p.is_negative())];
        if w.is_zero() {
            return Some(axes);
        }
        let a2 = pack_axis(&(&(&a * &a) / &w))?;
        let p2 = pack_axis(&(&(&p * &p) / &w))?;
        axes.extend_from_slice(&a2);
        axes.extend_from_slice(&p2);
        Some(axes)
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        for c in &self.c {
            put_q(out, c);
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rotor> {
        let mut c: [Q; 16] = std::array::from_fn(|_| Q::zero());
        for slot in &mut c {
            *slot = take_q(b, pos)?;
        }
        Some(Rotor { c })
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
/// (its carried `Φ⁻¹`). The stored tape is the geodesic normal form.
/// Equal words compare by rotor projective equality, never by a tape or a digest.
#[derive(Debug)]
pub struct Word(Vec<(LetterId, bool)>, std::sync::OnceLock<Rotor>);

impl Default for Word {
    fn default() -> Self {
        Word(Vec::new(), std::sync::OnceLock::new())
    }
}

impl Clone for Word {
    fn clone(&self) -> Self {
        let w = Word(self.0.clone(), std::sync::OnceLock::new());
        if let Some(r) = self.1.get() {
            let _ = w.1.set(r.clone());
        }
        w
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        // M1: vertex equality is A1 of the rotor (λ ∈ ℚ×, either sign).
        // q^4 = −4 and 1 are one vertex; mixed-plane spellings of one
        // rotor are one vertex even when the geodesic tape is not unique.
        self.rotor().projective_eq(&other.rotor())
    }
}
impl Eq for Word {}

impl Word {
    fn raw(tape: Vec<(LetterId, bool)>) -> Word {
        Word(tape, std::sync::OnceLock::new())
    }

    pub fn identity() -> Word {
        Word::raw(Vec::new())
    }

    pub fn from_letters(tape: &[(LetterId, bool)]) -> Word {
        Word::raw(tape.to_vec())
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
        self.1.take();
    }

    /// M1 key: A1 of the rotor, never a hash of a spoken or geodesic tape.
    /// Mixed-plane spellings of one rotor share this key.
    pub fn key(&self) -> [u8; 32] {
        self.rotor().key()
    }

    /// Same conjugacy class: plane conjugacy representative, or rotor
    /// class invariants off the plane. Never a digest comparison.
    pub fn same_class(&self, other: &Word) -> bool {
        match (self.form(), other.form()) {
            (Some(a), Some(b)) => a.conjugacy_eq(&b),
            (None, None) => self.rotor().class_eq(&other.rotor()),
            _ => false,
        }
    }

    /// Transport identifier of the conjugacy class. Class identity is
    /// [`same_class`], never this digest.
    pub fn class_key(&self) -> [u8; 32] {
        match self.form() {
            Some(f) => f.conjugacy_key(),
            None => self.rotor().class_key(),
        }
    }

    /// Shortest tape for this vertex: unit exponents mod 4, primes as
    /// spelled, planes in slot order.
    pub fn geodesic(&self) -> Word {
        self.normal_form()
    }

    /// The formed product `γ` of the tape, built once and memoised.
    pub fn rotor(&self) -> Rotor {
        self.1
            .get_or_init(|| {
                self.0.iter().fold(Rotor::identity(), |acc, &(id, inv)| {
                    acc.geo(&if inv { letter_inverse(id) } else { letter(id) })
                })
            })
            .clone()
    }

    /// The word read backwards: the inverse transport.
    pub fn inverse(&self) -> Word {
        Word::raw(self.0.iter().rev().map(|&(id, inv)| (id, !inv)).collect())
    }

    /// Concatenate tapes: `self` then `other` (last letter of `other` acts first).
    pub fn compose(&self, other: &Word) -> Word {
        let mut t = self.0.clone();
        t.extend_from_slice(&other.0);
        Word::raw(t)
    }

    /// Plane-closed: A3 tape. Off-plane: A2 run reduction. The key is A1
    /// of the rotor either way; a mixed-plane word is never keyed as 1.
    pub fn normal_form(&self) -> Word {
        if let Some(f) = self.form() {
            return f.to_word();
        }
        run_reduce(self.letters())
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
        let mut f = PlaneForm { parity, rot };
        reduce_unit_exponents(&mut f);
        Some(f)
    }

    /// Two words are the same plane element: reduced forms, or A1 of the
    /// rotor off the plane. Never tape equality.
    pub fn same_form(&self, other: &Word) -> bool {
        match (self.form(), other.form()) {
            (Some(a), Some(b)) => a == b,
            (None, None) => self.rotor().projective_eq(&other.rotor()),
            _ => false,
        }
    }

    /// The plane element is the identity: even sheet, trivial rotation.
    pub fn abelian_identity(&self) -> bool {
        self.form().is_some_and(|f| f.is_identity())
    }

    /// Order of the normal form: Infinite when any prime-5 or prime-13
    /// exponent is nonzero, else the order of the unit part under sheet parity.
    pub fn order(&self) -> Order {
        let n = self.normal_form();
        for &(id, _) in n.letters() {
            if id.raw() >= 7 {
                return Order::Infinite;
            }
        }
        match n.form() {
            Some(f) => match unit_order(&f) {
                Some(p) => Order::Finite(p),
                None => Order::Infinite,
            },
            None if n.is_empty() => Order::Finite(1),
            None => Order::Finite(4),
        }
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
        Some(Word::raw(tape))
    }
}

/// Reduce quarter-turn exponents mod 4; three quarters is one inverse.
fn reduce_unit_exponents(f: &mut PlaneForm) {
    for i in 1..=6 {
        let mut n = f.rot[i as usize].rem_euclid(4);
        if n == 3 {
            n = -1;
        }
        f.rot[i as usize] = n;
    }
}

fn complementary(a: (u8, u8), b: (u8, u8)) -> bool {
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

fn plane_nf(w: &Word) -> Word {
    match w.form() {
        Some(f) => f.to_word(),
        None => w.clone(),
    }
}

/// A2 step 1: split into per-plane runs, reduce each, commute complementary
/// planes until a same-plane merge, never rewrite as the identity.
fn run_reduce(tape: &[(LetterId, bool)]) -> Word {
    let tape: Vec<(LetterId, bool)> = tape.iter().copied().filter(|(id, _)| id.raw() != 0).collect();
    if tape.is_empty() {
        return Word::identity();
    }
    let mut runs: Vec<Word> = Vec::new();
    let mut cur: Vec<(LetterId, bool)> = Vec::new();
    let mut cur_plane: Option<(u8, u8)> = None;
    for &ltr in &tape {
        let p = plane_of(ltr.0);
        if cur.is_empty() || p == cur_plane {
            cur.push(ltr);
            cur_plane = p;
        } else {
            runs.push(plane_nf(&Word::from_letters(&cur)));
            cur = vec![ltr];
            cur_plane = p;
        }
    }
    if !cur.is_empty() {
        runs.push(plane_nf(&Word::from_letters(&cur)));
    }
    runs.retain(|w| !w.is_empty());
    loop {
        let n = runs.len();
        let mut merged = false;
        let mut i = 1usize;
        while i < runs.len() {
            let pi = runs[i].letters().first().and_then(|(id, _)| plane_of(*id));
            let Some(pi) = pi else {
                i += 1;
                continue;
            };
            let mut j = i;
            let mut gap_ok = true;
            while j > 0 {
                j -= 1;
                let pj = runs[j].letters().first().and_then(|(id, _)| plane_of(*id));
                let Some(pj) = pj else {
                    gap_ok = false;
                    break;
                };
                if pj == pi {
                    break;
                }
                if !complementary(pj, pi) {
                    gap_ok = false;
                    break;
                }
                if j == 0 && pj != pi {
                    gap_ok = false;
                }
            }
            if gap_ok && j < i && runs[j].letters().first().and_then(|(id, _)| plane_of(*id)) == Some(pi) {
                let piece = runs.remove(i);
                let joined = runs[j].compose(&piece);
                runs[j] = plane_nf(&joined);
                if runs[j].is_empty() {
                    runs.remove(j);
                }
                merged = true;
                break;
            }
            i += 1;
        }
        if !merged || runs.len() >= n {
            break;
        }
        runs.retain(|w| !w.is_empty());
    }
    runs.into_iter().fold(Word::identity(), |a, b| a.compose(&b))
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

const RELATOR_N: usize = 12;

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

/// The plane group element a word represents: sheet parity and the
/// rotation in (e0, e1). Rotation exponents are those of the unflipped
/// spelling; a sheet conjugates a rotation to its inverse, so this is
/// not the commuting sum of the tape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaneForm {
    pub parity: bool,
    pub rot: [i64; CAYLEY_ALPHABET_LEN],
}

impl Default for PlaneForm {
    fn default() -> Self {
        PlaneForm { parity: false, rot: [0; CAYLEY_ALPHABET_LEN] }
    }
}

impl PlaneForm {
    pub fn identity() -> PlaneForm {
        PlaneForm::default()
    }

    pub fn is_identity(&self) -> bool {
        !self.parity && self.rot.iter().all(|&x| x == 0)
    }

    /// Canonical conjugacy representative. Even sheet: `{r, −r}` with the
    /// first nonzero component positive. Odd sheet: each exponent mod 2
    /// in `{0,1}`. Compared as coefficients, never hashed.
    pub fn conjugacy_rep(&self) -> PlaneForm {
        if self.parity {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for (i, &n) in self.rot.iter().enumerate() {
                rot[i] = n.rem_euclid(2);
            }
            PlaneForm { parity: true, rot }
        } else {
            let neg: [i64; CAYLEY_ALPHABET_LEN] = std::array::from_fn(|i| -self.rot[i]);
            let pick_neg = match self.rot.iter().zip(neg.iter()).find(|(&a, &b)| a != b) {
                Some((&a, _)) => a < 0,
                None => false,
            };
            PlaneForm { parity: false, rot: if pick_neg { neg } else { self.rot } }
        }
    }

    /// Same conjugacy class: the representatives agree as coefficients.
    pub fn conjugacy_eq(&self, other: &PlaneForm) -> bool {
        self.conjugacy_rep() == other.conjugacy_rep()
    }

    /// Transport identifier of [`conjugacy_rep`]. Class identity is
    /// [`conjugacy_eq`], never this digest.
    pub fn conjugacy_key(&self) -> [u8; 32] {
        let r = self.conjugacy_rep();
        let mut buf = Vec::with_capacity(1 + 8 * CAYLEY_ALPHABET_LEN);
        buf.push(r.parity as u8);
        for &n in &r.rot {
            buf.extend_from_slice(&n.to_le_bytes());
        }
        *blake3::hash(&buf).as_bytes()
    }

    /// Plane-group product: rotations ⋊ sheet parity. A trailing sheet
    /// conjugates the leading rotation to its inverse.
    pub fn compose(&self, other: &PlaneForm) -> PlaneForm {
        let parity = self.parity ^ other.parity;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        if other.parity {
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i] + other.rot[i];
            }
        } else {
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = self.rot[i] + other.rot[i];
            }
        }
        PlaneForm { parity, rot }
    }

    pub fn inverse(&self) -> PlaneForm {
        if self.parity {
            *self
        } else {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { parity: false, rot }
        }
    }

    /// The sheet acting on exponents: `S R S⁻¹ = R⁻¹`.
    pub fn conjugate_by(&self, by: &PlaneForm) -> PlaneForm {
        if !by.parity {
            *self
        } else if self.parity {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { parity: true, rot }
        } else {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { parity: false, rot }
        }
    }

    /// Half-turn on (e1,e2): the column flip of the sensor plane, two
    /// quarter-turns, the sheet letter of the plane group.
    fn sheet_word() -> Word {
        let id = LetterId::quarter((1, 2)).expect("plane (1,2) is a letter");
        Word::raw(vec![(id, false), (id, false)])
    }

    pub fn to_word(&self) -> Word {
        let mut w = if self.parity {
            Self::sheet_word()
        } else {
            Word::identity()
        };
        for (i, &n) in self.rot.iter().enumerate() {
            if n == 0 {
                continue;
            }
            let Some(id) = LetterId::try_new(i as u8) else { continue };
            if id.raw() == 0 {
                continue;
            }
            let inv = n < 0;
            for _ in 0..n.unsigned_abs() {
                w.push(id, inv);
            }
        }
        w
    }

    pub fn key(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(&[self.parity as u8]);
        for &n in &self.rot {
            h.update(&n.to_le_bytes());
        }
        *h.finalize().as_bytes()
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.push(self.parity as u8);
        for &n in &self.rot {
            out.extend_from_slice(&n.to_le_bytes());
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PlaneForm> {
        let parity = *b.get(*pos)? != 0;
        *pos += 1;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        for n in &mut rot {
            *n = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        Some(PlaneForm { parity, rot })
    }
}

/// Order of a word's normal form: finite unit/sheet, or infinite when a
/// prime-5 or prime-13 exponent stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Finite(u32),
    Infinite,
}

fn unit_order(f: &PlaneForm) -> Option<u32> {
    if f.is_identity() {
        return Some(1);
    }
    if f.parity && f.rot.iter().all(|&x| x == 0) {
        return Some(2);
    }
    let q01 = LetterId::quarter((0, 1)).map(|id| id.raw() as usize);
    let p5 = LetterId::pythagorean_5((0, 1)).map(|id| id.raw() as usize);
    let p13 = LetterId::pythagorean_13((0, 1)).map(|id| id.raw() as usize);
    for (i, &n) in f.rot.iter().enumerate() {
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
    let q = q01.map(|i| f.rot[i]).unwrap_or(0).rem_euclid(4) as u32;
    let rot = if q == 0 { 1 } else { 4 / gcd_u32(q, 4) };
    if f.parity {
        Some(if rot % 2 == 0 { rot } else { rot * 2 })
    } else {
        Some(rot)
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
        let quarter_unit = unit_of_block(&Word::raw(vec![(quarter, false)]).plane_block()?)?;
        let pi5 = Gauss::new(2, 1);
        let pi13 = Gauss::new(3, 2);
        // ((2+i)/(2−i))² = (2+i)⁴/25 and ((3+2i)/(3−2i))² = (3+2i)⁴/169:
        // compare with the letters' measured actions.
        let t5 = pi5.mul(&pi5).mul(&pi5).mul(&pi5);
        let t13 = pi13.mul(&pi13).mul(&pi13).mul(&pi13);
        let action = |id: LetterId, weight: i64| -> Option<Gauss> {
            let b = Word::raw(vec![(id, false)]).plane_block()?;
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
            let scaled = phi.scale(&qn(3, 2));
            assert!(phi.projective_eq(&scaled), "a positive weight is the same transport");
            assert!(phi.projective_eq(&phi.scale(&qn(-3, 2))), "a negative weight is the same transport");
            assert_eq!(phi.normalised(), scaled.normalised());
            assert_eq!(phi.normalised(), phi.scale(&qn(-1, 1)).normalised());
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
        assert_eq!(w.order(), Order::Finite(2));
        let half = spell_images((-1, 0), (0, -1)).expect("rot180");
        assert!(w.same_form(&half.word));
    }

    /// PlaneForm is the coefficient group: products and inverses are exact
    /// and O(1); a sheet conjugates exponents.
    #[test]
    fn plane_form_is_the_coefficient_group() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let rot = Word::from_letters(&[(q, false)]).form().unwrap();
        let sheet = PlaneForm { parity: true, rot: [0; CAYLEY_ALPHABET_LEN] };
        assert!(rot.compose(&rot.inverse()).is_identity());
        assert!(sheet.compose(&sheet).is_identity(), "a sheet has order 2");
        let conj = rot.conjugate_by(&sheet);
        assert_eq!(conj, rot.inverse(), "S R S⁻¹ = R⁻¹");
        assert_eq!(rot.compose(&sheet), sheet.compose(&rot.inverse()));
        assert_eq!(rot.key(), rot.to_word().form().unwrap().key());
        let w = Word::from_letters(&[(q, false)]);
        let r = w.rotor();
        assert!(r.geo(&r.inverse().unwrap()).is_identity());
        let n = w.compose(&w.inverse()).normal_form();
        assert!(n.rotor().is_identity(), "normal form of a cancelling tape is 1");
    }

    /// §52.1: conjugacy key is invariant on the four conjugation lines.
    #[test]
    fn plane_conjugacy_four_lines() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false), (q, false)]).form().unwrap();
        let r2 = Word::from_letters(&[(q, false)]).form().unwrap();
        let sheet = PlaneForm { parity: true, rot: [0; CAYLEY_ALPHABET_LEN] };
        let conj = |g: &PlaneForm, x: &PlaneForm| g.compose(&x.compose(&g.inverse()));
        assert_eq!(conj(&r2, &r).conjugacy_key(), r.conjugacy_key(), "(0,r') even");
        assert_eq!(conj(&sheet, &r).conjugacy_key(), r.conjugacy_key(), "(1,r'') even class of r and -r");
        let odd = r.compose(&sheet);
        assert_eq!(conj(&r2, &odd).conjugacy_key(), odd.conjugacy_key(), "(0,r') odd");
        assert_eq!(conj(&sheet, &odd).conjugacy_key(), odd.conjugacy_key(), "(1,r'') odd");
    }

    #[test]
    fn conjugacy_key_iff_conjugate() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false)]).form().unwrap();
        let inv = r.inverse();
        assert!(r.conjugacy_eq(&inv), "r and -r are one even class");
        assert_eq!(r.conjugacy_key(), inv.conjugacy_key(), "r and -r are one even class");
        let sheet = PlaneForm { parity: true, rot: [0; CAYLEY_ALPHABET_LEN] };
        let odd = r.compose(&sheet);
        let odd2 = inv.compose(&sheet);
        assert!(odd.conjugacy_eq(&odd2), "odd class is r mod 2");
        assert_eq!(odd.conjugacy_key(), odd2.conjugacy_key(), "odd class is r mod 2");
        assert!(!r.conjugacy_eq(&odd), "sheet splits the class");
        assert_ne!(r.conjugacy_key(), odd.conjugacy_key(), "sheet splits the class");
    }

    #[test]
    fn rotor_class_key_invariant() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let w = Word::from_letters(&[(q, false)]);
        let r = w.rotor();
        let s = Word::from_letters(&[(LetterId::quarter((0, 2)).unwrap(), false)]).rotor();
        let conj = s.geo(&r).geo(&s.inverse().unwrap());
        assert_eq!(r.class_key(), conj.class_key());
        let scaled = r.scale(&qn(3, 2));
        assert_eq!(r.class_key(), scaled.class_key());
        let f = w.form().unwrap();
        assert_eq!(w.class_key(), f.conjugacy_key());
    }

    /// Two tapes of one vertex have one key; a tape and its geodesic
    /// replay to the same affine.
    #[test]
    fn two_tapes_of_one_vertex_have_one_key() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let a = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
        let b = Word::identity();
        assert!(a.rotor().projective_eq(&b.rotor()), "q^4 and 1 are one rotor under ℚ×");
        assert_eq!(a, b, "vertex equality is A1, not a digest of either tape");
        assert_eq!(a.key(), b.key(), "q^4 and 1 are one A1 key");
        let long = Word::raw(vec![(q, false), (q, false), (q, false), (q, false), (q, false)]);
        let geo = long.geodesic();
        assert_eq!(geo.len(), 1, "five quarters geodesic to one");
        let ma = long.matrix().expect("affine");
        let mb = geo.matrix().expect("affine");
        assert_eq!(ma, mb, "tape and geodesic replay the same affine");
        let g = relators();
        assert_eq!(g.len(), RELATOR_N);
        assert!(g.iter().any(|r| r.kind == RelatorKind::QuarterOrderFour));
        assert!(g.iter().any(|r| r.kind == RelatorKind::PlaneCommute));
        assert!(g.iter().any(|r| r.kind == RelatorKind::SheetConjugate));
        assert!(g.iter().any(|r| r.kind == RelatorKind::PrimeInfinite));
        assert!(g.iter().any(|r| r.kind == RelatorKind::QuarterOrderFour && r.word.geodesic().is_empty()));
    }

    #[test]
    fn a1_canonical_is_projective_either_sign() {
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false)]).rotor();
        let q4 = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
        assert_eq!(q4.key(), Word::identity().key(), "q^4 and 1 share an A1 key");
        assert_eq!(r.normalised().key(), r.scale(&qn(-5, 2)).normalised().key());
        assert!(r.projective_eq(&r.scale(&qn(-1, 1))));
        let next = Word::from_letters(&[(q, false), (q, false)]).rotor();
        assert_ne!(r.key(), next.key(), "R and R·q are distinct vertices");
        let f = Word::from_letters(&[(q, false), (q, false), (q, false)]).form().unwrap();
        assert_eq!(f.rot[q.raw() as usize], -1, "A3 writes three quarters as −1");
    }
}
