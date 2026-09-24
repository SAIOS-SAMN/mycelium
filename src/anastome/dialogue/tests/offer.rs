// 47.3 — the offer and the answer it gets: the home is named by the act,
// and the witness answers with its own key and only what it reached.

use crate::anastome::dialogue::*;

fn section_at(payload: &[u8], vk: [u8; 32]) -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_vk(vk);
    c.append(crate::chronometry::ActKind::List, [0; 4], payload, [0; 32], Vec::new()).expect("appends");
    let r = c.receipts()[0].clone();
    ReceiptSection {
        prefix: crate::court::admission::SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_receipt_hash,
            k: r.step_index,
            height: r.height,
            chart: vec![1, 2, 3],
            payload: payload.to_vec(),
        },
        sig: [0x4a; 64],
        receipt: r,
    }
}

/// The offer names WHO OFFERED and carries 47.1's signed position for it.
/// Without the first the witness has only the socket it was dialled on,
/// and a socket is not a vantage: it can glue the form as itself, compare
/// it with itself, and agree. Without the second nothing is verifiable at
/// all — the act carried `k: 0` and two all-zero signatures over a bespoke
/// prefix that had no production signer and no verifier.
#[test]
fn an_offer_names_the_home_that_made_it_and_signs_its_own_position() {
    let addr = crate::transducer::address::Address::new(vec![0xA1u8, 0xB2, 0xC3]);
    let home = [3u8; 32];
    let attested = [9u8; 32];
    let section = section_at(addr.as_bytes(), home);
    let offer = Offer {
        addr: addr.clone(),
        home,
        elem: vec![7u8, 7, 7, 7],
        voices: vec![attested],
        deflation: [1, 0, 2, 0],
        section: section.clone(),
    };
    let o = decode_offer(&encode_offer(&offer).expect("frames")).expect("an offer parses whole");
    assert_eq!(o.home, home, "the home's key rides the act");
    assert_eq!(o.addr, addr);
    assert_eq!(o.elem, offer.elem);
    assert_eq!(o.voices, vec![attested], "and is not the attesting voices");
    assert_eq!(o.deflation, [1, 0, 2, 0]);

    // 47.1: the signature's payload is the OFFERED ADDRESS, so it binds the
    // form by its content and not by the spelling — the witness re-derives
    // the address from the tape it was sent (law 2).
    assert_eq!(
        o.section.prefix.payload,
        addr.as_bytes(),
        "the prefix's payload is the address the offer is for"
    );
    assert_eq!(o.section.receipt.vk, home, "and the receipt names the home");
    assert_eq!(o.section.sig, section.sig, "the signature survives whole");
    assert_eq!(o.section.prefix, section.prefix);

    // A frame truncated inside its section is refused, never half-read.
    let whole = encode_offer(&offer).expect("frames");
    assert!(
        decode_offer(&whole[..whole.len() - 8]).is_none(),
        "a truncated section is not an offer"
    );
}

/// THE WITNESS ANSWERS WITH ONE OF THREE ARMS, and the home matches on
/// which. Every arm names the key that answered: an address is where a
/// hypha answers, a vantage is a key. This was a `glued: bool` and then a
/// struct of two byte vectors whose LENGTHS the home tested to tell the
/// arms apart, which is checklist (b) in a longer form.
#[test]
fn a_witness_answers_with_one_of_three_arms_and_names_the_key() {
    let vk = [0x2fu8; 32];

    // REACHED: the second vantage, as a tape, never the offered form.
    let form = vec![1u8, 2, 3, 4, 5];
    let w = decode_offer_reply(&encode_offer_reply(&Witnessed::Reached { vk, form: form.clone() }).expect("frames"))
        .expect("an answer parses whole");
    assert_eq!(w.vk(), vk, "the vantage is the key the witness names");
    match w {
        Witnessed::Reached { form: got, .. } => assert_eq!(got, form),
        _ => panic!("the arm it was encoded as"),
    }

    // UNMEASURED: it replayed and has re-derived nothing of its own.
    let none = decode_offer_reply(&encode_offer_reply(&Witnessed::Unmeasured { vk }).expect("frames"))
        .expect("an answer with no form still parses");
    assert!(
        matches!(none, Witnessed::Unmeasured { .. }),
        "unmeasured is its own arm, neither agreeing nor refusing"
    );
    assert_eq!(none.vk(), vk, "and it still names the key that answered");

    // REFUSED: the 34.1 constant travels, not the fact that there was one.
    let why = crate::court::verdict::REFUSE_NOT_CLOSED;
    let slashed =
        decode_offer_reply(&encode_offer_reply(&Witnessed::Refused { vk, why: why.as_bytes().to_vec() }).expect("frames"))
            .expect("a refusal parses whole");
    match slashed {
        Witnessed::Refused { why: got, .. } => assert_eq!(
            String::from_utf8(got).expect("a 34.1 constant is text"),
            why,
            "the reason travels, not the fact that there was one"
        ),
        _ => panic!("the arm it was encoded as"),
    }

    // An arm the layout does not define is refused, never guessed at.
    assert!(decode_offer_reply(&[9u8; 40]).is_none(), "an undefined arm refuses the frame");
    assert!(decode_offer_reply(&[1u8]).is_none(), "a truncated answer names no vantage");
    assert!(decode_offer_reply(&[]).is_none());
}
