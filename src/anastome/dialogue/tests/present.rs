// 78.160 P1 — the presentation ask and its answer, read back as the exact
// values they were written from (Concordance 1.4), and never read as a fetch.

use crate::anastome::dialogue::*;
use crate::transducer::address::Address;

/// A presentation ask and its answer read back whole, and the ask is never
/// read as a fetch.
#[test]
fn presentation_round_trips_never_as_fetch() {
    let voice = [4u8; 32];
    let motion = crate::transducer::cga::translator_at(&[num_rational::BigRational::from_integer(3.into()), num_rational::BigRational::from_integer(0.into())]);
    let ask = encode_presentation_request(&voice, &motion);
    let (v, m) = decode_presentation_request(&ask).expect("the ask reads back");
    assert_eq!(v, voice);
    assert!(m.projective_eq(&motion));
    assert!(decode_fetch(&ask).is_none(), "a fetch decoder refuses the ask's marker");
    let mut longer = ask.clone();
    longer.push(0);
    assert!(decode_presentation_request(&longer).is_none(), "a byte left over is no ask");
    let fetch = encode_fetch::<Address>(&voice, &[], 4096);
    assert!(decode_presentation_request(&fetch).is_none(), "a fetch is no ask");

    let words = vec![Taught { at: crate::chronometry::Stamp::default(), word: vec![1, 2, 3] }];
    let answer = PresentationAnswer {
        read: true,
        cites: vec![(Address::from(vec![5u8; 6]), Address::from(vec![6u8; 7]), -3)],
        composes: true,
        residue: 0,
        holes: 1,
    };
    let body = encode_presentation_reply(&words, &answer).expect("the answer frames");
    let (w, a) = decode_presentation_reply(&body).expect("the answer reads back");
    assert_eq!(w, words);
    assert_eq!(a, answer);
    let mut cut = body.clone();
    cut.pop();
    assert!(decode_presentation_reply(&cut).is_none(), "a cut answer is no answer");
}
