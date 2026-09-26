// EXPANSION 2 — THE ALGEBRAIC REFINEMENT BID on the wire: the word, the
// held word it is redundant against, and the witness the court re-derives
// — the same operator, or the gauge that conjugates the held onto it.

use crate::anastome::dialogue::{decode_refine_algebraic, encode_refine_algebraic, AlgebraicWitness};
use crate::transducer::address::Address;
use crate::transducer::alphabet::{LetterId, Word};

/// Both witnesses cross the wire whole and come back as themselves.
#[test]
fn the_algebraic_bid_round_trips_both_witnesses() {
    let voice = [7u8; 32];
    let word = Address::from_bytes([4u8, 1, 2, 3]);
    let held = Address::from_bytes([4u8, 9, 8]);
    let gamma = Word::from_letters(&[(LetterId::turn_90((0, 2)).expect("90° turn"), true)]);
    for witness in [AlgebraicWitness::SameOperator, AlgebraicWitness::Section(Box::new(gamma.clone()))] {
        let body = encode_refine_algebraic(&voice, &word, &held, &witness).expect("the addresses frame");
        let (v, w, h, back) = decode_refine_algebraic(&body).expect("the bid decodes");
        assert_eq!(v, voice);
        assert_eq!(w, word);
        assert_eq!(h, held);
        match (&witness, &back) {
            (AlgebraicWitness::SameOperator, AlgebraicWitness::SameOperator) => {}
            (AlgebraicWitness::Section(a), AlgebraicWitness::Section(b)) => {
                assert_eq!(a.letters(), b.letters(), "the gauge's tape is the tape sent")
            }
            _ => panic!("the witness kind changed on the wire"),
        }
    }
}

/// A frame cut short of its witness is malformed, not a bid of no witness.
#[test]
fn a_bid_cut_short_of_its_witness_is_malformed() {
    let voice = [7u8; 32];
    let word = Address::from_bytes([4u8, 1]);
    let held = Address::from_bytes([4u8, 2]);
    let gamma = Word::from_letters(&[(LetterId::turn_90((0, 2)).expect("90° turn"), false)]);
    let body = encode_refine_algebraic(&voice, &word, &held, &AlgebraicWitness::Section(Box::new(gamma)))
        .expect("the addresses frame");
    let short = &body[..body.len() - 1];
    assert!(decode_refine_algebraic(short).is_none());
    let mut wrong_kind = body.clone();
    let kind_at = 33 + 2 + word.as_bytes().len() + 2 + held.as_bytes().len();
    wrong_kind[kind_at] = 9;
    assert!(decode_refine_algebraic(&wrong_kind).is_none(), "an unknown witness kind is no witness");
}

/// LD0 step 6(b) — A FORMED GAUGE CROSSES AS ITSELF. A section witness the
/// alphabet does not spell is an action with no letters; written as its
/// letters alone it crossed as the identity, a different witness.
#[test]
fn a_formed_witness_crosses_the_wire_as_its_action() {
    let voice = [7u8; 32];
    let word = Address::from_bytes([4u8, 1]);
    let held = Address::from_bytes([4u8, 2]);
    let spelled = Word::from_letters(&[(LetterId::turn_90((0, 2)).expect("90° turn"), false)]);
    let formed = Word::formed(spelled.rotor());
    assert!(formed.letters().is_empty() && !formed.is_identity(), "an action no tape spells");
    let body = encode_refine_algebraic(&voice, &word, &held, &AlgebraicWitness::Section(Box::new(formed.clone())))
        .expect("the addresses frame");
    let (_, _, _, back) = decode_refine_algebraic(&body).expect("the bid decodes");
    let AlgebraicWitness::Section(gamma) = back else { panic!("a section witness") };
    assert_eq!(*gamma, formed, "the action crossed, not the identity");
    assert!(!gamma.is_identity());
}
