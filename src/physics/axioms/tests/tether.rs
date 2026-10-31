// 59, 63–65, 72.1 — the tether `V = s_b s_a`, carried whole, and the contact
// regime read off its class.

use crate::transducer::blade::{self, tether, Contact};
use crate::transducer::cga;

fn q(n: i64) -> crate::core::Q {
    crate::core::Q::from_integer(n.into())
}

/// THE REGIME IS READ OFF THE TETHER EXACTLY AND SCALE-FREE (59.3, 72.1).
///
/// `δ = −⟨V⟩₀ / √⟨VṼ⟩₀` is irrational in general, so the regime is read by
/// sign and by the equal-degree comparison `⟨V⟩₀²` against `⟨VṼ⟩₀`; a global
/// dilation moves nothing, and no root is taken. There is no horizon: the
/// band that decided whether a tether formed is retired (SQ1 items 2 and 3),
/// and the regime is content.
#[test]
fn contact_of_reads_tether_exactly_and_scale_free() {
    // A BODY AGAINST ITSELF is `δ = −1`: internal tangency, a contact
    // interface and not overlap (65).
    let a = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade;
    let v = tether(&a, &a).expect("a body has a centroid");
    assert_eq!(Contact::of(&v), Some(Contact::TangentInternal), "a body against itself touches itself from inside");
    assert!(v.scalar_part() > q(0), "and ⟨V⟩₀ = r² is positive");

    let far = blade::shape_of(&[(0, 100, 0), (1, 100, 0)]).blade;
    let v_far = tether(&a, &far).expect("two centroids");
    assert_eq!(Contact::of(&v_far), Some(Contact::Apart), "distant bodies are separated");

    // SCALE-FREE: doubling every coordinate does not move the regime.
    let a2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]).blade;
    let far2 = blade::shape_of(&[(0, 200, 0), (1, 200, 0), (0, 201, 0), (1, 201, 0)]).blade;
    let scaled = tether(&a2, &far2).expect("two centroids");
    assert_eq!(Contact::of(&scaled), Contact::of(&v_far), "a dilation does not change the regime");

    // The weight is strictly positive — `r_a² r_b²` — and never rooted.
    assert!(v.geo(&v.reverse()).scalar_part() > q(0));
    assert!(v_far.geo(&v_far.reverse()).scalar_part() > q(0));
}

/// THE TETHER'S SCALAR PART IS THE NUCLEI'S INNER PRODUCT, AND ITS WEIGHT
/// THE NUCLEI'S SQUARES (T3).
///
/// `⟨V⟩₀ = s_a · s_b = ½(r_a² + r_b² − ‖P_a − P_b‖²)`, linear in each sphere,
/// and `⟨VṼ⟩₀ = s_a² s_b² = r_a² r_b²`. Asserted so the linear form is a
/// measured fact.
#[test]
fn tether_is_product_of_conformal_spheres() {
    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(0, 5, 0), (1, 5, 0), (0, 6, 0)]).blade,
        blade::shape_of(&[(3, 3, 0)]).blade,
    ];
    for a in &bodies {
        for b in &bodies {
            let v = tether(a, b).expect("both centroids read");
            let sa = cga::sphere_at(&a.centroid().expect("a"), &a.radius2());
            let sb = cga::sphere_at(&b.centroid().expect("b"), &b.radius2());
            assert_eq!(v.scalar_part(), sa.geo(&sb).scalar_part(), "⟨V⟩₀ = s_a · s_b");
            assert_eq!(v.geo(&v.reverse()).scalar_part(), a.radius2() * b.radius2(), "⟨VṼ⟩₀ = r_a² r_b²");
        }
    }
}

/// T3 — THE TETHER DEPENDS ONLY ON THE RELATIVE MOTOR (ruled 2026-09-22).
///
/// A versor is `λ` times an orthogonal map, so a common motor scales
/// `s_a · s_b` by `λ²` and the scale-free reading does not move: absolute
/// position is gauge. Asserted on a motor with a turn AND a displacement, so
/// the fixture is not a translation identity in disguise.
#[test]
fn tether_regime_is_invariant_under_common_motor() {
    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(4, 7, 0), (5, 7, 0), (4, 8, 0)]).blade,
        blade::shape_of(&[(-3, 2, 0)]).blade,
    ];
    let nucleus = |b: &blade::Blade| cga::sphere_at(&b.centroid().expect("a centroid"), &b.radius2());
    let motor = cga::at_three::translator(&q(11), &q(-4), &q(2)).geo(&cga::rotator(&q(1), &q(1)));
    // A RAY, NOT A UNIT: `rotator(1,1) = 1 + e₁₂` has `M M̃ = 2`, and the
    // sandwich scales the inner product by `λ² = 4` (57.2).
    let lambda = motor.geo(&motor.reverse()).scalar_part();
    assert_eq!(lambda, q(2), "the fixture's motor is a ray of norm 2, not a unit");
    for a in &bodies {
        for b in &bodies {
            let (sa, sb) = (nucleus(a), nucleus(b));
            let v = sb.geo(&sa);
            let moved = motor.sandwich(&sb).geo(&motor.sandwich(&sa));
            assert_eq!(moved.scalar_part(), &lambda * &lambda * v.scalar_part(), "⟨V⟩₀ picks up λ²");
            assert_eq!(Contact::of(&moved), Contact::of(&v), "and the regime does not move");
        }
    }
    // `s² = +r² > 0`: the nucleus is SPACELIKE, the dual-sphere chart where
    // contact is reachable (59.2).
    for b in &bodies {
        let s = nucleus(b);
        assert_eq!(s.geo(&s).scalar_part(), b.radius2(), "s² = +r², spacelike");
        assert!(b.radius2() > q(0), "and every extracted body is invertible");
    }
    // A body against itself is the identity motor, internal tangency.
    let v = tether(&bodies[0], &bodies[0]).expect("a centroid");
    assert_eq!(&v.scalar_part() * &v.scalar_part(), v.geo(&v.reverse()).scalar_part(), "δ = −1 exactly");
}

/// A MOTOR OF NEGATIVE NORM DOES NOT FLIP THE CONTACT SIDE (ruled
/// 2026-09-22).
///
/// `Cl(4,1)` is indefinite, so a versor may have `λ = M M̃ < 0`; the sandwich
/// is `λ` times an orthogonal map, the inner product picks up `λ² > 0`, and
/// the sign the side is read from cannot invert.
#[test]
fn sandwich_keeps_contact_side_under_negative_norm_motor() {
    use crate::transducer::cga::{Mv, E1, EM};
    let motor = Mv::blade(E1, q(1)).geo(&Mv::blade(EM, q(1)));
    let lambda = motor.geo(&motor.reverse()).scalar_part();
    assert_eq!(lambda, q(-1), "an indefinite signature admits a NEGATIVE versor norm");
    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(0, 9, 0), (1, 9, 0)]).blade,
        blade::shape_of(&[(0, 1, 0)]).blade,
    ];
    let nucleus = |b: &blade::Blade| cga::sphere_at(&b.centroid().expect("a centroid"), &b.radius2());
    for a in &bodies {
        for b in &bodies {
            let (sa, sb) = (nucleus(a), nucleus(b));
            let before = sa.geo(&sb).scalar_part();
            let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
            assert_eq!(after, &lambda * &lambda * &before, "the inner product picks up λ², never λ");
            assert_eq!(before > q(0), after > q(0), "a negative-norm motor does not move the contact side");
        }
    }
}

/// THE TETHER CARRIES WHAT THE SCALAR THREW AWAY (64, 72.1).
///
/// (1) A product of two vectors has grades 0 and 2 only. (2) Its grade-2
/// part is simple — a lone tether never carries a screw; the screw is the
/// network's (59.7, 60.4). (3) The coaxal family `λ s_a + μ s_b` lies in that
/// 2-blade. (4) Two pairs at the SAME regime and the same scale-free reading
/// carry DIFFERENT tethers: a relation read as its scalar could not tell
/// them apart, which is why `V` is carried whole.
#[test]
fn tether_separates_pairs_of_equal_scalar_reading() {
    let a = blade::shape_of(&[(0, 0, 0)]).blade;
    let along_r = blade::shape_of(&[(4, 0, 0)]).blade;
    let along_c = blade::shape_of(&[(0, 4, 0)]).blade;
    let (na, nr) = (a.nucleus().expect("a"), along_r.nucleus().expect("r"));
    let v = tether(&a, &along_r).expect("both nuclei");
    for g in [1u32, 3, 4, 5] {
        assert!(v.grade(g).is_zero(), "the product of two vectors has no grade {g} part");
    }
    let blade2 = v.grade(2);
    assert!(!blade2.is_zero(), "two distinct bodies relate through a 2-blade");
    assert!(cga::bivector_is_simple(&blade2), "a wedge of two vectors can never carry a screw");
    for (l, m) in [(1i64, 0i64), (0, 1), (1, 1), (3, -2), (-5, 7)] {
        let s = na.scale(&q(l)).add(&nr.scale(&q(m)));
        assert!(blade2.wedge_vector(2, &s).is_zero(), "the member ({l}, {m}) lies in the 2-blade");
    }
    let other = tether(&a, &along_c).expect("both nuclei");
    let reading = |v: &cga::Mv| &v.scalar_part() * &v.scalar_part() / v.geo(&v.reverse()).scalar_part();
    assert_eq!(reading(&v), reading(&other), "two pairs at the same scale-free reading");
    assert_eq!(Contact::of(&v), Contact::of(&other), "and in the same regime");
    assert!(!v.projective_eq(&other), "yet different tethers — carried whole, they stay apart");
}

/// THE REGIME HAS FIVE ARMS, NOT THREE (65), and `Contact::of` agrees with
/// `δ` computed independently from the centroids and radii.
#[test]
fn contact_of_reads_five_regimes_exactly() {
    let small = blade::shape_of(&[(0, 0, 0)]).blade;
    let big = blade::shape_of(&[
        (0, 0, 0), (1, 0, 0), (2, 0, 0), (0, 1, 0), (1, 1, 0),
        (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0),
    ])
    .blade;
    // `δ = (d² − r_a² − r_b²) / 2 r_a r_b`, read by `num` and `den2` from
    // the bodies directly — a second derivation, not the tether's.
    let check = |a: &blade::Blade, b: &blade::Blade| {
        let (pa, pb) = (a.centroid().expect("a"), b.centroid().expect("b"));
        let d2 = pa.iter().zip(&pb).fold(q(0), |s, (x, y)| s + (x - y) * (x - y));
        let num = d2 - a.radius2() - b.radius2();
        let den2 = q(4) * a.radius2() * b.radius2();
        let want = match ((&num * &num).cmp(&den2), num > q(0)) {
            (std::cmp::Ordering::Equal, true) => Contact::TangentExternal,
            (std::cmp::Ordering::Equal, false) => Contact::TangentInternal,
            (std::cmp::Ordering::Less, _) => Contact::Intersecting,
            (std::cmp::Ordering::Greater, true) => Contact::Apart,
            (std::cmp::Ordering::Greater, false) => Contact::Contained,
        };
        let got = Contact::of(&tether(a, b).expect("centroids")).expect("a weight");
        assert_eq!(got, want, "the tether's regime is δ's");
        got
    };
    assert_eq!(check(&small, &small), Contact::TangentInternal, "δ = −1 is contact, not overlap");
    assert_eq!(check(&big, &big), Contact::TangentInternal);
    let centred = blade::shape_of(&[(1, 1, 0)]).blade;
    assert_eq!(check(&centred, &big), Contact::Contained, "nested, δ < −1");
    let far = blade::shape_of(&[(0, 200, 0)]).blade;
    assert_eq!(check(&small, &far), Contact::Apart, "disjoint, δ > 1");
    let mut saw_secant = false;
    for d in 1..12i64 {
        if check(&big, &blade::shape_of(&[(0, d, 0)]).blade) == Contact::Intersecting {
            saw_secant = true;
        }
    }
    assert!(saw_secant, "the secant regime is reachable");
}

/// THE ANTIPODAL INVOLUTION `σ: s ↦ −s` EXCHANGES THE CONTACT BOUNDARY AND
/// SQUARES TO ONE (65).
#[test]
fn antipodal_exchanges_contact_boundary_and_squares_to_identity() {
    let all = [
        Contact::Apart,
        Contact::TangentExternal,
        Contact::Intersecting,
        Contact::TangentInternal,
        Contact::Contained,
    ];
    for c in all {
        assert_eq!(c.antipodal().antipodal(), c, "σ∘σ = id");
    }
    assert_eq!(Contact::Apart.antipodal(), Contact::Contained);
    assert_eq!(Contact::TangentExternal.antipodal(), Contact::TangentInternal);
    assert_eq!(Contact::Intersecting.antipodal(), Contact::Intersecting, "the secant regime is self-dual");
    assert!(Contact::TangentExternal.is_tangent());
    assert!(Contact::TangentInternal.is_tangent());
    for c in [Contact::Apart, Contact::Intersecting, Contact::Contained] {
        assert!(!c.is_tangent(), "{c:?} is not a contact interface");
    }
    for c in [Contact::Apart, Contact::TangentExternal, Contact::TangentInternal, Contact::Contained] {
        assert_ne!(c.antipodal(), c, "{c:?} is exchanged, not fixed");
    }
}
