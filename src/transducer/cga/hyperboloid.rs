//! 54.7 Axiom B — a lamina on the hyperboloid sheet, the directed geodesic
//! transvection between two, and the base point a frame stands at.

use super::group::is_versor;
use super::multivector::{one, Mv, E1, E2, E3, EM};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// AXIOM B — A LAMINA LANDS ON THE HYPERBOLOID SHEET, EXACTLY OVER ℚ
/// (54.7, ruled 2026-09-21).
///
/// ```text
///     v(u) = (2u₁, 2u₂, 2u₃, 1+|u|²) / (1−|u|²),     u = (r,c,k)/M
/// ```
///
/// Rational stereographic projection from the pole `(0,0,0,1)`. Every
/// component is a rational polynomial in `u`, so there is no square root
/// and no Diophantine refusal:
///
/// ```text
///     v² = [4|u|² − (1+|u|²)²] / (1−|u|²)² = −(1−|u|²)²/(1−|u|²)² = −1
/// ```
///
/// THE SHEET, NOT THE CONE. `v² = −1` is the INTERIOR `ℍ³`, where a lamina
/// and a member frame actually sit; the null cone `p² = 0` is the ideal
/// boundary `∂_∞ℍ³`, the asymptotic horizon of infinite geodesic rays. A
/// first cut put laminae on the cone, and there
/// [`geodesic_transvection`] degenerates: `p_a² = 0` collapses the
/// transport step and the operator carries nothing.
///
/// `M` is the LATTICE QUANTUM: an integer with `M² > r²+c²+k²`, which keeps
/// `|u| < 1` so the denominator never vanishes and the point stays on the
/// upper sheet. It sets the unit of one lattice step and nothing else — the
/// curvature is still read off a remainder, never carried as a constant
/// (51.2, 51.11). `None` where `M` does not bound the point: that lamina is
/// outside this chart and says so rather than being rounded into it.
pub fn hyperbolic_lamina(r: &BigRational, c: &BigRational, k: &BigRational, m: &BigRational) -> Option<Mv> {
    if m.is_zero() {
        return None;
    }
    let (u1, u2, u3) = (r / m, c / m, k / m);
    let n2 = &u1 * &u1 + &u2 * &u2 + &u3 * &u3;
    let denom = one() - &n2;
    if denom.is_zero() || denom.is_negative() {
        // `|u| ≥ 1` is off this chart — outside the sheet's stereographic
        // image, refused rather than projected onto its edge (rule 11).
        return None;
    }
    let two = BigRational::from_integer(num_bigint::BigInt::from(2));
    let s = one() / &denom;
    Some(
        Mv::blade(E1, &two * &u1 * &s)
            .add(&Mv::blade(E2, &two * &u2 * &s))
            .add(&Mv::blade(E3, &two * &u3 * &s))
            .add(&Mv::blade(EM, (one() + &n2) * &s)),
    )
}

/// THE POLE OF THE SHEET — `v₀ = e₋`, the point `u = 0` maps to, and the
/// reference a frame's own base point is obtained from by its sandwich.
pub fn hyperboloid_pole() -> Mv {
    Mv::blade(EM, one())
}

/// AXIOM B's TRANSITION — the directed geodesic transvection carrying
/// `v_a` to `v_b` on the sheet, exactly over ℚ (54.7).
///
/// ```text
///     R = 1 − v_b v_a  ∈  Cl⁰(3,1)(ℚ)
/// ```
///
/// VERIFIED BY THE PRODUCT, not asserted. With `v_a² = v_b² = −1`:
///
/// ```text
///     R R̃ = 1 − 2(v_a·v_b) + v_b v_a² v_b = 2(1 − v_a·v_b) ≥ 4 > 0
///     R v_a R̃ = 2(1 − v_a·v_b) v_b        ⟹  R v_a R̃ / R R̃ = v_b
/// ```
///
/// so the norm is a strictly positive rational — invertible with no square
/// root — and the transport is exact, forward-directed, and needs no
/// projective sign fix.
///
/// WHY NOT `Ω = m n` WITH `m = v_b+v_a`, `n = v_b−v_a`. On null rays those
/// are orthogonal (`m·n = p_b² − p_a² = 0`), so they anticommute and
/// `Ω² = −m²n² = 4s² ∈ ℚ*`: an INVOLUTION of order two, a half-turn that
/// swaps its endpoints. Edges built from it would oscillate `v_a ↔ v_b`
/// rather than propagate along the geodesic, and every scalar check would
/// still pass.
///
/// THE PLANE IS CARRIED, not a magnitude: `⟨R⟩₂ = v_a ∧ v_b` is the
/// geodesic 2-blade, with
/// `(v_a∧v_b)² = (v_a·v_b)² − v_a²v_b² = cosh²d − 1 > 0` — hyperbolic, and
/// `d` is never formed (rule 1).
pub fn geodesic_transvection(va: &Mv, vb: &Mv) -> Option<Mv> {
    let on_sheet = |v: &Mv| {
        let s = v.geo(v);
        s.scalar_part() == -one() && s.sub(&Mv::scalar(s.scalar_part())).is_zero()
    };
    if !on_sheet(va) || !on_sheet(vb) {
        return None;
    }
    let r = Mv::scalar(one()).sub(&vb.geo(va));
    // `R R̃ = 2(1 − v_a·v_b)` must be a non-zero pure scalar, which is what
    // `is_versor` reads — and it is ≥ 4 on the sheet, so the only way this
    // refuses is a vector that is not where it claimed to be.
    is_versor(&r)?;
    Some(r)
}

/// THE BASE POINT A FRAME STANDS AT: its own versor applied to the pole,
/// renormalised onto the sheet.
///
/// `v = V v₀ Ṽ` has `v² = −(V Ṽ)²`, so dividing by the weight puts it back
/// on `v² = −1` — one exact rational division, no root.
pub fn frame_base_point(v: &Mv) -> Option<Mv> {
    let w = is_versor(v)?;
    if w.is_zero() {
        return None;
    }
    let p = v.sandwich(&hyperboloid_pole());
    let out = p.scale(&(one() / w));
    let s = out.geo(&out);
    (s.scalar_part() == -one() && s.sub(&Mv::scalar(s.scalar_part())).is_zero()).then_some(out)
}
