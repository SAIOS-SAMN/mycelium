// 45.2, 45.3 — THE LEDGER IS THE CLOCK: causal order is partial, and an
// extent is replayed from the chain that holds it.

use crate::cognition::*;

/// A2 / 5.2b. Supersession on the dialect book is CAUSAL ORDER (45.3), not
/// a height scalar. `push_cocycle` compared `tu < old_tu`, and two heights
/// from two organs are exactly the quantity 45.3 says is partial: two
/// transitions that are concurrent were ordered by that comparison and one
/// was silently discarded (W1, and rule 15's "when unsure, hold").
///
/// A transition that demonstrably follows the incumbent supersedes it. One
/// that is merely CONCURRENT with it — neither receipt's frontier below the
/// other's — is kept on the chain beside it, and nothing is chosen.
#[test]
fn cocycle_supersedes_by_causal_order_and_keeps_concurrent() {
use crate::transducer::alphabet::{LetterId, Word};
let inv = crate::transducer::stalk::Invariant(vec![1]);
// Two words on the sensor plane: one quarter turn and two. Both keep
// the plane closed, so both lift to versors, and they are not the same
// projective element.
let vsr = |n: usize| {
    let q = LetterId::quarter((0, 1)).unwrap();
    let w = Word::from_letters(&vec![(q, false); n]);
    (rotor_versor(&w.rotor()).expect("versor"), w.rotor().class_key().expect("ck"))
};
let (v1, ck1) = vsr(1);
let (v2, ck2) = vsr(2);
assert!(!crate::transducer::element::same_versor(&v1, &v2), "two transitions");

// ORDERED: both stand at receipts on this mind's own chain, so the
// second's frontier dominates the first's and it supersedes.
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
k.ingest(&a, &b, [1; 32]);
let r0 = k.chain().head_hash();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v1.clone(), ck1.clone(), r0);
k.ingest(&a, &[(0, 0, 5), (0, 1, 3)], [1; 32]);
let r1 = k.chain().head_hash();
assert_ne!(r0, r1, "the mind moved");
let chain_before = k.dialect_chain_len();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v2.clone(), ck2.clone(), r1);
let standing = k
    .cocycle_at
    .fiber(crate::network::nerve::ROOT)
    .next()
    .map(|(_, row)| row.0.clone())
    .expect("a row stands");
assert!(
    crate::transducer::element::same_versor(&standing, &v2),
    "the later receipt supersedes"
);
assert_eq!(k.dialect_chain_len(), chain_before + 1, "the superseded one is kept");

// CONCURRENT: the second stands at a PEER's receipt, admitted but never
// cited, so neither frontier is below the other. Nothing supersedes.
let mut k = Kernel::new();
k.ingest(&a, &b, [1; 32]);
let mine = k.chain().head_hash();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v1.clone(), ck1.clone(), mine);

let mut peer = crate::chronometry::ReceiptChain::with_vk([9; 32]);
peer.append(b"p0", [0; 32], Vec::new()).expect("appends");
let pr = peer.receipts()[0].clone();
let pre = crate::court::admission::SignedPrefix {
    genesis: [7; 32],
    epoch: 2,
    prev_receipt_hash: pr.prev_receipt_hash,
    k: pr.step_index,
    height: pr.height,
    chart: vec![],
    payload: pr.delta.clone(),
};
assert!(matches!(
    k.admit_receipt(([7; 32], 2), &pre, &pr),
    crate::court::admission::Admitted::Parent(_)
));
let chain_before = k.dialect_chain_len();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v2.clone(), ck2, pr.chain_hash);
let standing = k
    .cocycle_at
    .fiber(crate::network::nerve::ROOT)
    .next()
    .map(|(_, row)| row.0.clone())
    .expect("a row stands");
assert!(
    crate::transducer::element::same_versor(&standing, &v1),
    "concurrent: the incumbent is not displaced"
);
assert_eq!(
    k.dialect_chain_len(),
    chain_before + 1,
    "and the concurrent one is kept, not dropped"
);
}

/// Standing T is a height difference on the home chain; a witness
/// re-derives it from the receipts (T16).
#[test]
fn extent_replays_from_the_home_chain() {
let mut k = Kernel::new();
k.set_height(4);
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
k.ingest(&a, &b, [1; 32]);
let addr = k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address();
assert_eq!(k.t_extent(addr.as_bytes(), 4), 0, "extent is zero at first vantage");
k.set_height(11);
assert_eq!(
    k.t_extent(addr.as_bytes(), 11),
    7,
    "extent is h_now − h_install on the home chain"
);
let mut k2 = Kernel::new();
k2.set_height(11);
k2.standing_h.insert(&addr, 4);
assert_eq!(
    k2.t_extent(addr.as_bytes(), 11),
    7,
    "a witness re-derives the same extent from the recorded height"
);
}
