// 50.2 step 2, 1.5, law 12 — THE DERIVATION: what a word vertex is, the
// bytes it travels as, and the re-derivation the court runs on it.

use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Verified, Word};

fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

fn addr(tag: u8) -> Address {
    Address::from_bytes([TAG_FORM, tag])
}

/// A22's case: no generator was reached, so the residue is the whole
/// word and there is nothing to compose. `is_spelled` names it, and the
/// tape carries no product beside the residue.
#[test]
fn a_spelled_derivation_is_its_residue_and_carries_no_second_word() {
    let d = Derivation::spelled(quarter(2));
    assert!(d.is_spelled(), "no citation");
    assert_eq!(d.word(), &quarter(2), "the word IS the residue");
    assert_eq!(d.verify_with(&|_: &Address, _: &Address| None), Verified::Composes);

    let mut b = Vec::new();
    d.to_bytes(&mut b).expect("the derivation writes");
    let mut only_residue = Vec::new();
    (0u16).to_le_bytes().iter().for_each(|x| only_residue.push(*x));
    quarter(2).to_bytes(&mut only_residue);
    assert_eq!(b, only_residue, "the product is not written twice (50.2 step 2)");

    let mut pos = 0;
    assert_eq!(Derivation::from_bytes(&b, &mut pos).as_ref(), Some(&d), "and it reads back");
    assert_eq!(pos, b.len(), "consuming exactly its bytes");
}

/// The half turn reached from the quarter turn: one citation, exponent
/// two, no residue left. The court re-derives the product from the
/// citation and agrees.
#[test]
fn a_factored_derivation_travels_its_citations_and_re_derives_the_product() {
    let d = Derivation::compose(&[(addr(1), addr(2), 2, quarter(1))], Word::identity());
    assert!(!d.is_spelled(), "it cites a generator");
    assert_eq!(d.generators, vec![(addr(1), addr(2), 2)], "the form, the vertex, the exponent");
    assert!(
        d.word().rotor().projective_eq(&quarter(2).rotor()),
        "the quarter turn twice is the half turn"
    );

    let mut b = Vec::new();
    d.to_bytes(&mut b).expect("the derivation writes");
    let mut pos = 0;
    let back = Derivation::from_bytes(&b, &mut pos).expect("it reads back");
    assert_eq!(pos, b.len(), "consuming exactly its bytes");
    assert_eq!(back.generators, d.generators, "the citations survive the wire");

    let holds = |f: &Address, v: &Address| -> Option<Word> {
        match (f == &addr(1), v == &addr(2)) {
            (true, true) => Some(quarter(1)),
            _ => None,
        }
    };
    assert_eq!(back.verify_with(&holds), Verified::Composes, "a court that holds it agrees");
}

/// A negative exponent is the generator's inverse applied that many
/// times: the three-quarter turn is the quarter turn once backwards.
#[test]
fn a_negative_exponent_applies_the_generators_inverse() {
    let d = Derivation::compose(&[(addr(1), addr(2), -1, quarter(1))], Word::identity());
    assert!(
        d.word().rotor().projective_eq(&quarter(3).rotor()),
        "one quarter turn backwards is three forwards"
    );
}

/// 50.2 step 2's staleness answer. The product is on the tape, so a
/// speaker could carry citations that do not reach it; the court's
/// re-derivation is what refuses that, and it is a refusal at the
/// vertex, never a smaller answer.
#[test]
fn citations_that_do_not_compose_to_the_product_are_refused() {
    let honest = Derivation::compose(&[(addr(1), addr(2), 2, quarter(1))], Word::identity());
    let lying = Derivation::factored(honest.generators.clone(), Word::identity(), quarter(3));
    let holds = |_: &Address, _: &Address| Some(quarter(1));
    assert_eq!(honest.verify_with(&holds), Verified::Composes);
    assert_eq!(lying.verify_with(&holds), Verified::Differs, "the court re-derives and disagrees");
}

/// A cited generator the reader does not hold is the deferral of 21
/// step 3 — the sentence is docketed and the syllabus serves the
/// generator — and it is reported apart from a disagreement, because a
/// court that refused it here would refuse every voice ahead of its own
/// cover.
#[test]
fn a_citation_the_reader_does_not_hold_is_named_apart_from_a_disagreement() {
    let d = Derivation::compose(&[(addr(1), addr(2), 2, quarter(1))], Word::identity());
    assert_eq!(
        d.verify_with(&|_: &Address, _: &Address| None),
        Verified::Unresolved { form: addr(1), vertex: addr(2) },
        "not held is not wrong"
    );
}

/// 1.5: two derivations reaching one vertex are one word. The citation
/// list is content the court replays and is deliberately not identity.
#[test]
fn two_derivations_that_reach_one_vertex_carry_one_word() {
    let a = Derivation::compose(&[(addr(1), addr(2), 2, quarter(1))], Word::identity());
    let b = Derivation::compose(&[(addr(3), addr(4), -2, quarter(3))], Word::identity());
    assert_ne!(a.generators, b.generators, "two presentations, two proofs");
    assert!(
        a.word().rotor().projective_eq(&b.word().rotor()),
        "one vertex: the address cannot tell them apart, and must not"
    );
}

/// Rule 10 on a decoder: a citation count no remaining byte can satisfy
/// refuses the read and reserves nothing.
#[test]
fn a_citation_count_the_tape_cannot_satisfy_is_refused() {
    let mut b = (u16::MAX).to_le_bytes().to_vec();
    b.extend_from_slice(&[0u8; 4]);
    let mut pos = 0;
    assert_eq!(Derivation::from_bytes(&b, &mut pos), None, "a malformed count reads nothing");
}
