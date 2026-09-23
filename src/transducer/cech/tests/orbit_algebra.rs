// 78.11, CG1 — a cell's orbit algebra: congruence per mode ray by commutator
// with the round, the failing grade, mode and plane on the refusal;
// coherence jointly; nothing read where nothing crossed or nothing read.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cech::*;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::{self, ConstructorOrbit, Mv, E1, E2, E3};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The round `e₁ ∧ e₂` and the two constructors whose meet it is.
fn round() -> (Mv, Vec<Mv>) {
    let nuclei = vec![Mv::blade(E1, q(1, 1)), Mv::blade(E2, q(1, 1))];
    (meet_of(&nuclei).expect("two nuclei meet"), nuclei)
}

/// A CIRCUIT TURNING IN THE ROUND'S OWN PLANE KEEPS IT: every mode ray
/// commutes with the round — congruent — and the quarter turn closes at
/// four; the constructors close on their two rays.
#[test]
fn a_turn_in_the_rounds_plane_is_congruent_and_closes() {
    let (m, nuclei) = round();
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let cell = read_cell(1, Some(&m), Some(&nuclei), &[Some(turn)], &mut stroke());
    assert_eq!(cell.congruence, Congruence::Congruent);
    assert_eq!(cell.coherence, Coherence::Closes(4));
    assert_eq!(cell.constructors, ConstructorOrbit::Closed { rays: 2 });
}

/// A CIRCUIT CARRYING THE ROUND IS REFUSED WITH ITS CONTENT: at the cell's
/// grade, the failing circuit and mode, the mode's class and plane, and the
/// bracket — the direction the round is carried.
#[test]
fn a_turn_that_carries_the_round_refuses_with_grade_mode_and_plane() {
    let (m, nuclei) = round();
    let across = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    let cell = read_cell(2, Some(&m), Some(&nuclei), &[Some(Mv::scalar(q(1, 1))), Some(across)], &mut stroke());
    let Congruence::Incongruent { grade, failures } = &cell.congruence else { panic!("refused: {:?}", cell.congruence) };
    assert_eq!(*grade, 2, "the failing grade rides the refusal");
    let [f] = failures.as_slice() else { panic!("one failing mode: {failures:?}") };
    assert_eq!((f.circuit, f.mode, f.kind, f.plane.clone()), (1, 0, BivectorKind::Elliptic, vec![E2 | E3]));
    assert!(!f.bracket.is_zero(), "the bracket is the direction the round is carried");
}

/// COHERENCE IS JOINT: a rosette leaves the cell quasi-periodic and HELD,
/// never refused; a screw holds it open by its parabolic mode.
#[test]
fn a_rosette_is_held_and_a_screw_is_open() {
    let (m, nuclei) = round();
    let rosette = cga::rotator(&q(3, 1), &q(4, 1));
    let held = read_cell(1, Some(&m), Some(&nuclei), &[Some(rosette)], &mut stroke());
    assert_eq!((held.congruence, held.coherence), (Congruence::Congruent, Coherence::Held));
    let screw = cga::rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let open = read_cell(1, Some(&m), Some(&nuclei), &[Some(screw)], &mut stroke());
    assert_eq!(open.coherence, Coherence::Open(BivectorKind::Parabolic));
}

/// NOTHING IS CLASSED OFF WHAT DID NOT READ: an unread circuit leaves the
/// cell unread by count; a missing round is named; a cell no circuit
/// crosses was asked nothing.
#[test]
fn unread_circuits_a_missing_round_and_no_circuit_are_named() {
    let (m, nuclei) = round();
    let unread = read_cell(1, Some(&m), Some(&nuclei), &[None], &mut stroke());
    assert_eq!((unread.congruence, unread.coherence), (Congruence::Unread { unread: 1 }, Coherence::Unread));
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    assert_eq!(read_cell(1, None, None, &[Some(turn)], &mut stroke()).congruence, Congruence::NoRound);
    let none = read_cell(1, Some(&m), Some(&nuclei), &[], &mut stroke());
    assert_eq!((none.congruence, none.capacity), (Congruence::NoCircuit, None));
}
