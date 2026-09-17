// 1.5, 16.2 — the word: its element is its rotor, and a word that carries
// an action no tape spells composes, inverts and reduces as that action.

use crate::transducer::alphabet::*;
use num_bigint::BigInt;

/// `3 + e₀₁ + e₀₂`: a rotor of weight 11 that no coordinate plane holds and
/// no tape spells — a formed word is the whole of what it is (37.7).
fn eleven() -> Rotor {
    let q = |n: i64| Q::from_integer(BigInt::from(n));
    Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b0101, q(1)))
}

/// A formed word is not the empty tape: composed with a letter on either
/// side, inverted, and reduced, it is still the element it carries.
#[test]
fn a_formed_word_composes_inverts_and_reduces_as_its_rotor() {
    let r = eleven();
    let w = Word::formed(r.clone());
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter (0,1)"), false)]);
    assert!(w.rotor().projective_eq(&r), "the formed word carries its rotor");
    assert!(!w.is_identity(), "and is not the identity, though its tape is empty");
    assert!(
        w.compose(&q).rotor().projective_eq(&r.geo(&q.rotor())),
        "composed on the right, the product of the two actions"
    );
    assert!(
        q.compose(&w).rotor().projective_eq(&q.rotor().geo(&r)),
        "composed on the left, the product in that order"
    );
    assert!(
        w.inverse().compose(&w).is_identity(),
        "a formed word and its inverse compose to the identity"
    );
    assert!(w.normal_form().rotor().projective_eq(&r), "the normal form keeps the element");
    assert!(w.geodesic().rotor().projective_eq(&r), "and so does the geodesic");
    assert!(
        w.rebase(&q).rotor().projective_eq(&q.rotor().geo(&r).geo(&q.inverse().rotor())),
        "rebased by a letter, it is the conjugate"
    );
}

/// A letter word is unchanged by the same operations: its rotor is its
/// tape's product before and after.
#[test]
fn a_letter_word_composes_as_its_tape_still() {
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter (0,1)"), false)]);
    let p = Word::from_letters(&[(LetterId::pythagorean_5((2, 3)).expect("(3,4,5) on (2,3)"), false)]);
    let both = q.compose(&p);
    assert_eq!(both.letters().len(), 2, "the tape is the two letters");
    assert!(
        both.rotor().projective_eq(&q.rotor().geo(&p.rotor())),
        "and its rotor is the product the tape spells"
    );
    assert!(q.inverse().compose(&q).is_identity(), "a letter and its inverse close");
}
