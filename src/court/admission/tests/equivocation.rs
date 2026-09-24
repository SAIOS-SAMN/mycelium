// 47.4 — THE PROOF A THIRD PARTY PRESENTS: one signer, one K, two hashes,
// both signatures verified; anything else refuses by its own name.

use super::super::*;
use crate::chronometry::{KStepReceipt, ReceiptChain};

const G: [u8; 32] = [7; 32];
const E: u32 = 2;

/// A hypha's signing key from a name: a KDF over the name, which is what a
/// fixture wants and is not an identity (46).
fn court_key(name: &str) -> crate::court::identity::HyphaKey {
    use ed25519_dalek::SigningKey;
    let seed = crate::court::identity::HyphaKey::seed_from_name(name);
    let sk = SigningKey::from_bytes(&seed);
    crate::court::identity::HyphaKey { vk: sk.verifying_key(), sk }
}

/// The first receipt of a fresh chain under `vk`, measuring `lane` on its
/// frame: two such chains under one key are two roots at K 0.
fn first_receipt(vk: [u8; 32], lane: u64) -> KStepReceipt {
    let mut c = ReceiptChain::with_vk(vk);
    c.append(crate::chronometry::ActKind::List, [lane, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone()
}

fn prefix_for(r: &KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: G,
        epoch: E,
        prev_receipt_hash: r.prev_hash(),
        k: r.stamp.k,
        height: r.stamp.h,
        chart: vec![1, 2, 3],
        payload: r.stamp.key(),
    }
}

/// The prefix signed by `key` and verified under its own vk: the one way a
/// caller holds 47.2 step 2.
fn signed<'a>(prefix: &'a SignedPrefix, key: &crate::court::identity::HyphaKey) -> Verified<'a> {
    let sig = key.sign(&prefix.bytes());
    verify_signature(prefix, &key.vk_bytes(), &sig).expect("a key verifies its own signature")
}

/// Two receipts under one key at one K with two chain hashes, each signed
/// and each one claim with its prefix, are the proof, and the proof names
/// the signer and the K the court burns against.
#[test]
fn one_signer_one_k_two_hashes_both_verified_is_the_proof() {
    let key = court_key("liar");
    let vk = key.vk_bytes();
    let (ra, rb) = (first_receipt(vk, 1), first_receipt(vk, 2));
    assert_eq!(ra.stamp.k, rb.stamp.k, "one K");
    assert_ne!(ra.chain_hash, rb.chain_hash, "two roots");
    let (pa, pb) = (prefix_for(&ra), prefix_for(&rb));
    let proof = admit_equivocation(signed(&pa, &key), &ra, signed(&pb, &key), &rb).expect("a proof");
    assert_eq!(proof, Equivocated { vk, k: 0 }, "the signer and the K it lied at");
}

/// One hash twice is a duplicate frame; two signers are two chains; two K
/// on one chain is a chain. None is an equivocation, and each says so by
/// the name 34.1 gives it.
#[test]
fn a_duplicate_two_signers_or_two_k_are_not_a_proof() {
    let key = court_key("liar");
    let vk = key.vk_bytes();
    let ra = first_receipt(vk, 1);
    let pa = prefix_for(&ra);
    assert_eq!(
        admit_equivocation(signed(&pa, &key), &ra, signed(&pa, &key), &ra).err(),
        Some(REFUSE_NOT_EQUIVOCATION),
        "one receipt twice is a duplicate, not a proof"
    );
    let honest = court_key("honest");
    let rc = first_receipt(honest.vk_bytes(), 2);
    let pc = prefix_for(&rc);
    assert_eq!(
        admit_equivocation(signed(&pa, &key), &ra, signed(&pc, &honest), &rc).err(),
        Some(REFUSE_NOT_EQUIVOCATION),
        "two signers are two chains"
    );
    let mut c = ReceiptChain::with_vk(vk);
    let r0 = c.append(crate::chronometry::ActKind::List, [1, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let r1 = c.append(crate::chronometry::ActKind::List, [2, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let (p0, p1) = (prefix_for(&r0), prefix_for(&r1));
    assert_eq!(
        admit_equivocation(signed(&p0, &key), &r0, signed(&p1, &key), &r1).err(),
        Some(REFUSE_NOT_EQUIVOCATION),
        "two K on one chain is a chain"
    );
}

/// A half that disagrees with itself is not half a proof: a receipt whose
/// hash nobody wrote, a prefix at another K than its receipt, or a
/// signature under another key than the receipt names is `REFUSE_RECEIPT`,
/// as `admit` refuses the same disagreement on a single frame.
#[test]
fn a_half_that_disagrees_with_itself_is_refused_as_a_receipt() {
    let key = court_key("liar");
    let vk = key.vk_bytes();
    let (ra, rb) = (first_receipt(vk, 1), first_receipt(vk, 2));
    let (pa, pb) = (prefix_for(&ra), prefix_for(&rb));
    let mut forged = rb.clone();
    forged.chain_hash[0] ^= 1;
    assert!(!forged.rederives(), "a tampered hash does not re-derive");
    assert!(rb.rederives(), "and the receipt as written does");
    assert_eq!(
        admit_equivocation(signed(&pa, &key), &ra, signed(&pb, &key), &forged).err(),
        Some(REFUSE_RECEIPT),
        "a hash nobody wrote"
    );
    let mut at_another_k = pb.clone();
    at_another_k.k += 1;
    assert_eq!(
        admit_equivocation(signed(&pa, &key), &ra, signed(&at_another_k, &key), &rb).err(),
        Some(REFUSE_RECEIPT),
        "a prefix at another K than its receipt"
    );
    let forger = court_key("forger");
    assert_eq!(
        admit_equivocation(signed(&pa, &key), &ra, signed(&pb, &forger), &rb).err(),
        Some(REFUSE_RECEIPT),
        "a signature under another key than the receipt names"
    );
}
