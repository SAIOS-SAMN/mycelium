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
// A second voice: the first has its one claim in flight on this lane
// (§3.1 as ruled, S13.15), and the act here only needs the mind to move.
k.ingest(&a, &[(0, 0, 5), (0, 1, 3)], [2; 32]);
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

let key = super::common::court_key("peer");
let mut peer = crate::chronometry::ReceiptChain::with_vk(key.vk_bytes());
peer.append(crate::chronometry::ActKind::List, b"p0", [0; 32], Vec::new()).expect("appends");
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
    k.admit_receipt(([7; 32], 2), super::common::verified(&pre, &key), &pr),
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

/// A court with one provisional word standing, ruled by two voices. The
/// fixture 45.1's four acts each need, built through the constructors the
/// production code exports (§3.4).
fn docketed(me: [u8; 32]) -> (Kernel, crate::transducer::address::Address) {
    let mut k = Kernel::new();
    k.bind_identity(me);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke(super::common::one_stroke_transport);
    let addr = k.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    (k, crate::transducer::address::Address::from(addr))
}

/// 45.1: A RULING IS AN ADMITTED ACT, so it is a step of this court's own
/// clock and a receipt in its chain. It sealed none: the book admitted the
/// word to the ruled cover, or evaporated its award, or excised it as a
/// cavity, at no position in the chain that is supposed to record it.
#[test]
fn a_ruling_seals_a_receipt_at_the_word_it_ruled() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let (mut k, addr) = docketed([0x5a; 32]);
    let before = k.k_step();
    assert!(k.apply_verdict(&addr, RULING_UPHELD), "the ruling lands");
    assert_eq!(k.k_step(), before + 1, "a ruling is one step of the court's clock");
    assert_eq!(
        k.chain().receipts().last().map(|r| r.delta.clone()),
        Some(addr.clone().into_vec()),
        "and the receipt names the word it ruled on"
    );
}

/// And a ruling that rules NOTHING seals nothing: both refusals return
/// before the provisional is removed, which is the one point past which
/// the book has moved on every path.
#[test]
fn a_ruling_that_rules_nothing_seals_nothing() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let (mut k, addr) = docketed([0x5a; 32]);
    assert!(k.apply_verdict(&addr, RULING_UPHELD), "the first lands");
    let held = k.k_step();
    assert!(!k.apply_verdict(&addr, RULING_UPHELD), "a verdict lands once");
    assert!(!k.apply_verdict(&addr, 0xfe), "an unknown ruling rules nothing");
    assert_eq!(k.k_step(), held, "neither moved the clock");
}

/// 45.1: a slash is a ruling on the other arm, and seals its own receipt.
#[test]
fn a_slash_seals_a_receipt() {
    use crate::anastome::dialogue::RULING_SLASHED;
    let (mut k, addr) = docketed([0x5b; 32]);
    let before = k.k_step();
    k.apply_verdict(&addr, RULING_SLASHED);
    assert_eq!(k.k_step(), before + 1, "a slash is one step too");
}

/// 45.1: "a syllabus page served" is one of the acts the chapter names.
/// The door wrote `receipt K … h …` beside every page it served — a
/// receipt nothing sealed, so the K it named was whatever the last
/// unrelated act had left there.
#[test]
fn a_syllabus_page_served_seals_a_receipt() {
    let (mut k, addr) = docketed([0x5a; 32]);
    let before = k.k_step();
    k.note_served(&addr, [3; 32]);
    assert_eq!(k.k_step(), before + 1, "a page served is one step");
}

/// 45.1, 47.3: an install on a second vantage seals a receipt, at the
/// installed address. A book that cannot say where an install stands
/// cannot re-derive it.
#[test]
fn an_install_seals_a_receipt_at_the_installed_address() {
    let (mut k, addr) = docketed([0x5c; 32]);
    let before = k.k_step();
    k.home_install(&addr, [0xbb; 32]);
    assert_eq!(k.k_step(), before + 1, "an install is one step");
    assert!(
        k.chain().receipts().last().is_some_and(|r| r.delta == addr.as_bytes()),
        "sealed at the installed address"
    );
}

/// 45.1, 47.5: a lapsed citation releases an escrow and unlists a draw, so
/// it is this court's own act. And a lapse measures its aperture in
/// receipts, so a court whose lapses were not receipted stopped advancing
/// the very quantity it lapses on.
#[test]
fn a_lapsed_citation_seals_a_receipt() {
    let mut k = Kernel::new();
    k.bind_identity([0x5d; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let laddr = k.pending.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    k.standing_h.insert(&laddr, 0);
    k.set_height(crate::physics::aperture_cycle() as u64 + 1);
    let before = k.k_step();
    k.lapse_docket(
        &mut crate::physics::Physics::default().stroke(),
        super::common::one_stroke_transport,
    );
    assert_eq!(k.k_step(), before + 1, "a lapsed citation is one step");
    assert!(
        k.chain().receipts().last().is_some_and(|r| r.delta == laddr),
        "sealed at the address whose draw was released"
    );
}

/// 47.5: Λ is `from_transport(rtt_floor, aperture_cycle())`, "never a
/// constant". A home this node has measured no round trip to has no Λ, so
/// nothing lapses against it — an unmeasured transport is not a fast one
/// (rule 11), and `transport_deadline` says the same of an unmeasured RTT.
/// The lapse read `aperture_cycle()` alone, so a citation waited a number
/// that had nothing to do with the home it waits on.
#[test]
fn a_citation_does_not_lapse_against_an_unmeasured_transport() {
    let mut k = Kernel::new();
    k.bind_identity([0x5d; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let laddr = k.pending.fiber(crate::network::nerve::ROOT).next().unwrap().0;
    k.standing_h.insert(&laddr, 0);
    // Standing far past any aperture: the extent is not what holds it.
    k.set_height(crate::physics::aperture_cycle() as u64 * 4);
    let before = k.k_step();
    k.lapse_docket(
        &mut crate::physics::Physics::default().stroke(),
        super::common::unmeasured_transport,
    );
    assert_eq!(k.k_step(), before, "no lapse, so no receipt");
    assert!(k.pending.get(&laddr).is_some(), "and the listing stands");
}
