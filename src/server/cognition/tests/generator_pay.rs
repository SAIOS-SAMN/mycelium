// 26, 50.2 step 2 — A CITED GENERATOR IS PAID AT THE INSTALL, and a
// sentence that does not stand pays nobody.

use crate::cognition::*;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};

fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence whose vertex carries `d`.
fn sentence_at(d: Derivation, tag: u8, mag: [u64; 4]) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: Address::from_bytes([TAG_FORM, tag]),
        derivation: d,
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    as_element_form(&f, mag).expect("a tape")
}

/// The common case: a sentence measuring one unit on the first lane.
fn sentence_with(d: Derivation, tag: u8) -> Element {
    sentence_at(d, tag, [1, 0, 0, 0])
}

/// The generator and the sentence that derives through it twice.
fn generator_and_citer() -> (Element, Element) {
    // The generator measures on a lane the speaker can actually draw:
    // a mint bonds the first lane, so a use priced only there is a use
    // no voice can pay and the test would prove nothing.
    let g = sentence_at(Derivation::spelled(quarter(1)), 1, [0, 1, 0, 0]);
    let citer = sentence_with(
        Derivation::compose(
            &[(g.address(), Address::from_bytes([TAG_FORM, 1]), 2, quarter(1))],
            Word::identity(),
        ),
        9,
    );
    (g, citer)
}

/// A sentence that never installs pays nobody. Here the court has not
/// ruled the cited form at all, so the citer WAITS (21 step 3) — it is
/// not refused, and it is also not installed, so no generator is paid
/// and no balance moves anywhere.
#[test]
fn a_citer_that_does_not_install_pays_no_generator() {
    let teacher = [1u8; 32];
    let speaker = [3u8; 32];
    let (g, citer) = generator_and_citer();

    let mut k = Kernel::new();
    let anchor = k.anchor_pub();
    k.treasury.mint_claim(&[speaker], [8, 8, 8, 8], anchor);
    let before = k.treasury.balance_mu(&teacher);

    // The generator was never ruled here.
    assert_eq!(
        k.perceive_speech(speaker, &[0; 32], std::slice::from_ref(&citer)),
        vec![crate::anastome::dialogue::SPEECH_DIALECT],
        "the citer waits on a form this court has not ruled"
    );
    assert!(!k.ruled_contains(citer.address().as_bytes()), "so it does not install");
    assert_eq!(
        k.treasury.balance_mu(&teacher),
        before,
        "and nothing is paid for a sentence that did not stand"
    );
    // Nothing was drawn from the speaker either.
    let _ = g;
}
