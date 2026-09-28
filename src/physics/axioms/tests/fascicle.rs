// 78.52 ROW 2 — HYPHAL FASCICLES, read on exact fascicles: the three
// streamlines, the slide as the fascicle kissing at infinity, and the fascicle
// kissing at a finite point, which is conformal but not rigid.

use crate::physics::fascicle::{streamline_of, Fascicle, Streamline};
use crate::transducer::cga::{point_at, sphere_at, translator_at, Mv};
use num_traits::One;

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

fn sphere(c: [i64; 3], r2: (i64, i64)) -> Mv {
    sphere_at(&c.map(|x| q(x, 1)), &q(r2.0, r2.1))
}

fn point(c: [i64; 3]) -> Mv {
    point_at(&c.map(|x| q(x, 1)))
}

fn one() -> Mv {
    Mv::scalar(num_rational::BigRational::one())
}

/// The squared distance between two conformal points, read off the algebra:
/// `|a − b|² = −2 (A · B) / ((A · e_∞)(B · e_∞))`.
fn dist2(a: &Mv, b: &Mv) -> num_rational::BigRational {
    let inf = crate::transducer::cga::e_inf();
    let (wa, wb) = (a.geo(&inf).scalar_part(), b.geo(&inf).scalar_part());
    q(-2, 1) * a.geo(b).scalar_part() / (wa * wb)
}

/// THE ELLIPTIC STREAMLINE. Two spheres of `r² = 2` at `0` and `(2,0,0)` cut
/// orthogonally (`δ² = 0`): their fascicle turns 180° about their circle —
/// the stream closes after two — and the circle's rational points `(1,1,0)`
/// and `(1,0,1)` stand fixed. Two unit spheres a unit apart cut at 60°
/// (`δ² = ¼`): the turn is 120°, closing after three.
#[test]
fn an_elliptic_fascicle_turns_about_its_circle_and_closes_at_its_period() {
    let f = Fascicle { first: sphere([0, 0, 0], (2, 1)), second: sphere([2, 0, 0], (2, 1)) };
    assert_eq!(f.inversive(), Some(q(0, 1)), "orthogonal: δ² = 0");
    assert_eq!(f.streamline(), Some(Streamline::Elliptic));
    assert!(f.stream(2).projective_eq(&one()), "the stream closes after two");
    for p in [point([1, 1, 0]), point([1, 0, 1])] {
        assert!(f.act(&p).is_some_and(|x| x.projective_eq(&p)), "the circle stands fixed");
    }
    let sixty = Fascicle { first: sphere([0, 0, 0], (1, 1)), second: sphere([1, 0, 0], (1, 1)) };
    assert_eq!(sixty.inversive(), Some(q(1, 4)), "δ² = ¼");
    assert!(!sixty.stream(2).projective_eq(&one()) && sixty.stream(3).projective_eq(&one()), "120°: closes after three");
    assert_eq!(streamline_of(&f.versor()), Some(Streamline::Elliptic), "the versor alone reads its streamline");
}

/// THE PARABOLIC STREAMLINE. The slide by `(2,0,0)` is the fascicle of
/// parallel planes kissing at `e_∞`: `δ² = 1`, and its versor at unit weight
/// IS the translator — rigid, carrying two points a unit apart to two points
/// a unit apart. Two unit spheres kissing at `(1,0,0)` are parabolic too: the
/// contact stands fixed and the stream never closes — but the fascicle is
/// conformal, NOT rigid: two points a unit apart are carried to points
/// another distance apart. The slide perception sees is the fascicle
/// kissing at infinity.
#[test]
fn a_parabolic_fascicle_flows_about_its_contact_and_the_slide_is_the_fascicle_kissing_at_infinity() {
    let d = [q(2, 1), q(0, 1), q(0, 1)];
    let planes = Fascicle::parallel(&d).expect("a slide");
    assert_eq!(planes.inversive(), Some(q(1, 1)), "parallel planes: δ² = 1");
    assert!(planes.versor().projective_eq(&translator_at(&d)), "the translator is the fascicle");
    let (a, b) = (point([3, 0, 0]), point([3, 1, 0]));
    let (a2, b2) = (planes.act(&a).expect("weighed"), planes.act(&b).expect("weighed"));
    assert_eq!(dist2(&a2, &b2), q(1, 1), "the slide is rigid");
    let kissing = Fascicle { first: sphere([0, 0, 0], (1, 1)), second: sphere([2, 0, 0], (1, 1)) };
    assert_eq!(kissing.streamline(), Some(Streamline::Parabolic));
    let contact = point([1, 0, 0]);
    assert!(kissing.act(&contact).is_some_and(|x| x.projective_eq(&contact)), "the contact stands fixed");
    assert!(!kissing.stream(1).projective_eq(&one()) && !kissing.stream(4).projective_eq(&one()), "it never closes");
    let (a2, b2) = (kissing.act(&a).expect("weighed"), kissing.act(&b).expect("weighed"));
    assert!(dist2(&a2, &b2) != q(1, 1), "a finite contact is conformal, not rigid");
    assert_eq!(streamline_of(&translator_at(&d)), Some(Streamline::Parabolic), "the translator reads parabolic");
}

/// THE HYPERBOLIC STREAMLINE. Concentric spheres of `r² = 1` and `r² = 4`
/// are apart (`δ² = 25/16`): their fascicle breathes — a point at `(1,0,0)` is
/// carried to `(4,0,0)`, lengths scaled by `r₂² / r₁² = 4`, and a sphere of
/// `r² = 1/4` about the centre to one of `r² = 4`, its extent — and so its
/// second moments — scaled by `(r₂² / r₁²)² = 16`.
#[test]
fn a_hyperbolic_fascicle_dilates_by_the_ratio_of_its_squares() {
    let f = Fascicle { first: sphere([0, 0, 0], (1, 1)), second: sphere([0, 0, 0], (4, 1)) };
    assert_eq!(f.inversive(), Some(q(25, 16)), "apart: δ² = 25/16");
    assert_eq!(f.streamline(), Some(Streamline::Hyperbolic));
    assert!(f.act(&point([1, 0, 0])).is_some_and(|x| x.projective_eq(&point([4, 0, 0]))), "lengths × 4");
    let small = sphere([0, 0, 0], (1, 4));
    assert!(f.act(&small).is_some_and(|x| x.projective_eq(&sphere([0, 0, 0], (4, 1)))), "extent × 16");
    assert_eq!(streamline_of(&f.versor()), Some(Streamline::Hyperbolic));
}
