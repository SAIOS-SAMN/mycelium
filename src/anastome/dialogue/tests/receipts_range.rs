// 47.2 step 3, S13.10 — the place-range act: the request and the reply
// round trip, a reply is a page within its budget, and a short one refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, Place, ReceiptChain};

/// The receipts request and reply round-trip, a reply is a page within its
/// budget, and a short one refuses.
#[test]
fn receipts_request_round_trips_with_reply_within_budget() {
    let (after, upto) = (Some(Place { k: 3, i: 1 }), Place { k: 9, i: 0 });
    let req = encode_receipts_request(&[1; 32], &[2; 32], after, upto);
    assert_eq!(decode_receipts_request(&req), Some(ReceiptsRequest { voice: [1; 32], signer: [2; 32], after, upto, replay: false }));
    assert!(decode_receipts_request(&req[..req.len() - 1]).is_none(), "a short request refuses");
    // 78.147 — an era replay is the same span with its tag.
    let replay = encode_era_request(&[1; 32], &[2; 32], after, upto);
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
    assert!(decode_receipts_reply(&all).is_some_and(|back| super::fixtures::one_run(&back, &line)), "the page reads back whole");
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

/// D1.3 — A HEAD RECEIPT OVER THE CALLER'S BUDGET IS REFUSED: the page is
/// the reply's header alone, never a body past the budget the frame codec
/// would refuse after it was built; at the budget exactly, it rides.
#[test]
fn encode_receipts_reply_refuses_head_over_budget() {
    let mut c = ReceiptChain::with_signer([2; 32]);
    c.append(ActKind::List, [1, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let line: Vec<_> = c.own().collect();
    let mut tape = Vec::new();
    line[0].write(&mut tape).expect("writes");
    let whole = RECEIPTS_REPLY_HEADER + tape.len();
    let short = encode_receipts_reply(&line, whole - 1);
    assert_eq!(decode_receipts_reply(&short).map(|v| v.len()), Some(0), "refused: nothing rides");
    assert_eq!(short.len(), RECEIPTS_REPLY_HEADER, "the header alone");
    let page = encode_receipts_reply(&line, whole);
    assert!(decode_receipts_reply(&page).is_some_and(|back| super::fixtures::one_run(&back, &line)), "at the budget exactly, it rides");
}

/// D4.6 — THE RECEIPTS PAGE FILLS THE FRAME LESS ITS ENVELOPES: the tapes'
/// budget is the frame less the reading height behind the reply and the
/// reply's own count, and the page holds as many of the smallest tape as fit
/// it — no reserve of 64 beside them.
#[test]
fn receipts_page_fills_budget_less_header() {
    let b = crate::network::wire::BODY_BUDGET;
    let (at_most, bytes) = crate::cognition::Court::receipts_bounds().expect("a frame");
    assert_eq!(bytes, b - READING_HEIGHT_LEN - RECEIPTS_REPLY_HEADER);
    assert_eq!(at_most, bytes / crate::chronometry::KStepReceipt::MIN_TAPE);
    assert_eq!(crate::cognition::Court::receipts_reply_budget(), Some(b - READING_HEIGHT_LEN));
}

/// D4.6 — THE ENTRIES REPLY COUNTS ITS OWN HEADER: given the frame whole, a
/// page of more entries than fit holds exactly `⌊(B − 12)/ℓ⌋`, `ℓ` the
/// entry's own tape length.
#[test]
fn entries_reply_counts_reply_header() {
    use crate::economy::{Entry, Kind};
    let b = crate::network::wire::BODY_BUDGET;
    let row = Entry { kind: Kind::Mint, who: [1; 32], mu: [1, 0, 0, 0].into(), at: crate::chronometry::Stamp { h: 3, vk: [2; 32], k: 3, i: 0 }, receipt: [0; 32], arm: None };
    let fit = (b - ENTRIES_REPLY_HEADER) / row.tape_len();
    let rows = vec![row; fit + 1];
    let page = encode_entries_reply(0, &rows, b).expect("the page counts");
    assert_eq!(decode_entries_reply(&page).map(|(_, r)| r.len()), Some(fit));
    assert!(page.len() <= b);
}

/// D1.3, D4.6 — EVERY PAGE'S ENVELOPE IS ITS CODEC'S OWN HEADER: an empty
/// page of each reply is exactly the header its budget is read beside.
#[test]
fn page_envelope_equals_codec_header() {
    let b = crate::network::wire::BODY_BUDGET;
    assert_eq!(encode_receipts_reply(&[], b).len(), RECEIPTS_REPLY_HEADER);
    assert_eq!(encode_entries_reply(0, &[], b).map(|v| v.len()), Some(ENTRIES_REPLY_HEADER));
    let words = encode_syllabus_reply(&[]).expect("an empty page");
    assert_eq!(words.len(), SYLLABUS_REPLY_HEADER);
    let mut fetch = words;
    append_class_records(&mut fetch, &[]).expect("no records");
    assert_eq!(fetch.len(), SYLLABUS_REPLY_HEADER + CLASS_RECORDS_HEADER);
    assert_eq!(encode_cited_request(&[1; 32], &[]).map(|v| v.len()), Some(CITED_REQUEST_HEADER));
    assert_eq!(encode_quote_reply_within(1, &[1, 2, 3, 4], 5, &[], b).map(|v| v.len()), Some(QUOTE_REPLY_HEADER));
    let mut widest = vec![1u8];
    crate::transducer::address::put_key_count(&mut widest, usize::MAX);
    assert_eq!(widest.len(), atlas::LEDGER_PAGE_HEADER, "the more flag and the widest count");
}
