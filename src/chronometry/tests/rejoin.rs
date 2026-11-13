// #273, MU-ENGINEERING §8 — the rejoin: a restarted chain takes back the
// receipts of its own line a peer holds above its head, and nothing else.

use crate::chronometry::*;
use crate::court::admission::Reconstructed;

fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [3, 0, 1, 7], Vec::new(), [9; 32], parents).expect("appends")
}

fn rec(r: &KStepReceipt) -> Reconstructed<KStepReceipt> {
    Reconstructed::receipt(r).expect("re-derives")
}

/// A CHAIN THAT LOST ITS NEWEST RECEIPTS takes them back in line order and
/// stands at the head it had signed: the next act is signed above them, so
/// no place is signed twice.
#[test]
fn rejoin_takes_back_lost_own_receipts_and_signs_above_them() {
    let signer = [0x21u8; 32];
    let mut whole = ReceiptChain::with_signer(signer);
    let mut kept = Vec::new();
    let mut lost = Vec::new();
    for n in 0..5 {
        let r = step(&mut whole, Vec::new());
        match n < 2 {
            true => kept.push(r),
            false => lost.push(r),
        }
    }
    // The restart restores the line as its disk last held it.
    let mut restored = ReceiptChain::from_nodes(None, signer, kept).expect("the kept line settles");
    assert!(restored.head_place() < whole.head_place(), "the restart restored an older head");
    for r in &lost {
        assert_eq!(restored.rejoin(rec(r)), Ok(()), "own receipt at {:?} continues the head", r.stamp.place());
    }
    assert_eq!(restored.head_place(), whole.head_place(), "the head it had signed");
    let next = step(&mut restored, Vec::new());
    assert!(lost.iter().all(|r| r.stamp.place() != next.stamp.place()), "no place signed twice");
    assert_eq!(next.stamp.place(), step(&mut whole, Vec::new()).stamp.place(), "signs exactly above");
}

/// What is not this line's continuation is refused by name and not held: a
/// peer's receipt, a receipt held already, and one past a gap.
#[test]
fn rejoin_refuses_foreign_held_and_discontinuous_receipts() {
    let signer = [0x22u8; 32];
    let mut whole = ReceiptChain::with_signer(signer);
    let a = step(&mut whole, Vec::new());
    let _b = step(&mut whole, Vec::new());
    let c = step(&mut whole, Vec::new());
    let mut restored = ReceiptChain::from_nodes(None, signer, vec![a.clone()]).expect("settles");
    let mut peer = ReceiptChain::with_signer([0x23u8; 32]);
    let theirs = step(&mut peer, Vec::new());
    assert_eq!(restored.rejoin(rec(&theirs)), Err(RejoinRefusal::NotOwn));
    assert_eq!(restored.rejoin(rec(&a)), Err(RejoinRefusal::Held));
    assert_eq!(restored.rejoin(rec(&c)), Err(RejoinRefusal::Discontinuous), "c follows b, which is not held");
    assert_eq!(restored.head_place(), Some(a.stamp.place()), "nothing refused was held");
}
