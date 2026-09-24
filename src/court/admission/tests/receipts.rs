// 47.2, 78.24 — ADMISSION'S TESTS: what a receipt must re-derive against the
// ledger this node holds before the ledger will hold it, and what it is
// answered with when it does not.

use super::super::*;
use crate::chronometry::{KStepReceipt, Parent, Place, ReceiptChain};

const G: [u8; 32] = [7; 32];
const E: u32 = 2;

/// Act `k` of a line, while every act closes its orbit.
fn at(k: u64) -> Place {
    Place { k, i: 0 }
}

/// A signer's line of `frames.len()` steps, each measuring its lane and
/// naming the parents given for it: real nodes, heights that re-derive,
/// hashes that link.
fn line(vk: [u8; 32], frames: &[(u64, Vec<Parent>)]) -> Vec<KStepReceipt> {
    let mut c = ReceiptChain::with_vk(vk);
    frames
        .iter()
        .map(|(lane, parents)| {
            c.append(crate::chronometry::ActKind::List, [*lane, 0, 0, 0], Vec::new(), [0; 32], parents.clone())
                .expect("appends")
                .clone()
        })
        .collect()
}

fn prefix_for(r: &KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: G,
        epoch: E,
        prev_receipt_hash: r.prev_hash(),
        at: r.stamp.place(),
        height: r.stamp.h,
        chart: vec![1, 2, 3],
        payload: r.stamp.key(),
    }
}

/// Admit on the receiver's ledger as the kernel does: a `Parent` is held.
fn take(ledger: &mut ReceiptChain, r: &KStepReceipt) -> Admitted {
    let got = admit((G, E), &prefix_for(r), r, ledger);
    if let Admitted::Parent(_) = got {
        ledger.admit(r.clone());
    }
    got
}

/// The prefix is 47.1's bytes in 47.1's order, and it reads back whole.
#[test]
fn the_signed_prefix_is_written_out_and_decodes() {
    let p = SignedPrefix {
        genesis: G,
        epoch: E,
        prev_receipt_hash: [9; 32],
        at: Place { k: 41, i: 3 },
        height: 77,
        chart: vec![5, 0, 12],
        payload: b"an address, written".to_vec(),
    };
    let b = p.bytes();
    assert_eq!(&b[..32], &G, "H_genesis first");
    assert_eq!(&b[32..36], &E.to_le_bytes(), "then E_origin");
    assert_eq!(&b[36..68], &[9u8; 32], "then prev_receipt_hash");
    assert_eq!(&b[68..76], &41u64.to_le_bytes(), "then K");
    assert_eq!(&b[76..84], &3u64.to_le_bytes(), "then the act's place in its orbit");
    assert_eq!(&b[84..92], &77u64.to_le_bytes(), "then height");
    assert_eq!(SignedPrefix::read(&b).as_ref(), Some(&p), "and it reads back");
    assert!(SignedPrefix::read(&b[..b.len() - 1]).is_none(), "a short prefix refuses whole");
}

/// 47.2 step 1 runs BEFORE any carrier is touched: a frame from another
/// genesis or another epoch is refused on the origin, and the ledger holds
/// nothing of it.
#[test]
fn prefix_refuses_foreign_genesis_or_epoch_before_carriers() {
    let r = line([1; 32], &[(1, vec![])]).remove(0);
    let mut ledger = ReceiptChain::with_vk([9; 32]);
    let mut foreign = prefix_for(&r);
    foreign.genesis = [8; 32];
    assert_eq!(admit((G, E), &foreign, &r, &ledger), Admitted::Refused(REFUSE_ORIGIN));
    let mut old_epoch = prefix_for(&r);
    old_epoch.epoch = E - 1;
    assert_eq!(admit((G, E), &old_epoch, &r, &ledger), Admitted::Refused(REFUSE_ORIGIN));
    assert!(ledger.line_head(&[1; 32]).is_none(), "no carrier was touched");
    assert!(matches!(take(&mut ledger, &r), Admitted::Parent(_)), "the same receipt on this node's own origin admits");
}

/// 47.2 step 3: a receipt naming the head held as its predecessor is a
/// parent; one whose predecessor stands past the head is a GAP up to that
/// predecessor, and nothing is acted on until the missing receipts are
/// fetched. The back-fill closes it and the next step admits normally.
#[test]
fn receipt_gap_backfills_before_acting() {
    let rs = line([1; 32], &[(1, vec![]), (2, vec![]), (3, vec![])]);
    let mut ledger = ReceiptChain::with_vk([9; 32]);
    assert!(matches!(take(&mut ledger, &rs[0]), Admitted::Parent(_)));
    assert_eq!(
        take(&mut ledger, &rs[2]),
        Admitted::Gap { vk: [1; 32], from: Some(at(0)), to: at(1) },
        "the predecessor stands past the head: docket and back-fill, act on nothing"
    );
    assert_eq!(ledger.line_head(&[1; 32]).map(|r| r.stamp), Some(rs[0].stamp), "the gapped frame moved no head");
    assert!(matches!(take(&mut ledger, &rs[1]), Admitted::Parent(_)), "back-fill");
    assert!(matches!(take(&mut ledger, &rs[2]), Admitted::Parent(_)), "the line closed, so the frame admits");
}

/// A first sighting of a signer past its genesis is a gap back to its
/// genesis.
#[test]
fn a_signer_first_seen_mid_chain_is_a_gap_to_its_genesis() {
    let rs = line([2; 32], &[(1, vec![]), (2, vec![])]);
    let ledger = ReceiptChain::with_vk([9; 32]);
    assert_eq!(admit((G, E), &prefix_for(&rs[1]), &rs[1], &ledger), Admitted::Gap { vk: [2; 32], from: None, to: at(0) });
}

/// THE LINE'S CONTINUITY IS THE PREDECESSOR A RECEIPT NAMES (78.23 as
/// amended): its own signer's, at a place it follows — the next act of the
/// orbit or the first of the next. A receipt naming a predecessor it does
/// not follow, or another signer's, or none past genesis, is refused
/// before the ledger's head is read.
#[test]
fn a_receipt_names_a_predecessor_it_follows() {
    let rs = line([1; 32], &[(1, vec![]), (2, vec![])]);
    let ledger = ReceiptChain::with_vk([9; 32]);
    let refused = |r: &KStepReceipt| admit((G, E), &prefix_for(r), r, &ledger);
    let mut leap = rs[1].clone();
    leap.stamp.k += 1;
    assert_eq!(refused(&leap), Admitted::Refused(REFUSE_RECEIPT), "two orbits past its predecessor");
    let mut inner = rs[1].clone();
    inner.stamp.i = 2;
    assert_eq!(refused(&inner), Admitted::Refused(REFUSE_RECEIPT), "two acts past it in the orbit");
    let mut orphan = rs[1].clone();
    orphan.prev = None;
    assert_eq!(refused(&orphan), Admitted::Refused(REFUSE_RECEIPT), "no predecessor past genesis");
    let mut borrowed = rs[1].clone();
    if let Some(p) = borrowed.prev.as_mut() {
        p.at.vk = [2; 32];
    }
    assert_eq!(refused(&borrowed), Admitted::Refused(REFUSE_RECEIPT), "another signer's predecessor");
}

/// 47.2 step 4. The height on the frame is a CLAIM, re-derived from the
/// predecessor and the parents the receipt names at their stamps. A parent
/// this ledger does not hold is the hole it names, never height 0.
#[test]
fn claimed_height_that_does_not_rederive_is_refused() {
    let peer = line([3; 32], &[(1, vec![])]).remove(0);
    // The signer held the peer when it cited it.
    let mut signer = ReceiptChain::with_vk([1; 32]);
    let r0 = signer.append(crate::chronometry::ActKind::List, [1, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    signer.admit(peer.clone());
    let r1 = signer
        .append(crate::chronometry::ActKind::List, [2, 0, 0, 0], Vec::new(), [0; 32], vec![Parent { at: peer.stamp, hash: peer.chain_hash }])
        .expect("appends")
        .clone();
    let rs = [r0, r1];
    let mut ledger = ReceiptChain::with_vk([9; 32]);
    assert!(matches!(take(&mut ledger, &rs[0]), Admitted::Parent(_)));
    assert_eq!(
        admit((G, E), &prefix_for(&rs[1]), &rs[1], &ledger),
        Admitted::MissingParent(Parent { at: peer.stamp, hash: peer.chain_hash }),
        "an unheld citation is the hole it names, not a refusal of the frame"
    );
    assert!(matches!(take(&mut ledger, &peer), Admitted::Parent(_)), "the hole fetched");
    let mut lying = rs[1].clone();
    lying.stamp.h += 5;
    assert_eq!(admit((G, E), &prefix_for(&lying), &lying, &ledger), Admitted::Refused(REFUSE_RECEIPT), "a height that does not re-derive is refused");
    let mut split = prefix_for(&rs[1]);
    split.height += 1;
    assert_eq!(admit((G, E), &split, &rs[1], &ledger), Admitted::Refused(REFUSE_RECEIPT), "a prefix that disagrees with the receipt it travels with");
    assert!(matches!(take(&mut ledger, &rs[1]), Admitted::Parent(_)));
}

/// 47.2 step 3's last two arms, read off the ledger. The same node twice is
/// STALE. A different chain hash at a place already held is an
/// equivocation proof (47.4).
#[test]
fn a_duplicate_is_stale_and_a_second_hash_at_one_k_is_equivocation() {
    let rs = line([1; 32], &[(1, vec![]), (2, vec![])]);
    let fork = line([1; 32], &[(9, vec![])]).remove(0);
    let mut ledger = ReceiptChain::with_vk([9; 32]);
    assert!(matches!(take(&mut ledger, &rs[0]), Admitted::Parent(_)));
    assert!(matches!(take(&mut ledger, &rs[1]), Admitted::Parent(_)));
    let stale = take(&mut ledger, &rs[0]);
    assert_eq!(stale, Admitted::Stale);
    assert_eq!(stale.reason(), Some(REFUSE_RECEIPT_STALE), "the reply names 34.1's constant for a duplicate frame");
    assert_ne!(fork.chain_hash, rs[0].chain_hash, "one place, two hashes");
    let proof = take(&mut ledger, &fork);
    assert_eq!(proof, Admitted::Equivocation { at: at(0), held: rs[0].chain_hash, claimed: fork.chain_hash });
    assert_eq!(proof.reason(), Some(REFUSE_EQUIVOCATION), "and for the proof of 47.4");
    assert!(take(&mut ledger, &rs[1]).reason().is_some(), "a second hearing of the head is stale too");
}

/// 78.24 (d2) — WHAT ADMISSION KNOWS IS WHAT THE LEDGER HOLDS: a ledger
/// written and read back admits the next node of a signer as a parent and
/// one past the head as a gap, with nothing persisted beside it.
#[test]
fn admission_after_a_restore_reads_the_restored_ledger() {
    let rs = line([1; 32], &[(1, vec![]), (2, vec![]), (3, vec![])]);
    let mut ledger = ReceiptChain::with_vk([9; 32]);
    ledger.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("its own genesis");
    assert!(matches!(take(&mut ledger, &rs[0]), Admitted::Parent(_)));
    let mut tape = Vec::new();
    ledger.write(&mut tape).expect("writes");
    let mut back = ReceiptChain::read(&tape, &mut 0).expect("reads");
    assert_eq!(
        take(&mut back, &rs[2]),
        Admitted::Gap { vk: [1; 32], from: Some(at(0)), to: at(1) },
        "the signer's head came back with the ledger"
    );
    assert!(matches!(take(&mut back, &rs[1]), Admitted::Parent(_)), "and its next node admits, not a gap from zero");
}

/// A court key over a real prefix, for the two 47.2 steps below.
fn court_key() -> crate::court::identity::HyphaKey {
    use ed25519_dalek::SigningKey;
    let seed = crate::court::identity::HyphaKey::seed_from_name("court");
    let sk = SigningKey::from_bytes(&seed);
    crate::court::identity::HyphaKey { vk: sk.verifying_key(), sk }
}

/// 47.2 steps 1 and 2, IN THE CHAPTER'S ORDER: the origin is this mesh's
/// and the signature verifies over the 47.1 prefix under the key the frame
/// claims. The admitting case is a real ed25519 signature over the real
/// prefix bytes.
#[test]
fn a_frame_is_admitted_only_under_a_signature_over_its_own_prefix() {
    let court = court_key();
    let vk = court.vk_bytes();
    let stranger = crate::court::identity::HyphaKey::vk_from_name("stranger");
    let r = line(vk, &[(1, Vec::new())]).remove(0);
    let p = prefix_for(&r);
    let sig = court.sign(&p.bytes());
    let verified = admit_signature((G, E), &p, &vk, &sig).expect("the court's own signature over its own prefix");
    assert_eq!(verified.signer(), vk, "verified under the key the frame claims");
    assert_eq!(verified.prefix(), &p, "over exactly these prefix bytes");
    assert_eq!(admit_signature(([8; 32], E), &p, &vk, &sig).err(), Some(REFUSE_ORIGIN), "another genesis is refused before any bytes are verified");
    assert_eq!(admit_signature((G, E + 1), &p, &vk, &sig).err(), Some(REFUSE_ORIGIN), "and so is another epoch of this genesis");
    assert_eq!(admit_signature((G, E), &p, &stranger, &sig).err(), Some(REFUSE_SIGNATURE), "the stranger did not sign this prefix");
    let mut other = p.clone();
    other.payload = b"another address".to_vec();
    assert_eq!(admit_signature((G, E), &other, &vk, &sig).err(), Some(REFUSE_SIGNATURE), "a signature over one position does not admit another");
    assert_eq!(admit_signature((G, E), &p, &vk, &[0u8; 64]).err(), Some(REFUSE_SIGNATURE), "and a zero signature admits nothing at all");
    assert!(verify_signature(&p, &vk, &sig).is_ok(), "step 2 on its own verifies");
    assert_eq!(verify_signature(&p, &stranger, &sig).err(), Some(REFUSE_SIGNATURE));
}
