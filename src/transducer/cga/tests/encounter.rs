// 78.110 E1 — THE ENCOUNTER, a test per clause of the ruling: lift is
// `(v·F)/(g·D)` over the orbit's own modes; a neighbour commuting with the
// orbit changes its rate and never tilts it; a neighbour with an
// anticommuting part tilts it; where an inverse does not exist, lift is drag.

use crate::transducer::cga::multivector::one;
use crate::transducer::cga::*;
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The 90° turn on the plane `mask`: `1 + e_ab`, a turn of order four.
fn turn(mask: usize) -> Mv {
    Mv::scalar(one()).add(&Mv::blade(mask, q(1)))
}

/// A NEIGHBOUR COMMUTING WITH THE ORBIT IS ALL DRAG, AND ITS LIFT DOES NOT
/// TILT. The orbit turns on `e₁e₂` (magnetism, its bounce the period four); a
/// neighbour on the same plane commutes: the drag is the whole force, `F·D⁻¹`
/// the identity, and the lift `v·g⁻¹` the bounce itself — a rate, no plane.
#[test]
fn encounter_drags_commuting_neighbour_without_tilt() {
    let e = encounter(&turn(E1 | E2), &turn(E1 | E2), &mut stroke()).expect("read");
    assert!(!e.fell_to_drag, "every inverse exists");
    assert_eq!(e.drag, Mv::blade(E1 | E2, q(1)), "the force is all drag");
    assert!(!e.tilts(), "a rate, no plane: {:?}", e.lift);
    assert!(e.lift.projective_eq(&Mv::scalar(q(4))), "the lift is the bounce, the period four");
}

/// A NEIGHBOUR WITH AN ANTICOMMUTING PART TILTS THE ORBIT. The neighbour turns
/// on `e₁e₂` and `e₂e₃` at once (`1 + e₁₂ + e₂₃`): its drag is the `e₁e₂`
/// part, and the rest — anticommuting with the orbit's plane — carries a lift
/// with a bivector part: a plane neither the orbit nor the drag turns in.
#[test]
fn encounter_tilts_orbit_by_anticommuting_part() {
    let neighbour = Mv::scalar(one()).add(&Mv::blade(E1 | E2, q(1))).add(&Mv::blade(E2 | E3, q(1)));
    let e = encounter(&turn(E1 | E2), &neighbour, &mut stroke()).expect("read");
    assert!(!e.fell_to_drag);
    assert_eq!(e.drag, Mv::blade(E1 | E2, q(1)), "the commuting part is the drag");
    assert!(e.tilts(), "the anticommuting part tilts: {:?}", e.lift);
}

/// WHERE NO INVERSE EXISTS, LIFT IS DRAG (the ruling). A neighbour wholly
/// orthogonal to the orbit's plane has no drag — zero, no inverse — so its lift
/// is the drag; and an orbit whose mode is null (a slide: no square to split
/// by) takes the force whole as drag.
#[test]
fn encounter_reads_lift_as_drag_without_inverse() {
    let orthogonal = encounter(&turn(E1 | E2), &turn(E2 | E3), &mut stroke()).expect("read");
    assert!(orthogonal.fell_to_drag, "no drag, no inverse");
    // Equality is projective and zero is no ray: the zero drag is read as
    // zero, and the lift with it.
    assert!(orthogonal.drag.is_zero() && orthogonal.lift.is_zero(), "lift is drag: none");
    let slide = translator_at(&[q(1), q(0), q(0)]);
    let null = encounter(&slide, &turn(E1 | E2), &mut stroke()).expect("read");
    assert!(null.fell_to_drag, "a null mode has no square to split by");
    assert_eq!(null.lift, null.drag, "lift is drag: a ray changes rate, never tilts");
}

/// A NEIGHBOUR WITH NO GENERATOR, OR AN ORBIT WHOSE MODES DO NOT READ, IS NOT
/// ENCOUNTERED — refused by name, never read as no force.
#[test]
fn encounter_refuses_unread_neighbour() {
    let mixed = Mv::scalar(one()).add(&Mv::blade(E1, q(1)));
    assert_eq!(encounter(&turn(E1 | E2), &mixed, &mut stroke()).err(), Some(Unencountered::NeighbourUnread));
}
