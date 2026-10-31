// 21 step 3, 47.3 (2026-10-14) — the citation asked by address, read back as
// the exact values it was written from (Concordance 1.4), and never read as a
// fetch or a presentation ask, nor they as it.

use crate::anastome::dialogue::*;
use crate::transducer::address::Address;

/// A citation ask reads back whole, and is never read as a fetch or a
/// presentation ask, nor they as it.
#[test]
fn cited_request_round_trips_never_as_fetch() {
    let voice = [7u8; 32];
    let addrs = vec![Address::from(vec![1u8; 5]), Address::from(vec![2u8; 40])];
    let ask = encode_cited_request(&voice, &addrs).expect("the ask frames");
    let (v, a) = decode_cited_request(&ask).expect("the ask reads back");
    assert_eq!(v, voice);
    assert_eq!(a, addrs);
    assert!(decode_fetch(&ask).is_none(), "a fetch decoder refuses the ask's marker");
    assert!(decode_presentation_request(&ask).is_none(), "nor is it a presentation ask");
    let fetch = encode_fetch::<Address>(&voice, &[], 4096);
    assert!(decode_cited_request(&fetch).is_none(), "a fetch is no citation ask");
    let motion = crate::transducer::cga::translator_at(&[num_rational::BigRational::from_integer(1.into())]);
    assert!(decode_cited_request(&encode_presentation_request(&voice, &motion)).is_none(), "nor a presentation ask");
    let mut longer = ask.clone();
    longer.push(0);
    assert!(decode_cited_request(&longer).is_none(), "a byte left over is no ask");
    let mut cut = ask.clone();
    cut.pop();
    assert!(decode_cited_request(&cut).is_none(), "a cut ask is no ask");
    let wide = Address::from(vec![3u8; usize::from(u16::MAX) + 1]);
    assert!(encode_cited_request(&voice, &[wide]).is_none(), "an address past its two-byte length is refused, never widened");
}
