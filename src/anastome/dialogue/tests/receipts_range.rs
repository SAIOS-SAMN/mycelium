// 47.2 step 3, S13.10 — the K-range act: the request and the reply round
// trip, a reply is a page within its budget, and a short one refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, ReceiptChain};

#[test]
fn the_receipts_request_and_reply_round_trip_within_the_budget() {
    let req = encode_receipts_request(&[1; 32], &[2; 32], 3, 9);
    assert_eq!(
        decode_receipts_request(&req),
        Some(ReceiptsRequest { voice: [1; 32], signer: [2; 32], from: 3, to: 9 })
    );
    assert!(decode_receipts_request(&req[..80]).is_none(), "a short request refuses");

    let mut c = ReceiptChain::with_vk([2; 32]);
    for d in [b"a".as_slice(), b"b", b"c"] {
        c.append(ActKind::List, d, [0; 32], Vec::new()).expect("appends");
    }
    let all = encode_receipts_reply(c.receipts(), 1 << 16);
    assert_eq!(decode_receipts_reply(&all).as_deref(), Some(c.receipts()), "the page reads back whole");
    let mut one = Vec::new();
    c.receipts()[0].write(&mut one);
    let page = encode_receipts_reply(c.receipts(), 4 + 2 * one.len() + 1);
    assert_eq!(
        decode_receipts_reply(&page).map(|v| v.len()),
        Some(2),
        "a budget for two carries two, never a receipt cut short"
    );
    assert!(decode_receipts_reply(&all[..all.len() - 1]).is_none(), "a reply that does not parse to its end refuses");
}
