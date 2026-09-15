// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 4 — THE SYMMETRY A VORTEX HOLDS: two discharges of one loop
//! prove the loop admits their difference, and that difference is a measured
//! rotor rather than a losing candidate.

use super::common::*;
use crate::transducer::alphabet::{LetterId, PlaneForm, Word};
use crate::transducer::sentence::*;

fn cert(axis: Axis, word: Word, after: crate::transducer::address::Address) -> PivotCertificate {
    PivotCertificate {
        chord: 0,
        axis,
        before: ClassVector::of_word(&order4()).expect("class").class_key(),
        after,
        rank_move: RankMove::Equal,
        mu: Mu::Unmeasured,
        by: PivotBy::Word { face: 0, word, side: Side::Right },
    }
}

/// TWO DISCHARGES OF ONE LOOP PROVE A SYMMETRY. `w_a⁻¹ · w_b` carries one
/// discharge to the other, and it STABILISES the loop's class because each
/// leaves that class where the other does. It is a rotation the mind
/// measured — 37.7's object — and it is what the collapse used to clear.
#[test]
fn two_discharges_of_one_loop_name_the_symmetry_between_them() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let s = PlaneForm::sheet_word();

    let a = cert(Axis::Unit(0), q.clone(), closed.clone());
    let b = cert(Axis::Orientation(0), s.clone(), closed);

    let sym = symmetry(&a, &b).expect("two different moves to one class name a symmetry");
    // The symmetry is exactly `w_a⁻¹ · w_b`, re-derived rather than trusted.
    let want = q.inverse().compose(&s).normal_form().rotor();
    assert!(
        sym.projective_eq(&want),
        "the symmetry is the element carrying one discharge to the other"
    );
    assert!(!sym.is_identity(), "and it is not the identity, or there is no freedom");
}

/// TWO DISCHARGES TO DIFFERENT CLASSES ARE NOT A SYMMETRY. `w_a⁻¹·w_b` then
/// fixes nothing, so there is nothing for it to be a symmetry OF, and the
/// precondition is checked rather than assumed (rule 11).
#[test]
fn discharges_to_two_classes_name_no_symmetry() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let other = ClassVector::of_word(&order4()).expect("quarter").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let s = PlaneForm::sheet_word();

    let a = cert(Axis::Unit(0), q, closed);
    let b = cert(Axis::Orientation(0), s, other);
    assert!(
        symmetry(&a, &b).is_none(),
        "a symmetry stabilises one class; two classes stabilise nothing"
    );
}

/// ONE MOVE SPELLED TWICE IS NOT TWO MOVES. An identity symmetry means the
/// loop admits nothing extra — a reading, not a failure — so it is refused
/// rather than adjoined as a generator that turns by nothing.
#[test]
fn one_move_spelled_twice_admits_no_freedom() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    // `q` and `q⁵` are one element and two tapes (1.5).
    let q5 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false); 5]);
    assert!(q.operator_eq(&q5), "one element, two spellings");

    let a = cert(Axis::Unit(0), q, closed.clone());
    let b = cert(Axis::Unit(0), q5, closed);
    assert!(
        symmetry(&a, &b).is_none(),
        "two spellings of one move give the identity, which is no degree of freedom"
    );
}

/// EVERY LOSER IS ITS OWN DEGREE OF FREEDOM. A vortex of three candidates
/// holds two symmetries against the winner, and they are two — summing them
/// would be a count deciding (rule 2).
#[test]
fn each_losing_candidate_is_its_own_symmetry() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let s = PlaneForm::sheet_word();
    let qq = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false); 2]);

    let win = cert(Axis::Unit(0), q, closed.clone());
    let held = vec![
        cert(Axis::Orientation(0), s, closed.clone()),
        cert(Axis::Unit(0), qq, closed),
    ];
    let syms = symmetries(&win, &held);
    assert_eq!(syms.len(), 2, "two losers, two degrees of freedom, got {}", syms.len());
    for s in &syms {
        assert!(!s.is_identity(), "each is a real symmetry");
    }
}
