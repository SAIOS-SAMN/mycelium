// 47.1, 47.2 — ADMISSION: a receipt is admitted against the origin, the
// signature, the head and the height, in that order.

use crate::cognition::*;

/// 45.2 ON THE VOICE. Height is `1 + max(h(prev), parent heights)`, so a
/// mind that never admits a peer receipt never names a parent and its
/// height is its own K wearing TUniversal's name. This asserts the other
/// thing: an admitted receipt is CITED by the next step, and the height
/// that step seals is above the peer's, not above this mind's own head.
#[test]
fn an_admitted_receipt_is_cited_and_carries_height_above_the_peers() {
use crate::court::admission::{Admitted, SignedPrefix};
let origin = ([7u8; 32], 2u32);

// A peer chain that has run ahead of this mind: three steps, so its
// head sits at a height this mind's own strokes have not reached.
let mut peer = crate::chronometry::ReceiptChain::with_vk([9; 32]);
for d in [b"p0".as_slice(), b"p1", b"p2"] {
    peer.append(d, [0; 32], Vec::new()).expect("peer appends");
}

let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
k.ingest(&a, &b, [1; 32]);
let mine = k.height();
assert!(mine < peer.head_height(), "the peer is ahead: {mine} < {}", peer.head_height());

// The prefix must be walked in order: this mind holds none of the
// peer's chain, so its head alone is a GAP, and the back-fill is what
// makes the height re-derivable.
let pre = |r: &crate::chronometry::KStepReceipt| SignedPrefix {
    genesis: origin.0,
    epoch: origin.1,
    prev_receipt_hash: r.prev_receipt_hash,
    k: r.step_index,
    height: r.height,
    chart: vec![],
    payload: r.delta.clone(),
};
let head = peer.receipts()[2].clone();
assert!(
    matches!(k.admit_receipt(origin, &pre(&head), &head), Admitted::Gap { .. }),
    "a chain this mind has never held is a gap, not an admission"
);
for r in peer.receipts().iter().take(3) {
    assert!(
        matches!(k.admit_receipt(origin, &pre(r), r), Admitted::Parent(_)),
        "back-fill in order admits K {}",
        r.step_index
    );
}
assert_eq!(k.pending_parents(), 3, "three admitted, none cited yet");
assert_eq!(k.known_head(&[9; 32]), Some((2, head.chain_hash)));

// The next step cites them, and its height is above the peer's head.
let before = k.height();
let c = vec![(0, 0, 5), (0, 1, 3)];
k.ingest(&a, &c, [1; 32]);
assert_eq!(k.pending_parents(), 0, "the step consumed them");
assert!(
    k.height() > peer.head_height(),
    "height climbed on an ADMITTED receipt: {before} → {} over the peer's {}",
    k.height(),
    peer.head_height()
);
}

/// 47.2 step 2 verifies a signature under the CLAIMED key, and the claim
/// is the receipt's own `vk`. An unbound chain seals every receipt with
/// `[0u8; 32]`, so a hypha's own receipts named a signer that cannot sign:
/// the fleet refused every one on the signature and admitted nothing.
/// Binding is lossless -- the vk is written and read, never hashed -- so
/// the history keeps its chain hashes and gains its author.
#[test]
fn a_bound_mind_seals_receipts_under_the_key_that_signs_for_it() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
k.ingest(&a, &b, [1; 32]);
let before: Vec<[u8; 32]> = k.chain().receipts().iter().map(|r| r.chain_hash).collect();
assert!(!before.is_empty(), "the mind sealed receipts");
assert!(
    k.chain().receipts().iter().all(|r| r.vk == [0u8; 32]),
    "unbound, a chain names no signer"
);

k.bind_identity([0xab; 32]);
assert!(
    k.chain().receipts().iter().all(|r| r.vk == [0xab; 32]),
    "binding stamps the signer the receipts always had"
);
let after: Vec<[u8; 32]> = k.chain().receipts().iter().map(|r| r.chain_hash).collect();
assert_eq!(before, after, "the vk is not in the chain hash: history is intact");
assert!(k.chain().verify_chain_integrity(), "and the chain still verifies");

// A receipt admitted from a peer keeps ITS author, never this mind's.
let mut peer = crate::chronometry::ReceiptChain::with_vk([9; 32]);
peer.append(b"p0", [0; 32], Vec::new()).expect("appends");
let r = peer.receipts()[0].clone();
let pre = crate::court::admission::SignedPrefix {
    genesis: [7; 32],
    epoch: 2,
    prev_receipt_hash: r.prev_receipt_hash,
    k: r.step_index,
    height: r.height,
    chart: vec![],
    payload: r.delta.clone(),
};
assert!(matches!(
    k.admit_receipt(([7; 32], 2), &pre, &r),
    crate::court::admission::Admitted::Parent(_)
));
k.bind_identity([0xab; 32]);
assert_eq!(k.known_head(&[9; 32]).map(|h| h.0), Some(0), "the peer's author stands");
}

/// A receipt whose origin is not this node's touches nothing: no head is
/// remembered, no parent is queued, and the mind's next step cites none.
#[test]
fn a_foreign_origin_is_refused_before_the_mind_holds_anything() {
use crate::court::admission::{Admitted, SignedPrefix, REFUSE_ORIGIN};
let mut peer = crate::chronometry::ReceiptChain::with_vk([9; 32]);
peer.append(b"p0", [0; 32], Vec::new()).expect("appends");
let r = peer.receipts()[0].clone();
let mut k = Kernel::new();
let foreign = SignedPrefix {
    genesis: [1; 32],
    epoch: 2,
    prev_receipt_hash: r.prev_receipt_hash,
    k: r.step_index,
    height: r.height,
    chart: vec![],
    payload: r.delta.clone(),
};
assert_eq!(
    k.admit_receipt(([7; 32], 2), &foreign, &r),
    Admitted::Refused(REFUSE_ORIGIN)
);
assert_eq!(k.pending_parents(), 0);
assert!(k.known_head(&[9; 32]).is_none());
}

/// 47.4 — TWO ROOTS AT ONE K BURN THE SIGNER'S BOND. The proof is two
/// signed receipts under one key, equal K, different chain hashes; the
/// court burns the bond on all four lanes, one `burn` entry, supply down
/// by the same vector, so §6 holds through the forfeiture (R9). It was
/// DETECTED and answered `REFUSE_EQUIVOCATION`, and nothing burned: a
/// signer could tell two peers two truths, be caught by both, and pay
/// nothing. A second frame at the same K burns nothing more.
#[test]
fn two_roots_at_one_k_burn_the_signers_bond() {
use crate::court::admission::{Admitted, SignedPrefix};
let origin = ([7u8; 32], 2u32);
let liar = [9u8; 32];
let pre = |r: &crate::chronometry::KStepReceipt| SignedPrefix {
    genesis: origin.0,
    epoch: origin.1,
    prev_receipt_hash: r.prev_receipt_hash,
    k: r.step_index,
    height: r.height,
    chart: vec![],
    payload: r.delta.clone(),
};

// One signer, two chains, two different first receipts at K 0.
let mut told_a = crate::chronometry::ReceiptChain::with_vk(liar);
told_a.append(b"one", [0; 32], Vec::new()).expect("appends");
let mut told_b = crate::chronometry::ReceiptChain::with_vk(liar);
told_b.append(b"two", [0; 32], Vec::new()).expect("appends");
let (ra, rb) = (told_a.receipts()[0].clone(), told_b.receipts()[0].clone());
assert_eq!(ra.step_index, rb.step_index, "one K");
assert_ne!(ra.chain_hash, rb.chain_hash, "two roots");

let mut k = Kernel::new();
let bond = [3u64, 3, 3, 3];
k.treasury_mut().genesis(liar, bond, (0, [0; 32]));
k.treasury_mut().enroll(liar, bond, (0, [0; 32]));
assert_eq!(k.treasury().bond_of(&liar), Some(bond), "the liar is bonded");
let supply = k.treasury().supply().expect("measured");

assert!(matches!(k.admit_receipt(origin, &pre(&ra), &ra), Admitted::Parent(_)));
assert!(matches!(
    k.admit_receipt(origin, &pre(&rb), &rb),
    Admitted::Equivocation { .. }
));
assert_eq!(k.treasury().bond_of(&liar), Some([0; 4]), "the bond is burned whole");
assert_eq!(
    k.treasury().supply().expect("still measured"),
    std::array::from_fn(|l| supply[l] - bond[l]),
    "supply falls by exactly what left circulation"
);
assert!(k.treasury().audit().is_ok(), "and §6 holds through the forfeiture");

// The same lie again costs nothing more: there is no bond left to take.
let after = k.treasury().supply();
assert!(matches!(
    k.admit_receipt(origin, &pre(&rb), &rb),
    Admitted::Equivocation { .. }
));
assert_eq!(k.treasury().supply(), after, "a second frame burns nothing");
}
