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
    for lane in 1..=3u64 {
        c.append(ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends");
    }
    let line: Vec<_> = c.own().cloned().collect();
    let all = encode_receipts_reply(&line, 1 << 16);
    assert_eq!(decode_receipts_reply(&all), Some(line.clone()), "the page reads back whole");
    // The first two nodes' own widths: genesis names no predecessor, so it
    // is the narrower of the two.
    let (mut one, mut two) = (Vec::new(), Vec::new());
    line[0].write(&mut one).expect("writes");
    line[1].write(&mut two).expect("writes");
    let page = encode_receipts_reply(&line, 4 + one.len() + two.len() + 1);
    assert_eq!(
        decode_receipts_reply(&page).map(|v| v.len()),
        Some(2),
        "a budget for two carries two, never a receipt cut short"
    );
    assert!(decode_receipts_reply(&all[..all.len() - 1]).is_none(), "a reply that does not parse to its end refuses");
}
