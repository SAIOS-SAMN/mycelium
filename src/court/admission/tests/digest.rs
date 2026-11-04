// MU-ANTI-FLATTENING 13, MU-HOSTING §5a.10 — A DIGEST LOCATES, LINKS AND
// BINDS, AND NEVER ADMITS: the commitment sense reads only a reconstructed
// object, and the link sense tells a chain that continues from one that
// forks.

use super::super::*;
use crate::chronometry::{ActKind, KStepReceipt, Parent, ReceiptChain};

/// A signer's line of `n` acts, sealed.
fn line(signer: [u8; 32], n: u64) -> Vec<KStepReceipt> {
    let mut c = ReceiptChain::with_signer(signer);
    (0..n).map(|lane| c.append(ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone()).collect()
}

/// The 47.1 prefix a signer writes for `r`.
fn prefix_for(r: &KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: r.prev_hash(),
        at: r.stamp.place(),
        height: r.stamp.h,
        chart: Vec::new(),
        payload: r.stamp.key(),
    }
}

/// A COMMITMENT BINDS ONLY A RECONSTRUCTED OBJECT: the predecessor a
/// signed prefix commits to binds the receipt it was signed for, once that
/// receipt is reconstructed, and binds no other reconstructed receipt. A
/// receipt whose chain hash does not re-derive carries the committed
/// predecessor byte for byte, and still nothing binds it: it is never
/// reconstructed, so `binds` has no object to read (and a claimed receipt
/// is refused by the compiler, the `compile_fail` doc test on
/// `Commitment`).
#[test]
fn commitment_binds_only_a_reconstructed_object() {
    let rs = line([1; 32], 3);
    let signed = Commitment::signed(&prefix_for(&rs[1]));
    let r1 = Reconstructed::receipt(&rs[1]).expect("a sealed receipt is reconstructed");
    assert!(signed.binds(&r1), "the commitment binds the receipt it was signed for");
    let r2 = Reconstructed::receipt(&rs[2]).expect("a sealed receipt is reconstructed");
    assert!(!signed.binds(&r2), "and binds no other receipt");
    let mut forged = rs[1].clone();
    forged.mu = [0, 0, 0, 9];
    assert_eq!(forged.prev_hash(), signed.bytes(), "the forged receipt carries the committed predecessor");
    assert!(Reconstructed::receipt(&forged).is_none(), "and is never reconstructed, so no commitment binds it");
}

/// THE LINK SENSE: a citation continues the chain held at its stamp, two
/// records at one place with two chain hashes fork the line, and a
/// citation of a record at another stamp cites nothing held there.
#[test]
fn predecessor_continues_held_record_and_reads_fork() {
    let rs = line([1; 32], 2);
    let fork = ReceiptChain::with_signer([1; 32]).append(ActKind::List, [9, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    assert_eq!(fork.stamp.place(), rs[0].stamp.place(), "the fork stands at the line's first place");
    let named = rs[1].prev.expect("the second act names the first");
    assert!(Predecessor::named(&named).continues(&Predecessor::of(&rs[0])), "the predecessor named is the record held");
    assert!(!Predecessor::named(&named).continues(&Predecessor::of(&fork)), "a fork at that place is not the record named");
    assert!(Predecessor::of(&rs[0]).forks(&Predecessor::of(&fork)), "one place, two hashes: the line forks");
    assert!(!Predecessor::of(&rs[0]).forks(&Predecessor::of(&rs[0])), "one record does not fork itself");
    let held = Parent { at: rs[0].stamp, hash: rs[0].chain_hash };
    assert!(Predecessor::cites(&named, &held), "the citation names the record held at its stamp");
    let elsewhere = Parent { at: rs[1].stamp, hash: rs[0].chain_hash };
    assert!(!Predecessor::cites(&named, &elsewhere), "the same hash at another stamp is not the record cited");
}

/// THE ADDRESS SENSE: a locator finds the place another names, and says
/// nothing else.
#[test]
fn locator_locates_by_its_bytes_alone() {
    let rs = line([1; 32], 2);
    let (a, b) = (Locator::of_bytes(rs[0].chain_hash), Locator::of_bytes(rs[1].chain_hash));
    assert!(a.locates(&Locator::of_bytes(a.bytes())), "a locator finds the place it names");
    assert!(!a.locates(&b), "and not another");
}
