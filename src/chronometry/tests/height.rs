// 45.2 — height is the mesh's, not this mind's: it climbs on the receipts
// a step cites.

use crate::chronometry::*;

/// One act with nothing named, for the claims about the clock alone.
fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [0; 4], Vec::new(), [9; 32], parents).expect("appends").clone()
}

/// The receipt a chain holds at its head, as a parent is cited.
fn cite(r: &KStepReceipt) -> Parent {
    Parent { at: r.stamp, hash: r.chain_hash }
}

/// The head's height is one plus the maximum of its predecessor's height and its parents' heights.
#[test]
fn head_height_equals_one_plus_max_of_parents() {
    let mut a = ReceiptChain::with_signer([1; 32]);
    step(&mut a, Vec::new());
    assert_eq!(a.head_height(), 0, "h(genesis) = 0");
    step(&mut a, Vec::new());
    assert_eq!(a.head_height(), 1, "h = 1 + h(prev)");
    let mut b = ReceiptChain::with_signer([2; 32]);
    step(&mut b, Vec::new());
    step(&mut b, Vec::new());
    let bh = step(&mut b, Vec::new());
    assert_eq!(b.head_height(), 2);
    a.admit(bh.clone());
    step(&mut a, vec![cite(&bh)]);
    assert_eq!(a.head_height(), 1 + 2, "h = 1 + max(h(prev)=1, h(parent)=2)");
}

/// HT1, 45.2 — A LINE'S FIRST ACT THAT CITES A PARENT STANDS ABOVE IT: with
/// no predecessor, `h = 1 + max` over the parents, and `0` is the mesh
/// genesis alone. The writer stamps it so, the ledger's own integrity check
/// re-derives it, and a peer's admission re-derives it too — the three
/// readers of one rule. It stood at 0 wherever the line had no predecessor.
#[test]
fn append_stands_act_above_cited_parent() {
    let mut b = ReceiptChain::with_signer([2; 32]);
    for _ in 0..4 {
        step(&mut b, Vec::new());
    }
    let parent = b.head().expect("b has acted").clone();
    assert_eq!(parent.stamp.h, 3);
    let mut c = ReceiptChain::with_signer([3; 32]);
    c.admit(parent.clone());
    let first = step(&mut c, vec![cite(&parent)]);
    assert_eq!((first.stamp.k, first.stamp.h), (0, 3 + 1), "K 0, and one above the parent it cites");
    assert!(c.verify_chain_integrity(), "the ledger re-derives it");
    let mut d = ReceiptChain::with_signer([4; 32]);
    d.admit(parent);
    assert!(
        matches!(crate::court::admission::admit_backfilled(&first, &d), crate::court::admission::Admitted::Parent(_)),
        "and a peer admits it at that height"
    );
    let mut g = ReceiptChain::with_signer([5; 32]);
    assert_eq!(step(&mut g, Vec::new()).stamp.h, 0, "a first act citing nothing is at 0");
}

/// X8, 45.2, 78.24 — A LEDGER THAT CITES A PEER RESTORES WITH IT, held once
/// at its own stamp. Height is `1 + max` over the parents, so re-deriving it
/// needs every parent in hand: the line without the peer node refuses, the
/// ledger with it rehydrates with every height and order back, and a peer
/// node whose content was changed refuses the rehydration.
#[test]
fn from_nodes_rehydrates_only_with_cited_peer_nodes() {
    let mut peer = ReceiptChain::with_signer([2; 32]);
    step(&mut peer, Vec::new());
    let head = step(&mut peer, Vec::new());

    let mut c = ReceiptChain::with_signer([1; 32]);
    step(&mut c, Vec::new());
    c.admit(head.clone());
    let citing = step(&mut c, vec![cite(&head)]);
    assert_eq!(c.head_height(), 1 + 1, "h = 1 + max(h(prev)=0, h(parent)=1)");
    let line: Vec<KStepReceipt> = c.own().collect();
    let all: Vec<KStepReceipt> = c.nodes().collect();
    assert_eq!(c.admitted().len(), 1, "one peer node is held, once, at its stamp");

    assert!(ReceiptChain::from_nodes(None, [1; 32], line).is_none(), "the line alone cannot re-derive a height over a parent it does not hold");
    let back = ReceiptChain::from_nodes(None, [1; 32], all.clone()).expect("the ledger rehydrates whole");
    assert_eq!(back.head_height(), 2, "the height is re-derived, not restored");
    assert_eq!(back.height_of(&cite(&head)), Some(1), "and the parent's with it");
    let mut budget = crate::physics::Physics::default().stroke_n(8);
    assert_eq!(back.precedes(&head.stamp, &citing.stamp, &mut budget), Some(true), "the peer precedes the receipt that cited it");
    assert_eq!(back.precedes(&citing.stamp, &head.stamp, &mut budget), Some(false), "and not the other way");

    let tampered: Vec<KStepReceipt> = all
        .into_iter()
        .map(|mut r| {
            if r.stamp.vk == [2; 32] {
                r.mu = [7, 0, 0, 0];
            }
            r
        })
        .collect();
    assert!(
        ReceiptChain::from_nodes(None, [1; 32], tampered).is_none(),
        "a peer node whose content changed is not a parent this court can check"
    );
}

/// 78.24, 45.8 — THE LEDGER WRITTEN AS ITS LINE READS BACK AS ITSELF: every
/// node once at its stamp, the line verified, the head where it was.
#[test]
fn from_line_round_trips() {
    let mut peer = ReceiptChain::with_signer([2; 32]);
    let p0 = step(&mut peer, Vec::new());
    let mut c = ReceiptChain::with_signer([1; 32]);
    step(&mut c, Vec::new());
    c.admit(p0.clone());
    step(&mut c, vec![cite(&p0)]);
    let tape = c.line_image().expect("the line writes");
    let (back, torn) = ReceiptChain::from_line(&tape).expect("and reads back");
    assert!(!torn, "whole");
    assert_eq!(back.nodes().collect::<Vec<_>>(), c.nodes().collect::<Vec<_>>());
    assert_eq!((back.head_k(), back.head_height(), back.head_hash()), (c.head_k(), c.head_height(), c.head_hash()));
    assert_eq!(back.line_head(&[2; 32]).map(|r| r.stamp), Some(p0.stamp), "the peer's line is read off the same file");
}

/// EVERY NODE THE LEDGER HOLDS IS FOUND AT ITS STAMP — its own and the
/// ones it admitted alike — and a stamp it does not hold, or holds under
/// another hash, is unmeasured.
#[test]
fn height_of_finds_held_nodes_at_stamps() {
    let mut peer = ReceiptChain::with_signer([2; 32]);
    let p0 = step(&mut peer, Vec::new());
    let mut c = ReceiptChain::with_signer([1; 32]);
    step(&mut c, Vec::new());
    step(&mut c, Vec::new());
    c.admit(p0.clone());
    for r in c.own() {
        assert_eq!(c.height_of(&cite(&r)), Some(r.stamp.h), "its own, at its stamp");
    }
    assert_eq!(c.height_of(&cite(&p0)), Some(0), "and one it admitted");
    assert_eq!(c.height_of(&Parent { at: p0.stamp, hash: [0xab; 32] }), None, "a stamp held under another hash is unmeasured");
    assert_eq!(c.at_tick(0).len(), 2, "tick 0 holds this chain's genesis and the peer's: the mesh's width there");
}
