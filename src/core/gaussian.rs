// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 37.7 P2 — THE GAUSSIAN FACTOR OF A WITNESS. `π_p = gcd_{ℤ[i]}(s + t·i,
//! p)`, Euclid over ℤ[i], exact and bounded by the norm (rule 10).
//!
//! The two-squares problem is never solved from `p` alone. The transducer
//! MEASURED the rotor, so the transducer already holds the letter; 36.3 is
//! exactly the statement that the effector was missing, not the object.
//! What arrives is a witness — the residue block a demand carries (50.5 as
//! amended) — and the factor is read off it.
//!
//! A witness whose gcd with `p` is a unit names a prime it does not carry.
//! That is refused, not rounded: `None`.

use num_bigint::BigInt;
use num_traits::{Signed, Zero};

/// A Gaussian integer `a + b·i`, exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gauss {
    pub a: BigInt,
    pub b: BigInt,
}

impl Gauss {
    pub fn new(a: BigInt, b: BigInt) -> Gauss {
        Gauss { a, b }
    }

    pub fn from_i64(a: i64, b: i64) -> Gauss {
        Gauss::new(BigInt::from(a), BigInt::from(b))
    }

    /// `N(a + bi) = a² + b²`, a non-negative rational integer. It is the
    /// Euclidean size Euclid's algorithm descends on, and the quantity a
    /// prime is read against.
    pub fn norm(&self) -> BigInt {
        &self.a * &self.a + &self.b * &self.b
    }

    pub fn is_zero(&self) -> bool {
        self.a.is_zero() && self.b.is_zero()
    }

    /// A UNIT of ℤ[i]: `±1`, `±i`, the four elements of norm one. A gcd
    /// that lands on one carries no factor at all.
    pub fn is_unit(&self) -> bool {
        self.norm() == BigInt::from(1)
    }

    pub fn mul(&self, o: &Gauss) -> Gauss {
        Gauss::new(
            &self.a * &o.a - &self.b * &o.b,
            &self.a * &o.b + &self.b * &o.a,
        )
    }

    pub fn sub(&self, o: &Gauss) -> Gauss {
        Gauss::new(&self.a - &o.a, &self.b - &o.b)
    }

    /// `a − b·i`. The conjugate of a prime is a prime, and the two are the
    /// two shells a rotation peels: `π` and `π̄` divide a rotor's arc
    /// independently and their exponents need not agree.
    pub fn conj(&self) -> Gauss {
        Gauss::new(self.a.clone(), -&self.b)
    }

    /// EXACT division, or `None` when `o` does not divide `self` in ℤ[i].
    /// This is the division a peel does — as against `div_round`, which is
    /// Euclid's step and always answers. A shell that does not divide
    /// exactly is a shell the peel does not take (rule 11).
    pub fn div_exact(&self, o: &Gauss) -> Option<Gauss> {
        use num_integer::Integer;
        let n = o.norm();
        if n.is_zero() {
            return None;
        }
        let num = self.mul(&o.conj());
        let (ad, ar) = num.a.div_rem(&n);
        let (bd, br) = num.b.div_rem(&n);
        if !ar.is_zero() || !br.is_zero() {
            return None;
        }
        Some(Gauss::new(ad, bd))
    }

    /// THE VALUATION AT `pi`: how many times `pi` divides exactly, and what
    /// remains. A peel, and bounded by the norm's own descent — each
    /// division strictly lowers it, so the loop's bound is the target's
    /// size and not a count chosen for it (rule 10). A unit divisor would
    /// not descend and is refused by that same reading rather than by a
    /// guard placed ahead of it.
    pub fn valuation(&self, pi: &Gauss) -> (i64, Gauss) {
        let mut v = 0i64;
        let mut z = self.clone();
        while let Some(next) = z.div_exact(pi) {
            if next.norm() >= z.norm() {
                break;
            }
            z = next;
            v += 1;
        }
        (v, z)
    }

    /// Euclidean division in ℤ[i]: the quotient whose coefficients are the
    /// NEAREST rational integers to the exact quotient, which is what makes
    /// the remainder's norm strictly smaller. Division by zero has no
    /// quotient and says so.
    pub fn div_round(&self, o: &Gauss) -> Option<Gauss> {
        let n = o.norm();
        if n.is_zero() {
            return None;
        }
        // (a+bi)(c−di) / N, rounded to nearest on each coordinate.
        let re = &self.a * &o.a + &self.b * &o.b;
        let im = &self.b * &o.a - &self.a * &o.b;
        Some(Gauss::new(round_div(&re, &n), round_div(&im, &n)))
    }

    pub fn rem(&self, o: &Gauss) -> Option<Gauss> {
        let q = self.div_round(o)?;
        Some(self.sub(&q.mul(o)))
    }

    /// A1's sign step at the altitude of ℤ[i]: the associate of this
    /// element in the first octant, `a > b ≥ 0`, reached by multiplying
    /// by one of the four units and conjugating. Two courts that factor
    /// one witness reach ONE representative and compare content, never
    /// "the same up to a unit" (law 2).
    pub fn canonical(&self) -> Gauss {
        let (mut a, mut b) = (self.a.clone(), self.b.clone());
        // Into the first quadrant by units.
        for _ in 0..4 {
            if a.is_negative() || b.is_negative() {
                let (na, nb) = (-b.clone(), a.clone());
                a = na;
                b = nb;
            }
        }
        // Then reflect so the real part leads.
        if b > a {
            std::mem::swap(&mut a, &mut b);
        }
        Gauss::new(a, b)
    }
}

/// Round `n / d` to the nearest rational integer, halves away from zero.
/// Exact: no float and no truncation that would break the norm descent.
fn round_div(n: &BigInt, d: &BigInt) -> BigInt {
    let two = BigInt::from(2);
    match n.is_negative() {
        false => (n * &two + d) / (d * &two),
        true => (n * &two - d) / (d * &two),
    }
}

/// 37.7 P2 — THE FACTOR, read off the witness. `gcd_{ℤ[i]}(s + t·i, p)` by
/// Euclid, in A1 canonical form. The descent is on the norm, which is a
/// non-negative integer that strictly falls, so the loop's bound is the
/// witness's own size and not a count someone chose (rule 10).
///
/// `None` unless the gcd's NORM IS THE PRIME, which is P2's own
/// condition and the only one: a split prime `p` factors as `π_p·π̄_p`
/// with `N(π_p) = p`. A unit gcd (norm 1) is a witness naming a prime it
/// does not carry; a gcd of `p` itself (norm `p²`) is the witness being
/// zero, or divisible by `p`, and carrying no factor either. Both are
/// `REFUSE_LETTER_WITNESS` at the court and neither is rounded to the
/// nearest answer.
pub fn prime_factor(witness: &Gauss, p: &BigInt) -> Option<Gauss> {
    let mut x = witness.clone();
    let mut y = Gauss::new(p.clone(), BigInt::zero());
    while !y.is_zero() {
        let r = x.rem(&y)?;
        x = y;
        y = r;
    }
    if x.norm() != *p {
        return None;
    }
    Some(x.canonical())
}

/// The witness a `Demand::RotorPrime` carries, read as a Gaussian integer:
/// the plane block `[[s, −t], [t, s]]` is `s + t·i`. A block whose entries
/// are not rational integers is not a Gaussian arc and refuses.
pub fn from_block(block: &[[crate::core::Q; 2]; 2]) -> Option<Gauss> {
    let (s, t) = (&block[0][0], &block[1][0]);
    if !s.is_integer() || !t.is_integer() {
        return None;
    }
    Some(Gauss::new(s.to_integer(), t.to_integer()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(a: i64, b: i64) -> Gauss {
        Gauss::from_i64(a, b)
    }

    /// 37.7 P2 on the prime the alphabet does not carry. `17 = 4² + 1²`,
    /// and the witness `4 + i` carries the factor: the gcd is that factor
    /// itself, in canonical form, and its norm is the prime.
    #[test]
    fn prime_factor_reads_factor_of_prime_norm() {
        let p = BigInt::from(17);
        let pi = prime_factor(&g(4, 1), &p).expect("the witness carries 17");
        assert_eq!(pi.norm(), p, "the factor's norm IS the prime it factors");
        assert_eq!(pi, g(4, 1), "in A1 canonical form: the real part leads");
    }

    /// TWO COURTS, ONE FACTOR. Different associates of one factor — the
    /// four units and the conjugate — reach the same canonical
    /// representative, so two organs that each ran Euclid compare CONTENT
    /// and never "the same up to a unit" (law 2, P3).
    #[test]
    fn canonical_maps_associates_to_one_representative() {
        let want = g(4, 1);
        for w in [g(4, 1), g(-1, 4), g(-4, -1), g(1, -4), g(1, 4)] {
            assert_eq!(w.canonical(), want, "{w:?} is the same factor");
        }
    }

    /// A witness that does not carry the prime is REFUSED, not rounded:
    /// the gcd is a unit and there is no letter to open
    /// (`REFUSE_LETTER_WITNESS`). `2 + i` has norm 5 and is coprime to 17.
    #[test]
    fn prime_factor_refuses_witness_without_prime() {
        assert_eq!(prime_factor(&g(2, 1), &BigInt::from(17)), None, "a unit gcd is no factor");
        // gcd(0, p) is p, whose norm is p² and not p: the arithmetic is
        // right and the witness still carries nothing.
        assert_eq!(prime_factor(&g(0, 0), &BigInt::from(17)), None, "and no witness is no factor");
        assert_eq!(prime_factor(&g(17, 0), &BigInt::from(17)), None, "nor a witness p divides");
    }

    /// The alphabet's own primes come back too — the arithmetic does not
    /// know which primes the seventeen already spell, and must not.
    #[test]
    fn prime_factor_factors_alphabet_primes() {
        assert_eq!(prime_factor(&g(2, 1), &BigInt::from(5)).expect("5"), g(2, 1));
        assert_eq!(prime_factor(&g(3, 2), &BigInt::from(13)).expect("13"), g(3, 2));
    }

    /// Rule 10: Euclid descends on the norm, which is a non-negative
    /// integer that strictly falls, so the remainder of a division is
    /// always smaller than the divisor and the loop is bounded by the
    /// witness's own size.
    #[test]
    fn rem_smaller_than_divisor() {
        for (x, y) in [(g(17, 0), g(4, 1)), (g(100, 37), g(6, 5)), (g(-9, 14), g(3, 2))] {
            let r = x.rem(&y).expect("a nonzero divisor divides");
            assert!(r.norm() < y.norm(), "{r:?} is smaller than {y:?}");
        }
    }

    /// The witness travels as a plane block (50.5): `[[s, −t], [t, s]]`
    /// is `s + t·i`, and a block that is not a Gaussian arc refuses.
    #[test]
    fn from_block_reads_gaussian_and_refuses_non_integer() {
        use num_rational::BigRational;
        let q = |n: i64| BigRational::from_integer(BigInt::from(n));
        let block = [[q(4), q(-1)], [q(1), q(4)]];
        assert_eq!(from_block(&block).expect("an arc"), g(4, 1));
        let half = BigRational::new(BigInt::from(1), BigInt::from(2));
        assert_eq!(from_block(&[[half, q(0)], [q(0), q(1)]]), None, "not a Gaussian integer");
    }
}
