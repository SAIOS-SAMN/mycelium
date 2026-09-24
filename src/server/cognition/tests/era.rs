// 45.8, 47.3 — THE ERA: a fold proposed, witnessed and committed on two
// signatures; what it archives, what it refuses, the chain that continues
// over its root, and the K-range read across the archive.

use crate::anastome::dialogue::ReceiptSection;
use crate::chronometry::{ActKind, Place, ReceiptChain};
use crate::cognition::*;
use crate::court::admission::SignedPrefix;
use crate::verdict::{
    REFUSE_FOLD_INEXACT, REFUSE_FOLD_OPEN, REFUSE_RECEIPT, REFUSE_SIGNATURE, REFUSE_WITNESS_DEPENDENT,
    REFUSE_WITNESS_ROOM,
};

/// A witness's signed position over a root hash, as a fixture: its own
/// chain of one receipt and a 47.1 prefix whose payload is the hash,
/// signed by the key its name derives.
fn co_sign(name: &str, root_hash: [u8; 32]) -> ReceiptSection {
    let key = super::common::court_key(name);
    let mut c = ReceiptChain::with_vk(key.vk_bytes());
    let head = c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let prefix = SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload: root_hash.to_vec(),
    };
    let sig = key.sign(&prefix.bytes());
    ReceiptSection { prefix, sig, receipt: head }
}

/// Three sealed acts and nothing open propose a fold; the witness
/// re-derives the prefix it fetched and refuses a wrong net, a short
/// prefix, or its own missing bond; the home seals only on a signature
/// that verifies, from another key, over this root; then the prefix is
/// cold under its root with both signatures, the `Fold` receipt cites the
/// root and continues K and height, the ledger conserves, the snapshot
/// carries it, the witness holds the same era under the home's key, and a
/// second fold folds the fold receipt itself. A listing open at the first
/// receipt proposes nothing; a ledger that does not conserve proposes
/// nothing.
#[test]
fn a_fold_is_proposed_witnessed_and_committed_on_two_signatures() {
// Three strokes, each one act and its close (OK1): orbit `n` holds its act
// at `(n, 0)` and its close at `(n, 1)`. A fold cuts only at a close.
let act = |k: u64| Place { k, i: 0 };
let close = |k: u64| Place { k, i: 1 };
let home = [0x5c; 32];
let mut k = Kernel::new();
k.bind_identity(home);
assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "no receipt, nothing to fold");
k.append_step(ActKind::List, [1; 32], [2; 32]);
assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "an orbit not yet closed is no cut");
k.orbit();
for _ in 0..2 {
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.orbit();
}
let head_hash = k.chain().head_hash();
let head_height = k.chain().head_height();
let before = k.treasury().audit().expect("conserves before");
let p = k.propose_fold().expect("a final prefix proposes");
assert_eq!((p.after, p.root.at, p.root.hash, p.root.height), (None, close(2), head_hash, head_height));
assert!(p.epochs.iter().any(|(who, _)| *who == [0x11; 32]), "the fixture hypha's genesis nets");
assert_eq!(k.fold_proposal(), Some(&p), "held for the exchange to carry");

// THE WITNESS re-derives the prefix it fetched.
let receipts = k.receipts_range(&home, p.after, p.root.at, 10);
let rows = k.treasury().rows_below(p.root.at);
let witness = Kernel::new();
assert_eq!(witness.verify_fold(home, &p, &receipts, &rows), Ok(()), "the prefix re-derives");
let mut wrong = p.clone();
wrong.epochs.clear();
assert_eq!(witness.verify_fold(home, &wrong, &receipts, &rows), Err(REFUSE_FOLD_INEXACT), "a net that is not the rows'");
assert_eq!(witness.verify_fold(home, &p, &receipts[..2], &rows), Err(REFUSE_RECEIPT), "a prefix that does not reach the root");
assert_eq!(Kernel::boot().verify_fold(home, &p, &receipts, &rows), Err(REFUSE_WITNESS_ROOM), "a witness without bond");

// THE HOME seals only on a signature that verifies, from another key, over this root.
let home_sig = [3u8; 64];
let mut dependent = co_sign("w", p.root.hash);
dependent.receipt.stamp.vk = home;
assert_eq!(k.commit_fold(home_sig, &dependent), Err(REFUSE_WITNESS_DEPENDENT), "a court is not its own witness");
let mut forged = co_sign("w", p.root.hash);
forged.sig[0] ^= 1;
assert_eq!(k.commit_fold(home_sig, &forged), Err(REFUSE_SIGNATURE));
assert_eq!(k.commit_fold(home_sig, &co_sign("w", [9; 32])), Err(REFUSE_RECEIPT), "a position over another root");
assert!(k.fold_proposal().is_some(), "the proposal stands through a witness's refusals");
let w = co_sign("w", p.root.hash);
let done = k.commit_fold(home_sig, &w).expect("two signatures seal the fold");
assert_eq!((done.first, done.last, done.at, done.witness), (act(0), close(2), Some(act(3)), w.receipt.stamp.vk));
assert_eq!(done.h, head_height + 1, "height continues over the fold");
assert!(k.fold_proposal().is_none(), "sealed, nothing pends");
let root = k.chain().root().expect("the chain stands on its root");
assert_eq!(root, p.root);
let fold = k.chain().opening().expect("the hot line opens at the fold").clone();
assert_eq!((fold.kind, fold.stamp.k, fold.prev_hash()), (ActKind::Fold, 3, head_hash));
assert_eq!(
    fold.cites,
    vec![crate::chronometry::Cite::Root {
        at: crate::chronometry::Stamp::new(root.height, home, root.at),
        hash: head_hash
    }],
    "the fold node cites the archived head by its stamp and hash"
);
assert!(k.chain().verify_chain_integrity(), "the hot chain re-verifies from its root");
assert_eq!(k.k_step(), 3, "K continues past the fold: the fold receipt opens orbit 3");
let era = &k.archive()[0];
assert_eq!(
    (era.root, era.signer, era.witness, era.sig_home, era.sig_witness),
    (root, home, w.receipt.stamp.vk, home_sig, w.sig),
    "the era carries the witness and both signatures"
);
assert_eq!((era.receipts.len(), era.first_place()), (6, act(0)));
assert!(ReceiptChain::from_receipts(era.receipts.clone()).is_some(), "the archived prefix re-verifies as a chain of its own");
assert_eq!(k.treasury().audit().expect("conserves after"), before);
assert_eq!(
    k.receipts_range(&home, None, act(3), 10).iter().map(|r| r.stamp.place()).collect::<Vec<_>>(),
    vec![act(0), close(0), act(1), close(1), act(2), close(2), act(3)],
    "the place-range read spans ARC1 and the hot chain"
);

let snap = k.snapshot();
let mut back = Kernel::new();
assert!(back.restore(&snap), "the snapshot carries the root and the era");
assert_eq!(back.chain().root(), Some(root));
assert_eq!(back.archive(), k.archive());
assert!(back.chain().verify_chain_integrity());

// THE WITNESS holds the same era under the home's key and serves it.
let mut held = Kernel::new();
held.hold_era(era.clone());
assert_eq!(held.receipts_range(&home, None, close(2), 10), era.receipts, "a witnessed era is read under its signer's key");
assert_eq!(held.rows_range(&home, None, act(9), 0, 10).0, 0, "another signer's rows are not this court's to serve");

assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "the fold receipt's orbit is open: no cut yet");
k.orbit();
let p2 = k.propose_fold().expect("the fold receipt's orbit closed, and is itself final");
assert_eq!((p2.after, p2.root.at), (Some(close(2)), close(3)));
assert!(k.commit_fold(home_sig, &co_sign("w", p2.root.hash)).is_ok());
assert_eq!(k.archive().len(), 2);
assert_eq!(k.chain().root().map(|r| r.at), Some(close(3)));

let mut open = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
assert_eq!(super::common::speak_pair(&mut open, &a, &b, [7; 32]), Outcome::GermPending);
assert_eq!(open.final_prefix_head(), None, "a listing open at the first receipt leaves nothing final");
assert_eq!(open.propose_fold(), Err(REFUSE_FOLD_OPEN));
assert!(open.chain().root().is_none() && open.archive().is_empty(), "nothing was archived");

let mut broken = Kernel::new();
broken.append_step(ActKind::List, [1; 32], [2; 32]);
broken.orbit();
broken.treasury.credit([9; 32], [1, 0, 0, 0]);
assert_eq!(broken.propose_fold(), Err(REFUSE_FOLD_INEXACT), "a ledger that does not conserve proposes nothing");
assert!(broken.chain().root().is_none() && broken.fold_proposal().is_none());
}
