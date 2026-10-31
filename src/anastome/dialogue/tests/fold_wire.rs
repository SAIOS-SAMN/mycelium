// 47.3, S13.10 — the fold proposal, the witness's three answers and the
// rows act round-trip; a short frame refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, Place, ReceiptChain, Root};
use crate::court::admission::SignedPrefix;
use crate::economy::{Entry, Kind};

/// The fold proposal, the witness's three answers and the rows act
/// round-trip, and a short frame refuses.
#[test]
fn fold_witness_round_trips_with_rows() {
    let mut c = ReceiptChain::with_signer([6; 32]);
    let head = c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let root = Root { at: Place { k: 4, i: 2 }, hash: [5; 32], height: 9 };
    let section = ReceiptSection {
        prefix: SignedPrefix {
            genesis: [8; 32],
            epoch: 2,
            prev_receipt_hash: head.prev_hash(),
            at: head.stamp.place(),
            height: head.stamp.h,
            chart: vec![],
            payload: root.hash.to_vec(),
        },
        sig: [9u8; 64],
        receipt: head.clone(),
    };
    let after = Some(Place { k: 1, i: 0 });
    let p = FoldProposal {
        after,
        root,
        epochs: vec![([1; 32], [1, 2, 3, 4]), ([2; 32], [0, 0, 7, 0])],
        section: section.clone(),
    };
    let body = encode_fold(&p).expect("frames");
    let back = decode_fold(&body).expect("reads");
    assert_eq!((back.after, back.root, back.epochs), (after, root, p.epochs.clone()));
    let first = FoldProposal { after: None, ..p.clone() };
    assert_eq!(decode_fold(&encode_fold(&first).expect("frames")).map(|b| b.after), Some(None), "a prefix from genesis");
    assert_eq!(
        (back.section.prefix, back.section.sig, back.section.receipt),
        (section.prefix.clone(), section.sig, head.clone())
    );
    assert!(decode_fold(&body[..body.len() - 1]).is_none(), "a short proposal refuses");

    for w in [
        FoldWitness::Refused { vk: [3; 32], why: b"why".to_vec() },
        FoldWitness::Unmeasured { vk: [3; 32] },
        FoldWitness::CoSigned { section: Box::new(section.clone()) },
    ] {
        let b = encode_fold_reply(&w).expect("frames");
        match (w, decode_fold_reply(&b).expect("reads")) {
            (FoldWitness::Refused { vk, why }, FoldWitness::Refused { vk: v2, why: w2 }) => {
                assert_eq!((vk, why), (v2, w2))
            }
            (FoldWitness::Unmeasured { vk }, FoldWitness::Unmeasured { vk: v2 }) => assert_eq!(vk, v2),
            (FoldWitness::CoSigned { section: s }, FoldWitness::CoSigned { section: s2 }) => {
                assert_eq!((s.prefix, s.sig), (s2.prefix, s2.sig))
            }
            _ => panic!("the arm changed on the wire"),
        }
    }
    assert!(decode_fold_reply(&[7]).is_none(), "an arm the layout does not define refuses");

    let (from, upto) = (Some(Place { k: 3, i: 0 }), Place { k: 9, i: 1 });
    let req = encode_rows_request(&[1; 32], &[2; 32], from, upto, 5);
    assert_eq!(decode_rows_request(&req), Some(RowsRequest { voice: [1; 32], signer: [2; 32], from, upto, cursor: 5 }));
    assert!(decode_rows_request(&req[..req.len() - 1]).is_none(), "a short request refuses");
    let rows: Vec<Entry> = (0..3u8)
        .map(|i| Entry {
            kind: Kind::Mint,
            who: [i; 32],
            mu: [i as u64, 0, 0, 0],
            at: crate::chronometry::Stamp { h: 3, vk: [2; 32], k: 3, i: 0 },
            receipt: [0; 32],
            arm: [None, Some(crate::economy::Arm::Pouw), Some(crate::economy::Arm::Powr)][i as usize],
        })
        .collect();
    let all = encode_rows_reply(7, &rows, 1 << 16).expect("the page counts");
    assert_eq!(decode_rows_reply(&all), Some((7, rows.clone())));
    // The header is the total (u64) and the page's count (u32).
    let page = encode_rows_reply(7, &rows, 12 + 2 * Entry::TAPE).expect("the page counts");
    assert_eq!(decode_rows_reply(&page).map(|(t, r)| (t, r.len())), Some((7, 2)), "a budget for two carries two");
    assert!(decode_rows_reply(&all[..all.len() - 1]).is_none(), "a short page refuses");
}
