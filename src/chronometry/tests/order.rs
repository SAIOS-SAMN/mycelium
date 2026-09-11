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
    // Each pair pays for the two walks it reaches: 1+2, 2+3, 3+1 visits.
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

/// An exhausted walk is unmeasured, never "does not precede": a budget
/// that cannot reach the second frontier refuses, and the caller holds
/// the pair as concurrent (45.3) instead of ordering it on a short read.
#[test]
fn a_frontier_the_budget_cannot_finish_is_refused_not_shortened() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [0; 4], [3u8], [9; 32], Vec::new()).unwrap();
    let (r0, r2) = (&c.receipts()[0], &c.receipts()[2]);
    // r0 costs one visit, r2 three: one unit reaches neither pair.
    let mut short = budget(1);
    assert_eq!(c.precedes(r0, r2, &mut short), None, "unmeasured, not false");
    let mut short = budget(2);
    assert!(c.frontier(&mut short).is_none(), "the head walk is three deep");
    let mut enough = budget(3);
    assert!(c.frontier(&mut enough).is_some(), "measured at its own size");
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
