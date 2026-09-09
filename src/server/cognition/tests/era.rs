// 45.8 — THE ERA: a fold behind a fully final prefix, what it archives,
// what it refuses, and the chain that continues over it.

use crate::chronometry::ActKind;
use crate::cognition::*;
use crate::verdict::{REFUSE_FOLD_INEXACT, REFUSE_FOLD_OPEN};

/// A court with three sealed acts and nothing open folds them: the prefix
/// goes cold into `ARC1` under its root, the `Fold` receipt cites the
/// root and continues K and height, the hot chain re-verifies from the
/// root, the ledger conserves, the snapshot carries the root and the
/// era, and a later fold folds the fold receipt itself. A court with a
/// listing open at its first receipt folds nothing.
#[test]
fn a_court_folds_a_final_prefix_into_arc1_and_the_chain_continues() {
let mut k = Kernel::new();
k.bind_identity([0x5c; 32]);
assert_eq!(k.fold_era(), Err(REFUSE_FOLD_OPEN), "no receipt, nothing to fold");
for _ in 0..3 {
    k.append_step(ActKind::List, [1; 32], [2; 32]);
}
let head_hash = k.chain().head_hash();
let head_height = k.chain().head_height();
assert_eq!(k.final_prefix_head(), Some(2), "nothing open: the whole chain is final");
let before = k.treasury().audit().expect("conserves before");

let done = k.fold_era().expect("a final prefix folds");
assert_eq!((done.k0, done.k1, done.k), (0, 2, 3), "the prefix 0..2 folded, the fold receipt is K 3");
assert_eq!(done.h, head_height + 1, "height continues over the fold");
let root = k.chain().root().expect("the chain stands on its fold root");
assert_eq!((root.k, root.hash, root.height), (2, head_hash, head_height));
assert_eq!(k.chain().receipts().len(), 1, "one hot receipt: the fold");
let fold = &k.chain().receipts()[0];
assert_eq!((fold.kind, fold.step_index, fold.prev_receipt_hash), (ActKind::Fold, 3, head_hash));
assert_eq!(fold.delta, head_hash.to_vec(), "the fold receipt cites the archived prefix's head");
assert!(k.chain().verify_chain_integrity(), "the hot chain re-verifies from its root");
assert_eq!(k.k_step(), 4, "K continues past the fold");
assert_eq!(k.archive().len(), 1);
let era = &k.archive()[0];
assert_eq!((era.root, era.receipts.len(), era.first_k()), (root, 3, Some(0)));
assert!(
    crate::chronometry::ReceiptChain::from_receipts(era.receipts.clone()).is_some(),
    "the archived prefix re-verifies as a chain of its own"
);
assert_eq!(k.treasury().audit().expect("conserves after"), before);

let snap = k.snapshot();
let mut back = Kernel::new();
assert!(back.restore(&snap), "the snapshot carries the root and the era");
assert_eq!(back.chain().root(), Some(root));
assert_eq!(back.chain().receipts(), k.chain().receipts());
assert_eq!(back.archive(), k.archive());
assert!(back.chain().verify_chain_integrity());

let again = k.fold_era().expect("the fold receipt is itself final");
assert_eq!((again.k0, again.k1, again.k), (3, 3, 4));
assert_eq!(k.archive().len(), 2);
assert_eq!(k.chain().root().map(|r| r.k), Some(3));

let mut open = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
assert_eq!(open.ingest(&a, &b, [7; 32]), Outcome::GermPending);
assert_eq!(open.final_prefix_head(), None, "a listing open at the first receipt leaves nothing final");
assert_eq!(open.fold_era(), Err(REFUSE_FOLD_OPEN));
assert!(open.chain().root().is_none(), "nothing was archived");
assert!(open.archive().is_empty());

let mut broken = Kernel::new();
broken.append_step(ActKind::List, [1; 32], [2; 32]);
broken.treasury.credit([9; 32], [1, 0, 0, 0]);
assert_eq!(broken.fold_era(), Err(REFUSE_FOLD_INEXACT), "a ledger that does not conserve folds nothing");
assert!(broken.chain().root().is_none(), "and nothing was archived or sealed");
assert_eq!(broken.chain().receipts().len(), 1);
}
