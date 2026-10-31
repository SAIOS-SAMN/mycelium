// 45, 78.24 — the ledger is the temporal authority, and a single changed
// byte of a node breaks the continuity law.

use crate::chronometry::*;

/// The ledger is the temporal authority and tamper-evident: K moves only at an orbit's close, and any changed byte of its line is refused or read as torn.
#[test]
fn receipt_chain_clocks_k_and_refuses_changed_bytes() {
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
    let tape = c.line_image().expect("the line writes");
    // EVERY SINGLE BYTE CHANGED, anywhere in the file: the line is refused,
    // or reads as a write that did not land — never as another chain. A
    // node's hash is re-derived from what it carries and its line's anchor
    // must meet it.
    for at in 0..tape.len() {
        let mut bad = tape.clone();
        bad[at] ^= 1;
        assert!(
            ReceiptChain::from_line(&bad).is_none_or(|(_, torn)| torn),
            "byte {at} of {} changed reads as a whole line",
            tape.len()
        );
    }
    assert!(ReceiptChain::from_line(&tape).is_some_and(|(_, torn)| !torn), "and the ledger as written reads");
}

/// 45.1, S13.6, 78.24 — THE NODE CARRIES ITS ACT'S KIND: on the tape after
/// its stamp, in the chain hash, read back as written; a kind changed no
/// longer re-derives, and a byte that names no kind refuses the read.
#[test]
fn act_kind_rides_tape_and_chain_hash() {
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
    other.kind = ActKind::Ruling(Ruled::Upheld);
    assert!(!other.rederives(), "the kind is in the chain hash");
    // EC1: the ruling's arm is its kind byte, so a ruling re-armed is
    // another node, and every arm reads back as itself.
    for arm in Ruled::ALL {
        let byte = ActKind::Ruling(arm).to_byte();
        assert_eq!(ActKind::from_byte(byte), Some(ActKind::Ruling(arm)), "{} rides its own byte", arm.name());
    }
    let upheld = KStepReceipt::seal(
        r.stamp,
        ActKind::Ruling(Ruled::Upheld),
        r.mu,
        r.prev,
        r.parents.clone(),
        r.cites.clone(),
        r.operator_hash,
    )
    .expect("seals");
    assert!(upheld.rederives(), "an upheld ruling sealed whole");
    let mut slashed = upheld.clone();
    slashed.kind = ActKind::Ruling(Ruled::Slashed);
    assert!(!slashed.rederives(), "the arm is in the chain hash: a ruling cannot be re-armed in flight");
    let mut moved = r.clone();
    moved.stamp.vk = [2; 32];
    assert!(!moved.rederives(), "and so is the signer: the stamp is the node's key");
    let at = Stamp::LEN;
    assert_eq!(tape[at], ActKind::Install.to_byte(), "the kind byte follows the stamp");
    let mut bad = tape.clone();
    bad[at] = 0xEE;
    assert!(KStepReceipt::read(&bad, &mut 0).is_none(), "a byte naming no kind refuses the read");
    for k in [
        ActKind::Attest,
        ActKind::Fold,
        ActKind::Adopted,
        ActKind::Settled,
        ActKind::Refused,
        ActKind::Return,
        ActKind::RankStep,
        ActKind::Reconnect,
    ] {
        assert_eq!(ActKind::from_byte(k.to_byte()), Some(k), "every kind round-trips its byte");
    }
    // 78.94 — a retired kind and an action no court seals name no node.
    for k in [ActKind::Learned, ActKind::Why, ActKind::Speech] {
        assert_eq!(ActKind::from_byte(k.to_byte()), None, "{} is sealed by no court", String::from_utf8_lossy(k.name()));
    }
}

/// 78.24 — A NODE NAMES WHAT IT ACTS ON BY COORDINATES: a word's chart and
/// admitting stamp, a fold's root; each rides the tape and the hash.
#[test]
fn cites_ride_tape_and_chain_hash() {
    let at = Stamp { h: 4, vk: [3; 32], k: 9, i: 2 };
    let word = Cite::Word { chart: crate::transducer::stalk::Invariant::counts(&[1, 2]), class: None, at };
    let root = Cite::Root { at, hash: [5; 32] };
    let mut c = ReceiptChain::with_vk([1; 32]);
    let r = c
        .append(ActKind::Ruling(Ruled::Retired), [0; 4], vec![word.clone(), root.clone()], [9; 32], Vec::new())
        .expect("seals")
        .clone();
    let mut tape = Vec::new();
    r.write(&mut tape).expect("writes");
    let back = KStepReceipt::read(&tape, &mut 0).expect("reads");
    assert_eq!(back.cites, vec![word, root], "the citations come back as written");
    let mut other = r.clone();
    other.cites.pop();
    assert!(!other.rederives(), "the citations are in the chain hash");
}
