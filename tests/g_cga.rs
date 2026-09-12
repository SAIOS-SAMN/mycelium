#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "§2.4 binds cognition; this crate is a test"
)]
//! Gate — the exact conformal geometric algebra (Cl(3,1)), the volumetric body
//! (operator ruling 2026-08-30). The cure for dimensional volumetric collapse:
//! objects become spheres with volume, rigid motions become one versor sandwich,
//! and — the thesis made arithmetic — the demand `Q = B·A⁻¹` only EXISTS once
//! the object has volume, because a null point is non-invertible.
use mycelium::transducer::cga::{
    base, boundary_dual, e_inf, e_o, on_sphere, point, rotator, sphere, translator, Mv,
};
use num_bigint::BigInt;
use num_rational::BigRational;

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}
fn frac(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The null cone is exact: e_o, e_∞ square to 0, pair to −1; the base lives ON
/// the cone (X·X = 0); and distance is bilinear one level up (X(a)·X(b) =
/// −½|a−b|²) — no roots, no epsilon.
#[test]
fn the_null_cone_is_exact() {
    assert!(e_o().geo(&e_o()).scalar_part() == q(0), "e_o² = 0");
    assert!(e_inf().geo(&e_inf()).scalar_part() == q(0), "e_∞² = 0");
    assert!(e_o().geo(&e_inf()).scalar_part() == q(-1), "e_o·e_∞ = −1");

    let a = point(&q(1), &q(2));
    assert!(a.geo(&a).scalar_part() == q(0), "the base lives on the cone: X·X = 0");

    // |a−b|² = 3² + 4² = 25  ⇒  X(a)·X(b) = −25/2.
    let b = point(&q(4), &q(6));
    assert!(
        a.geo(&b).scalar_part() == frac(-25, 2),
        "distance is bilinear one level up: X(a)·X(b) = −½|a−b|²"
    );
}

/// A rigid translation is ONE versor sandwich acting on the whole body — where
/// the 0-skeleton needed 2·area independent point-diffs. `T(d) X(x) T̃ = X(x+d)`,
/// exact, read straight back out of the homogeneous representative.
#[test]
fn a_translation_is_one_versor_sandwich() {
    let x = point(&q(2), &q(-3));
    let t = translator(&q(5), &q(4));
    let moved = t.sandwich(&x);
    assert_eq!(
        base(&moved),
        Some((q(7), q(1))),
        "T(5,4) shifts (2,−3) to (7,1) in one algebraic step"
    );
    // It still lands on the cone — it is a genuine point, not a smear.
    assert!(moved.geo(&moved).scalar_part() == q(0), "the moved body is still on the cone");
}

/// A sphere is a volumetric body: a base point lies on it iff X(p)·S = 0, exact.
#[test]
fn a_sphere_carries_volume() {
    let s = sphere(&q(0), &q(0), &q(9)); // centre origin, r² = 9
    assert!(on_sphere(&point(&q(3), &q(0)), &s), "(3,0) is on the r²=9 sphere");
    assert!(on_sphere(&point(&q(0), &q(3)), &s), "(0,3) too");
    assert!(!on_sphere(&point(&q(2), &q(0)), &s), "(2,0) is inside, not on it");
    // Its centre reads back exactly.
    assert_eq!(base(&s), Some((q(0), q(0))), "the sphere's centre is the origin");
}

/// THE THESIS, AS ARITHMETIC: the demand `Q = B·A⁻¹` cannot form on volumeless
/// bodies. A null point has no inverse (X·X = 0); a SPHERE (S·S = r² ≠ 0) does.
/// Volume is exactly what makes the coboundary demand factorable.
#[test]
fn volume_is_what_makes_the_quotient_exist() {
    let p_a = point(&q(1), &q(1));
    let p_b = point(&q(4), &q(1));
    assert!(p_a.versor_inverse().is_none(), "a null point is non-invertible — the collapse");
    assert!(Mv::quotient(&p_b, &p_a).is_none(), "no demand forms between volumeless points");

    let s_a = sphere(&q(1), &q(1), &q(4));
    let s_b = sphere(&q(4), &q(1), &q(4));
    assert!(s_a.versor_inverse().is_some(), "a sphere has volume, hence an inverse");
    let qd = Mv::quotient(&s_b, &s_a).expect("the demand forms once there is volume");
    // S_A is invertible: S_A · S_A⁻¹ = 1 exactly.
    let id = s_a.geo(&s_a.versor_inverse().unwrap());
    assert!(id.scalar_part() == q(1), "S·S⁻¹ = 1 (scalar)");
    assert!(id.sub(&Mv::scalar(q(1))).is_zero(), "S·S⁻¹ is a pure scalar 1");
    assert!(!qd.is_zero(), "the demand between two distinct spheres is non-trivial");
}

/// The versor sandwich rotates the whole body exactly. The π-rotor e1e2 (unit,
/// rational — the hole the Cayley chart cannot reach) sends x ↦ −x; the
/// unnormalised quarter-turn 1+e1e2 sends (1,0) ↦ (0,−1) with its scale absorbed
/// by homogeneity (read back through `base`).
#[test]
fn the_sandwich_rotates_the_whole_body() {
    let pi = rotator(&q(0), &q(1)); // e1e2 — a π rotation
    let turned = pi.sandwich(&point(&q(3), &q(2)));
    assert_eq!(base(&turned), Some((q(-3), q(-2))), "π rotation sends x ↦ −x");

    let quarter = rotator(&q(1), &q(1)); // 1 + e1e2, unnormalised 90°
    let spun = quarter.sandwich(&point(&q(1), &q(0)));
    assert_eq!(
        base(&spun),
        Some((q(0), q(-1))),
        "quarter turn (1,0) ↦ (0,−1), homogeneous scale divided out"
    );
}

/// The boundary projection ∂σ = σ ⌟ I⁻¹ is well-defined on a volumetric body —
/// the bridge from the multivector body to its cellular boundary/2-chain.
#[test]
fn the_body_projects_to_a_boundary() {
    let s = sphere(&q(2), &q(1), &q(4));
    let d = boundary_dual(&s).expect("the pseudoscalar is invertible");
    assert!(!d.is_zero(), "the boundary projection carries the body's dual");
}
