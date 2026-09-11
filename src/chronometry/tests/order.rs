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
    let mut d = ReceiptChain::with_vk([3; 32]);
    d.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).unwrap();
    let mut e = ReceiptChain::with_vk([4; 32]);
    e.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).unwrap();
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

/// An unmeasured frontier is refused, never shortened: a budget with
/// nothing left answers `None` and the caller holds the pair as concurrent
/// (45.3) instead of ordering it on a short read.
///
/// AND THE COST OF ASKING DOES NOT GROW WITH THE CHAIN. It did: the
/// frontier was re-walked over the whole ancestry per question, one spend
/// per visit, so ordering two receipts cost the chain's length — and a
/// four-receipt chain under `Budget { remaining: 3 }` refused an order
/// that plainly held, leaving supersession on the dialect book to depend
/// on how much memory the machine had free. A frontier is carried state;
/// reading one is a keyed read, and two of them order any pair.
#[test]
fn ordering_costs_one_read_per_frontier_and_an_empty_budget_refuses() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    for i in 1..=64u8 {
        c.append(ActKind::List, [0; 4], [i], [9; 32], Vec::new()).unwrap();
    }
    let (r0, rn) = (&c.receipts()[0], &c.receipts()[63]);
    let mut two = budget(2);
    assert_eq!(
        c.precedes(r0, rn, &mut two),
        Some(true),
        "sixty-four receipts deep and two reads answer it"
    );
    let mut two = budget(2);
    assert_eq!(c.precedes(rn, r0, &mut two), Some(false), "and the other way");
    let mut one = budget(1);
    assert_eq!(c.precedes(r0, rn, &mut one), None, "unmeasured, not false");
    let mut one = budget(1);
    assert!(c.frontier(&mut one).is_some(), "the head reads its own");
    let mut none = budget(0);
    assert!(c.frontier(&mut none).is_none(), "nothing left is unmeasured");
}

#[test]
fn frontier_visits_each_receipt_once() {
    let mut c = ReceiptChain::with_vk([7; 32]);
    c.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).unwrap();
    let mut b = budget(8);
    let f = c.frontier(&mut b).expect("measured");
    assert_eq!(f.get(&[7; 32]).copied(), Some(1), "greatest K of this vk");
    assert_eq!(f.len(), 1, "one vk, visited once");
}
