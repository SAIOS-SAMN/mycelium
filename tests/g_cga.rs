#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "§2.4 binds cognition; this crate is a test"
)]
//! Gate — the exact conformal geometric algebra (Cl(4,1), 51.2), the volumetric
//! body (ruled 2026-08-30). The cure for dimensional volumetric
//! collapse: objects become spheres with volume, rigid motions become one versor
//! sandwich, and — the thesis made arithmetic — the demand `Q = B·A⁻¹` only
//! EXISTS once the object has volume, because a null point is non-invertible.
//! The base has three coordinates, row, column and frame (51.1).
use mycelium::transducer::cga::{
    base_at, boundary_dual, e_inf, e_o, on_sphere, point_at, rotator, rotator_on_blade, sphere_at, translator_at, Mv,
    E1, E3,
};
use num_bigint::BigInt;
use num_rational::BigRational;

// THE CORPUS'S `n = 3`, NAMED FOR THIS GATE (PH2, BQ4): the embeddings take
// any `n`, and these fixtures read row, column and frame.
fn point(a: &BigRational, b: &BigRational, c: &BigRational) -> Mv {
    point_at(&[a.clone(), b.clone(), c.clone()])
}
fn sphere(a: &BigRational, b: &BigRational, c: &BigRational, r2: &BigRational) -> Mv {
    sphere_at(&[a.clone(), b.clone(), c.clone()], r2)
}
fn translator(a: &BigRational, b: &BigRational, c: &BigRational) -> Mv {
    translator_at(&[a.clone(), b.clone(), c.clone()])
}
fn base(p: &Mv) -> Option<(BigRational, BigRational, BigRational)> {
    let x = base_at(p, 3)?;
    Some((x[0].clone(), x[1].clone(), x[2].clone()))
}

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}
fn frac(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The null cone is exact: e_o, e_∞ square to 0, pair to −1; the base lives ON
/// the cone (X·X = 0); and distance is bilinear one level up (X(a)·X(b) =
/// −½|a−b|²) — no roots, no epsilon — on all three coordinates.
#[test]
fn the_null_cone_is_exact() {
    assert!(e_o().geo(&e_o()).scalar_part() == q(0), "e_o² = 0");
    assert!(e_inf().geo(&e_inf()).scalar_part() == q(0), "e_∞² = 0");
    assert!(e_o().geo(&e_inf()).scalar_part() == q(-1), "e_o·e_∞ = −1");

    let a = point(&q(1), &q(2), &q(0));
    assert!(a.geo(&a).scalar_part() == q(0), "the base lives on the cone: X·X = 0");

    // |a−b|² = 3² + 4² + 12² = 169  ⇒  X(a)·X(b) = −169/2.
    let b = point(&q(4), &q(6), &q(12));
    assert!(
        a.geo(&b).scalar_part() == frac(-169, 2),
        "distance is bilinear one level up: X(a)·X(b) = −½|a−b|², the frame axis counted"
    );
}

/// A rigid translation is ONE versor sandwich acting on the whole body — where
/// the 0-skeleton needed 2·area independent point-diffs. `T(d) X(x) T̃ = X(x+d)`,
/// exact, read straight back out of the homogeneous representative.
#[test]
fn a_translation_is_one_versor_sandwich() {
    let x = point(&q(2), &q(-3), &q(1));
    let t = translator(&q(5), &q(4), &q(-1));
    let moved = t.sandwich(&x);
    assert_eq!(
        base(&moved),
        Some((q(7), q(1), q(0))),
        "T(5,4,−1) shifts (2,−3,1) to (7,1,0) in one algebraic step"
    );
    // It still lands on the cone — it is a genuine point, not a smear.
    assert!(moved.geo(&moved).scalar_part() == q(0), "the moved body is still on the cone");
}

/// A sphere is a volumetric body: a base point lies on it iff X(p)·S = 0, exact.
#[test]
fn a_sphere_carries_volume() {
    let s = sphere(&q(0), &q(0), &q(0), &q(9)); // centre origin, r² = 9
    assert!(on_sphere(&point(&q(3), &q(0), &q(0)), &s), "(3,0,0) is on the r²=9 sphere");
    assert!(on_sphere(&point(&q(0), &q(3), &q(0)), &s), "(0,3,0) too");
    assert!(on_sphere(&point(&q(0), &q(0), &q(3)), &s), "and (0,0,3): the frame axis is a base axis");
    assert!(!on_sphere(&point(&q(2), &q(0), &q(0)), &s), "(2,0,0) is inside, not on it");
    // Its centre reads back exactly.
    assert_eq!(base(&s), Some((q(0), q(0), q(0))), "the sphere's centre is the origin");
}

/// THE THESIS, AS ARITHMETIC: the demand `Q = B·A⁻¹` cannot form on volumeless
/// bodies. A null point has no inverse (X·X = 0); a SPHERE (S·S = r² ≠ 0) does.
/// Volume is exactly what makes the coboundary demand factorable.
#[test]
fn volume_is_what_makes_the_quotient_exist() {
    let p_a = point(&q(1), &q(1), &q(0));
    let p_b = point(&q(4), &q(1), &q(0));
    // 78.32(1): no inverse is taken. A null point is no versor, so no
    // demand forms on it; a sphere is one, and the quotient is the ray
    // `S_B S̃_A`, read from either side.
    use mycelium::transducer::cga::is_versor;
    assert!(!is_versor(&p_a), "a null point is no versor — the collapse");
    assert!(!is_versor(&p_b), "so no demand forms between volumeless points");

    let s_a = sphere(&q(1), &q(1), &q(0), &q(4));
    let s_b = sphere(&q(4), &q(1), &q(0), &q(4));
    assert!(is_versor(&s_a), "a sphere has volume, hence it acts");
    let qd = s_b.geo(&s_a.reverse());
    // `S_A S̃_A` is the identity ray: a non-zero scalar.
    let id = s_a.geo(&s_a.reverse());
    assert!(id.scalar_part() != q(0), "S S̃ is a non-zero scalar");
    assert!(id.projective_eq(&Mv::scalar(q(1))), "the identity ray");
    assert!(!qd.is_zero(), "the demand between two distinct spheres is non-trivial");
}

/// The versor sandwich rotates the whole body exactly. The π-rotor e1e2 (unit,
/// rational — the hole the Cayley chart cannot reach) sends x ↦ −x on its plane
/// and leaves the frame axis; the unnormalised 90° turn 1+e1e2 sends (1,0,0)
/// ↦ (0,−1,0) with its scale absorbed by homogeneity (read back through `base`);
/// and the same turn on `e1e3` carries the row axis onto the frame axis (51.2).
#[test]
fn the_sandwich_rotates_the_whole_body() {
    let pi = rotator(&q(0), &q(1)); // e1e2 — a π rotation
    let turned = pi.sandwich(&point(&q(3), &q(2), &q(5)));
    assert_eq!(base(&turned), Some((q(-3), q(-2), q(5))), "π rotation sends x ↦ −x on its plane, k untouched");

    let turn_90 = rotator(&q(1), &q(1)); // 1 + e1e2, unnormalised 90°
    let spun = turn_90.sandwich(&point(&q(1), &q(0), &q(0)));
    assert_eq!(
        base(&spun),
        Some((q(0), q(-1), q(0))),
        "a 90° turn: (1,0,0) ↦ (0,−1,0), homogeneous scale divided out"
    );
    let turn_90_13 = rotator_on_blade(&q(1), &q(1), E1 | E3);
    let lifted = turn_90_13.sandwich(&point(&q(1), &q(0), &q(0)));
    assert_eq!(
        base(&lifted),
        Some((q(0), q(0), q(-1))),
        "the same turn on e1e3 carries the row axis onto the frame axis"
    );
}

/// The boundary projection ∂σ = σ ⌟ I⁻¹ is well-defined on a volumetric body —
/// the bridge from the multivector body to its cellular boundary/2-chain.
#[test]
fn the_body_projects_to_a_boundary() {
    let s = sphere(&q(2), &q(1), &q(0), &q(4));
    // Read in the corpus's space, n = 3, named.
    let d = boundary_dual(&s, 3).expect("the pseudoscalar is invertible");
    assert!(!d.is_zero(), "the boundary projection carries the body's dual");
}
