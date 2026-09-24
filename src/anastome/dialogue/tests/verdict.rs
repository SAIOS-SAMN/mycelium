// 42, 47.1 — ACT_VERDICT on the wire: a ruling parses back to the ruling
// it was, on every payload arm, with its receipt section whole.

use crate::anastome::dialogue::*;

fn section() -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_vk([0x5c; 32]);
    c.append(crate::chronometry::ActKind::List, [0; 4], b"a ruled address", [0; 32], Vec::new()).expect("appends");
    let r = c.receipts()[0].clone();
    ReceiptSection {
        prefix: crate::court::admission::SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_receipt_hash,
            k: r.step_index,
            height: r.height,
            chart: vec![1, 2, 3],
            payload: r.delta.clone(),
        },
        sig: [0x3a; 64],
        receipt: r,
    }
}

/// EVERY ARM OF THE PAYLOAD TAG ROUND-TRIPS. There was no round-trip test
/// for this act at all, and the arm with no payload did not consume its own
/// tag byte in the decoder: `p` was left pointing at the zero the encoder
/// had written, so the receipt section behind it was read one byte early
/// and `decode_verdict` returned `None`. RULING_UPHELD and RULING_SLASHED
/// carry no payload, so the two ordinary rulings never decoded — the door
/// returned without answering and the ruling court blocked on a reply that
/// never came — while REBIND and ATTACH, which write a tag the decoder did
/// consume, parsed fine. This asserts the arm that was broken first.
#[test]
fn a_ruling_with_no_payload_parses_back_to_the_ruling_it_was() {
    let court = [0x11u8; 32];
    let addr = crate::transducer::address::Address::new(vec![0xAB, 0xCD, 0xEF]);
    let head = [0x9e; 32];
    let s = section();

    for ruling in [RULING_UPHELD, RULING_DEMOTED, RULING_SLASHED] {
        let body = encode_verdict(&court, &addr, ruling, &head, &s).expect("frames");
        let v = decode_verdict(&body).expect("a ruling with no payload parses whole");
        assert_eq!(v.voice, court, "the court that ruled");
        assert_eq!(v.addr, addr, "the word it ruled on");
        assert_eq!(v.ruling, ruling);
        assert_eq!(v.receipt, head, "the court head the ruling binds to");
        assert!(v.pi.is_none() && v.attach.is_none(), "no payload was written");
        assert_eq!(v.section.prefix, s.prefix, "47.1's prefix survives whole");
        assert_eq!(v.section.sig, s.sig);
        assert_eq!(v.section.receipt.step_index, s.receipt.step_index);
        assert_eq!(v.section.receipt.vk, s.receipt.vk, "and the signer it names");
    }
}

/// The π* arm (RULING_REBIND) and the attach arm (RULING_ATTACHED) carry a
/// payload between the tag and the receipt section, and the section is
/// still found at the right offset behind it.
#[test]
fn a_rebind_and_an_attach_carry_their_payload_and_still_find_the_section() {
    let court = [0x22u8; 32];
    let addr = crate::transducer::address::Address::new(vec![1, 2, 3, 4]);
    let head = [0x7f; 32];
    let s = section();

    let pi = crate::transducer::anchor::Sel::Hue(3);
    let body = encode_verdict_payload(&court, &addr, RULING_REBIND, &head, Some(&pi), &s).expect("frames");
    let v = decode_verdict(&body).expect("a rebind parses whole");
    assert_eq!(v.ruling, RULING_REBIND);
    assert_eq!(v.pi, Some(pi), "the selection the court rebinds to");
    assert_eq!(v.section.receipt.vk, s.receipt.vk, "the section is behind it, whole");

    let base = crate::transducer::address::Address::new(vec![9, 9]);
    let g = crate::transducer::element::identity_versor();
    let body = encode_verdict_attach(&court, &addr, &head, &base, &g, &s).expect("frames");
    let v = decode_verdict(&body).expect("an attach parses whole");
    assert_eq!(v.ruling, RULING_ATTACHED);
    let (b, gg) = v.attach.expect("the base and the transport");
    assert_eq!(b, base);
    assert!(
        crate::transducer::element::same_versor(&gg, &g),
        "the transport by projective equality, never by bytes"
    );
    assert_eq!(v.section.receipt.vk, s.receipt.vk);
}

/// 42, 47.2: A RULING'S ANSWER IS FOUR ARMS, and the ruling court must be
/// able to tell them apart. It was one byte, `applied as u8`, and every
/// outcome answered `0`: another mesh, a bad signature, a chain gap, a
/// stale frame, an equivocation, and a ruling that found nothing to move.
/// A gap means "send me the back-fill" and an equivocation is 47.4's burn
/// proof — the mesh's only bond-slashing evidence — and both read as "your
/// ruling did not land".
#[test]
fn a_ruling_is_answered_with_the_arm_it_reached() {
    for r in [Ruled::Applied, Ruled::Inert] {
        let back = decode_ruled(&encode_ruled(&r).expect("frames")).expect("parses whole");
        assert_eq!(
            std::mem::discriminant(&back),
            std::mem::discriminant(&r),
            "applied and inert are two answers, not one byte"
        );
    }

    let why = crate::court::verdict::REFUSE_EQUIVOCATION;
    let back = decode_ruled(&encode_ruled(&Ruled::Refused(why.as_bytes().to_vec())).expect("frames"))
        .expect("a refusal parses whole");
    match back {
        Ruled::Refused(got) => assert_eq!(
            String::from_utf8(got).expect("a 34.1 constant is text"),
            why,
            "the 34.1 constant travels, so a burn proof is not lost at the door"
        ),
        _ => panic!("the arm it was encoded as"),
    }

    let back = decode_ruled(&encode_ruled(&Ruled::Gap { from: 7, to: 19 }).expect("frames"))
        .expect("a gap parses whole");
    match back {
        Ruled::Gap { from, to } => assert_eq!(
            (from, to),
            (7, 19),
            "a gap names the span to back-fill, which a zero byte could not"
        ),
        _ => panic!("the arm it was encoded as"),
    }

    assert!(decode_ruled(&[9u8]).is_none(), "an undefined arm refuses the frame");
    assert!(decode_ruled(&[]).is_none());
    assert!(decode_ruled(&[3u8, 0, 0]).is_none(), "a truncated gap is not a gap");
}

/// A TAG THE LAYOUT DOES NOT DEFINE IS REFUSED, never ignored. A decoder
/// that skips what it does not recognise reads the rest of the frame at an
/// offset it guessed (rule 11, A21 1: the decoder is total on well-formed
/// keys and refuses otherwise).
#[test]
fn an_unknown_payload_tag_is_refused_rather_than_skipped() {
    let s = section();
    let addr = crate::transducer::address::Address::new(vec![5, 5]);
    let mut body = encode_verdict(&[0x33; 32], &addr, RULING_UPHELD, &[0; 32], &s).expect("frames");
    let section = super::super::encode_receipt_section(&s.prefix, &s.sig, &s.receipt).expect("frames");
    let tag = body.len() - section.len() - 1;
    assert_eq!(body[tag], 0, "the no-payload tag is where the encoder put it");
    body[tag] = 0x7e;
    assert!(decode_verdict(&body).is_none(), "an undefined tag refuses the frame");
    // And a frame truncated inside its receipt section refuses too, rather
    // than admitting a partial ruling (rule 11).
    let whole = encode_verdict(&[0x33; 32], &addr, RULING_UPHELD, &[0; 32], &s).expect("frames");
    let short = &whole[..tag + 3];
    assert!(decode_verdict(short).is_none(), "a truncated section is not a ruling");
}
