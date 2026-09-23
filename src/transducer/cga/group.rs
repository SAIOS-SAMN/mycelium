//! A4 — the rational conformal group `Spin(4,1)(ℚ)`: dilation, reflection,
//! and the closed-form tests of versorhood and projective equality.

use super::embedding::{e_inf, e_o, point_at};
use super::multivector::{one, Mv, E1};
use num_rational::BigRational;
use num_traits::Zero;

/// The bivector `E = e_o ∧ e_∞` (the dilation generator's plane).
fn e_plane() -> Mv {
    // e_o e_∞ = e_o·e_∞ + e_o∧e_∞ = −1 + E.
    e_o().geo(&e_inf()).add(&Mv::scalar(one()))
}

/// THE DILATOR by an exact rational ρ > 0, as an UNNORMALIZED versor
/// `(ρ+1) ± (ρ−1) E`: a positive scalar multiple of exp(±½ ln ρ E), so the
/// sandwich carries points on the cone to their ρ-scaled images up to the
/// homogeneous scale, with no logarithm and no root. The orientation of E is
/// settled against the embedding itself (the form that carries X(1,0,0) to
/// X(ρ,0,0)), never by a chosen sign.
pub fn dilator(rho: &BigRational) -> Mv {
    let e = e_plane();
    let plus = Mv::scalar(rho + one()).add(&e.scale(&(rho - one())));
    // Probed on the first base axis alone, which every base has: the
    // dilation's orientation does not depend on `n`.
    let target = point_at(std::slice::from_ref(rho));
    let probe = plus.sandwich(&point_at(&[one()]));
    match projective_ratio(&probe, &target) {
        Some(_) => plus,
        None => Mv::scalar(rho + one()).sub(&e.scale(&(rho - one()))),
    }
}

/// THE REFLECTOR across the plane `e1 = 0` (rows negated): the odd versor
/// `e1`, whose sandwich sends `X(x1, x2, x3)` to `X(−x1, x2, x3)` up to the
/// homogeneous scale.
pub fn reflector() -> Mv {
    Mv::blade(E1, one())
}

/// Is `m` a versor: `m m̃` a nonzero scalar? Returns that scalar.
pub fn is_versor(m: &Mv) -> Option<BigRational> {
    let prod = m.geo(&m.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&Mv::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s)
}

/// PROJECTIVE EQUALITY — TWO RAYS ARE EQUAL BY PROPORTION (EQ1, 78.12):
/// `a = λ b` for some `λ ∈ ℚ×`, read as proportionality of the two
/// coefficient vectors by cross multiplication, `a_i b_k = a_k b_i` against
/// one pivot slot `k`, exact; returns λ.
///
/// It read `a b̃` having a scalar part only, which holds for a blade and a
/// versor and FAILS for a non-simple bivector — `b b̃` carries a grade-4
/// part there, so such an element compared unequal to itself. Proportion is
/// the definition of one ray, for every element; no product is taken.
pub fn projective_ratio(a: &Mv, b: &Mv) -> Option<BigRational> {
    // Both in one algebra, the larger of the two, so every slot either
    // holds is read.
    let (a, b) = a.common(b);
    let pivot = (0..b.slot_count()).find(|&k| !b.coeff(k).is_zero())?;
    let (ak, bk) = (a.coeff(pivot), b.coeff(pivot));
    if ak.is_zero() {
        return None;
    }
    let proportional = (0..a.slot_count()).all(|i| a.coeff(i) * bk == ak * b.coeff(i));
    proportional.then(|| ak / bk)
}
