// 73.2(5), OR1 — the centre of a motion, read from its fixed-point equation
// over ℚ or refused by name.

use crate::transducer::cga::{self, Centre, Mv};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

/// A turn about a point holds the point: the centre reads back as it,
/// exactly, on the axis the turn leaves still.
#[test]
fn a_turn_about_a_point_holds_that_point() {
    let c = [q(1), q(2), q(0)];
    let neg: Vec<BigRational> = c.iter().map(|x| -x).collect();
    let turn = cga::rotator(&q(3), &q(4));
    let about = cga::translator_at(&c).geo(&turn).geo(&cga::translator_at(&neg));
    match cga::centre_of(&about, 3) {
        Centre::Fixed { point, .. } => {
            assert_eq!(&point[..2], &[q(1), q(2)], "the turn holds its centre: {point:?}")
        }
        other => panic!("a turn about a point has a finite centre: {other:?}"),
    }
}

/// A translation holds nothing finite and holds `e_∞`: its centre is at
/// infinity. A dilation holds directions and scales both its poles: refused.
/// A multivector that is no versor has no action: refused.
#[test]
fn a_translation_is_centred_at_infinity_and_a_dilation_is_refused() {
    let slide = cga::translator_at(&[q(2), q(0), q(0)]);
    assert!(matches!(cga::centre_of(&slide, 3), Centre::AtInfinity { .. }), "{:?}", cga::centre_of(&slide, 3));
    assert_eq!(cga::centre_of(&cga::dilator(&q(2)), 3), Centre::Scales);
    let not_a_versor = Mv::scalar(q(1)).add(&Mv::blade(cga::E1, q(1)));
    assert_eq!(cga::centre_of(&not_a_versor, 3), Centre::Unread);
}

/// 73.1 — AN ORBIT IS READ WHOLE: a turn on the grid plane is a resonant
/// orbit on `e12`, its precession written in canonical coordinates, and a
/// translation is an escape whose centre is at infinity.
#[test]
fn an_orbit_reads_its_type_plane_precession_and_centre() {
    use crate::transducer::sentence::OrbitReading;
    let turn = cga::rotator(&q(0), &q(1));
    let r = OrbitReading::of(&turn, 3);
    assert!(r.kind.starts_with("resonant"), "{}", r.line());
    assert_eq!(r.planes, vec![cga::E1 | cga::E2], "{}", r.line());
    assert!(r.line().contains("precession [e12 1]"), "{}", r.line());
    let slide = OrbitReading::of(&cga::translator_at(&[q(1), q(0), q(0)]), 3);
    assert!(slide.kind.starts_with("escape"), "{}", slide.line());
    assert!(matches!(slide.centre, Centre::AtInfinity { .. }), "{}", slide.line());
    // PC1: its precession occupies one parabolic plane, read by the census's
    // own reader, and no turn.
    assert!(slide.planes.is_empty(), "{}", slide.line());
    assert_eq!(slide.parabolic, vec![(1, cga::Pole::Infinity)], "{}", slide.line());
}
