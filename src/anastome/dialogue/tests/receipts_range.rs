// 47.2 step 3, S13.10 — the place-range act: the request and the reply
// round trip, a reply is a page within its budget, and a short one refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, Place, ReceiptChain};

/// The receipts request and reply round-trip, a reply is a page within its
/// budget, and a short one refuses.
#[test]
fn receipts_request_and_reply_round_trip_within_budget() {
    let (after, upto) = (Some(Place { k: 3, i: 1 }), Place { k: 9, i: 0 });
    let req = encode_receipts_request(&[1; 32], &[2; 32], after, upto);
    assert_eq!(decode_receipts_request(&req), Some(ReceiptsRequest { voice: [1; 32], signer: [2; 32], after, upto, replay: false }));
    assert!(decode_receipts_request(&req[..req.len() - 1]).is_none(), "a short request refuses");
    // 78.147 — an era replay is the same span with its tag.
    let replay = encode_replay_request(&[1; 32], &[2; 32], after, upto);
    assert_eq!(decode_receipts_request(&replay).map(|r| (r.after, r.upto, r.replay)), Some((after, upto, true)), "the replay ask");
    let mut wrong = replay.clone();
    wrong.push(1);
    assert!(decode_receipts_request(&wrong).is_none(), "a request with bytes past its tag refuses");
    let from_start = encode_receipts_request(&[1; 32], &[2; 32], None, upto);
    assert_eq!(
        decode_receipts_request(&from_start).map(|r| (r.after, r.upto)),
        Some((None, upto)),
        "a span from the line's start"
    );

    let mut c = ReceiptChain::with_signer([2; 32]);
    for lane in 1..=3u64 {
        c.append(ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends");
    }
    let line: Vec<_> = c.own().collect();
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
