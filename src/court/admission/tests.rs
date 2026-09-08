// 47.2 — ADMISSION'S TESTS: what a receipt must re-derive before this
// node's book will hold it, and what it is answered with when it does not.

use super::*;
use crate::chronometry::ReceiptChain;

const G: [u8; 32] = [7; 32];
const E: u32 = 2;

/// A chain of `n` steps under one key, each step naming the parents given
/// for it. Returns the chain so its receipts are real ones: heights that
/// re-derive, hashes that link.
fn chain(vk: [u8; 32], steps: &[(&[u8], Vec<Parent>)]) -> ReceiptChain {
    let mut c = ReceiptChain::with_vk(vk);
    for (delta, parents) in steps {
        c.append(delta, [0; 32], parents.clone()).expect("appends");
    }
    c
}

fn prefix_for(r: &KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: G,
        epoch: E,
        prev_receipt_hash: r.prev_receipt_hash,
        k: r.step_index,
        height: r.height,
        chart: vec![1, 2, 3],
        payload: r.delta.clone(),
    }
}

/// The prefix is 47.1's bytes in 47.1's order, and it reads back whole.
/// Nothing in it is a digest of frame bytes: the payload is the address.
#[test]
fn the_signed_prefix_is_written_out_and_decodes() {
    let p = SignedPrefix {
        genesis: G,
        epoch: E,
        prev_receipt_hash: [9; 32],
        k: 41,
        height: 77,
        chart: vec![5, 0, 12],
        payload: b"an address, written".to_vec(),
    };
    let b = p.bytes();
    assert_eq!(&b[..32], &G, "H_genesis first");
    assert_eq!(&b[32..36], &E.to_le_bytes(), "then E_origin");
    assert_eq!(&b[36..68], &[9u8; 32], "then prev_receipt_hash");
    assert_eq!(&b[68..76], &41u64.to_le_bytes(), "then K");
    assert_eq!(&b[76..84], &77u64.to_le_bytes(), "then height");
    assert_eq!(
        SignedPrefix::read(&b).as_ref(),
        Some(&p),
        "and it reads back"
    );
    assert!(
        SignedPrefix::read(&b[..b.len() - 1]).is_none(),
        "a short prefix refuses whole"
    );
}

/// 47.2 step 1 runs BEFORE any carrier is touched: a frame from another
/// genesis or another epoch is refused on the origin, not on its content,
/// and the signer's head is not remembered from it.
#[test]
fn prefix_refuses_foreign_genesis_or_epoch_before_carriers() {
    let c = chain([1; 32], &[(b"a".as_slice(), vec![])]);
    let r = c.receipts()[0].clone();
    let mut a = Admissions::new();

    let mut foreign = prefix_for(&r);
    foreign.genesis = [8; 32];
    assert_eq!(
        a.admit((G, E), &foreign, &r, |_| Some(0)),
        Admitted::Refused(REFUSE_ORIGIN)
    );
    let mut old_epoch = prefix_for(&r);
    old_epoch.epoch = E - 1;
    assert_eq!(
        a.admit((G, E), &old_epoch, &r, |_| Some(0)),
        Admitted::Refused(REFUSE_ORIGIN)
    );
    assert!(a.head(&[1; 32]).is_none(), "no carrier was touched");

    assert!(
        matches!(
            a.admit((G, E), &prefix_for(&r), &r, |_| Some(0)),
            Admitted::Parent(_)
        ),
        "the same receipt on this node's own origin admits"
    );
}

/// 47.2 step 3: K = K_h + 1 with the matching prev is a parent; K beyond
/// that is a GAP, and nothing is acted on until the missing receipts are
/// fetched. The back-fill closes it and the next step admits normally.
#[test]
fn receipt_gap_backfills_before_acting() {
    let c = chain(
        [1; 32],
        &[
            (b"a".as_slice(), vec![]),
            (b"b".as_slice(), vec![]),
            (b"c".as_slice(), vec![]),
        ],
    );
    let (r0, r1, r2) = (
        c.receipts()[0].clone(),
        c.receipts()[1].clone(),
        c.receipts()[2].clone(),
    );
    let h = |p: &Parent| {
        c.receipts()
            .iter()
            .find(|x| x.chain_hash == p.hash)
            .map(|x| x.height)
    };
    let mut a = Admissions::new();

    assert!(matches!(
        a.admit((G, E), &prefix_for(&r0), &r0, h),
        Admitted::Parent(_)
    ));
    assert_eq!(
        a.admit((G, E), &prefix_for(&r2), &r2, h),
        Admitted::Gap {
            vk: [1; 32],
            from: 0,
            to: 2
        },
        "K jumps the head: docket and back-fill, act on nothing"
    );
    assert_eq!(
        a.head(&[1; 32]),
        Some((0, r0.chain_hash)),
        "the gapped frame moved no head"
    );

    assert!(
        matches!(
            a.admit((G, E), &prefix_for(&r1), &r1, h),
            Admitted::Parent(_)
        ),
        "back-fill"
    );
    assert!(
        matches!(
            a.admit((G, E), &prefix_for(&r2), &r2, h),
            Admitted::Parent(_)
        ),
        "the chain closed, so the frame admits"
    );
}

/// A first sighting of a signer at K > 0 is a gap back to its genesis,
/// not an admission: this node cannot re-derive the height of a chain
/// whose prefix it has never held.
#[test]
fn a_signer_first_seen_mid_chain_is_a_gap_to_its_genesis() {
    let c = chain(
        [2; 32],
        &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])],
    );
    let r1 = c.receipts()[1].clone();
    let h = |p: &Parent| {
        c.receipts()
            .iter()
            .find(|x| x.chain_hash == p.hash)
            .map(|x| x.height)
    };
    let mut a = Admissions::new();
    assert_eq!(
        a.admit((G, E), &prefix_for(&r1), &r1, h),
        Admitted::Gap {
            vk: [2; 32],
            from: 0,
            to: 1
        }
    );
}

/// 47.2 step 4. The height on the frame is a CLAIM and is re-derived
/// from the parents the receipt names. A parent this node does not hold
/// leaves the height unmeasured, which refuses; a height that re-derives
/// to something else refuses on the same code.
#[test]
fn claimed_height_that_does_not_rederive_is_refused() {
    let c = chain(
        [1; 32],
        &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])],
    );
    let r1 = c.receipts()[1].clone();
    let held = |p: &Parent| {
        c.receipts()
            .iter()
            .find(|x| x.chain_hash == p.hash)
            .map(|x| x.height)
    };
    let mut a = Admissions::new();
    a.admit(
        (G, E),
        &prefix_for(&c.receipts()[0]),
        &c.receipts()[0],
        held,
    );

    let mut lying = r1.clone();
    lying.height = r1.height + 5;
    assert_eq!(
        a.admit((G, E), &prefix_for(&lying), &lying, held),
        Admitted::Refused(REFUSE_RECEIPT),
        "a height that does not re-derive is refused"
    );
    assert_eq!(
        a.admit((G, E), &prefix_for(&r1), &r1, |_| None),
        Admitted::Refused(REFUSE_RECEIPT),
        "a parent this node does not hold is UNMEASURED, never height 0"
    );
    // The prefix and the receipt are one claim, not two opinions.
    let mut split = prefix_for(&r1);
    split.height = r1.height + 1;
    assert_eq!(
        a.admit((G, E), &split, &r1, held),
        Admitted::Refused(REFUSE_RECEIPT),
        "a prefix that disagrees with the receipt it travels with"
    );
    assert!(matches!(
        a.admit((G, E), &prefix_for(&r1), &r1, held),
        Admitted::Parent(_)
    ));
}

/// 47.2 step 3's last two arms. The same receipt twice is STALE, and is
/// ignored. A different chain hash at a K already held is not a later
/// opinion to prefer: it is an equivocation proof (47.4).
#[test]
fn a_duplicate_is_stale_and_a_second_hash_at_one_k_is_equivocation() {
    let c = chain(
        [1; 32],
        &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])],
    );
    let d = chain([1; 32], &[(b"z".as_slice(), vec![])]);
    let (r0, r1) = (c.receipts()[0].clone(), c.receipts()[1].clone());
    let held = |p: &Parent| {
        c.receipts()
            .iter()
            .chain(d.receipts())
            .find(|x| x.chain_hash == p.hash)
            .map(|x| x.height)
    };
    let mut a = Admissions::new();
    assert!(matches!(
        a.admit((G, E), &prefix_for(&r0), &r0, held),
        Admitted::Parent(_)
    ));
    assert!(matches!(
        a.admit((G, E), &prefix_for(&r1), &r1, held),
        Admitted::Parent(_)
    ));

    let stale = a.admit((G, E), &prefix_for(&r0), &r0, held);
    assert_eq!(stale, Admitted::Stale);
    assert_eq!(
        stale.reason(),
        Some(REFUSE_RECEIPT_STALE),
        "the reply names 34.1's constant for a duplicate frame"
    );

    let fork = d.receipts()[0].clone();
    assert_ne!(fork.chain_hash, r0.chain_hash, "one K, two hashes");
    let proof = a.admit((G, E), &prefix_for(&fork), &fork, held);
    assert_eq!(
        proof,
        Admitted::Equivocation {
            k: 0,
            held: r0.chain_hash,
            claimed: fork.chain_hash
        }
    );
    assert_eq!(
        proof.reason(),
        Some(REFUSE_EQUIVOCATION),
        "and for the proof of 47.4"
    );
    assert!(
        a.admit((G, E), &prefix_for(&r1), &r1, held)
            .reason()
            .is_some(),
        "a second hearing of the head is stale too"
    );
}
