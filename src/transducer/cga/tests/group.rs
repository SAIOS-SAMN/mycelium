// A4, 51.2 — the rational conformal group acts exactly, and the frame axis
// is a base coordinate.

use super::common::q;
use crate::transducer::cga::*;
use num_traits::Zero;

/// The dilator scales points exactly with no root or logarithm; the
/// reflector negates rows; both are versors; projective equality reads
/// the scalar ratio and refuses non-proportional blades; bytes round-trip.
#[test]
fn rational_versors_act_exactly_and_are_verified_in_closed_form() {
    let d = dilator(&q(3, 2));
    assert!(is_versor(&d).is_some(), "the dilator is a versor");
    let moved = d.sandwich(&point(&q(2, 1), &q(-4, 1), &q(6, 1)));
    assert_eq!(base(&moved), Some((q(3, 1), q(-6, 1), q(9, 1))), "X(2,−4,6) → X(3,−6,9) at ρ = 3/2");
    let r = reflector();
    let flipped = r.sandwich(&point(&q(2, 1), &q(5, 1), &q(7, 1)));
    assert_eq!(base(&flipped), Some((q(-2, 1), q(5, 1), q(7, 1))));
    let s = sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(5, 1));
    assert_eq!(projective_ratio(&s.scale(&q(7, 3)), &s), Some(q(7, 3)));
    assert!(projective_ratio(&s, &sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(6, 1))).is_none());
    let composed = translator(&q(1, 1), &q(0, 1), &q(2, 1)).geo(&d);
    let mut bytes = Vec::new();
    composed.to_bytes(&mut bytes);
    let mut pos = 0usize;
    assert_eq!(Mv::from_bytes(&bytes, &mut pos), Some(composed));
}

#[test]
fn a4_projective_eq_either_sign_and_key() {
    let t = translator(&q(2, 1), &q(0, 1), &q(0, 1));
    assert!(t.projective_eq(&t.scale(&q(-3, 2))));
    assert_eq!(t.canonical().unwrap().key(), t.scale(&q(-3, 2)).canonical().unwrap().key());
    assert_eq!(t, t.scale(&q(5, 1)), "PartialEq is projective");
    let u = translator(&q(3, 1), &q(0, 1), &q(0, 1));
    assert_ne!(t.key(), u.key());
}

/// 51.2 — THE THIRD COORDINATE IS A BASE COORDINATE: translators along
/// the three axes commute, a rotation in the `e1e3` plane carries the
/// row axis onto the frame axis exactly, and a body in one frame at
/// `k = 0` is carried there and nowhere else by a plane motion.
#[test]
fn the_frame_axis_is_a_base_coordinate() {
    let (z, o) = (q(0, 1), q(1, 1));
    let t1 = translator(&q(2, 1), &z, &z);
    let t3 = translator(&z, &z, &q(5, 1));
    assert!(t1.geo(&t3).projective_eq(&t3.geo(&t1)), "translators commute (51.2)");
    assert_eq!(
        base(&t1.geo(&t3).sandwich(&point(&z, &z, &z))),
        Some((q(2, 1), z.clone(), q(5, 1))),
        "and compose by adding the displacements"
    );
    let quarter_13 = rotator_on_blade(&o, &o, E1 | E3);
    let moved = base(&quarter_13.sandwich(&point(&o, &z, &z))).expect("a rotation keeps the point finite");
    assert!(moved.0.is_zero() && moved.1.is_zero() && !moved.2.is_zero(), "the row axis turns onto the frame axis: {moved:?}");
    let plane = rotator(&o, &o).geo(&translator(&q(3, 1), &q(-1, 1), &z));
    let carried = base(&plane.sandwich(&point(&q(4, 1), &q(4, 1), &z))).expect("finite");
    assert!(carried.2.is_zero(), "a plane motion keeps k = 0: {carried:?}");
}
