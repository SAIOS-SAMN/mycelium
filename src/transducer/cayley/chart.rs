// 52.1 — THE CHART: `V ≡ 1 + B + ½ B∧B` with `B = ⟨V⟩₂ / ⟨V⟩₀`, in either
// direction, exact or refused.

use crate::transducer::cga::{self, Mv};
use crate::transducer::element::Versor;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

fn half() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(2))
}

/// `B∧B`: the grade-4 part of a bivector's square (its inner square is the
/// scalar part and its commutator with itself is nothing).
fn outer_square(b: &Mv) -> Mv {
    b.geo(b).grade(4)
}

/// Is `m` even: nothing on the odd grades.
fn is_even(m: &Mv) -> bool {
    m.grade(1).is_zero() && m.grade(3).is_zero() && m.grade(5).is_zero()
}

/// THE CAYLEY BIVECTOR of an even versor, `⟨V⟩₂ / ⟨V⟩₀` (52.1): the element
/// of `Λ²(ℚ^{4,1})` the versor is derived from, projective in `V`, so `V` and
/// `λV` read one bivector. Refused at the chart's whole boundary and nowhere
/// else: no scalar part — a half turn, or an odd versor, whose derivation is
/// its mirror (52.4) — and an even element that is not a versor, read as
/// `V Ṽ` not a scalar or the grade-4 part other than `⟨V⟩₂∧⟨V⟩₂ / (2⟨V⟩₀)`,
/// which is what `V Ṽ ∈ ℚ` forces of a versor.
pub fn bivector_of(v: &Versor) -> Option<Versor> {
    let a = v.scalar_part();
    if a.is_zero() || !is_even(v) {
        return None;
    }
    cga::is_versor(v).then_some(())?;
    let b = v.grade(2).scale(&(BigRational::one() / &a));
    let quad = outer_square(&b).scale(&(half() * &a));
    if !v.grade(4).sub(&quad).is_zero() {
        return None;
    }
    Some(b)
}

/// THE VERSOR a bivector derives, `1 + B + ½ B∧B` (52.1): one outer square,
/// no exponential and no root. Refused when `b` is not a bivector, and when
/// the element has no weight — `V Ṽ = 0`, the parameter of a dilation plane
/// on the light cone — which is the one place the chart has no versor.
pub fn versor_of_bivector(b: &Versor) -> Option<Versor> {
    if !b.grade(2).sub(b).is_zero() {
        return None;
    }
    let v = Mv::scalar(BigRational::one()).add(b).add(&outer_square(b).scale(&half()));
    cga::is_versor(&v).then_some(v)
}
