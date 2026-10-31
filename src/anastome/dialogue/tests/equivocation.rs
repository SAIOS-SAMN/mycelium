// 47.4, MU-XSEPTUM 5.5 — ACT_EQUIVOCATION on the wire: two receipt sections
// parse back whole in the order presented, or the frame refuses whole.

use crate::anastome::dialogue::*;

/// A root under one key, measuring `lane`: two lanes, two roots at K 0.
fn section(lane: u64, sig: u8) -> ReceiptSection {
    let mut c = crate::chronometry::ReceiptChain::with_signer([0x5c; 32]);
    let r = c
        .append(crate::chronometry::ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new())
        .expect("appends")
        .clone();
    ReceiptSection {
        prefix: crate::court::admission::SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_hash(),
            at: r.stamp.place(),
            height: r.stamp.h,
            chart: vec![1, 2],
            payload: r.stamp.key(),
        },
        sig: [sig; 64],
        receipt: r,
    }
}

/// The two halves of a proof travel as 47.1's receipt sections, each
/// framed by its length, and read back as the sections they were: prefix,
/// signature and receipt, in the order the presenter put them.
#[test]
fn equivocation_round_trips_sections_in_order() {
    let (a, b) = (section(1, 0x11), section(2, 0x22));
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
fn decode_equivocation_refuses_truncated_padded_or_misversioned() {
    let (a, b) = (section(1, 0x11), section(2, 0x22));
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
    act[0] = ACT_OFFER;
    assert!(decode_equivocation(&act).is_none(), "another act is not read as this one");
}
