// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §3.7, LAW-FIELD v1 §2 (a motion) — AN ELEMENT OF THE CYCLOTOMIC
//! FIELD `ℚ(ζ_ℓ)`: a turning's motion held exactly, as a polynomial in `ζ`
//! over ℚ reduced by the cyclotomic polynomial `Φ_ℓ`, never a trigonometric value.

use crate::core::Q;
use num_bigint::BigInt;
use num_traits::{One, Zero};

/// `num / den` for two integer polynomials, coefficients ascending, `den`
/// monic and dividing `num` exactly (the cyclotomic recursion's quotients).
fn divided_monic(num: &[BigInt], den: &[BigInt]) -> Vec<BigInt> {
    let (Some(d), Some(n)) = (den.len().checked_sub(1), num.len().checked_sub(1)) else { return Vec::new() };
    let Some(span) = n.checked_sub(d) else { return Vec::new() };
    let mut rest = num.to_vec();
    let mut quotient = vec![BigInt::zero(); span + 1];
    for i in (0..=span).rev() {
        let Some(c) = rest.get(i + d).cloned() else { continue };
        for (j, dj) in den.iter().enumerate() {
            if let Some(slot) = rest.get_mut(i + j) {
                *slot -= &c * dj;
            }
        }
        if let Some(slot) = quotient.get_mut(i) {
            *slot = c;
        }
    }
    quotient
}

/// THE ℓ-TH CYCLOTOMIC POLYNOMIAL `Φ_ℓ` over ℤ, coefficients ascending:
/// `xˡ − 1` divided exactly by `Φ_d` for every proper divisor `d` of ℓ.
/// Monic, of degree `φ(ℓ)`. Empty for ℓ = 0, which has no roots of unity.
fn cyclotomic_polynomial(l: usize) -> Vec<BigInt> {
    if l == 0 {
        return Vec::new();
    }
    let mut p = vec![BigInt::zero(); l + 1];
    if let Some(c) = p.first_mut() {
        *c = -BigInt::one();
    }
    if let Some(c) = p.get_mut(l) {
        *c = BigInt::one();
    }
    for d in (1..l).filter(|d| l % d == 0) {
        p = divided_monic(&p, &cyclotomic_polynomial(d));
    }
    p
}

/// `p` reduced modulo the monic integer polynomial `m`: every term at or
/// past `m`'s degree replaced by its remainder, so the result has fewer
/// terms than `m`'s degree.
fn reduced(mut p: Vec<Q>, m: &[BigInt]) -> Vec<Q> {
    let Some(d) = m.len().checked_sub(1) else { return Vec::new() };
    for i in (d..p.len()).rev() {
        let Some(c) = p.get(i).cloned() else { continue };
        if c.is_zero() {
            continue;
        }
        for (j, mj) in m.iter().enumerate() {
            if let Some(slot) = p.get_mut(i - d + j) {
                *slot -= &c * Q::from_integer(mj.clone());
            }
        }
    }
    p.resize(d, Q::zero());
    p
}

/// AN ELEMENT OF `ℚ(ζ_ℓ)`: its conductor ℓ and its coefficients on the
/// power basis `1, ζ, …, ζ^{φ(ℓ)−1}`, reduced by `Φ_ℓ`, so each element has
/// one writing in its field. The field is abstract: `ζ` is a root of `Φ_ℓ`,
/// and no value of it is read. Two elements of different conductors are
/// compared in the field of the least common multiple, where both stand.
#[derive(Clone, Debug)]
pub struct Cyclotomic {
    conductor: usize,
    coefficients: Vec<Q>,
}

impl Cyclotomic {
    /// THE ROOT OF UNITY `ζ_ℓ^k`, reduced in `ℚ(ζ_ℓ)`. `None` for ℓ = 0.
    pub fn root(conductor: usize, power: usize) -> Option<Cyclotomic> {
        let modulus = cyclotomic_polynomial(conductor);
        let k = power.checked_rem(conductor)?;
        let mut p = vec![Q::zero(); k + 1];
        *p.get_mut(k)? = Q::one();
        Some(Cyclotomic { conductor, coefficients: reduced(p, &modulus) })
    }

    /// The identity, `1` in `ℚ = ℚ(ζ₁)`.
    pub fn one() -> Cyclotomic {
        Cyclotomic { conductor: 1, coefficients: vec![Q::one()] }
    }

    /// The conductor ℓ of the field the element is written in.
    pub fn conductor(&self) -> usize {
        self.conductor
    }

    /// The coefficients on the power basis, ascending.
    pub fn coefficients(&self) -> &[Q] {
        &self.coefficients
    }

    /// THE SAME ELEMENT IN `ℚ(ζ_L)`, for ℓ dividing `L`: `ζ_ℓ ↦ ζ_L^{L/ℓ}`,
    /// the inclusion of fields. `None` where ℓ does not divide `L`.
    pub fn lifted(&self, to: usize) -> Option<Cyclotomic> {
        let step = match to.checked_rem(self.conductor)? {
            0 => to / self.conductor,
            _ => return None,
        };
        let mut p = vec![Q::zero(); self.coefficients.len().checked_mul(step)? + 1];
        for (i, c) in self.coefficients.iter().enumerate() {
            *p.get_mut(i * step)? = c.clone();
        }
        Some(Cyclotomic { conductor: to, coefficients: reduced(p, &cyclotomic_polynomial(to)) })
    }

    /// THE PRODUCT in the field of the two conductors' least common
    /// multiple. `None` where that multiple passes the machine word.
    pub fn product(&self, other: &Cyclotomic) -> Option<Cyclotomic> {
        let l = (self.conductor / num_integer::gcd(self.conductor, other.conductor)).checked_mul(other.conductor)?;
        let (a, b) = (self.lifted(l)?, other.lifted(l)?);
        let mut p = vec![Q::zero(); a.coefficients.len() + b.coefficients.len()];
        for (i, x) in a.coefficients.iter().enumerate() {
            for (j, y) in b.coefficients.iter().enumerate() {
                *p.get_mut(i + j)? += x * y;
            }
        }
        Some(Cyclotomic { conductor: l, coefficients: reduced(p, &cyclotomic_polynomial(l)) })
    }

    /// THE ORDER OF THE ELEMENT as a root of unity: the least `m ≥ 1` with
    /// `zᵐ = 1`, read among the divisors of its conductor by exact products.
    /// `None` where no such `m` divides the conductor: the element is no
    /// root of unity of its field's, an orbit that never returns.
    pub fn order(&self) -> Option<usize> {
        let one = Cyclotomic::one();
        let mut power = self.clone();
        for m in 1..=self.conductor {
            if self.conductor % m == 0 && power == one {
                return Some(m);
            }
            power = power.product(self)?;
        }
        None
    }
}

impl PartialEq for Cyclotomic {
    /// EQUAL IN THE FIELD OF THE TWO CONDUCTORS' LEAST COMMON MULTIPLE,
    /// coefficient by coefficient on its power basis.
    fn eq(&self, other: &Cyclotomic) -> bool {
        let Some(l) = (self.conductor / num_integer::gcd(self.conductor, other.conductor)).checked_mul(other.conductor) else {
            return false;
        };
        match (self.lifted(l), other.lifted(l)) {
            (Some(a), Some(b)) => a.coefficients == b.coefficients,
            _ => false,
        }
    }
}

impl Eq for Cyclotomic {}
