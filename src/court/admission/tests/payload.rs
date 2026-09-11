// 42, 34.1 — the payload a ruling carries agrees with its ruling byte, or
// the act is malformed and refused by that name.

use super::super::*;
use crate::anastome::dialogue::{
    decode_verdict, encode_verdict, encode_verdict_attach, encode_verdict_payload, ReceiptSection,
    RULING_ATTACHED, RULING_REBIND, RULING_SLASHED, RULING_UPHELD,
};

fn section() -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_vk([0x5c; 32]);
    c.append(crate::chronometry::ActKind::List, [0; 4], b"a ruled address", [0; 32], Vec::new()).expect("appends");
    let r = c.receipts()[0].clone();
    ReceiptSection {
        prefix: SignedPrefix {
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

/// A REBIND WITH NO SELECTION, and an ATTACH with no base and transport,
/// name a payload the frame does not carry: `REFUSE_MALFORMED`, decided on
/// the parsed act before any carrier is touched. B1 answered both with
/// `REFUSE_FORM_TAPE`, which says a form's tape does not re-derive — a
/// different fault, and a reader acting on the constant would have looked
/// for a tape. A ruling that carries what it names reads back as the
/// payload it carries; an ordinary ruling carries none.
#[test]
fn a_ruling_that_names_a_payload_it_does_not_carry_is_malformed() {
    let court = [0x11u8; 32];
    let addr = crate::transducer::address::Address::new(vec![0xAB, 0xCD, 0xEF]);
    let head = [0x9e; 32];
    let s = section();

    let bare_rebind = decode_verdict(&encode_verdict(&court, &addr, RULING_REBIND, &head, &s).expect("frames"))
        .expect("the frame parses; it is the fields that disagree");
    assert!(
        matches!(admit_payload(&bare_rebind), Err(why) if why == REFUSE_MALFORMED),
        "a rebind with no selection is refused by its own name"
    );
    let bare_attach = decode_verdict(&encode_verdict(&court, &addr, RULING_ATTACHED, &head, &s).expect("frames"))
        .expect("parses");
    assert!(
        matches!(admit_payload(&bare_attach), Err(why) if why == REFUSE_MALFORMED),
        "and so is an attach with no base"
    );

    let pi = crate::transducer::anchor::Sel::Hue(3);
    let rebind = decode_verdict(
        &encode_verdict_payload(&court, &addr, RULING_REBIND, &head, Some(&pi), &s).expect("frames"),
    )
    .expect("parses");
    assert!(
        matches!(admit_payload(&rebind), Ok(Payload::Rebind(ref t)) if *t == pi),
        "a rebind that carries its selection reads back as that selection"
    );
    let base = crate::transducer::address::Address::new(vec![9, 9]);
    let g = crate::transducer::element::identity_versor();
    let attach = decode_verdict(&encode_verdict_attach(&court, &addr, &head, &base, &g, &s).expect("frames"))
        .expect("parses");
    assert!(
        matches!(admit_payload(&attach), Ok(Payload::Attach(ref b, ref v)) if *b == base && crate::transducer::element::same_versor(v, &g)),
        "an attach that carries its base and transport reads back as both, the transport by projective equality"
    );
    for ruling in [RULING_UPHELD, RULING_SLASHED] {
        let plain = decode_verdict(&encode_verdict(&court, &addr, ruling, &head, &s).expect("frames"))
            .expect("parses");
        assert!(
            matches!(admit_payload(&plain), Ok(Payload::Plain)),
            "an ordinary ruling carries no payload and names none"
        );
    }
}
