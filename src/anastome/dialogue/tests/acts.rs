// 26 — the five acts round-trip: every payload that goes out parses back
// to the same coordinates.

use crate::anastome::dialogue::*;

/// 7.1, 47.1 — A COUNT THAT DOES NOT FIT ITS FIELD IS A REFUSAL, not a
/// shorter frame. `len().min(255) as u8` wrote 255 and then 255 members,
/// so the reader took a frame that ended inside the next field and refused
/// the whole act while the sender believed it spoke.
#[test]
fn encode_refuses_count_past_field() {
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

    // The same at a syllabus page.
    let words: Vec<Taught> = (0..256).map(|_| Taught { at: crate::chronometry::Stamp::default(), word: vec![0u8] }).collect();
    assert!(encode_syllabus_reply(&words[..255]).is_some());
    assert!(encode_syllabus_reply(&words).is_none());
}

/// Every act round-trips exactly — the calculus is closed and lossless.
#[test]
fn acts_round_trips() {
    use crate::transducer::sentence::tests::fixtures::link;
    let chain = vec![link(0, 0, 1, 4), link(2, 3, 0, 7)];
    let voice = [9u8; 32];
    let solver = [7u8; 32];
    let listing = crate::transducer::element::listing_key(
        &chain,
        &crate::transducer::element::IDENTITY_FRAME,
    )
    .expect("one entry per position is A5's chain");
    let ask = encode_quote_request(&voice).expect("the ask fits its lengths");
    assert_eq!(decode_quote_request(&ask).unwrap(), (voice, Vec::new()));
    // L8q: the listing rides WHOLE. The field carries an element's own
    // bytes, so the round trip is over what the court actually serves and
    // what the voice actually decodes — not over a chain of links the
    // voice could only subtract from.
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
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
    let (held, at) = decode_root_reply(&encode_root_reply(Some(solver), 9)).unwrap();
    assert_eq!((held, at), (Some(solver), 9));
    let (none, at0) = decode_root_reply(&encode_root_reply(None, 3)).unwrap();
    assert_eq!((none, at0), (None, 3));
    assert!(
        decode_root_reply(&[1u8]).is_none(),
        "a truncated solver is refused, never a zero voice"
    );
}

/// 26, law 8 — THE RETIRED ACTS' BYTES ARE REFUSED BY KIND. One fixture per
/// retired act, in the layout its retired codec wrote: a docket pull, a
/// teach and a settle receipt. Each names an action of the retired kind and
/// is long enough to reach the door's refusal (a body of five bytes or
/// more is answered `REFUSE_NOT_A_SENTENCE`, never decoded).
#[test]
fn retired_act_fixtures_name_retired_kind() {
    use crate::physics::actions::{Action, Kind};
    let voice = [9u8; 32];
    // `[ACT_DOCKET][voice 32][since K u64][ticks u64]`
    let mut docket = vec![ACT_DOCKET];
    docket.extend_from_slice(&voice);
    docket.extend_from_slice(&41u64.to_le_bytes());
    docket.extend_from_slice(&7u64.to_le_bytes());
    // `[ACT_TEACH][len u32][element bytes]`
    let teach = vec![ACT_TEACH, 2, 0, 0, 0, 0xAB, 0xCD];
    // `[ACT_SETTLE][germ address][nvoices u8][voices 32×n][μ⃗ 4×u64]`
    let listing = crate::transducer::element::listing_key(
        &[crate::transducer::sentence::tests::fixtures::link(0, 0, 1, 4)],
        &crate::transducer::element::IDENTITY_FRAME,
    )
    .expect("a one-link chain is A5's");
    let mut settle = vec![ACT_SETTLE];
    crate::transducer::address::put_address(&mut settle, &listing).expect("the listing frames");
    settle.push(1);
    settle.extend_from_slice(&voice);
    push_mu(&mut settle, &[0, 0, 12, 0]);
    for (body, action) in [(docket, Action::Docket), (teach, Action::Teach), (settle, Action::Settle)] {
        assert!(body.len() >= 5, "{action:?} reaches the door's refusal");
        let read = act_tag(&body).and_then(Action::of_wire);
        assert_eq!(read, Some(action));
        assert_eq!(read.map(Action::kind), Some(Kind::Retired), "{action:?} is refused by its kind");
    }
}

/// LD0 step 5 — A CLASS PAGE CARRIES ITS REPRESENTATIVE AS A RAY: the rotor
/// inline as its canonical coordinates, read back as the rotor it was; a
/// page cut inside a record is refused whole, never shortened.
#[test]
fn class_page_round_trips_and_refuses_cut_page() {
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
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
fn encode_peers_reply_refuses_address_past_wall() {
    let addrs = vec!["10.0.0.1:7000".to_string(), "10.0.0.2:7000".to_string()];
    let body = encode_peers_reply(&addrs).expect("two short addresses fit");
    assert_eq!(decode_peers_reply(&body), Some(addrs), "and read back whole");
    let long = vec!["a".repeat(usize::from(u16::MAX) + 1)];
    assert_eq!(encode_peers_reply(&long), None, "an address past two bytes refuses the reply");
}

/// #16 — A DOOR DEFERRAL IS NOT A PEERS REPLY. Every deficit tried used to
/// decode as a peer list when the client skipped `read_deferral` first.
#[test]
fn decode_peers_reply_refuses_deferral() {
    for deficit in [[0; 4], [1, 0, 2, 3], [u64::MAX, 0, 0, 1]] {
        let bytes = encode_deferral(PENDING_CODE, &deficit);
        assert!(
            decode_peers_reply(&bytes).is_none(),
            "deferral deficit {deficit:?} must not decode as peers"
        );
    }
    let mut trailing = encode_peers_reply(&["127.0.0.1:1".into()]).expect("fits");
    trailing.push(0);
    assert!(
        decode_peers_reply(&trailing).is_none(),
        "bytes after the last address refuse the reply"
    );
}

/// 78.141 — A SOLVE NAMES ITS PAYEE BY ITS VOICE'S SIGNATURE: signed by the
/// key whose verifying half is its voice, with and without its rule, it
/// reads back signed; a byte of it changed, or its voice swapped for
/// another's, reads unsigned; one with no signature reads unsigned; bytes
/// after it that are no signature refuse it.
#[test]
fn decode_solve_signs_payee_by_voice_key() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let outputs = spoken_outputs(&[vec![vec![1u8, 2], vec![0, 3]].into()]).expect("a sample");
    for rule in [None, Some(&b"a rule's tape"[..])] {
        let body = encode_solve_with_rule(&key, &[7u8; 32], &outputs, rule).expect("fits");
        let act = decode_solve(&body).expect("reads back");
        assert!(act.signed, "signed by its voice");
        assert_eq!(act.task, [7u8; 32], "wire v1.2: the task by its 32-byte citation");
        assert_eq!((act.voice, act.outputs.clone(), act.rule.as_deref()), (key.verifying_key().to_bytes(), outputs.clone(), rule));
        let mut tampered = body.clone();
        tampered[40] ^= 1;
        assert!(decode_solve(&tampered).is_some_and(|a| !a.signed), "a changed act is not what was signed");
        let other = ed25519_dalek::SigningKey::from_bytes(&[6u8; 32]).verifying_key().to_bytes();
        let mut swapped = body.clone();
        // The voice follows the act's tag and its claim kind (track 26 S4c).
        swapped[2..34].copy_from_slice(&other);
        assert!(decode_solve(&swapped).is_some_and(|a| !a.signed), "another voice named as payee");
        let unsigned = body.get(..body.len() - 68).expect("a signature at the end").to_vec();
        assert!(decode_solve(&unsigned).is_some_and(|a| !a.signed), "no signature, no payee");
        let mut trailing = body.clone();
        trailing.push(0);
        assert!(decode_solve(&trailing).is_none(), "bytes after the signature");
    }
}

/// TRACK 26 S2 — A SOLVE STATES THE GROUND ITS CLAIM WAS SPOKEN UNDER, and
/// reads it back exactly, at the symbol's own width; a solve stating none
/// reads none; the stated ground is signed with the act, so a changed
/// ground reads unsigned; an empty stated ground is no ground and refuses
/// the act.
#[test]
fn solve_round_trips_stated_ground() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let outputs = spoken_outputs(&[vec![vec![1u8, 2], vec![0, 3]].into()]).expect("a sample");
    let tape = &b"a rule's tape"[..];
    for ground in [vec![7u8], vec![1u8, 2]] {
        let body = encode_solve_spoken(&key, &[7u8; 32], &outputs, Some(tape), Some(&ground)).expect("fits");
        let act = decode_solve(&body).expect("reads back");
        assert_eq!((act.ground.as_deref(), act.rule.as_deref(), act.signed), (Some(&ground[..]), Some(tape), true));
        // The ground's last byte stands just before `SIGN` and the signature.
        let mut changed = body.clone();
        let at = body.len() - 69;
        changed[at] ^= 1;
        assert!(decode_solve(&changed).is_some_and(|a| !a.signed), "the stated ground is signed with the act");
    }
    let none = decode_solve(&encode_solve_spoken(&key, &[7u8; 32], &outputs, Some(tape), None).expect("fits")).expect("reads back");
    assert_eq!((none.ground, none.signed), (None, true), "no ground stated, none read");
    let empty = encode_solve_spoken(&key, &[7u8; 32], &outputs, Some(tape), Some(&[])).expect("fits");
    assert!(decode_solve(&empty).is_none(), "an empty stated ground is refused");
}
