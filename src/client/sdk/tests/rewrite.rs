// MU-CLIENT §5.4, §1a step 8 (S3) — THE MIND PROPOSES A REWRITE: a ruled word
// the rest of the language composes without it is reached by two
// derivations, and the face between them is a `Retire` the mind asserts.

use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::rewrite::{Rewrite, RewriteKind, RewritePayload};

/// The 90° turn on the plane of axes 0 and 1, `n` times, as a transport: a
/// connection carrying the word and nothing else.
fn turned(n: usize) -> Element {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the plane (0,1)");
    crate::transducer::element::from_demand(&Word::from_letters(&vec![(q, false); n])).expect("a connection carrying the word")
}

/// A mind that learned each word as the court taught it.
fn taught(name: &str, words: &[&Element]) -> Mind {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named(name));
    for w in words {
        assert!(mind.absorb_taught(&w.to_bytes().expect("the record writes")).is_some(), "the court's word is learned");
    }
    mind
}

/// 27, 3.8 — THE 90° TURN COMPOSES THE 180° TURN, NOT THE REVERSE. A mind
/// holding both as ruled words on the plane (0,1) reads the 180° turn over
/// its language without it: the 90° turn squared, exact. That is a second
/// derivation of the 180° turn, and its retirement names it, cites the 90°
/// turn, frees the lanes it carried, and reconstructs blind. The 90° turn has
/// no such derivation: the 180° turn reaches it at no integer exponent, so
/// it admits no retirement.
#[test]
fn retirement_of_names_a_ruled_word_the_rest_composes() {
    let (q90, q180) = (turned(1), turned(2));
    let mind = taught("retirer", &[&q90, &q180]);
    let rw = mind.retirement_of(&q180.address()).expect("the 180° turn is the 90° turn squared");
    assert!(crate::transducer::rewrite::reconstruct(&rw).is_none(), "the proposal reconstructs blind");
    let r = Rewrite::from_element(&rw).expect("the tape reads");
    assert_eq!(r.kind, RewriteKind::Retire, "a retirement");
    assert!(matches!(r.before.as_slice(), [b] if *b.address() == q180.address()), "naming the 180° turn");
    let RewritePayload::Retire { cites, freed } = r.payload else { panic!("a retirement's payload") };
    assert_eq!(cites, vec![q90.address()], "citing the 90° turn that composes it");
    assert_eq!(freed, q180.magnitude(), "freeing the lanes the 180° turn carried");
    assert!(mind.retirement_of(&q90.address()).is_none(), "the 180° turn does not compose the 90° turn");
}

/// A WORD THE MIND DOES NOT HOLD AS RULED ADMITS NO RETIREMENT: one alone in
/// the language is composed by nothing else, and one never learned has no
/// generator to read.
#[test]
fn retirement_of_refuses_a_word_nothing_else_composes() {
    let (q90, q180) = (turned(1), turned(2));
    let mind = taught("alone", &[&q180]);
    assert!(mind.retirement_of(&q180.address()).is_none(), "nothing else composes the 180° turn");
    assert!(mind.retirement_of(&q90.address()).is_none(), "a word not held has nothing to read");
}

/// S3 — THE PROPOSAL IS ASSERTED, NEVER APPLIED. When the 90° turn is the
/// word newly held, the 180° turn stands on the hole it stands on, so the
/// 180° turn's retirement is read and asserted once; the mind's language
/// still holds the 180° turn, because only a court retires it. Asserted
/// again, it is not said twice.
#[test]
fn propose_rewrites_asserts_retirement_and_keeps_the_word() {
    let (q90, q180) = (turned(1), turned(2));
    let mut mind = taught("proposer", &[&q180, &q90]);
    let said = mind.propose_rewrites(&q90);
    let [rw] = said.as_slice() else { panic!("one retirement proposed: {said:?}") };
    let r = Rewrite::from_element(rw).expect("the tape reads");
    assert!(matches!(r.before.as_slice(), [b] if *b.address() == q180.address()), "the 180° turn's retirement");
    assert!(mind.language.holds(q180.address().as_bytes()), "the mind still holds the 180° turn");
    assert!(mind.propose_rewrites(&q90).is_empty(), "said once");
}

/// RULING 5 (MU-CLIENT §5.4, #146) — THE MIND PROPOSES REFINE FROM OPEN AXIS.
/// Refines a chart within a dimension that already stands and reconstructs blind.
#[test]
fn refine_of_proposes_refine_and_reconstructs() {
    use crate::transducer::address::Address;
    use crate::transducer::stalk::{ChartAxis, Invariant};
    let mind = Mind::open("127.0.0.1:1", Voice::named("refiner"));
    let chart = Invariant(vec![ChartAxis::Count(1)]);
    let before = Address::from_bytes([1u8]);
    let after = Address::from_bytes([2u8]);
    let rw = mind.refine_of(&chart, 2, &before, &after).expect("a refine proposal");
    assert!(crate::transducer::rewrite::reconstruct(&rw).is_none(), "reconstructs green");
    let r = Rewrite::from_element(&rw).expect("the tape reads");
    assert_eq!(r.kind, RewriteKind::Refine);
    assert_eq!(r.claimed[crate::transducer::rewrite::LANE_SHELL], 1);
}

/// RULING 5 (MU-CLIENT §5.4, #146) — THE MIND PROPOSES AXIS-OPEN FROM PERSISTENT RESIDUE.
/// Extends a new dimension of perception with the orbit's holonomy rotor.
#[test]
fn axis_open_of_proposes_new_dimension_and_reconstructs() {
    use crate::transducer::address::Address;
    use crate::transducer::alphabet::LetterId;
    let mind = Mind::open("127.0.0.1:1", Voice::named("axis_opener"));
    let q = LetterId::turn_90((0, 1)).expect("sensor plane");
    let rotor = Word::from_letters(&[(q, false)]).rotor();
    let before = Address::from_bytes([3u8]);
    let after = Address::from_bytes([4u8]);
    let rw = mind.axis_open_of(&rotor, 4, &before, &after).expect("an axis open proposal");
    assert!(crate::transducer::rewrite::reconstruct(&rw).is_none(), "reconstructs green");
    let r = Rewrite::from_element(&rw).expect("the tape reads");
    assert_eq!(r.kind, RewriteKind::AxisOpen);
    assert_eq!(r.claimed[crate::transducer::rewrite::LANE_SHELL], 1);
}

/// RULING 5 (MU-CLIENT §5.4, #146) — THE MIND PROPOSES TRANSLATION FROM PLACEMENT.
/// Connects two distinct charts across a chord overlap.
#[test]
fn translation_of_proposes_translation_and_reconstructs() {
    use crate::transducer::address::Address;
    let mind = Mind::open("127.0.0.1:1", Voice::named("translator"));
    let a = Address::from_bytes([5u8]);
    let b = Address::from_bytes([6u8]);
    let before = Address::from_bytes([7u8]);
    let after = Address::from_bytes([8u8]);
    let gauge = crate::transducer::element::identity_versor();
    let rw = mind.translation_of(a.clone(), b.clone(), gauge, &before, &after).expect("a translation proposal");
    assert!(crate::transducer::rewrite::reconstruct(&rw).is_none(), "reconstructs green");
    let r = Rewrite::from_element(&rw).expect("the tape reads");
    assert_eq!(r.kind, RewriteKind::Translation);
    assert_eq!(r.claimed[crate::transducer::rewrite::LANE_GAUGE], 2);
}

/// RULING 5 (MU-CLIENT §5.4, #146) — THE MIND PROPOSES REANCHOR FROM RE-GAUGING.
/// Moves anchor coordinates on S while leaving class invariant.
#[test]
fn reanchor_of_proposes_reanchor_and_reconstructs() {
    use crate::transducer::address::Address;
    use crate::transducer::alphabet::LetterId;
    let mind = Mind::open("127.0.0.1:1", Voice::named("reanchorer"));
    let q = LetterId::turn_90((0, 1)).expect("sensor plane");
    let w = Word::from_letters(&[(q, false)]);
    let before = Address::from_bytes([9u8]);
    let after = Address::from_bytes([10u8]);
    let rw = mind.reanchor_of(vec![w], &before, &after).expect("a reanchor proposal");
    assert!(crate::transducer::rewrite::reconstruct(&rw).is_none(), "reconstructs green");
    let r = Rewrite::from_element(&rw).expect("the tape reads");
    assert_eq!(r.kind, RewriteKind::Reanchor);
    assert_eq!(r.claimed[crate::transducer::rewrite::LANE_SPATIAL], 1);
}
