// 45, 78.24 — the ledger is the temporal authority, and a single changed
// byte of a node breaks the continuity law.

use crate::chronometry::*;

#[test]
fn the_ledger_is_temporal_authority_and_tamper_evident() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    assert_eq!(c.next_step_index(), 0, "genesis K = 0");
    c.append(ActKind::List, [1, 0, 0, 0], Vec::new(), [9; 32], Vec::new()).expect("genesis");
    c.append(ActKind::List, [2, 0, 0, 0], Vec::new(), [9; 32], Vec::new()).expect("second");
    let r = c.append(ActKind::List, [3, 0, 0, 0], Vec::new(), [8; 32], Vec::new()).expect("third").clone();
    assert_eq!((r.stamp.k, r.stamp.i), (0, 2), "three acts of one orbit: K the orbit, i the act's place");
    assert_eq!(r.print(), "2:3:0:0:0:0.2", "the stamp prints h:S:G:T:H:K.i");
    assert_eq!(c.head_k(), 0, "head_k is the head's orbit");
    let (_, frame) = c.open_orbit().expect("open");
    c.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close");
    assert_eq!(c.next_step_index(), 1, "K is the algebraic clock: it moved at the close");
    assert!(c.verify_chain_integrity());
    let mut tape = Vec::new();
    c.write(&mut tape).expect("the ledger writes");
    // The frame of the middle node, one lane of it changed: 32 vk, a root
    // tag, the count, then the genesis node whole, then the second's
    // stamp and kind.
    let mut first = Vec::new();
    c.at_place(crate::chronometry::Place::GENESIS).expect("genesis").write(&mut first).expect("writes");
    let lane = 32 + 1 + 8 + first.len() + Stamp::LEN + 1;
    let mut bad = tape.clone();
    bad[lane] ^= 1;
    assert!(ReceiptChain::read(&bad, &mut 0).is_none(), "a single byte of one node refuses the ledger whole");
    assert!(ReceiptChain::read(&tape, &mut 0).is_some(), "and the ledger as written reads");
}

/// 45.1, S13.6, 78.24 — THE NODE CARRIES ITS ACT'S KIND: on the tape after
/// its stamp, in the chain hash, read back as written; a kind changed no
/// longer re-derives, and a byte that names no kind refuses the read.
#[test]
fn the_node_carries_its_kind_on_the_tape_and_in_the_hash() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [0; 4], Vec::new(), [9; 32], Vec::new()).expect("genesis");
    let r = c.append(ActKind::Install, [0; 4], Vec::new(), [9; 32], Vec::new()).expect("second").clone();
    assert_eq!(r.kind, ActKind::Install);
    let mut tape = Vec::new();
    r.write(&mut tape).expect("writes");
    let mut p = 0usize;
    let back = KStepReceipt::read(&tape, &mut p).expect("reads back");
    assert_eq!(back, r, "the kind rides the tape");
    assert_eq!(p, tape.len(), "and the read ends where the tape does");
    let mut other = r.clone();
    other.kind = ActKind::Ruling;
    assert!(!other.rederives(), "the kind is in the chain hash");
    let mut moved = r.clone();
    moved.stamp.vk = [2; 32];
    assert!(!moved.rederives(), "and so is the signer: the stamp is the node's key");
    let at = Stamp::LEN;
    assert_eq!(tape[at], ActKind::Install.to_byte(), "the kind byte follows the stamp");
    let mut bad = tape.clone();
    bad[at] = 0xEE;
    assert!(KStepReceipt::read(&bad, &mut 0).is_none(), "a byte naming no kind refuses the read");
    for k in [ActKind::Attest, ActKind::Fold, ActKind::Adopted, ActKind::Learned, ActKind::Settled] {
        assert_eq!(ActKind::from_byte(k.to_byte()), Some(k), "every kind round-trips its byte");
    }
}

/// 78.24 — A NODE NAMES WHAT IT ACTS ON BY COORDINATES: a word's chart and
/// admitting stamp, a fold's root; each rides the tape and the hash.
#[test]
fn a_nodes_citations_ride_the_tape_and_the_hash() {
    let at = Stamp { h: 4, vk: [3; 32], k: 9, i: 2 };
    let word = Cite::Word { chart: crate::transducer::stalk::Invariant::counts(&[1, 2]), class: None, at };
    let root = Cite::Root { at, hash: [5; 32] };
    let mut c = ReceiptChain::with_vk([1; 32]);
    let r = c.append(ActKind::Ruling, [0; 4], vec![word.clone(), root.clone()], [9; 32], Vec::new()).expect("seals").clone();
    let mut tape = Vec::new();
    r.write(&mut tape).expect("writes");
    let back = KStepReceipt::read(&tape, &mut 0).expect("reads");
    assert_eq!(back.cites, vec![word, root], "the citations come back as written");
    let mut other = r.clone();
    other.cites.pop();
    assert!(!other.rederives(), "the citations are in the chain hash");
}
