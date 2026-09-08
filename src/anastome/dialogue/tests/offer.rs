// 47.3 — the offer and the answer it gets: the home is named by the act,
// and the witness answers with its own key and only what it reached.

use crate::anastome::dialogue::*;

fn section_at(payload: &[u8], vk: [u8; 32]) -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_vk(vk);
    c.append(payload, [0; 32], Vec::new()).expect("appends");
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
    let o = decode_offer(&encode_offer(&offer)).expect("an offer parses whole");
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
    let whole = encode_offer(&offer);
    assert!(
        decode_offer(&whole[..whole.len() - 8]).is_none(),
        "a truncated section is not an offer"
    );
}

/// The witness answers with ITS OWN KEY, the form it reached, and the
/// refusal that slashed it — three states read off the content, with no bit
/// over the outcome of a replay. A witness that reached nothing says so
/// with an empty form; the offered tape is never echoed back, which the
/// home would read as a re-derivation that never happened.
#[test]
fn a_witness_answers_with_its_own_key_and_only_what_it_reached() {
    let vk = [0x2fu8; 32];
    let reached = vec![1u8, 2, 3, 4, 5];
    let w = decode_offer_reply(&encode_offer_reply(&Witnessed {
        vk,
        reached: reached.clone(),
        refused: Vec::new(),
    }))
    .expect("an answer parses whole");
    assert_eq!(w.vk, vk, "the vantage is the key the witness names");
    assert_eq!(w.reached, reached);
    assert!(w.refused.is_empty(), "it did not refuse");

    let none = decode_offer_reply(&encode_offer_reply(&Witnessed {
        vk,
        reached: Vec::new(),
        refused: Vec::new(),
    }))
    .expect("an answer with no form still parses");
    assert_eq!(none.vk, vk);
    assert!(
        none.reached.is_empty() && none.refused.is_empty(),
        "a witness that reached nothing is unmeasured, neither agreeing nor refusing"
    );

    // A SLASH NAMES ITS REFUSAL. The home logs and acts on the 34.1
    // constant; a bit would have said only "not admitted".
    let why = crate::court::verdict::REFUSE_NOT_CLOSED;
    let slashed = decode_offer_reply(&encode_offer_reply(&Witnessed {
        vk,
        reached: Vec::new(),
        refused: why.as_bytes().to_vec(),
    }))
    .expect("a refusal parses whole");
    assert_eq!(
        String::from_utf8(slashed.refused).expect("a 34.1 constant is text"),
        why,
        "the reason travels, not the fact that there was one"
    );

    assert!(decode_offer_reply(&[1u8]).is_none(), "a truncated answer names no vantage");
    assert!(decode_offer_reply(&[]).is_none());
}
