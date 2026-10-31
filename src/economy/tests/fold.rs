// 45.8, §2, §6 across a fold — the prefix re-expressed as epoch rows, and
// what a fold refuses.

use super::{at, place};
use crate::economy::*;
use crate::verdict::REFUSE_FOLD_INEXACT;

/// The rows at or below the fold's K are replaced by one `epoch` row per
/// identity with a nonzero net, anchored to the fold receipt; the audit
/// holds before and after and the supply is unchanged; the net is what
/// the rows moved — a transfer's two halves net to their two identities,
/// a stake nets nothing; rows above the fold stay in order; the tape
/// carries the epoch rows; a ledger that does not conserve folds nothing.
#[test]
fn fold_replaces_prefix_with_epoch_rows_and_conserves() {
    let a = |k: u64| at(k, [u8::try_from(k).expect("the test's heights are small literals, each one byte"); 32]);
    let mut t = Treasury::new();
    t.genesis([1; 32], [10, 10, 10, 10], a(1));
    t.enroll([1; 32], [2, 2, 2, 2], a(1));
    t.settle_subsidy([2; 32], [4, 0, 2, 0], a(2));
    assert_eq!(t.settle_use([1; 32], [2; 32], [3, 0, 0, 0], a(3)), [3, 0, 0, 0]);
    assert_eq!(t.burn([2; 32], [1, 0, 0, 0], a(4)), [1, 0, 0, 0]);
    t.settle_subsidy([3; 32], [0, 0, 0, 5], a(6));
    let before = t.audit().expect("conserves before the fold");
    let rows = t.entries().len();
    assert_eq!(rows, 7, "genesis, stake, mint, two transfer halves, burn, and a mint above the fold");

    let folded = t.fold(place(5), a(7)).expect("a conserving prefix folds");
    assert_eq!(t.audit().expect("conserves after the fold"), before, "the supply is unchanged");
    assert_eq!(folded, Folded { replaced: 6, rows: 2, net: [13, 10, 12, 10] });
    let epochs: Vec<&Entry> = t.entries().iter().filter(|e| e.kind == Kind::Epoch).collect();
    assert_eq!(
        epochs.iter().map(|e| (e.who, e.mu)).collect::<Vec<_>>(),
        vec![([1; 32], [7, 10, 10, 10]), ([2; 32], [6, 0, 2, 0])],
        "identity 1: genesis less the transfer out, its stake netting nothing; identity 2: mint plus the transfer in less the burn"
    );
    assert!(epochs.iter().all(|e| (e.at, e.receipt) == a(7)), "anchored to the fold receipt");
    assert_eq!(t.entries().len(), 3, "two epoch rows and the mint above the fold");
    assert_eq!((t.entries()[2].kind, t.entries()[2].who), (Kind::Mint, [3; 32]), "the row above the fold stands");
    assert_eq!(t.balance_mu(&[1; 32]), [5, 8, 8, 8], "the carriers did not move");
    assert_eq!(t.bond_of(&[1; 32]), Some([2, 2, 2, 2]));
    assert_eq!(t.minted(), Some([0, 0, 0, 5]), "the folded mints now stand in the epoch rows");

    let back = super::round_trip(&t).expect("the epoch rows read back");
    assert_eq!(back.entries(), t.entries());
    assert_eq!(back.audit().expect("a restored fold conserves"), before);

    let again = t.fold(place(7), a(8)).expect("a fold over epoch rows folds");
    assert_eq!(again.rows, 3, "one epoch row per identity, the mint above joining");
    assert_eq!(t.audit().expect("still conserves"), before);

    let mut broken = Treasury::new();
    broken.genesis([1; 32], [10, 0, 0, 0], a(1));
    assert!(broken.credit([1; 32], [1, 0, 0, 0]), "a credit with room lands");
    assert_eq!(broken.fold(place(1), a(2)), Err(REFUSE_FOLD_INEXACT));
    assert_eq!(broken.entries().len(), 1, "a ledger that does not conserve writes nothing");
}
