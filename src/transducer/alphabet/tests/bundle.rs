// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 2 — `π : Word → B × F`: the base a word turns by and the fibre
//! it turns at, and the sections of one base that a vocabulary should hold
//! once.

use crate::transducer::alphabet::bundle::{
    by_base, project, section_of, solve_conjugator, trivial_fibre, Base,
};
use crate::transducer::alphabet::{LetterId, PlaneForm, Word};
use crate::transducer::element::identity_versor;

/// THE BASE IS THE CONJUGACY CLASS, and conjugate words share it. That is
/// not a convention chosen here — conjugation is how a change of basepoint
/// acts, and the class invariants are exactly what it preserves (15.3), so
/// the cut is forced by the algebra.
#[test]
fn conjugate_words_share_a_base_and_unrelated_ones_do_not() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    // A sheet conjugates a rotation to its inverse — a stated relator — so
    // `q` and `s·q·s⁻¹` are two basepoints on ONE fibre.
    let conj = q.rebase(&sheet);
    assert!(
        !crate::transducer::alphabet::rewrite::tape_key(&q)
            .eq(&crate::transducer::alphabet::rewrite::tape_key(&conj)),
        "the two spellings differ, or there is no change of basepoint to see"
    );
    assert_eq!(
        Base::of_word(&q),
        Base::of_word(&conj),
        "conjugate words stand in one base"
    );

    // A half turn is a different class and therefore a different bundle.
    let half = Word::from_letters(&[
        (LetterId::quarter((0, 1)).expect("quarter"), false),
        (LetterId::quarter((0, 1)).expect("quarter"), false),
    ]);
    assert_ne!(
        Base::of_word(&q),
        Base::of_word(&half),
        "a quarter and a half turn are two bases, not one bundle"
    );
}

/// THE CONJUGATOR IS SOLVED AND VERIFIED, not asserted from the classes
/// matching. Conjugacy says SOME element relates them; this answers whether
/// the ALPHABET spells one, and proves it by the exact conjugation.
#[test]
fn the_conjugator_is_solved_and_re_derived() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    let conj = q.rebase(&sheet);

    // The solve is separated from the spelling so a refusal says which.
    let raw = crate::transducer::alphabet::bundle::solve_conjugator_rotor(&q.rotor(), &conj.rotor());
    assert!(
        raw.is_some(),
        "γ·a = b·γ has an invertible solution: a={:?} b={:?}",
        q.rotor(),
        conj.rotor()
    );
    let g = solve_conjugator(&q, &conj).expect("the alphabet spells this change of basepoint");
    // The claim is re-derived rather than trusted: γ·q·γ⁻¹ is the target.
    assert!(
        q.rotor().conjugate(&g.rotor()).projective_eq(&conj.rotor()),
        "γ carries the base representative onto the word"
    );

    // A word conjugate to itself takes the identity, and that is a real
    // answer rather than an absence.
    assert!(solve_conjugator(&q, &q).is_some_and(|g| g.is_identity()));
}

/// WORDS OF DIFFERENT CLASSES HAVE NO CONJUGATOR, and the refusal is about
/// the classes rather than about the alphabet — which is why the class test
/// comes first.
#[test]
fn words_of_two_classes_have_no_conjugator() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let half = Word::from_letters(&[
        (LetterId::quarter((0, 1)).expect("quarter"), false),
        (LetterId::quarter((0, 1)).expect("quarter"), false),
    ]);
    assert!(
        solve_conjugator(&q, &half).is_none(),
        "no change of basepoint turns a quarter into a half"
    );
}

/// THE PROJECTION KEEPS BOTH HALVES. A section is the base AND the
/// coordinate; dropping either would make the bundle unreconstructible, and
/// the fibre's lanes are separate carriers (law 5).
#[test]
fn the_projection_carries_the_base_and_the_fibre() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let hue = vec![(1u8, 2u8)];
    let s = project(&q, &identity_versor(), &hue, Some(&q)).expect("a versor projects");
    assert_eq!(s.base, Base::of_word(&q).expect("base"));
    assert_eq!(s.fibre.hue, hue, "the recolour is a fibre coordinate, not a base one");
    assert!(
        s.fibre.gauge.is_some_and(|g| g.is_identity()),
        "projected against itself the conjugator is the identity"
    );

    // WITHOUT A REPRESENTATIVE there is no conjugator to solve, and `None`
    // says "none was found" rather than "it is the identity" (rule 11).
    let t = project(&q, &identity_versor(), &hue, None).expect("a versor projects");
    assert!(t.fibre.gauge.is_none(), "no representative, no γ — not a silent identity");
}

/// THE VOCABULARY HOLDS THE BASE ONCE. Two words that are one base and two
/// sections group together; a third of another base does not join them.
/// This is the count that stops growing with the contexts.
#[test]
fn sections_of_one_base_group_and_another_base_does_not_join_them() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    let conj = q.rebase(&sheet);
    let half = Word::from_letters(&[
        (LetterId::quarter((0, 1)).expect("quarter"), false),
        (LetterId::quarter((0, 1)).expect("quarter"), false),
    ]);

    let mk = |w: &Word| {
        (
            w.clone(),
            project(w, &identity_versor(), &[], Some(&q)).expect("projects"),
        )
    };
    let grouped = by_base(&[mk(&q), mk(&conj), mk(&half)]);
    let qb = Base::of_word(&q).expect("base");
    let hb = Base::of_word(&half).expect("base");
    assert_eq!(
        grouped.get(qb.as_bytes()).map(|v| v.len()),
        Some(2),
        "the quarter and its conjugate are two sections of one base"
    );
    assert_eq!(
        grouped.get(hb.as_bytes()).map(|v| v.len()),
        Some(1),
        "and the half turn is its own bundle"
    );
}

/// A WORD THE LANGUAGE ALREADY HOLDS AS A SECTION is redundant — and the
/// answer carries the γ that shows it, because "redundant" without the
/// transition is an assertion and with it is a re-derivation anyone replays.
#[test]
fn a_conjugate_of_a_held_word_is_a_section_and_names_its_transition() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    let conj = q.rebase(&sheet);
    let half = Word::from_letters(&[
        (LetterId::quarter((0, 1)).expect("quarter"), false),
        (LetterId::quarter((0, 1)).expect("quarter"), false),
    ]);

    let (held, g) =
        section_of(std::slice::from_ref(&q), &conj).expect("the conjugate is a section of q");
    assert!(crate::transducer::alphabet::rewrite::tape_key(&held) == crate::transducer::alphabet::rewrite::tape_key(&q));
    assert!(
        q.rotor().conjugate(&g.rotor()).projective_eq(&conj.rotor()),
        "and the transition it names carries one onto the other"
    );

    // A word of another base is nobody's section.
    assert!(
        section_of(std::slice::from_ref(&q), &half).is_none(),
        "a half turn is not a section of a quarter's bundle"
    );
}

/// 37.7 — A GAUGE THAT EXISTS AND CANNOT BE SPELLED IS NOT A SILENCE.
///
/// `solve_conjugator` answers `None` for two different facts: no γ relates
/// the two words, and a γ relates them that the seventeen cannot say. The
/// first is a refusal about the words; the second is the measured
/// obstruction a generator is made of. `conjugation` separates them, and
/// separating them is what lets an unspellable transition reach the mint
/// instead of being dropped.
#[test]
fn a_conjugation_says_whether_the_alphabet_can_spell_it() {
    use crate::transducer::alphabet::bundle::{conjugation, Conjugation};
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    let conj = q.rebase(&sheet);

    // A relation the alphabet states outright is spelled.
    assert!(
        matches!(conjugation(&q, &conj), Some(Conjugation::Spelled(_))),
        "the sheet conjugation is one the seventeen can say"
    );
    // A word conjugate to itself is spelled by the identity.
    assert!(matches!(
        conjugation(&q, &q),
        Some(Conjugation::Spelled(w)) if w.is_identity()
    ));
    // Two classes are related by nothing, and that is the OTHER `None` —
    // a refusal about the words and not a candidate generator.
    let half = Word::from_letters(&[
        (LetterId::quarter((0, 1)).expect("quarter"), false),
        (LetterId::quarter((0, 1)).expect("quarter"), false),
    ]);
    assert!(
        conjugation(&q, &half).is_none(),
        "no γ relates a quarter to a half, spellable or otherwise"
    );
}

/// THE COSET IS REACHED FROM ANY MEMBER. `K = g·C(a)` and `a` commutes with
/// itself, so every `g·aᵏ` conjugates exactly as `g` does. They are offered
/// because the alphabet spells some members of a coset and not others, and
/// a solve returning one arbitrary element answers "unspellable" for a
/// relation the seventeen can say from a different basepoint.
#[test]
fn every_offered_conjugator_actually_conjugates() {
    use crate::transducer::alphabet::bundle::conjugators;
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter"), false)]);
    let sheet = PlaneForm::sheet_word();
    let conj = q.rebase(&sheet);
    let (a, b) = (q.rotor().normalised(), conj.rotor().normalised());
    let found = conjugators(&a, &b);
    assert!(!found.is_empty(), "the linear solve finds the coset");
    for g in &found {
        assert!(g.weight().is_some(), "every offered element is a versor");
        assert!(
            g.conjugate_holds(&a, &b),
            "and every one is verified by the exact sandwich, not presumed"
        );
    }
}

/// The trivial fibre is the coordinate a base stands at over itself: the
/// identity conjugator, an identity carriage, no recolour.
#[test]
fn the_trivial_fibre_is_the_base_over_itself() {
    let f = trivial_fibre();
    assert!(f.gauge.is_some_and(|g| g.is_identity()));
    assert!(f.hue.is_empty());
    assert!(crate::transducer::element::same_versor(
        &f.carriage,
        &identity_versor()
    ));
}
