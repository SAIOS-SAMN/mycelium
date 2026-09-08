// 47.4, XSEPTUM 5.5 — ACT_EQUIVOCATION on the wire: two receipt sections
// parse back whole in the order presented, or the frame refuses whole.

use crate::anastome::dialogue::*;

fn section(delta: &[u8], sig: u8) -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_vk([0x5c; 32]);
    c.append(delta, [0; 32], Vec::new()).expect("appends");
    let r = c.receipts()[0].clone();
    ReceiptSection {
        prefix: crate::court::admission::SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_receipt_hash,
            k: r.step_index,
            height: r.height,
            chart: vec![1, 2],
            payload: r.delta.clone(),
        },
        sig: [sig; 64],
        receipt: r,
    }
}

/// The two halves of a proof travel as 47.1's receipt sections, each
/// framed by its length, and read back as the sections they were: prefix,
/// signature and receipt, in the order the presenter put them.
#[test]
fn two_receipt_sections_parse_back_whole_in_the_order_presented() {
    let (a, b) = (section(b"one", 0x11), section(b"two", 0x22));
    let body = encode_equivocation(&a, &b).expect("two prefixes that fit frame");
    assert_eq!(body[0], ACT_EQUIVOCATION, "the act byte");
    assert_eq!(body[1], EQUIVOCATION_VERSION, "then the layout version");
    let (ga, gb) = decode_equivocation(&body).expect("two sections parse whole");
    assert_eq!(ga.prefix, a.prefix, "the first prefix, whole");
    assert_eq!(ga.sig, a.sig, "its signature");
    assert_eq!(ga.receipt, a.receipt, "and the receipt it binds to");
    assert_eq!(gb.prefix, b.prefix);
    assert_eq!(gb.sig, b.sig);
    assert_eq!(gb.receipt, b.receipt);
    assert_ne!(ga.receipt.chain_hash, gb.receipt.chain_hash, "two roots at one K, as presented");
}

/// A frame that is not exactly two whole sections is not a proof: a
/// truncated section, bytes left after the second, another layout version
/// or another act each refuse the frame rather than reading a partial one
/// (rule 11).
#[test]
fn a_truncated_padded_or_misversioned_frame_refuses_whole() {
    let (a, b) = (section(b"one", 0x11), section(b"two", 0x22));
    let body = encode_equivocation(&a, &b).expect("frames");
    assert!(
        decode_equivocation(&body[..body.len() - 1]).is_none(),
        "a truncated second section is not a proof"
    );
    let mut padded = body.clone();
    padded.push(0);
    assert!(decode_equivocation(&padded).is_none(), "bytes after the second section refuse the frame");
    let mut other = body.clone();
    other[1] = EQUIVOCATION_VERSION + 1;
    assert!(decode_equivocation(&other).is_none(), "another layout version is not read");
    let mut act = body;
    act[0] = ACT_VERDICT;
    assert!(decode_equivocation(&act).is_none(), "another act is not read as this one");
}
