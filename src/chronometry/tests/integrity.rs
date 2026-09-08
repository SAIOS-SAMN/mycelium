// 45 — the chain is the temporal authority, and a single changed byte
// breaks the continuity law.

use crate::chronometry::*;

#[test]
fn the_chain_is_temporal_authority_and_tamper_evident() {
    let mut c = ReceiptChain::new();
    assert_eq!(c.next_step_index(), 0, "genesis K = 0");
    c.append([1u8], [9; 32], Vec::new()).unwrap();
    c.append([2u8], [9; 32], Vec::new()).unwrap();
    let r = c.append([3u8], [8; 32], Vec::new()).unwrap();
    assert_eq!(r.step_index, 2, "K is the algebraic clock");
    assert_eq!(c.head_k(), 2, "head_k is the last receipt");
    assert_eq!(c.next_step_index(), 3, "next is one past the head");
    assert!(c.verify_chain_integrity());
    let mut t = c.clone();
    t.receipts[1].delta[0] ^= 1;
    assert!(!t.verify_chain_integrity(), "a single byte breaks the continuity law");
}
