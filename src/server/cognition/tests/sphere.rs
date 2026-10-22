// A CLOSED SPHERE AS A UNIVERSE'S SPACE (2026-10-22, proposed to netstratum
// and xylarium, both of whose universes are closed 3-spheres stated as the
// hyperbolic `Curved`): `Geometry::Sphere(R)`, its motions `Spin(4)`, its
// places on `v² = +1` about `e₊`, a throat's cap level `cos(2m/R)` and its
// fit short of the antipode.

use crate::anastome::dialogue::tube::{decode_tube_reading, encode_tube_reading, is_spin_4, is_spin_plus_31, place_in, Level, TubeReading};
use crate::anastome::dialogue::universe::Geometry;
use crate::server::cognition::mouth::sphere_throat;
use crate::transducer::cga::{rotator_on_blade, sphere_pole, translator_at, Mv, E1, EM, EP};
use num_rational::BigRational as Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// THE SPHERE IS ITS OWN KIND ON THE WIRE: it writes and reads back as
/// itself, a positive radius holds and a zero one does not, and it is never
/// the hyperbolic `Curved` of the same radius.
#[test]
fn a_sphere_is_its_own_kind_on_the_wire() {
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
fn a_rotation_of_the_sphere_places_a_mouth_exactly() {
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
fn a_motion_of_another_space_is_refused_on_the_sphere() {
    let space = Geometry::Sphere((17, 2));
    let translator = translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    assert!(!is_spin_4(&translator) && place_in(&space, &translator).is_none(), "a translator is no rotation of S³");
    let boost = rotator_on_blade(&q(5, 4), &q(3, 4), E1 | EM);
    assert!(!is_spin_4(&boost) && place_in(&space, &boost).is_none(), "a boost of ℍ³ is no rotation of S³");
}

/// A THROAT ON THE SPHERE IS A CAP SHORT OF THE ANTIPODE: a small throat's
/// level is the lower bound of `cos(2m/R)` — below `1`, and at most the true
/// cosine, so the cap is never undersized — and its ratio to `π` is below `1`,
/// so it fits; a throat whose angular radius passes `π` would cover the
/// sphere, and does not fit.
#[test]
fn a_throat_on_the_sphere_is_a_cap_short_of_the_antipode() {
    let r = q(17, 2);
    let (level, ratio) = sphere_throat(&q(1, 1), &r).expect("a positive mass and radius");
    assert!(level < q(1, 1) && level > q(9, 10), "cos(4/17), from below: {level}");
    assert!(Level::Sphere { level: level.clone(), ratio: ratio.clone() }.fits(), "4/17 is well short of π: {ratio}");
    let (_, past) = sphere_throat(&q(14, 1), &r).expect("read");
    assert!(!Level::Sphere { level: q(0, 1), ratio: past.clone() }.fits(), "56/17 passes π: {past}");
    assert!(sphere_throat(&q(0, 1), &r).is_none() && sphere_throat(&q(1, 1), &q(0, 1)).is_none());
}

/// A SPHERE'S LEVEL TRAVELS WHOLE in a tube's reading, its kind its own.
#[test]
fn a_sphere_level_reads_back_whole() {
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
