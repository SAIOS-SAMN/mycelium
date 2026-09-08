// 16.1, 9 — THE ROTOR: an even versor over the exact rationals, and the
// arithmetic that composes two of them.
//
//! A rotor is the coefficients of the even subalgebra, held exactly. It is
//! compared PROJECTIVELY — a rotor and its negation are one element — and
//! keyed by its coordinates written out, never by a digest of them.

use crate::transducer::alphabet::{Word, Q};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

use super::*;

pub(crate) fn q(v: i64) -> Q {
    Q::from_integer(BigInt::from(v))
}
pub(crate) fn qn(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// A6: two's-complement numerator and positive denominator, or refuse.
pub(crate) fn pack_axis(x: &Q) -> Option<[u64; 2]> {
    let n: i64 = x.numer().try_into().ok()?;
    let d: u64 = x.denom().try_into().ok()?;
    Some([n as u64, d])
}

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
pub(crate) fn reorder_sign(a: usize, b: usize) -> i64 {
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
    pub fn class_invariants(&self) -> Option<(Q, Q, u8, u8)> {
        let w = self.weight()?;
        if w.is_zero() {
            return None;
        }
        let a = self.c[0].clone();
        let p = self.c[15].clone();
        Some((
            &(&a * &a) / &w,
            &(&p * &p) / &w,
            u8::from(a.is_negative()),
            u8::from(p.is_negative()),
        ))
    }

    /// Same conjugacy-and-scale class: the invariants agree as rationals.
    /// A non-versor has no class.
    pub fn class_eq(&self, other: &Rotor) -> bool {
        match (self.class_invariants(), other.class_invariants()) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Conjugacy invariants written out (A21). Identity of class is
    /// [`class_eq`]. None when the element is not a versor.
    pub fn class_key(&self) -> Option<crate::transducer::a21::Address> {
        let (x, y, sa, sp) = self.class_invariants()?;
        let mut buf = vec![crate::transducer::a21::TAG_CLASS];
        put_q(&mut buf, &x);
        put_q(&mut buf, &y);
        buf.push(sa);
        buf.push(sp);
        Some(crate::transducer::a21::Address::new(buf))
    }

    /// Class key when the rotor is a versor; otherwise the A1 rotor key.
    /// Never the zero class of a non-versor.
    pub fn class_or_rotor_key(&self) -> crate::transducer::a21::Address {
        match self.class_key() {
            Some(k) => k,
            None => self.key(),
        }
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
        for (k, e) in v.iter().enumerate() {
            x.c[1 << k] = e.clone();
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

    /// A1 key: the canonical coefficients written out (A21). Identity is
    /// [`projective_eq`].
    pub fn key(&self) -> crate::transducer::a21::Address {
        let n = self.normalised();
        let mut buf = vec![crate::transducer::a21::TAG_ROTOR];
        n.to_bytes(&mut buf);
        crate::transducer::a21::Address::new(buf)
    }

    pub fn from_key(k: &[u8]) -> Option<Rotor> {
        if k.first() != Some(&crate::transducer::a21::TAG_ROTOR) {
            return None;
        }
        let mut p = 1usize;
        Rotor::from_bytes(k, &mut p)
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

    /// The plane word this rotor spells. None when it does not — never identity.
    pub fn spelled_word(&self) -> Option<Word> {
        let one = Q::from_integer(BigInt::from(1));
        let zero = Q::from_integer(BigInt::from(0));
        let e0 = self.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()]);
        let e1 = self.carry(&[zero.clone(), one, zero.clone(), zero]);
        let (a, b) = (e0?, e1?);
        let block = [[a[0].clone(), b[0].clone()], [a[1].clone(), b[1].clone()]];
        Some(spell(&block)?.word)
    }

    /// [`Word::order`] of [`Self::spelled_word`]. None when the rotor does not spell.
    pub fn word_order(&self) -> Option<Order> {
        self.spelled_word().map(|w| w.order())
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

pub(crate) fn rational_sqrt(x: &Q) -> Option<Q> {
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
