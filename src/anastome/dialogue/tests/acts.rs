// 26 — the five acts round-trip: every payload that goes out parses back
// to the same coordinates.

use crate::anastome::dialogue::*;

/// 7.1, 47.1 — A COUNT THAT DOES NOT FIT ITS FIELD IS A REFUSAL, not a
/// shorter frame. `len().min(255) as u8` wrote 255 and then 255 members,
/// so the reader took a frame that ended inside the next field and refused
/// the whole act while the sender believed it spoke.
#[test]
fn a_count_past_its_field_refuses_the_frame() {
    let voice = [3u8; 32];
    let charts: Vec<crate::transducer::stalk::Invariant> =
        (0..256u64).map(|i| crate::transducer::stalk::Invariant::counts(&[i])).collect();
    assert!(
        encode_quote_request_charts(&voice, &charts[..255]).is_some(),
        "255 charts fit the count the ask writes"
    );
    assert!(
        encode_quote_request_charts(&voice, &charts).is_none(),
        "256 do not, and the ask is refused rather than sent naming 255"
    );

    // The same at a settlement's parties and a syllabus page.
    let voices: Vec<[u8; 32]> = (0..=255u16).map(|i| [u8::try_from(i).expect("the range is 0..=255, one voice per byte value"); 32]).collect();
    let listing = crate::transducer::element::listing_key(
        &[(0i64, 0i64, 1u8, 4u8)],
        &crate::transducer::element::IDENTITY_FRAME,
    )
    .expect("a one-link chain is A5's");
    assert!(encode_settle(&listing, &voices[..255], &[0; 4]).is_some());
    assert!(encode_settle(&listing, &voices, &[0; 4]).is_none());

    let words: Vec<Taught> = (0..256).map(|_| Taught { at: crate::chronometry::Stamp::default(), word: vec![0u8] }).collect();
    assert!(encode_syllabus_reply(&words[..255]).is_some());
    assert!(encode_syllabus_reply(&words).is_none());
}

/// Every act round-trips exactly — the calculus is closed and lossless.
#[test]
fn the_five_acts_round_trip() {
    let chain = vec![(0i64, 0i64, 1u8, 4u8), (2, 3, 0, 7)];
    let voice = [9u8; 32];
    let solver = [7u8; 32];
    let listing = crate::transducer::element::listing_key(
        &chain,
        &crate::transducer::element::IDENTITY_FRAME,
    )
    .expect("one entry per position is A5's chain");
    let cites = vec![CiteWire {
        addr: listing.clone(),
        standing: 0,
        transport: crate::transducer::element::translator_versor(1, -1),
        body: chain.clone(),
    }];
    let ask = encode_quote_request(&voice).expect("the ask fits its lengths");
    assert_eq!(decode_quote_request(&ask).unwrap(), (voice, Vec::new()));
    // L8q: the listing rides WHOLE. The field carries an element's own
    // bytes, so the round trip is over what the court actually serves and
    // what the voice actually decodes — not over a chain of links the
    // voice could only subtract from.
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("the sensor plane");
    let word = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
    let element = crate::transducer::element::from_connection(
        &word,
        &crate::transducer::element::identity_versor(),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("a connection on the plane")
    .to_bytes().expect("the record writes");
    let items = vec![WorkItem {
        addr: listing.clone(),
        mu: [1, 2, 30, 4],
        element,
        open: Vec::new(),
    }];
    let reply = encode_quote_reply(3, &[5, 0, 9, 0], 4208, &items).expect("the reply fits");
    let (h, s, k, it) = decode_quote_reply(&reply).unwrap();
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
        at: crate::chronometry::Place { k: 41, i: 1 },
    };
    let settlement = Settlement {
        listing: listing.clone(),
        chain: chain.clone(),
        voice,
        cites: cites.clone(),
        before: [2, 0, 2, 0],
        after: [0, 0, 1, 0],
        at: crate::chronometry::Stamp::new(7, [3; 32], crate::chronometry::Place { k: 42, i: 0 }),
    };
    let body = encode_docket_reply_with(std::slice::from_ref(&docket), std::slice::from_ref(&settlement))
        .expect("the docket writes");
    assert_eq!(decode_docket_reply(&body).unwrap(), vec![docket.clone()]);
    assert_eq!(decode_settlements(&body), vec![settlement.clone()]);
    let cavity = vec![0xCAu8, 0xFE, 0xBA, 0xBE];
    let (paged, w, s, n) = encode_docket_reply_paged(
        std::slice::from_ref(&docket),
        std::slice::from_ref(&settlement),
        std::slice::from_ref(&cavity),
        usize::MAX,
    )
    .expect("the page writes");
    assert_eq!((w, s, n), (1, 1, 1));
    assert_eq!(decode_cavities(&paged), vec![cavity]);
    assert_eq!(decode_teach(&encode_teach(&[0xAB, 0xCD]).expect("it fits")).unwrap(), vec![0xAB, 0xCD]);
    let settle = encode_settle(&listing, &[voice, solver], &[0, 0, 12, 0]).expect("it fits");
    let (g2, vs, mu) = decode_settle(&settle).unwrap();
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

/// LD0 step 5 — A CLASS PAGE CARRIES ITS REPRESENTATIVE AS A RAY: the rotor
/// inline as its canonical coordinates, read back as the rotor it was; a
/// page cut inside a record is refused whole, never shortened.
#[test]
fn a_class_page_carries_its_representative_as_a_ray() {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("the sensor plane");
    let r = crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor();
    let page = ClassPage { key: r.class_key().expect("a class"), order: 4, rotor: r.clone() };
    let words = vec![Taught { at: crate::chronometry::Stamp::default(), word: vec![7u8] }];
    let mut body = encode_syllabus_reply(&words).expect("it fits");
    append_class_records(&mut body, &[page.clone(), page.clone()]).expect("the records fit");
    let back = decode_class_records(&body);
    assert_eq!(back, vec![page.clone(), page], "the rotor crossed as the rotor");
    assert!(back[0].rotor.projective_eq(&r));
    assert!(decode_class_records(&body[..body.len() - 1]).is_empty(), "a cut page names no classes");
}

/// STD1 3c, LD0 — A PEERS REPLY IS WHOLE OR REFUSED. It clamped the count to
/// two bytes and each address to its first 65 535 bytes, handing a peer a
/// truncated address as though it were whole; an address past its u16 now
/// refuses the reply, and one that fits round-trips exactly.
#[test]
fn a_peers_reply_past_its_wall_is_refused_never_clamped() {
    let addrs = vec!["10.0.0.1:7000".to_string(), "10.0.0.2:7000".to_string()];
    let body = encode_peers_reply(&addrs).expect("two short addresses fit");
    assert_eq!(decode_peers_reply(&body), Some(addrs), "and read back whole");
    let long = vec!["a".repeat(usize::from(u16::MAX) + 1)];
    assert_eq!(encode_peers_reply(&long), None, "an address past two bytes refuses the reply");
}
