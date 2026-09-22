//! A4 — the rational conformal group `Spin(4,1)(ℚ)`: dilation, reflection,
//! and the closed-form tests of versorhood and projective equality.

use super::embedding::{e_inf, e_o, point};
use super::multivector::{one, zero, Mv, BLADES, E1};
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
    let target = point(rho, &zero(), &zero());
    let probe = plus.sandwich(&point(&one(), &zero(), &zero()));
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

/// PROJECTIVE EQUALITY of two blades: `a = λ b` for some λ ∈ ℚ×. The test is
/// `a b̃` having a scalar part only, nonzero; returns λ. This is the exact
/// closed-form check replay and verification run — never a wedge, never a
/// rasterization.
pub fn projective_ratio(a: &Mv, b: &Mv) -> Option<BigRational> {
    let bb = b.norm2();
    if bb.is_zero() {
        // A null blade: `a b̃` has no scalar part at all, so the reverse
        // product decides nothing here. Exact componentwise proportionality
        // is what projective equality is for these.
        return proportional(a, b);
    }
    let prod = a.geo(&b.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&Mv::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s / bb)
}

/// Exact componentwise proportionality `a = λ b` (λ ≠ 0), for null blades
/// where the reverse product carries no scalar.
fn proportional(a: &Mv, b: &Mv) -> Option<BigRational> {
    let mut lambda: Option<BigRational> = None;
    for mask in 0..BLADES {
        let (x, y) = (a.coeff(mask), b.coeff(mask));
        match (x.is_zero(), y.is_zero()) {
            (true, true) => continue,
            (true, false) | (false, true) => return None,
            (false, false) => {
                let l = x / y;
                match &lambda {
                    None => lambda = Some(l),
                    Some(k) if *k == l => {}
                    Some(_) => return None,
                }
            }
        }
    }
    lambda
}
