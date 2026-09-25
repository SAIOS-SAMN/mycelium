// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 4 — THE SYMMETRY A VORTEX HOLDS: two discharges of one orbit
//! prove the orbit admits their difference, and that difference is a measured
//! rotor rather than a losing candidate.

use super::common::*;
use crate::transducer::alphabet::{LetterId, PlaneForm, Word};
use crate::transducer::sentence::*;

fn cert(axis: Axis, word: Word, after: crate::transducer::address::Address) -> PivotCertificate {
    PivotCertificate {
        chord: some_overlap(),
        axis,
        before: ClassVector::of_word(&order4()).expect("class").class_key(),
        after,
        rank_move: RankMove::Equal,
        mu: Mu::Unmeasured,
        by: PivotBy::Word { face: some_overlap(), word: Box::new(word), side: Side::Right },
    }
}

/// TWO DISCHARGES OF ONE ORBIT PROVE A SYMMETRY. `w_a⁻¹ · w_b` carries one
/// discharge to the other, and it STABILISES the orbit's class because each
/// leaves that class where the other does. It is a rotation the mind
/// measured — 37.7's object — and it is what the collapse used to clear.
#[test]
fn two_discharges_of_one_orbit_name_the_symmetry_between_them() {
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
/// orbit admits nothing extra — a reading, not a failure — so it is refused
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

/// ONE MOVE SPELLED TWICE IS ONE DISCHARGE, NOT A VORTEX. `candidates_for`
/// offers a held word on both sides of the chord; where both sides
/// discharge, the two certificates name one move — the vortex would hold
/// them as one candidate (`vortex_put` keys by the move) and `symmetry`
/// reads their difference as the identity — so the orbit discharges on it
/// rather than being held under a superposition of one hypothesis, which
/// nothing could collapse. Two different words on the same axis are still
/// two.
#[test]
fn a_word_discharging_on_both_sides_is_one_discharge_and_two_words_are_a_vortex() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let s = PlaneForm::sheet_word();
    let sided = |word: &Word, side: Side| PivotCertificate {
        by: PivotBy::Word { face: some_overlap(), word: Box::new(word.clone()), side },
        ..cert(Axis::Unit(0), word.clone(), closed.clone())
    };

    let both_sides = [sided(&q, Side::Right), sided(&q, Side::Left)];
    let moves = one_per_move(&both_sides);
    let [only] = moves.as_slice() else {
        panic!("two sides of one word are one move, got {}", moves.len())
    };
    assert_eq!(
        only.by.move_key(),
        q.rotor().key().into_vec(),
        "and the move is that word's"
    );
    assert!(
        matches!(select_discharge(&moves), Discharge::One(_)),
        "so the orbit discharges on it"
    );

    let two_words = [sided(&q, Side::Right), sided(&s, Side::Right)];
    let moves = one_per_move(&two_words);
    assert_eq!(moves.len(), 2, "two words on one axis are two moves");
    assert!(
        matches!(select_discharge(&moves), Discharge::Vortex(ref held) if held.len() == 2),
        "and a vortex of two"
    );
}

/// A SEARCH THE BUDGET CUT SHORT IS NOT A READING OF THE ALGEBRA.
///
/// `NoneDischarged` was returned whether every candidate had been tried
/// and refused or none had been tried at all, and the arm beneath it
/// speaks a demand, files torsion, or widens the cover. A demand raised on
/// an evaluation that never happened asks the mesh for a word this mind
/// may already hold and simply did not reach.
///
/// A discharge that HAPPENED is real however the stroke ended, so only the
/// empty set is ambiguous and only it consults the budget.
#[test]
fn a_starved_search_is_not_a_refusal_and_a_discharge_survives_one() {
    let closed = ClassVector::of_word(&Word::identity()).expect("identity").class_key();
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let one = cert(Axis::Unit(0), q, closed);

    assert_eq!(
        outcome_of(&[], false),
        Discharge::NoneDischarged,
        "every candidate tried and refused is the algebra's answer"
    );
    assert_eq!(
        outcome_of(&[], true),
        Discharge::Starved,
        "and none tried is not an answer at all"
    );
    assert!(
        matches!(outcome_of(std::slice::from_ref(&one), true), Discharge::One(_)),
        "a discharge that happened is real however the stroke ended"
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
