// 45.8 — the inverse of admission: a peer's folded line leaves the hot
// ledger, but for what this chain's own hot line still cites.

use crate::chronometry::*;

/// One act with nothing named.
fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [0; 4], Vec::new(), [9; 32], parents).expect("appends").clone()
}

fn cite(r: &KStepReceipt) -> Parent {
    Parent { at: r.stamp, hash: r.chain_hash }
}

/// Close the open orbit of `c`.
fn close(c: &mut ReceiptChain) -> KStepReceipt {
    let (_, frame) = c.open_orbit().expect("an orbit is open");
    c.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close").clone()
}

/// Fold `c` at its last close and seal the `Fold` node citing the root.
fn fold(c: &mut ReceiptChain, at: Place) -> KStepReceipt {
    let (_, root) = c.fold_before(at).expect("the orbit folds");
    let cites = vec![Cite::Root { at: Stamp::new(root.height, c.vk(), root.at), hash: root.hash }];
    c.append(ActKind::Fold, [0; 4], cites, [9; 32], Vec::new()).expect("the fold node").clone()
}

/// A PEER'S FOLD SHEDS ITS LINE HERE, and our own citations hold what they
/// cite: the peer's receipts at or below its root leave, the one our hot
/// line cites stays until our own fold lets it go, and the signer's line
/// keeps the head admission reads.
#[test]
fn a_peer_fold_sheds_its_line_but_what_our_hot_line_cites() {
    let mut e = ReceiptChain::with_vk([4; 32]);
    let e0 = step(&mut e, Vec::new());
    let e1 = step(&mut e, Vec::new());
    let shut = close(&mut e);
    let folded = fold(&mut e, shut.stamp.place());

    let mut c = ReceiptChain::with_vk([1; 32]);
    for r in [&e0, &e1, &shut] {
        c.admit(r.clone());
    }
    let mine = step(&mut c, vec![cite(&e1)]);
    assert_eq!(c.shed_folded(|_| false), 0, "no fold of the peer is held: nothing leaves");

    c.admit(folded.clone());
    assert_eq!(c.shed_folded(|_| false), 2, "the peer's act and close below its root leave");
    assert!(c.node(&e0.stamp).is_none() && c.node(&shut.stamp).is_none());
    assert!(c.node(&e1.stamp).is_some(), "our hot line cites it, so it stays");
    assert_eq!(c.line_head(&[4; 32]).map(|r| r.stamp), Some(folded.stamp), "the signer's head is its fold node");
    assert!(c.verify_chain_integrity(), "our line re-derives through what stayed");

    // THE INDEX IS DERIVED: a restored ledger reads the peer's root off its
    // fold node again, and what stayed stays.
    let tape = c.line_image().expect("writes");
    let (mut restored, _) = ReceiptChain::from_line(&tape).expect("restores whole");
    assert_eq!(restored.shed_folded(|_| false), 0, "nothing more leaves on restore");
    assert!(restored.node(&e1.stamp).is_some());

    // OUR OWN FOLD LETS IT GO: once the act citing it is cold, nothing hot
    // reads through it.
    let ours = close(&mut c);
    step(&mut c, Vec::new());
    c.fold_before(ours.stamp.place()).expect("our orbit folds");
    assert!(c.node(&mine.stamp).is_none());
    assert_eq!(c.shed_folded(|r| r.stamp == e1.stamp), 0, "a node the caller keeps stays");
    assert_eq!(c.shed_folded(|_| false), 1, "and leaves when nothing keeps it");
    assert!(c.verify_chain_integrity());
}

/// H2 (2026-10-03) — A SHED RECEIPT IS A HOLE BELOW ITS SIGNER'S HEAD THAT NO
/// BACK-FILL CROSSES. Once a peer's fold is held and its line shed, a receipt
/// below the fold root is gone while the signer's head stands past it; the
/// peer root names the place, and the very receipt, fetched back, is read
/// STALE by admission — at or below a head nothing is admitted. A third
/// court's frame citing that receipt as its parent can never be crossed:
/// the mesh's split on the fleet, as the ledger reads it.
#[test]
fn a_shed_receipt_is_a_hole_below_the_head_that_backfill_cannot_cross() {
    let mut e = ReceiptChain::with_vk([4; 32]);
    let e0 = step(&mut e, Vec::new());
    step(&mut e, Vec::new());
    let shut = close(&mut e);
    let folded = fold(&mut e, shut.stamp.place());
    let mut c = ReceiptChain::with_vk([1; 32]);
    for r in e.line_between(&[4; 32], None, folded.stamp.place(), 64) {
        c.admit(r);
    }
    for r in [&e0, &shut, &folded] {
        c.admit(r.clone());
    }
    assert!(c.shed_folded(|_| false) > 0);
    assert_eq!(c.peer_root(&[4; 32]), Some(shut.stamp.place()), "the peer's fold root, held");
    assert!(c.at(&[4; 32], e0.stamp.place()).is_none(), "the cited receipt is gone");
    let head = c.line_head(&[4; 32]).expect("the signer's head").stamp.place();
    assert!(head > e0.stamp.place(), "while the head stands past it: a hole below the head");
    assert_eq!(
        crate::court::admission::admit_backfilled(&e0, &c),
        crate::court::admission::Admitted::Stale,
        "fetched back, it is read stale: no back-fill crosses it"
    );
}
