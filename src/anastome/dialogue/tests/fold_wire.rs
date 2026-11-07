// 47.3, S13.10 — the fold proposal, the witness's three answers and the
// entries act round-trip; a short frame refuses.

use crate::anastome::dialogue::*;
use crate::chronometry::{ActKind, Place, ReceiptChain, Root};
use crate::court::admission::SignedPrefix;
use crate::economy::{Entry, Kind};

/// The fold proposal, the witness's three answers and the entries act
/// round-trip, and a short frame refuses.
#[test]
fn fold_witness_round_trips_with_rows() {
    let mut c = ReceiptChain::with_signer([6; 32]);
    let head = c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let root = Root { at: Place { k: 4, i: 2 }, hash: crate::court::admission::Predecessor::of_bytes([5; 32]), height: 9 };
    let section = ReceiptSection {
        prefix: SignedPrefix {
            genesis: [8; 32],
            epoch: 2,
            prev_receipt_hash: head.prev_hash(),
            at: head.stamp.place(),
            height: head.stamp.h,
            chart: vec![],
            payload: root.signed(),
        },
        sig: [9u8; 64],
        receipt: head.clone(),
    };
    let after = Some(Place { k: 1, i: 0 });
    let p = FoldProposal {
        after,
        root,
        epochs: vec![
            ([1; 32], crate::economy::tests::founding([1, 2, 3, 4])),
            ([2; 32], crate::economy::tests::founding(crate::economy::Mu::counts([0, 0, 7, 0]).part(3).expect("three parts"))),
        ],
        section: section.clone(),
    };
    let body = encode_fold(&p).expect("frames");
    let back = decode_fold(&body).expect("reads");
    use super::fixtures::{one_prefix, one_receipt};
    assert_eq!((back.after, &back.epochs), (after, &p.epochs));
    assert!(back.root.restates(&root), "the root, at its place and height with its hash");
    let first = FoldProposal { after: None, ..p.clone() };
    assert_eq!(decode_fold(&encode_fold(&first).expect("frames")).map(|b| b.after), Some(None), "a prefix from genesis");
    assert!(one_prefix(&back.section.prefix, &section.prefix));
    assert_eq!(back.section.sig, section.sig);
    assert!(one_receipt(&back.section.receipt, &head));
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
                assert!(one_prefix(&s.prefix, &s2.prefix) && s.sig == s2.sig)
            }
            _ => panic!("the arm changed on the wire"),
        }
    }
    assert!(decode_fold_reply(&[7]).is_none(), "an arm the layout does not define refuses");

    let (from, upto) = (Some(Place { k: 3, i: 0 }), Place { k: 9, i: 1 });
    let req = encode_entries_request(&[1; 32], &[2; 32], from, upto, 5);
    assert_eq!(decode_entries_request(&req), Some(EntriesRequest { voice: [1; 32], signer: [2; 32], from, upto, cursor: 5 }));
    assert!(decode_entries_request(&req[..req.len() - 1]).is_none(), "a short request refuses");
    let entries: Vec<Entry> = (0..3u8)
        .map(|i| Entry {
            kind: Kind::Mint,
            who: [i; 32],
            modes: crate::economy::tests::founding(crate::economy::Mu::counts([u64::from(i), 0, 0, 0]).part(usize::from(i) + 1).expect("a part")),
            at: crate::chronometry::Stamp { h: 3, vk: [2; 32], k: 3, i: 0 },
            receipt: [0; 32],
            arm: [None, Some(crate::economy::Arm::Pouw), Some(crate::economy::Arm::Powr)][i as usize],
        })
        .collect();
    let all = encode_entries_reply(7, &entries, 1 << 16).expect("the page counts");
    assert_eq!(decode_entries_reply(&all), Some((7, entries.clone())));
    // The header is the total (u64) and the page's count (u32).
    let page = encode_entries_reply(7, &entries, 12 + entries[0].tape_len() + entries[1].tape_len()).expect("the page counts");
    assert_eq!(decode_entries_reply(&page).map(|(t, r)| (t, r.len())), Some((7, 2)), "a budget for two carries two");
    assert!(decode_entries_reply(&all[..all.len() - 1]).is_none(), "a short page refuses");
}

/// TRACK 11, N7 — THE ENTRIES ACT ROUND-TRIPS under its name: the request
/// opens with `ACT_ENTRIES`, the byte the action table gives the ledger
/// entries' read, and request and reply read back the values written.
#[test]
fn entries_act_round_trips() {
    assert_eq!(ACT_ENTRIES, crate::physics::actions::Action::Entries.byte());
    assert_eq!(crate::physics::actions::Action::of_wire(ACT_ENTRIES), Some(crate::physics::actions::Action::Entries));
    let (from, upto) = (None, Place { k: 4, i: 2 });
    let req = encode_entries_request(&[3; 32], &[4; 32], from, upto, 9);
    assert_eq!(act_tag(&req), Some(ACT_ENTRIES));
    assert_eq!(decode_entries_request(&req), Some(EntriesRequest { voice: [3; 32], signer: [4; 32], from, upto, cursor: 9 }));
    let entry = Entry {
        kind: Kind::Mint,
        who: [5; 32],
        modes: crate::economy::tests::founding([2, 0, 0, 0]),
        at: crate::chronometry::Stamp { h: 4, vk: [4; 32], k: 4, i: 1 },
        receipt: [6; 32],
        arm: None,
    };
    let reply = encode_entries_reply(1, std::slice::from_ref(&entry), crate::network::wire::frame_budget()).expect("the page counts");
    assert_eq!(decode_entries_reply(&reply), Some((1, vec![entry])));
}
