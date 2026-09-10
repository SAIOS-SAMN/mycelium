// 26 — the five acts round-trip: every payload that goes out parses back
// to the same coordinates.

use crate::anastome::dialogue::*;

/// Every act round-trips exactly — the calculus is closed and lossless.
#[test]
fn the_five_acts_round_trip() {
    let chain = vec![(0i64, 0i64, 1u8, 4u8), (2, 3, 0, 7)];
    let voice = [9u8; 32];
    let solver = [7u8; 32];
    let listing = crate::transducer::element::listing_key(
        &chain,
        &crate::transducer::element::IDENTITY_FRAME,
    );
    let cites = vec![CiteWire {
        addr: listing.clone(),
        standing: 0,
        transport: crate::transducer::element::translator_versor(1, -1),
        gauge: 1,
        body: chain.clone(),
    }];
    assert_eq!(decode_quote_request(&encode_quote_request(&voice)).unwrap(), (voice, Vec::new()));
    let items = vec![WorkItem {
        addr: listing.clone(),
        mu: [1, 2, 30, 4],
        chain: chain.clone(),
        open: Vec::new(),
    }];
    let (h, s, k, it) = decode_quote_reply(&encode_quote_reply(3, &[5, 0, 9, 0], 4208, &items)).unwrap();
    assert_eq!(h, 3, "book height rides the quote");
    assert_eq!(s, [5, 0, 9, 0]);
    assert_eq!(k, 4208, "receipt-chain head K rides the quote, not TUniversal");
    assert_eq!(it, items);
    // A settlement rides the docket after the dockets, with its utterance.
    let docket = Docket {
        addr: listing.clone(),
        elem: vec![1, 2, 3],
        voices: vec![voice],
        escrowed: [1, 2, 3, 4],
        deflation: [0, 0, 7, 0],
        k: 41,
    };
    let settlement = Settlement {
        listing: listing.clone(),
        chain: chain.clone(),
        voice,
        cites: cites.clone(),
        before: [2, 0, 2, 0],
        after: [0, 0, 1, 0],
        k: 42,
        t_universal: 7,
    };
    let body = encode_docket_reply_with(std::slice::from_ref(&docket), std::slice::from_ref(&settlement));
    assert_eq!(decode_docket_reply(&body).unwrap(), vec![docket.clone()]);
    assert_eq!(decode_settlements(&body), vec![settlement.clone()]);
    let cavity = vec![0xCAu8, 0xFE, 0xBA, 0xBE];
    let (paged, w, s, n) = encode_docket_reply_paged(
        std::slice::from_ref(&docket),
        std::slice::from_ref(&settlement),
        std::slice::from_ref(&cavity),
        usize::MAX,
    );
    assert_eq!((w, s, n), (1, 1, 1));
    assert_eq!(decode_cavities(&paged), vec![cavity]);
    assert_eq!(decode_teach(&encode_teach(&[0xAB, 0xCD])).unwrap(), vec![0xAB, 0xCD]);
    let (g2, vs, mu) = decode_settle(&encode_settle(&listing, &[voice, solver], &[0, 0, 12, 0])).unwrap();
    assert_eq!((g2, vs, mu), (listing, vec![voice, solver], [0, 0, 12, 0]));
    let (held, at) = decode_root_reply(&encode_root_reply(Some(solver), 9)).unwrap();
    assert_eq!((held, at), (Some(solver), 9));
    let (none, at0) = decode_root_reply(&encode_root_reply(None, 3)).unwrap();
    assert_eq!((none, at0), (None, 3));
    assert!(
        decode_root_reply(&[1u8]).is_none(),
        "a truncated solver is refused, never a zero voice"
    );
}
