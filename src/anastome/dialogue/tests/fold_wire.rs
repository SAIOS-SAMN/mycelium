// 47.3, S13.10 — the fold proposal, the witness's three answers and the
// rows act round-trip; a short frame refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, ReceiptChain, Root};
use crate::court::admission::SignedPrefix;
use crate::economy::{Entry, Kind};

#[test]
fn the_fold_proposal_the_witness_answers_and_the_rows_round_trip() {
    let mut c = ReceiptChain::with_vk([6; 32]);
    c.append(ActKind::List, b"x", [0; 32], Vec::new()).expect("appends");
    let head = c.receipts()[0].clone();
    let root = Root { k: 4, hash: [5; 32], height: 9 };
    let section = ReceiptSection {
        prefix: SignedPrefix {
            genesis: [8; 32],
            epoch: 2,
            prev_receipt_hash: head.prev_receipt_hash,
            k: head.step_index,
            height: head.height,
            chart: vec![],
            payload: root.hash.to_vec(),
        },
        sig: [9u8; 64],
        receipt: head.clone(),
    };
    let p = FoldProposal {
        k0: 1,
        root,
        epochs: vec![([1; 32], [1, 2, 3, 4]), ([2; 32], [0, 0, 7, 0])],
        section: section.clone(),
    };
    let body = encode_fold(&p).expect("frames");
    let back = decode_fold(&body).expect("reads");
    assert_eq!((back.k0, back.root, back.epochs), (1, root, p.epochs.clone()));
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

    let req = encode_rows_request(&[1; 32], &[2; 32], 3, 9, 5);
    assert_eq!(
        decode_rows_request(&req),
        Some(RowsRequest { voice: [1; 32], signer: [2; 32], from: 3, to: 9, cursor: 5 })
    );
    let rows: Vec<Entry> = (0..3u8)
        .map(|i| Entry { kind: Kind::Mint, who: [i; 32], mu: [i as u64, 0, 0, 0], k_step: 3, receipt: [0; 32] })
        .collect();
    let all = encode_rows_reply(7, &rows, 1 << 16);
    assert_eq!(decode_rows_reply(&all), Some((7, rows.clone())));
    let page = encode_rows_reply(7, &rows, 8 + 2 * Entry::TAPE);
    assert_eq!(decode_rows_reply(&page).map(|(t, r)| (t, r.len())), Some((7, 2)), "a budget for two carries two");
    assert!(decode_rows_reply(&all[..all.len() - 1]).is_none(), "a short page refuses");
}
