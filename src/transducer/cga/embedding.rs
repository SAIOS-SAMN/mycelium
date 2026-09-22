//! A4, 51.2 — the conformal null basis, the embeddings of a base point and
//! a sphere, and the translation and rotation versors that carry them.

use super::multivector::{half, one, zero, Mv, E1, E2, E3, EM, EP, PSEUDO};
use num_rational::BigRational;
use num_traits::Zero;

/// `e_o = ½(e- − e+)` — the origin (null).
pub fn e_o() -> Mv {
    Mv::blade(EM, half()).sub(&Mv::blade(EP, half()))
}

/// `e_∞ = e- + e+` — the point at infinity (null).
pub fn e_inf() -> Mv {
    Mv::blade(EM, one()).add(&Mv::blade(EP, one()))
}

/// Embed a base point on the null cone: `X(x) = x + ½‖x‖² e_∞ + e_o`, with
/// `x = x1 e1 + x2 e2 + x3 e3` — row, column, frame.
pub fn point(x1: &BigRational, x2: &BigRational, x3: &BigRational) -> Mv {
    let x2n = x1 * x1 + x2 * x2 + x3 * x3;
    let mut m = Mv::blade(E1, x1.clone()).add(&Mv::blade(E2, x2.clone())).add(&Mv::blade(E3, x3.clone()));
    m = m.add(&e_inf().scale(&(half() * x2n)));
    m.add(&e_o())
}

/// A CHART BASE POINT ON THE NULL CONE OF A NON-EUCLIDEAN SPACE (54.7 B8,
/// as ruled 2026-09-21).
///
/// ```text
///     p(x) = x₁e₁ + x₂e₂ + x₃e₃ + x₀e₋ + R e₊
/// ```
///
/// with `x` on the hyperboloid `⟨x,x⟩_{3,1} = x₁² + x₂² + x₃² − x₀² = −R²`,
/// `x₀ > 0`. Then, since `e₊² = +1` and `e₋² = −1`,
///
/// ```text
///     p² = (x₁² + x₂² + x₃²) − x₀² + R² = ⟨x,x⟩_{3,1} + R² = 0
/// ```
///
/// so every chart base point is strictly null, and the inner product of two
/// of them is the NON-EUCLIDEAN invariant:
///
/// ```text
///     pᵢ · pⱼ = ⟨xᵢ,xⱼ⟩_{3,1} + R² = R²(1 − cosh(d_ℍ(xᵢ,xⱼ)/R))
/// ```
///
/// WHY NOT [`point`]. That is the flat embedding `x + ½‖x‖²e_∞ + e_o`: it
/// introduces a degenerate point at infinity and a Cartesian norm, which
/// puts a privileged planar boundary into a manifold whose sectional
/// curvature is not zero. Its translations are the parabolic `T(a) = 1 +
/// ½a e_∞`; here there is no `e_∞` generator to build one from, and a
/// geodesic motion is a hyperbolic transvection — a mixed bivector `eᵢ ∧
/// e₋` — with elliptic rotations `eᵢ ∧ eⱼ` stabilising a timelike axis and
/// parabolic limit rotations on null bivectors. `Isom(ℍ³) ≅ Spin⁺(3,1) ⊂
/// Spin(4,1)`.
///
/// EXACT OR REFUSED (law 4, rule 11). `x₀` is a COORDINATE of the point and
/// is never solved for: `x₀² = ‖x‖² + R²` has no rational root in general,
/// and a chart whose four coordinates do not satisfy the hyperboloid
/// condition exactly is refused here rather than rounded onto it.
pub fn hyperbolic_point(
    x1: &BigRational,
    x2: &BigRational,
    x3: &BigRational,
    x0: &BigRational,
    r: &BigRational,
) -> Option<Mv> {
    if x1 * x1 + x2 * x2 + x3 * x3 - x0 * x0 + r * r != zero() {
        return None;
    }
    if *x0 <= zero() {
        return None;
    }
    Some(
        Mv::blade(E1, x1.clone())
            .add(&Mv::blade(E2, x2.clone()))
            .add(&Mv::blade(E3, x3.clone()))
            .add(&Mv::blade(EM, x0.clone()))
            .add(&Mv::blade(EP, r.clone())),
    )
}

/// The dual sphere `S = X(c) − ½ r² e_∞`: a point at squared-distance `r2` from
/// `c`. `r2 = 0` recovers the point (a volumeless sphere). Off-cone iff r²≠0.
pub fn sphere(c1: &BigRational, c2: &BigRational, c3: &BigRational, r2: &BigRational) -> Mv {
    point(c1, c2, c3).sub(&e_inf().scale(&(half() * r2)))
}

/// The translation versor `T(d) = 1 − ½ d e_∞`, `d = d1 e1 + d2 e2 + d3 e3`.
/// Acts by sandwich: `T X(x) T̃ = X(x + d)`, up to the homogeneous scale.
/// Translators commute (51.2): the base is a lattice.
///
/// 54.7 R3 — THIS IS THE PARABOLIC PINNING, and it is what the hyperbolic
/// unification replaces. Its generator is `B = −½ d e_∞` with `e_∞² = 0`,
/// so `B² = 0`: a null bivector fixing the point at infinity, which is a
/// privileged planar boundary in a manifold whose sectional curvature is
/// not zero. [`transvection`] is the geodesic motion that has no such
/// fixed point.
pub fn translator(d1: &BigRational, d2: &BigRational, d3: &BigRational) -> Mv {
    let d = Mv::blade(E1, d1.clone()).add(&Mv::blade(E2, d2.clone())).add(&Mv::blade(E3, d3.clone()));
    let d_einf = d.geo(&e_inf());
    Mv::scalar(one()).sub(&d_einf.scale(&half()))
}

/// The Euclidean rotation versor in the `e1e2` plane as an EXACT even
/// multivector `s + b·e1e2` (unnormalized: the sandwich carries the scale
/// `s²+b²`, absorbed by homogeneity on the cone). The quarter turn is
/// `s=0, b=1` — the hole a rational chart cannot reach, an ordinary citizen here.
pub fn rotator(s: &BigRational, b: &BigRational) -> Mv {
    rotator_on_blade(s, b, E1 | E2)
}

/// A rotation versor on any bivector blade of the algebra: the three base
/// planes `e1e2`, `e1e3`, `e2e3` are the rotor chart's `(0,1)`, `(0,3)`,
/// `(1,3)` (51.2).
pub fn rotator_on_blade(s: &BigRational, b: &BigRational, blade: usize) -> Mv {
    Mv::scalar(s.clone()).add(&Mv::blade(blade, b.clone()))
}

/// Read the base point `(x1, x2, x3)` back out of a homogeneous conformal
/// point or sphere: the scale is `α = −(P·e_∞)`, then `x = (P_e1, P_e2,
/// P_e3)/α`. `None` at infinity (`α = 0`).
pub fn base(p: &Mv) -> Option<(BigRational, BigRational, BigRational)> {
    let alpha = -p.geo(&e_inf()).scalar_part();
    if alpha.is_zero() {
        return None;
    }
    Some((p.coeff(E1) / &alpha, p.coeff(E2) / &alpha, p.coeff(E3) / &alpha))
}

/// Does base point `p` lie on dual sphere `S`? `X(p)·S = 0`, exact.
pub fn on_sphere(p: &Mv, s: &Mv) -> bool {
    p.geo(s).scalar_part().is_zero()
}

/// THE BOUNDARY PROJECTION `∂σ = σ ⌟ I⁻¹` — the dual contraction against the
/// pseudoscalar, the bridge from the volumetric body to its boundary
/// 2-chain on the cube complex (which then enters the Mayer–Vietoris / L2 coboundary
/// sequence for settlement). Here realized as the geometric product with the
/// inverse pseudoscalar; the caller reads the grade it needs.
pub fn boundary_dual(sigma: &Mv) -> Option<Mv> {
    let i = Mv::blade(PSEUDO, one());
    Some(sigma.geo(&i.versor_inverse()?))
}
