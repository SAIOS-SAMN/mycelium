// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The composition of turnings and motions (78.171 §2, a motion; §3.1, §4):
//! exact over `ℚ(ζ_ℓ)`, the winding kept, refusals by name.

use super::super::*;
use super::fixtures::{degrees, frame, plane, turning};
use forms::coordinate::Square;
use num_bigint::BigInt;

/// The motion of two turnings on one plane is the product of their motions
/// in `ℚ(ζ_lcm)`: the angle summed and the motion multiplied agree, for
/// angles of several conductors, of either sign and past a whole turn.
#[test]
fn turning_then_agrees_with_motion_product() {
    let p = plane(&frame(2), 1, 2);
    let angles = [(30, 1), (72, 1), (-45, 1), (400, 1), (360, 7), (0, 1)];
    for &(a, ad) in &angles {
        for &(b, bd) in &angles {
            let (x, y) = (turning(&p, a, ad), turning(&p, b, bd));
            let composed = x.then(&y).expect("one plane");
            assert_eq!(composed.degrees(), &(degrees(a, ad) + degrees(b, bd)), "{a}/{ad} then {b}/{bd}");
            assert_eq!(composed.motion(), &x.motion().product(y.motion()).expect("in ℚ(ζ_lcm)"), "{a}/{ad} then {b}/{bd}");
        }
    }
}

/// A turning by `360°·t/ℓ` in lowest terms has order `ℓ`; a whole number
/// of turns has order 1 and keeps its winding.
#[test]
fn turning_order_equals_reduced_denominator() {
    let p = plane(&frame(2), 1, 2);
    assert_eq!(turning(&p, 120, 1).order(), Some(3));
    assert_eq!(turning(&p, 150, 1).order(), Some(12));
    assert_eq!(turning(&p, 720, 1).order(), Some(1));
    assert_eq!(turning(&p, 720, 1).winding(), BigInt::from(2));
    assert_eq!(turning(&p, -400, 1).winding(), BigInt::from(1), "K = ⌊|θ|/360⌋");
}

/// A turning on a plane reversed (`e_+2+1`) composes with one on `e_+1+2`
/// by the opposite angle: 30° on `e_+1+2` then 30° on `e_+2+1` is no
/// turning at all.
#[test]
fn motion_composes_reversed_plane_by_opposite_angle() {
    let f = frame(2);
    let m = Motion::of(vec![turning(&plane(&f, 1, 2), 30, 1), turning(&plane(&f, 2, 1), 30, 1)]).expect("one plane");
    assert!(m.turnings().is_empty(), "the angles cancel exactly");
}

/// Turnings on disjoint planes commute and are held as two components; an
/// angle asked of their holonomy is refused `ScalarCollapse` with both
/// planes, never summed into one number.
#[test]
fn holonomy_angle_refuses_two_planes_as_scalar_collapse() {
    let f = frame(4);
    let (a, b) = (plane(&f, 1, 2), plane(&f, 3, 4));
    let one_way = Motion::of(vec![turning(&a, 30, 1), turning(&b, 45, 1)]).expect("disjoint planes commute");
    let other_way = Motion::of(vec![turning(&b, 45, 1), turning(&a, 30, 1)]).expect("disjoint planes commute");
    assert_eq!(one_way, other_way, "the product is the same in either order");
    let h = Holonomy::of(one_way);
    assert_eq!(h.angle(), Err(FieldRefusal::ScalarCollapse { planes: vec![a.key().clone(), b.key().clone()] }));
    assert_eq!(h.monodromy(), Monodromy::Torsion { order: 24 }, "lcm(12, 8)");
}

/// Turnings on two planes sharing one unit whose motions are not rational
/// (30°, no whole number of quarter turns) are refused `NotExact` with
/// `IrrationalMotion`, the held plane and its angle.
#[test]
fn motion_refuses_irrational_motions_sharing_one_unit() {
    let f = frame(3);
    let (a, b) = (plane(&f, 1, 2), plane(&f, 2, 3));
    assert_eq!(
        Motion::of(vec![turning(&a, 30, 1), turning(&b, 30, 1)]),
        Err(FieldRefusal::NotExact { residue: Residue::IrrationalMotion { plane: a.key().clone(), degrees: degrees(30, 1) } })
    );
    assert_eq!(
        Motion::of(vec![turning(&a, 90, 1), turning(&b, 45, 1)]),
        Err(FieldRefusal::NotExact { residue: Residue::IrrationalMotion { plane: b.key().clone(), degrees: degrees(45, 1) } })
    );
}

/// A plane named by one unit twice holds no turning and is refused
/// `NotTurningPlane`; a plane past the frame is refused `PastFrame`.
#[test]
fn turning_plane_refuses_one_unit_twice_and_planes_past_frame() {
    let f = frame(2);
    assert!(matches!(TurningPlane::of(&f, Square::Positive, 1, 1), Err(FieldRefusal::NotTurningPlane(_))));
    assert_eq!(TurningPlane::of(&f, Square::Positive, 1, 3), Err(FieldRefusal::PastFrame { plane: 3 }));
    assert!(TurningPlane::of(&f, Square::Negative, 1, 2).is_ok(), "the vacuum's e₋₁e₋₂ is a turning plane");
}
