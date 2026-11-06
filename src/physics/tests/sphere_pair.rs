// 78.52 ROW 2 — HYPHAL FASCICLES, read on exact fascicles: the three
// streamlines, the slide as the fascicle kissing at infinity, and the fascicle
// kissing at a finite point, which is conformal but not rigid.

use crate::physics::sphere_pair::{streamline_of, Fascicle, Streamline};
use crate::transducer::cga::{membrane_at, point_at, translator_at, Mv, PairRegime};
use num_traits::One;

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

/// The membrane about `c` at the gauge `rho`: a body's own enclosure.
fn sphere(c: [i64; 3], rho: (i64, i64)) -> Mv {
    membrane_at(&c.map(|x| q(x, 1)), &q(rho.0, rho.1)).expect("a positive gauge")
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

/// THE ELLIPTIC STREAMLINE. The membranes at the gauges 3 about `0` and 4
/// about `(5,0,0)` cut orthogonally (a turn of order two): the pair's
/// fascicle turns 180° about the pair's circle — the stream closes after two
/// — and the circle's rational points `(9/5, 12/5, 0)` and `(9/5, 0, 12/5)`
/// stand fixed. Two unit membranes a unit apart cut at 60° (order three):
/// the turn is 120°, closing after three.
#[test]
fn fascicle_elliptic_turns_about_circle_closing_at_period() {
    let f = Fascicle { first: sphere([0, 0, 0], (3, 1)), second: sphere([5, 0, 0], (4, 1)) };
    assert_eq!(f.regime(), PairRegime::Finite(2), "orthogonal: a turn of order two");
    assert_eq!(f.streamline(), Some(Streamline::Elliptic));
    assert!(f.stream(2).projective_eq(&one()), "the stream closes after two");
    for p in [point_at(&[q(9, 5), q(12, 5), q(0, 1)]), point_at(&[q(9, 5), q(0, 1), q(12, 5)])] {
        assert!(f.act(&p).is_some_and(|x| x.projective_eq(&p)), "the circle stands fixed");
    }
    let sixty = Fascicle { first: sphere([0, 0, 0], (1, 1)), second: sphere([1, 0, 0], (1, 1)) };
    assert_eq!(sixty.regime(), PairRegime::Finite(3), "cut at 60°: a turn of order three");
    assert!(!sixty.stream(2).projective_eq(&one()) && sixty.stream(3).projective_eq(&one()), "120°: closes after three");
    assert_eq!(streamline_of(&f.versor()), Some(Streamline::Elliptic), "the versor alone reads its streamline");
}

/// THE PARABOLIC STREAMLINE. The slide by `(2,0,0)` is the fascicle of
/// parallel planes kissing at `e_∞`, parabolic, and its versor at unit weight
/// IS the translator — rigid, carrying two points a unit apart to two points
/// a unit apart. Two unit spheres kissing at `(1,0,0)` are parabolic too: the
/// contact stands fixed and the stream never closes — but the fascicle is
/// conformal, NOT rigid: two points a unit apart are carried to points
/// another distance apart. The slide perception sees is the fascicle
/// kissing at infinity.
#[test]
fn fascicle_parabolic_fixes_contact_and_slide_kisses_at_infinity() {
    let d = [q(2, 1), q(0, 1), q(0, 1)];
    let planes = Fascicle::parallel(&d).expect("a slide");
    assert_eq!(planes.regime(), PairRegime::Parabolic, "parallel planes kiss at e_∞");
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

/// THE HYPERBOLIC STREAMLINE. Concentric membranes at the gauges 1 and 2
/// are apart (a boost): the pair's fascicle breathes — a point at `(1,0,0)`
/// is carried to `(4,0,0)`, lengths scaled by `(ρ₂ / ρ₁)² = 4`, and the
/// membrane at `1/2` about the centre to the one at `2`, its gauge scaled by
/// 4 and so its second moments by 16.
#[test]
fn fascicle_hyperbolic_dilates_by_ratio_of_squares() {
    let f = Fascicle { first: sphere([0, 0, 0], (1, 1)), second: sphere([0, 0, 0], (2, 1)) };
    assert_eq!(f.regime(), PairRegime::Boost, "apart: a boost");
    assert_eq!(f.streamline(), Some(Streamline::Hyperbolic));
    assert!(f.act(&point([1, 0, 0])).is_some_and(|x| x.projective_eq(&point([4, 0, 0]))), "lengths × 4");
    let small = sphere([0, 0, 0], (1, 2));
    assert!(f.act(&small).is_some_and(|x| x.projective_eq(&sphere([0, 0, 0], (2, 1)))), "gauge × 4");
    assert_eq!(streamline_of(&f.versor()), Some(Streamline::Hyperbolic));
}
