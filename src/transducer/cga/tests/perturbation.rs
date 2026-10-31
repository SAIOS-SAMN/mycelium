// 78.46 — LIFT AND DRAG, a test per clause: the split is exact, it re-sums,
// the drag commutes with the orbit's mode and the lift anticommutes, the
// alignment is read off the pair of parts, and a non-blade refuses.

use crate::transducer::cga::multivector::one;
use crate::transducer::cga::*;

fn plane(a: usize, b: usize) -> Mv {
    Mv::blade(a, one()).geo(&Mv::blade(b, one()))
}

/// The orbit turns on `e₁e₂`. A neighbour on the SAME plane is pure drag
/// (aligned: it changes the rate, never the plane).
#[test]
fn lift_drag_reads_same_plane_as_pure_drag() {
    let b_o = plane(E1, E2);
    let b_n = plane(E1, E2).scale(&num_rational::BigRational::from_integer(3.into()));
    let split = lift_drag(&b_o, &b_n).expect("a blade with a non-zero square splits every bivector");
    assert_eq!(split.drag, b_n, "all of it commutes with the orbit");
    assert!(split.lift.is_zero(), "none of it tilts the plane");
    assert_eq!(split.alignment(), Alignment::Aligned);
}

/// A neighbour on `e₂e₃`, a plane sharing one axis with `e₁e₂`,
/// anticommutes with it: pure lift (orthogonal), its commutator a rotation
/// in `e₁e₃`, a plane neither generator is (60.4).
#[test]
fn lift_drag_reads_one_shared_axis_as_pure_lift() {
    let b_o = plane(E1, E2);
    let b_n = plane(E2, E3);
    let split = lift_drag(&b_o, &b_n).expect("splits");
    assert!(split.drag.is_zero());
    assert_eq!(split.lift, b_n);
    assert_eq!(split.alignment(), Alignment::Orthogonal);
    let commutator = b_o.geo(&b_n).sub(&b_n.geo(&b_o));
    assert_eq!(commutator, b_o.geo(&split.lift).scale(&num_rational::BigRational::from_integer(2.into())), "[B_o, B_n] = 2 B_o B_⊥");
    assert!(!commutator.grade(2).is_zero(), "the tilt is a rotation in a third plane");
}

/// In general both parts: a neighbour mixing the orbit's plane with a
/// tilted one splits into each, and they re-sum to it exactly.
#[test]
fn lift_drag_splits_mixed_neighbour_and_resums() {
    let b_o = plane(E1, E2);
    let b_n = plane(E1, E2).add(&plane(E1, E3));
    let split = lift_drag(&b_o, &b_n).expect("splits");
    assert_eq!(split.alignment(), Alignment::Both);
    assert_eq!(split.drag.add(&split.lift), b_n);
    assert!(b_o.geo(&split.drag).sub(&split.drag.geo(&b_o)).is_zero());
    assert!(b_o.geo(&split.lift).add(&split.lift.geo(&b_o)).is_zero());
}

/// A generator that is no blade has no involution to split by: `e₁e₂ + e₃e₊`
/// squares to a scalar plus a 4-vector, and it is refused, never
/// approximated (rule 11).
#[test]
fn lift_drag_refuses_non_blade_orbit_mode() {
    let not_a_blade = plane(E1, E2).add(&plane(E3, EP));
    assert!(lift_drag(&not_a_blade, &plane(E2, E3)).is_none());
}
