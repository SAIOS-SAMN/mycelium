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
    .map(|(_, row)| row.gauge.clone())
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
    .map(|(_, row)| row.gauge.clone())
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

/// 45.1: A RECEIPT FOR EVERY ADMITTED ACT. A ruling, an install on a
/// second vantage, a syllabus page served and a lapsed citation each move
/// this court's book, and each is a step of its own clock. None of them
/// sealed one: the book grew ruled words, released escrows and served
/// pages at no position in the chain that is supposed to record them, and
/// a reader replaying the chain found the acts nowhere in it.
#[test]
fn every_act_that_moves_the_book_seals_a_receipt() {
    use crate::anastome::dialogue::{RULING_SLASHED, RULING_UPHELD};

    // A RULING. Two voices docket a percept; the court rules on it.
    let mut k = Kernel::new();
    k.bind_identity([0x5a; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke();
    let addr = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    let before = k.k_step();
    assert!(k.apply_verdict(&addr, RULING_UPHELD), "the ruling lands");
    let after = k.k_step();
    assert_eq!(after, before + 1, "a ruling is one step of the court's clock");
    assert_eq!(
        k.chain().receipts().last().map(|r| r.delta.clone()),
        Some(addr.clone()),
        "and the receipt names the word it ruled on"
    );

    // A RULING THAT RULES NOTHING SEALS NOTHING: the same address twice,
    // and an unknown ruling byte, both refuse before the book moves.
    let held = k.k_step();
    assert!(!k.apply_verdict(&addr, RULING_UPHELD), "a verdict lands once");
    assert!(!k.apply_verdict(&addr, 0xfe), "an unknown ruling rules nothing");
    assert_eq!(k.k_step(), held, "neither moved the clock");

    // A SYLLABUS PAGE SERVED.
    let before = k.k_step();
    k.note_served(&addr, [3; 32]);
    assert_eq!(k.k_step(), before + 1, "a page served is one step");

    // A SLASH, which is a ruling on the other arm.
    let mut k2 = Kernel::new();
    k2.bind_identity([0x5b; 32]);
    assert_eq!(k2.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k2.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k2.court_stroke();
    let saddr = k2.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    let before = k2.k_step();
    k2.apply_verdict(&saddr, RULING_SLASHED);
    assert_eq!(k2.k_step(), before + 1, "a slash is one step too");

    // AN INSTALL ON A SECOND VANTAGE, and A LAPSE of what is left standing.
    let mut k3 = Kernel::new();
    k3.bind_identity([0x5c; 32]);
    assert_eq!(k3.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k3.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k3.court_stroke();
    let iaddr = k3.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    let before = k3.k_step();
    k3.home_install(&iaddr, [0xbb; 32]);
    assert_eq!(k3.k_step(), before + 1, "an install is one step");
    assert!(
        k3.chain().receipts().last().is_some_and(|r| r.delta == iaddr),
        "sealed at the installed address"
    );

    // A LAPSE. A citation standing an aperture of receipts behind the head
    // releases its draw (47.5); the release is this court's own act.
    let mut k4 = Kernel::new();
    k4.bind_identity([0x5d; 32]);
    assert_eq!(k4.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let laddr = k4.pending.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    k4.standing_h.insert(&laddr, 0);
    k4.set_height(crate::physics::aperture_cycle() as u64 + 1);
    let before = k4.k_step();
    k4.lapse_docket();
    assert_eq!(k4.k_step(), before + 1, "a lapsed citation is one step");
    assert!(
        k4.chain().receipts().last().is_some_and(|r| r.delta == laddr),
        "sealed at the address whose draw was released"
    );
}
