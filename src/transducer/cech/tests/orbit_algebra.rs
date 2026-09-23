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
    assert!(
        matches!(&f.round, RoundKept::Carried(bracket) if !bracket.is_zero()),
        "the bracket is the direction the round is carried: {:?}",
        f.round
    );
}

/// CG1b — A TURN INSIDE THE SPAN KEEPS THE ROUND AND MOVES THE CONSTRUCTORS:
/// the quarter turn in `e₁₂` keeps `e₁ ∧ e₂` and carries `e₁` onto `e₂`'s
/// ray — a permutation of the two. A turn by `(3, 4, 5)` in the same plane
/// keeps the round and carries every nucleus off the set.
#[test]
fn a_turn_inside_the_span_keeps_the_round_and_not_the_constructors() {
    let (m, nuclei) = round();
    let quarter = read_cell(1, Some(&m), Some(&nuclei), &[Some(cga::rotator(&q(1, 1), &q(1, 1)))], &mut stroke());
    assert_eq!((quarter.congruence.clone(), quarter.permutes.clone()), (Congruence::Congruent, Permutes::All));
    assert_eq!(quarter.modes[0].permutation, Some(vec![1, 0]), "e₁ ↦ e₂, e₂ ↦ −e₁: the swap, as rays");
    let rosette = read_cell(1, Some(&m), Some(&nuclei), &[Some(cga::rotator(&q(3, 1), &q(4, 1)))], &mut stroke());
    assert_eq!(rosette.congruence, Congruence::Congruent, "the round is kept");
    assert_eq!(rosette.permutes, Permutes::Moves(vec![(0, 0)]), "and the constructors are not");
}

/// CG1b — THE ROUND REVERSED IS THE HELD ℤ₂, NEVER INCONGRUENT: a half turn
/// in `e₂₃` carries `e₁ ∧ e₂` to `−e₁ ∧ e₂`, and it permutes the
/// constructors (`e₁` fixed, `e₂` onto its own ray).
#[test]
fn a_reversed_round_reads_the_held_z2() {
    let (m, nuclei) = round();
    let half = Mv::blade(E2 | E3, q(1, 1));
    let cell = read_cell(1, Some(&m), Some(&nuclei), &[Some(half)], &mut stroke());
    assert_eq!(cell.modes[0].round, RoundKept::Reversed);
    assert_eq!(cell.congruence, Congruence::HeldZ2 { reversed: 1 });
    assert_eq!(cell.permutes, Permutes::All);
}

/// A ZERO MEET IS NO ROUND: five nuclei in a four-dimensional span wedge to
/// zero, which commutes with everything — degeneration, not invariance. The
/// Gram form reads the configuration in its place.
#[test]
fn a_zero_meet_is_no_round_and_the_gram_reads_in_its_place() {
    let nuclei: Vec<Mv> = [E1, E2, E3, cga::EP, E1].iter().enumerate().map(|(i, &m)| Mv::blade(m, q(i as i64 + 1, 1))).collect();
    let m = meet_of(&nuclei).expect("the nuclei wedge");
    assert!(m.is_zero(), "dependent nuclei: the meet vanishes");
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let cell = read_cell(4, Some(&m), Some(&nuclei), &[Some(turn)], &mut stroke());
    assert_eq!(cell.congruence, Congruence::NoRound, "never Congruent");
    let g = cell.gram.expect("the Gram form reads");
    assert_eq!((g.rank, g.dependent()), (4, 1), "four directions, one dependence");
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
