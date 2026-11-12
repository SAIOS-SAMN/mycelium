// A CLOSED SPHERE AS A UNIVERSE'S SPACE (a closed 3-sphere is no longer
// stated as the hyperbolic `Curved`): `Geometry::Sphere(R)`, its motions `Spin(4)`, its
// places on `v² = +1` about `e₊`, a throat's cap level `cos(2m/R)` and its
// fit short of the antipode.

use crate::anastome::dialogue::tube::{decode_tube_reading, encode_tube_reading, is_spin_4, is_spin_plus_31, place_in, Level, TubeReading};
use crate::anastome::dialogue::universe::Geometry;
use crate::server::cognition::domain_passage::{fits_in, sphere_throat};
use crate::transducer::cga::{rotator_on_blade, sphere_pole, translator_at, Mv, E1, EM, EP};
use num_rational::BigRational as Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// THE SPHERE IS ITS OWN KIND ON THE WIRE: it writes and reads back as
/// itself, a positive radius holds and a zero one does not, and it is never
/// the hyperbolic `Curved` of the same radius.
#[test]
fn geometry_sphere_round_trips() {
    let s = Geometry::Sphere((17, 2));
    let mut b = Vec::new();
    s.put(&mut b);
    let mut p = 0;
    assert_eq!(Geometry::take(&b, &mut p), Some(s.clone()));
    assert_eq!(p, b.len(), "read whole");
    assert!(s.holds() && !Geometry::Sphere((0, 1)).holds());
    let mut c = Vec::new();
    Geometry::Curved((17, 2)).put(&mut c);
    assert_ne!(b, c, "a sphere is never written as the hyperbolic kind");
}

/// A ROTATION OF THE SPHERE PLACES A MOUTH ON IT, EXACTLY: the identity
/// stands at the pole `e₊`; the rotor `4/5 + 3/5 e₁e₊` turns the pole in its
/// plane by twice its angle, to `7/25 e₊ ± 24/25 e₁` — a rational point of
/// `S³`, `v² = +1`. A rotation of `S³` is no motion of `ℍ³`.
#[test]
fn place_in_sphere_places_rotation_exactly() {
    let space = Geometry::Sphere((17, 2));
    // `Mv`'s `==` is projective; a place is its own scale, read exactly.
    let unit = |v: &Mv| {
        let s = v.geo(v);
        *s.coeff(0) == q(1, 1) && s.sub(&Mv::scalar(q(1, 1))).is_zero()
    };
    let one = Mv::scalar(q(1, 1));
    let pole = place_in(&space, &one).expect("the identity is placed");
    assert!(*pole.coeff(EP) == q(1, 1) && pole.sub(&sphere_pole()).is_zero(), "the identity stands at the pole");
    let r = rotator_on_blade(&q(4, 5), &q(3, 5), E1 | EP);
    assert!(is_spin_4(&r), "a rotor of S³");
    assert!(!is_spin_plus_31(&r), "not a motion of ℍ³");
    let v = place_in(&space, &r).expect("placed");
    assert!(unit(&v), "on the sphere, exactly");
    assert_eq!(*v.coeff(EP), q(7, 25), "the pole turned by twice the rotor's angle");
    assert_eq!(v.coeff(E1) * v.coeff(E1), q(576, 625), "into its plane, 24/25 of the way");
}

/// A MOTION OF ANOTHER SPACE STANDS NOWHERE ON THE SPHERE: a translator of
/// flat space and a boost of `ℍ³` each move `e₋`, so neither is in
/// `Spin(4)`, and neither places a mouth.
#[test]
fn place_in_sphere_refuses_foreign_motion() {
    let space = Geometry::Sphere((17, 2));
    let translator = translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    assert!(!is_spin_4(&translator) && place_in(&space, &translator).is_none(), "a translator is no rotation of S³");
    let boost = rotator_on_blade(&q(5, 4), &q(3, 4), E1 | EM);
    assert!(!is_spin_4(&boost) && place_in(&space, &boost).is_none(), "a boost of ℍ³ is no rotation of S³");
}

/// A THROAT ON THE SPHERE IS A CAP SHORT OF THE ANTIPODE: a small throat's
/// level is the lower bound of `cos(2m/R)` — below `1`, and at most the true
/// cosine, so the cap is never undersized — and it fits; a throat whose
/// angular radius passes `π` would cover the sphere, and does not fit.
#[test]
fn sphere_throat_fits_cap_short_of_antipode() {
    let r = q(17, 2);
    let (level, _) = sphere_throat(&q(1, 1), &r).expect("a positive mass and radius");
    assert!(level < q(1, 1) && level > q(9, 10), "cos(4/17), from below: {level}");
    let space = Geometry::Sphere((17, 2));
    assert_eq!(fits_in(&space, &q(1, 1)), Some(true), "4/17 is well short of π");
    assert_eq!(fits_in(&space, &q(14, 1)), Some(false), "56/17 passes π");
    assert!(sphere_throat(&q(0, 1), &r).is_none() && sphere_throat(&q(1, 1), &q(0, 1)).is_none());
    assert_eq!(fits_in(&space, &q(0, 1)), None, "no throat without mass");
}

/// D1.2 PHASE A — THE SPHERE FIT IS DECIDED PAST ANY GRID: an angular radius
/// within `2⁻²⁰⁰` of `π` below it fits and one within it above does not,
/// where the ratio on the former `2⁻⁶⁴` grid could not tell the first from the
/// antipode.
#[test]
fn sphere_fit_is_decided_past_any_grid() {
    use crate::core::certified::{pi, Certified};
    let near = pi(200);
    let space = Geometry::Sphere((2, 1));
    // r = 2m/R = m on a sphere of radius 2.
    assert_eq!(fits_in(&space, near.lo()), Some(true), "just short of π");
    assert_eq!(fits_in(&space, near.hi()), Some(false), "just past π");
    let ratio = Certified::Exact(near.lo().clone()).div(&pi(64)).expect("π is no zero").hi().clone();
    assert!(ratio >= q(1, 1), "the former grid's ratio reads the fitting cap as past the antipode");
}

/// D1.2 PHASE A — THE FLAT FIT IS THE MEET WITH THE PERIOD IMAGE: the
/// horizon sphere of radius `2m` is apart from its image under the shortest
/// period `L` exactly where `2m/L < ½`; on `L = 4m` the two touch and do not
/// fit; past it they cut.
#[test]
fn flat_fit_meet_equals_half_ratio() {
    use crate::anastome::dialogue::universe::Metric;
    let torus = |l: u64| Geometry::Torus { periods: vec![(l, 1), (3 * l, 1)], metric: Metric::L2 };
    for (m, l) in [(q(1, 1), 5), (q(1, 1), 4), (q(1, 1), 3), (q(5, 2), 11), (q(5, 2), 10)] {
        let half = q(1, 2);
        let ratio = q(2, 1) * &m / q(i64::try_from(l).expect("small"), 1);
        assert_eq!(fits_in(&torus(l), &m), Some(ratio < half), "m {m}, L {l}");
    }
}

/// A SPHERE'S LEVEL TRAVELS WHOLE in a tube's reading, its kind its own.
#[test]
fn tube_reading_round_trips_sphere_level() {
    let (level, ratio) = sphere_throat(&q(1, 1), &q(17, 2)).expect("read");
    let reading = TubeReading {
        from_place: sphere_pole(),
        to_place: sphere_pole(),
        from_level: Level::Sphere { level: level.clone(), ratio: ratio.clone() },
        to_level: Level::Sphere { level, ratio },
        mass: (1, 1),
    };
    assert_eq!(decode_tube_reading(&encode_tube_reading(Some(&reading))), Some(Some(reading)));
}
