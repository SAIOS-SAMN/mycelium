//! A4, 51.2 — the conformal null basis, the embeddings of a base point and
//! a sphere, and the translation and rotation versors that carry them.

use super::multivector::{half, one, pseudo, zero, Mv, E1, E2, EM, EP};
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

/// Embed a base point of ℚⁿ on the null cone: `X(x) = x + ½‖x‖² e_∞ + e_o`,
/// with `x = Σ xᵢ eᵢ` over the point's own coordinates (71.2(3)). The ONE
/// producer of a conformal point, at every `n` (PH2: no fixed arity).
///
/// `‖x‖²` is READ FROM THE ALGEBRA, `⟨x x⟩₀`, so the signature that squares
/// every base axis to `+1` is the product's own (`metric_sign`) and is not
/// written a second time here as a sum of squares (law 12; audit N-2).
pub fn point_at(x: &[BigRational]) -> Mv {
    let v = base_vector(x);
    let x2n = v.norm2();
    v.add(&e_inf().scale(&(half() * x2n))).add(&e_o())
}

/// THE BASE VECTOR `x = Σ xᵢ eᵢ` of a point's own coordinates, each on its
/// own base axis: the ONE producer of the vector the embeddings, the
/// translator and the sheet are built on.
pub fn base_vector(x: &[BigRational]) -> Mv {
    x.iter()
        .enumerate()
        .fold(Mv::zero(), |m, (i, xi)| m.add(&Mv::blade(super::multivector::base_axis(i + 1), xi.clone())))
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
/// WHY NOT [`point_at`]. That is the flat embedding `x + ½‖x‖²e_∞ + e_o`: it
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
///
/// ON THE BASE'S OWN `n` COORDINATES (PH2): `x` holds every base
/// coordinate, and the condition is `‖x‖² − x₀² = −R²` at that `n`.
pub fn hyperbolic_point(x: &[BigRational], x0: &BigRational, r: &BigRational) -> Option<Mv> {
    if *x0 <= zero() {
        return None;
    }
    // THE CONDITION IS THE POINT'S OWN SQUARE, `p² = 0`, read from the
    // product: `e₋² = −1` and `e₊² = +1` are the signature's, not a formula
    // written beside it (law 12).
    let p = base_vector(x).add(&Mv::blade(EM, x0.clone())).add(&Mv::blade(EP, r.clone()));
    match p.norm2().is_zero() {
        true => Some(p),
        false => None,
    }
}

/// The dual sphere of ℚⁿ, `S = X(c) − ½ r² e_∞`, about a centre of the
/// base's own `n` coordinates: the ONE producer of a sphere. A point at
/// squared distance `r2` from `c`; `r2 = 0` recovers the point (a
/// volumeless sphere). Off-cone iff r² ≠ 0.
pub fn sphere_at(c: &[BigRational], r2: &BigRational) -> Mv {
    point_at(c).sub(&e_inf().scale(&(half() * r2)))
}

/// The base point back out of a conformal point or sphere at `n`: the scale
/// is `α = −(P·e_∞)`, then `xᵢ = P_{eᵢ}/α` for every base axis. `None` at
/// infinity (`α = 0`).
pub fn base_at(p: &Mv, n: usize) -> Option<Vec<BigRational>> {
    let alpha = -p.geo(&e_inf()).scalar_part();
    if alpha.is_zero() {
        return None;
    }
    Some((1..=n).map(|i| p.coeff(super::multivector::base_axis(i)) / &alpha).collect())
}

/// The translation versor `T(d) = 1 − ½ d e_∞`, `d = Σ dᵢ eᵢ` over the
/// displacement's own `n` coordinates (PH2: no fixed arity). Acts by
/// sandwich: `T X(x) T̃ = X(x + d)`, up to the homogeneous scale.
/// Translators commute (51.2): the base is a lattice.
///
/// 54.7 R3 — THIS IS THE PARABOLIC PINNING, and it is what the hyperbolic
/// unification replaces. Its generator is `B = −½ d e_∞` with `e_∞² = 0`,
/// so `B² = 0`: a null bivector fixing the point at infinity, which is a
/// privileged planar boundary in a manifold whose sectional curvature is
/// not zero. [`transvection`] is the geodesic motion that has no such
/// fixed point.
pub fn translator_at(d: &[BigRational]) -> Mv {
    let d_einf = base_vector(d).geo(&e_inf());
    Mv::scalar(one()).sub(&d_einf.scale(&half()))
}

/// The Euclidean rotation versor in the `e1e2` plane as an EXACT even
/// multivector `s + b·e1e2` (unnormalized: the sandwich carries the scale
/// `s²+b²`, absorbed by homogeneity on the cone). The 180° turn is
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

/// Does base point `p` lie on dual sphere `S`? `X(p)·S = 0`, exact.
pub fn on_sphere(p: &Mv, s: &Mv) -> bool {
    p.geo(s).scalar_part().is_zero()
}

/// THE BOUNDARY PROJECTION `∂σ = σ ⌟ I⁻¹` — the dual contraction against the
/// pseudoscalar, the bridge from the volumetric body to its boundary
/// 2-chain on the cube complex (which then enters the Mayer–Vietoris / L2 coboundary
/// sequence for settlement). Here realized as the geometric product with the
/// inverse pseudoscalar; the caller reads the grade it needs.
///
/// THE DUAL IS TAKEN IN A SPACE, and `n` names it. A value's own `n` is the
/// smallest algebra holding it, and a planar rotor sits in the algebra at
/// `n = 2` while living in the corpus's `n = 3`; its dual in the two is not
/// the same element. So the caller says which space the boundary is read
/// in, and the pseudoscalar is that space's.
pub fn boundary_dual(sigma: &Mv, n: u8) -> Option<Mv> {
    let i = Mv::blade(pseudo(n.max(sigma.n())), one());
    // THE DUAL'S ORIENTATION IS CONTENT (78.28(2)): `I Ĩ = ±1`, the sign
    // the metric gives the pseudoscalar, so `σ Ĩ` is divided by that sign
    // and by nothing else.
    let sign = super::group::weight_of(&i)?;
    Some(sigma.geo(&i.reverse()).scale(&(one() / sign)))
}
