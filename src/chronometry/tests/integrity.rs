// 45 — the chain is the temporal authority, and a single changed byte
// breaks the continuity law.

use crate::chronometry::*;

#[test]
fn the_chain_is_temporal_authority_and_tamper_evident() {
    let mut c = ReceiptChain::new();
    assert_eq!(c.next_step_index(), 0, "genesis K = 0");
    c.append(ActKind::List, [1u8], [9; 32], Vec::new()).unwrap();
    c.append(ActKind::List, [2u8], [9; 32], Vec::new()).unwrap();
    let r = c.append(ActKind::List, [3u8], [8; 32], Vec::new()).unwrap();
    assert_eq!(r.step_index, 2, "K is the algebraic clock");
    assert_eq!(c.head_k(), 2, "head_k is the last receipt");
    assert_eq!(c.next_step_index(), 3, "next is one past the head");
    assert!(c.verify_chain_integrity());
    let mut t = c.clone();
    t.receipts[1].delta[0] ^= 1;
    assert!(!t.verify_chain_integrity(), "a single byte breaks the continuity law");
}

/// 45.1, S13.6 — THE RECEIPT CARRIES ITS ACT'S KIND: on the tape as one
/// byte after the parents, in the chain hash, read back as written; a kind
/// changed no longer re-derives, and a byte that names no kind refuses the
/// read.
#[test]
fn the_receipt_carries_its_kind_on_the_tape_and_in_the_hash() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [1u8], [9; 32], Vec::new()).unwrap();
    let r = c.append(ActKind::Install, [2u8], [9; 32], Vec::new()).unwrap().clone();
    assert_eq!(r.kind, ActKind::Install);
    let mut tape = Vec::new();
    r.write(&mut tape);
    let mut p = 0usize;
    let back = KStepReceipt::read(&tape, &mut p).expect("reads back");
    assert_eq!(back, r, "the kind rides the tape");
    assert_eq!(p, tape.len(), "and the read ends where the tape does");
    let mut other = r.clone();
    other.kind = ActKind::Ruling;
    assert!(!other.rederives(), "the kind is in the chain hash");
    // The kind byte sits after the parents: step, height, vk, the count.
    let at = 8 + 8 + 32 + 4;
    assert_eq!(tape[at], ActKind::Install.to_byte());
    let mut bad = tape.clone();
    bad[at] = 0xEE;
    assert!(KStepReceipt::read(&bad, &mut 0).is_none(), "a byte naming no kind refuses the read");
    for k in [ActKind::Attest, ActKind::Fold, ActKind::Adopted, ActKind::Learned, ActKind::Settled] {
        assert_eq!(ActKind::from_byte(k.to_byte()), Some(k), "every kind round-trips its byte");
    }
}
