// 47.1, 47.2, 78.24 — ADMISSION: a receipt is admitted against the origin,
// the signature, the head and the height, in that order, and held once in
// the ledger at its stamp.

use crate::cognition::*;
use crate::court::admission::SignedPrefix;

const ORIGIN: ([u8; 32], u32) = ([7u8; 32], 2u32);

/// The prefix a receipt is signed over: its place, height and predecessor.
fn pre(r: &crate::chronometry::KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: ORIGIN.0,
        epoch: ORIGIN.1,
        prev_receipt_hash: r.prev_hash(),
        at: r.stamp.place(),
        height: r.stamp.h,
        chart: vec![],
        payload: r.stamp.key(),
    }
}

/// A signer's line of `n` steps, each measuring its own lane.
fn peer_line(vk: [u8; 32], n: u64) -> Vec<crate::chronometry::KStepReceipt> {
    let mut c = crate::chronometry::ReceiptChain::with_vk(vk);
    (0..n)
        .map(|i| {
            c.append(crate::chronometry::ActKind::List, [i + 1, 0, 0, 0], Vec::new(), [0; 32], Vec::new())
                .expect("peer appends")
                .clone()
        })
        .collect()
}

/// An anchor before any act: the empty stamp and hash.
fn before_genesis() -> crate::economy::Anchor {
    (crate::chronometry::Stamp::default(), [0; 32])
}

/// 45.2 ON THE VOICE. An admitted receipt is CITED by the next step, and the
/// height that step seals is above the peer's, not above this mind's head.
#[test]
fn an_admitted_receipt_is_cited_and_carries_height_above_the_peers() {
    use crate::court::admission::Admitted;
    let key = super::common::court_key("peer");
    let peer = peer_line(key.vk_bytes(), 3);
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2)];
    super::common::speak_pair(&mut k, &a, &b, [1; 32]);
    let local_h = k.height();
    let peer_h = peer[2].stamp.h;
    assert!(local_h < peer_h, "the peer is ahead: {local_h} < {peer_h}");
    let head = peer[2].clone();
    assert!(
        matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&head), &key), &head), Admitted::Gap { .. }),
        "a chain this mind has never held is a gap, not an admission"
    );
    for r in &peer {
        assert!(
            matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(r), &key), r), Admitted::Parent(_)),
            "back-fill in order admits K {}",
            r.stamp.k
        );
    }
    assert_eq!(k.pending_parents(), 3, "three admitted, none cited yet");
    assert_eq!(k.known_head(&key.vk_bytes()), Some((head.stamp.place(), head.chain_hash)));
    let c = vec![(0, 0, 5), (0, 1, 5)];
    super::common::speak_pair(&mut k, &a, &c, [2; 32]);
    assert_eq!(k.pending_parents(), 0, "the step consumed them");
    assert!(k.height() > peer_h, "height climbed on an ADMITTED receipt: {} over the peer's {peer_h}", k.height());
}

/// 78.24 — THE KEY IS A COORDINATE OF EVERY STAMP: a mind bound before its
/// first act seals every node under the key that signs for it, and a
/// ledger that has sealed under another key keeps it — the rebind is
/// refused and the line still verifies. A node admitted from a peer keeps
/// its own author.
#[test]
fn a_bound_mind_seals_under_its_key_and_a_sealed_ledger_keeps_its_own() {
    let mut k = Kernel::new();
    k.bind_identity([0xab; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2)];
    super::common::speak_pair(&mut k, &a, &b, [1; 32]);
    assert!(k.chain().own().count() > 0, "the mind sealed nodes");
    assert!(k.chain().own().all(|r| r.stamp.vk == [0xab; 32]), "under the key bound before its first act");
    k.bind_identity([0xcd; 32]);
    assert_eq!(k.chain().vk(), [0xab; 32], "a sealed ledger keeps the key it sealed under");
    assert!(k.chain().verify_chain_integrity(), "and its line still verifies");

    let key = super::common::court_key("peer");
    let r = peer_line(key.vk_bytes(), 1).remove(0);
    assert!(matches!(
        k.admit_receipt(ORIGIN, super::common::verified(&pre(&r), &key), &r),
        crate::court::admission::Admitted::Parent(_)
    ));
    assert_eq!(k.known_head(&key.vk_bytes()).map(|h| h.0), Some(crate::chronometry::Place::GENESIS), "the peer's author stands");
}

/// A receipt whose origin is not this node's touches nothing.
#[test]
fn a_foreign_origin_is_refused_before_the_mind_holds_anything() {
    use crate::court::admission::{Admitted, REFUSE_ORIGIN};
    let key = super::common::court_key("peer");
    let r = peer_line(key.vk_bytes(), 1).remove(0);
    let mut k = Kernel::new();
    let mut foreign = pre(&r);
    foreign.genesis = [1; 32];
    assert_eq!(k.admit_receipt(ORIGIN, super::common::verified(&foreign, &key), &r), Admitted::Refused(REFUSE_ORIGIN));
    assert_eq!(k.pending_parents(), 0);
    assert!(k.known_head(&key.vk_bytes()).is_none());
}

/// 47.2 CHECKS ONE SIGNER (A2): a receipt naming one key, carried under a
/// signature that verified under ANOTHER, is two claims and is refused
/// before the ledger is read.
#[test]
fn a_receipt_signed_by_another_key_than_it_names_is_refused() {
    use crate::court::admission::{Admitted, REFUSE_RECEIPT};
    let victim = super::common::court_key("victim");
    let forger = super::common::court_key("forger");
    let r = peer_line(victim.vk_bytes(), 1).remove(0);
    let mut k = Kernel::new();
    assert_eq!(
        k.admit_receipt(ORIGIN, super::common::verified(&pre(&r), &forger), &r),
        Admitted::Refused(REFUSE_RECEIPT),
        "the signature verified under the forger; the receipt names the victim"
    );
    assert!(k.known_head(&victim.vk_bytes()).is_none(), "and the victim's head is untouched");
    assert!(matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&r), &victim), &r), Admitted::Parent(_)));
}

/// Two roots under one key at one K, each measuring its own lane.
fn two_roots(liar: [u8; 32]) -> (crate::chronometry::KStepReceipt, crate::chronometry::KStepReceipt) {
    let a = peer_line(liar, 1).remove(0);
    let mut c = crate::chronometry::ReceiptChain::with_vk(liar);
    let b = c
        .append(crate::chronometry::ActKind::List, [9, 0, 0, 0], Vec::new(), [0; 32], Vec::new())
        .expect("appends")
        .clone();
    (a, b)
}

/// 47.4 — TWO ROOTS AT ONE K BURN THE SIGNER'S BOND, one `burn` entry,
/// supply down by the same vector, §6 holding; a second frame at the same
/// K burns nothing more.
#[test]
fn two_roots_at_one_k_burn_the_signers_bond() {
    use crate::court::admission::Admitted;
    let key = super::common::court_key("liar");
    let liar = key.vk_bytes();
    let (ra, rb) = two_roots(liar);
    assert_eq!(ra.stamp.k, rb.stamp.k, "one K");
    assert_ne!(ra.chain_hash, rb.chain_hash, "two roots");
    let mut k = Kernel::new();
    let bond = [3u64, 3, 3, 3];
    k.treasury_mut().genesis(liar, bond, before_genesis());
    k.treasury_mut().enroll(liar, bond, before_genesis());
    assert_eq!(k.treasury().bond_of(&liar), Some(bond), "the liar is bonded");
    let supply = k.treasury().supply().expect("measured");
    assert!(matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&ra), &key), &ra), Admitted::Parent(_)));
    assert!(matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&rb), &key), &rb), Admitted::Equivocation { .. }));
    assert_eq!(k.treasury().bond_of(&liar), Some([0; 4]), "the bond is burned whole");
    assert_eq!(
        k.treasury().supply().expect("still measured"),
        std::array::from_fn(|l| supply[l] - bond[l]),
        "supply falls by exactly what left circulation"
    );
    assert!(k.treasury().audit().is_ok(), "and §6 holds through the forfeiture");
    let after = k.treasury().supply();
    assert!(matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&rb), &key), &rb), Admitted::Equivocation { .. }));
    assert_eq!(k.treasury().supply(), after, "a second frame burns nothing");
}

/// 47.4 ON A THIRD PARTY'S PROOF (I21): the court burns on the proof
/// presented to it, once.
#[test]
fn a_third_party_proof_burns_the_bond_once() {
    use crate::court::admission::admit_equivocation;
    let key = super::common::court_key("liar");
    let liar = key.vk_bytes();
    let (ra, rb) = two_roots(liar);
    let mut k = Kernel::new();
    let bond = [3u64, 3, 3, 3];
    k.treasury_mut().genesis(liar, bond, before_genesis());
    k.treasury_mut().enroll(liar, bond, before_genesis());
    let supply = k.treasury().supply().expect("measured");
    assert!(k.known_head(&liar).is_none(), "this court heard neither truth itself");
    let proof = admit_equivocation(super::common::verified(&pre(&ra), &key), &ra, super::common::verified(&pre(&rb), &key), &rb)
        .expect("two signed roots at one K are the proof");
    assert_eq!(k.accept_equivocation(proof), Some(bond), "the reply reports the bond that left");
    assert_eq!(k.treasury().bond_of(&liar), Some([0; 4]), "burned whole");
    assert_eq!(
        k.treasury().supply().expect("still measured"),
        std::array::from_fn(|l| supply[l] - bond[l]),
        "supply falls by exactly what left circulation"
    );
    assert!(k.treasury().audit().is_ok(), "§6 holds through the forfeiture");
    assert_eq!(k.accept_equivocation(proof), None, "the same proof again finds nothing to burn");
    assert!(k.treasury().audit().is_ok());
}

/// 47.2 step 3, S13.10 — A GAP CLOSES WHEN THE MISSING RECEIPTS ARRIVE,
/// and the place-range read of what this node holds of the peer never runs
/// past a hole. The gap names the head held and the reported frame's
/// predecessor: what the back-fill must reach.
#[test]
fn a_gap_closes_when_the_missing_receipts_arrive() {
    use crate::court::admission::Admitted;
    let key = super::common::court_key("peer");
    let vk = key.vk_bytes();
    let peer = peer_line(vk, 4);
    let head = peer[3].clone();
    let place = |n: usize| peer[n].stamp.place();
    let mut k = Kernel::new();
    let first = k.admit_receipt(ORIGIN, super::common::verified(&pre(&head), &key), &head);
    assert_eq!(first, Admitted::Gap { vk, from: None, to: place(2) }, "from the line's start to the predecessor");
    assert_eq!(k.pending_parents(), 0, "nothing is cited while the chain is gapped");
    assert!(k.known_head(&vk).is_none());
    // The chase's own stroke, which every page it fetches is admitted on.
    let mut chase = k.stroke();
    assert_eq!(k.admit_backfill(&peer[..2], &mut chase), (2, None), "a page of two admits by its chain, citing nothing unheld");
    assert_eq!(k.known_head(&vk).map(|h| h.0), Some(place(1)), "the head this node holds moved to the second");
    assert_eq!(
        k.admit_receipt(ORIGIN, super::common::verified(&pre(&head), &key), &head),
        Admitted::Gap { vk, from: Some(place(1)), to: place(2) },
        "the page stopped short: still a gap"
    );
    assert_eq!(k.pending_parents(), 0, "and still nothing cited");
    assert_eq!(k.admit_backfill(&peer[2..3], &mut chase), (1, None), "the next page closes the chain");
    assert!(
        matches!(k.admit_receipt(ORIGIN, super::common::verified(&pre(&head), &key), &head), Admitted::Parent(_)),
        "the head admits once the chain reaches it"
    );
    assert_eq!(k.pending_parents(), 1, "only the closing frame is cited");
    assert_eq!(k.admit_backfill(&peer[..1], &mut chase), (1, None), "a receipt already held is passed over");
    let mut forged = peer[3].clone();
    forged.mu = [0, 0, 0, 9];
    assert_eq!(k.admit_backfill(&[forged], &mut chase), (0, None), "a receipt that does not re-derive admits nothing, and names no hole");
    let far = crate::chronometry::Place { k: 9, i: 0 };
    assert_eq!(k.receipts_range(&vk, None, far, 100), peer, "the place-range read: every admitted node in line order, never past a hole");
    assert_eq!(k.receipts_range(&vk, Some(place(1)), place(3), 1).len(), 1, "a page is bounded by its budget");
    assert_eq!(
        k.receipts_range(&vk, Some(place(1)), place(3), 100),
        peer[2..].to_vec(),
        "strictly after one place, up to and including the other"
    );
}
