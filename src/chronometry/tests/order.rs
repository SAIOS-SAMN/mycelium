// 45.3, T15 — the order two receipts stand in, read off their frontiers,
// and what an exhausted walk answers.

use super::common::budget;
use crate::chronometry::*;

#[test]
fn precedes_implies_lower_height_and_concurrency_is_kept() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [0; 4], [3u8], [9; 32], Vec::new()).unwrap();
    let r0 = &c.receipts()[0];
    let r1 = &c.receipts()[1];
    let r2 = &c.receipts()[2];
    // Each pair pays for the two frontiers it reads, and no more.
    let mut b = budget(8);
    assert_eq!(c.precedes(r0, r1, &mut b), Some(true), "earlier ≺ later");
    let mut b = budget(8);
    assert_eq!(c.precedes(r1, r2, &mut b), Some(true));
    let mut b = budget(8);
    assert_eq!(
        c.precedes(r2, r0, &mut b),
        Some(false),
        "later does not precede earlier"
    );
    assert!(r0.height < r1.height && r1.height < r2.height);
    // TWO GENESES OF TWO SIGNERS MUST DIFFER IN CONTENT to be two
    // receipts: the signer is written on a receipt and is NOT in its chain
    // hash (`bind_vk` says so), so two chains that append the same act
    // with the same delta seal the same hash and are the same receipt.
    // Both appended `[1u8]` here, so "two geneses are concurrent" was one
    // genesis compared with itself, which is concurrent for the wrong
    // reason and would have passed against any implementation at all.
    let mut d = ReceiptChain::with_vk([3; 32]);
    d.append(ActKind::List, [0; 4], [0xd0u8], [9; 32], Vec::new()).unwrap();
    let mut e = ReceiptChain::with_vk([4; 32]);
    e.append(ActKind::List, [0; 4], [0xe0u8], [9; 32], Vec::new()).unwrap();
    let mut mix = ReceiptChain::with_vk([1; 32]);
    mix.append(ActKind::List, [0; 4], [0u8], [9; 32], Vec::new()).unwrap();
    mix.admit(d.receipts()[0].clone());
    mix.admit(e.receipts()[0].clone());
    mix.append(
        ActKind::List,
        [0; 4],
        [9u8],
        [9; 32],
        vec![
            Parent {
                vk: [3; 32],
                k: 0,
                hash: d.head_hash(),
            },
            Parent {
                vk: [4; 32],
                k: 0,
                hash: e.head_hash(),
            },
        ],
    )
    .unwrap();
    let mut b = budget(8);
    assert_eq!(
        mix.precedes(&d.receipts()[0], &e.receipts()[0], &mut b),
        Some(false),
        "two geneses are concurrent"
    );
}

/// ORDERING TWO OF THIS CHAIN'S OWN RECEIPTS IS ONE READ, at any depth,
/// and a budget with nothing left refuses rather than answering short
/// (rule 11) — the caller then holds the pair as concurrent (45.3).
///
/// TWO SHAPES HAVE STOOD HERE AND BOTH WERE PLANES. First the frontier
/// was re-walked over the whole ancestry per question, one spend per
/// visit, so ordering cost the chain's LENGTH: at K=1704 against a stroke
/// of 3 it never once measured, and supersession on the dialect book was
/// decided by how much memory the machine had free. Then the frontier was
/// carried instead — a vector clock per receipt, a map as wide as the
/// mesh, receipts × PEERS on the heap and cloned on every read. Neither
/// was needed. Our own receipts are a line; the prev link is the order.
#[test]
fn ordering_our_own_is_one_read_at_any_depth_and_an_empty_budget_refuses() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    for i in 1..=64u8 {
        c.append(ActKind::List, [0; 4], [i], [9; 32], Vec::new()).unwrap();
    }
    let (r0, rn) = (&c.receipts()[0], &c.receipts()[63]);
    let mut one = budget(1);
    assert_eq!(
        c.precedes(r0, rn, &mut one),
        Some(true),
        "sixty-four receipts deep and one read answers it"
    );
    let mut one = budget(1);
    assert_eq!(c.precedes(rn, r0, &mut one), Some(false), "and the other way");
    let mut one = budget(1);
    assert_eq!(c.precedes(r0, r0, &mut one), Some(false), "nothing precedes itself");
    let mut none = budget(0);
    assert_eq!(c.precedes(r0, rn, &mut none), None, "unmeasured, not false");
}

/// A PEER RECEIPT CUTS OUR LINE IN TWO PLACES and nowhere else: the last
/// of ours it reaches, and the first of ours that reaches it. Every order
/// question against it is one of those two integers compared, and a peer
/// that cuts our line nowhere is concurrent with all of it.
#[test]
fn a_peer_receipt_is_placed_by_where_it_cuts_our_own_line() {
    // Every chain here appends distinct content: the signer is not in the
    // chain hash, so two chains sealing the same act seal one receipt.
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [0; 4], [0xc0u8], [9; 32], Vec::new()).unwrap();
    let mine0 = c.receipts()[0].clone();

    // A PEER THAT REACHES OUR FIRST RECEIPT: it cites it as a parent, so
    // everything of ours at or below that index precedes it.
    let mut d = ReceiptChain::with_vk([3; 32]);
    d.append(ActKind::List, [0; 4], [0xd0u8], [9; 32], Vec::new()).unwrap();
    d.admit(mine0.clone());
    d.append(
        ActKind::List,
        [0; 4],
        [0xd1u8],
        [9; 32],
        vec![Parent { vk: [1; 32], k: mine0.step_index, hash: mine0.chain_hash }],
    )
    .unwrap();
    let over = d.receipts()[1].clone();
    c.admit(over.clone());

    let mut b = budget(4);
    assert_eq!(
        c.precedes(&mine0, &over, &mut b),
        Some(true),
        "ours is below a peer receipt that cites it"
    );
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&over, &mine0, &mut b),
        Some(false),
        "and the peer is not below ours"
    );

    // UNTIL WE CITE IT, a peer receipt is above nothing of ours.
    let mut e = ReceiptChain::with_vk([4; 32]);
    e.append(ActKind::List, [0; 4], [0xe0u8], [9; 32], Vec::new()).unwrap();
    let apart = e.receipts()[0].clone();
    c.admit(apart.clone());
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&apart, &mine0, &mut b),
        Some(false),
        "an uncited peer receipt precedes nothing of ours"
    );
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&mine0, &apart, &mut b),
        Some(false),
        "and nothing of ours precedes it: they are concurrent"
    );
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&apart, &over, &mut b),
        Some(false),
        "two peers neither of which reaches the other are concurrent"
    );

    // ONCE WE CITE IT, every receipt of ours from that step on reaches it.
    c.append(
        ActKind::List,
        [0; 4],
        [0xc1u8],
        [9; 32],
        vec![Parent { vk: [4; 32], k: apart.step_index, hash: apart.chain_hash }],
    )
    .unwrap();
    let mine1 = c.receipts()[1].clone();
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&apart, &mine1, &mut b),
        Some(true),
        "the step that cited it is above it"
    );
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&apart, &mine0, &mut b),
        Some(false),
        "and the step before that still is not"
    );
}

/// A FOLD RENUMBERS OUR LINE (45.8), so the coordinates that cut it move
/// with it. A peer receipt reached before the fold is reached by every
/// receipt that survives it.
#[test]
fn a_fold_carries_the_peer_placements_with_the_line() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let mut e = ReceiptChain::with_vk([4; 32]);
    e.append(ActKind::List, [0; 4], [0xe0u8], [9; 32], Vec::new()).unwrap();
    let peer = e.receipts()[0].clone();
    c.append(ActKind::List, [0; 4], [0xc0u8], [9; 32], Vec::new()).unwrap();
    c.admit(peer.clone());
    c.append(
        ActKind::List,
        [0; 4],
        [0xc1u8],
        [9; 32],
        vec![Parent { vk: [4; 32], k: peer.step_index, hash: peer.chain_hash }],
    )
    .unwrap();
    c.append(ActKind::List, [0; 4], [0xc2u8], [9; 32], Vec::new()).unwrap();
    let (cold, _) = c.fold_before(1).expect("the first two fold");
    assert_eq!(cold.len(), 2, "two receipts left the hot chain");
    let survivor = c.receipts()[0].clone();
    let mut b = budget(4);
    assert_eq!(
        c.precedes(&peer, &survivor, &mut b),
        Some(true),
        "reached before the fold is reached by everything after it"
    );
}
