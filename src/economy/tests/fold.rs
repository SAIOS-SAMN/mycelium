// 45.8, §2, §6 across a fold — the prefix re-expressed as epoch rows, and
// what a fold refuses.

use super::{at, place, Counts};
use crate::economy::*;
use crate::verdict::REFUSE_FOLD_INEXACT;

/// The rows at or below the fold's K are replaced by one `epoch` row per
/// identity with a nonzero net, anchored to the fold receipt; the audit
/// holds before and after and the supply is unchanged; the net is what
/// the rows moved — a transfer's two halves net to the halves' two identities,
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
    assert_eq!(folded, Folded { replaced: 6, rows: 2, net: Mu::counts([13, 10, 12, 10]) });
    let epochs: Vec<&Entry> = t.entries().iter().filter(|e| e.kind == Kind::Epoch).collect();
    assert_eq!(
        epochs.iter().map(|e| (e.who, e.mu.clone())).collect::<Vec<_>>().n(),
        vec![([1; 32], [7, 10, 10, 10]), ([2; 32], [6, 0, 2, 0])],
        "identity 1: genesis less the transfer out, its stake netting nothing; identity 2: mint plus the transfer in less the burn"
    );
    assert!(epochs.iter().all(|e| (e.at, e.receipt) == a(7)), "anchored to the fold receipt");
    assert_eq!(t.entries().len(), 3, "two epoch rows and the mint above the fold");
    assert_eq!((t.entries()[2].kind, t.entries()[2].who), (Kind::Mint, [3; 32]), "the row above the fold stands");
    assert_eq!(t.balance_mu(&[1; 32]), [5, 8, 8, 8], "the carriers did not move");
    assert_eq!(t.bond_of(&[1; 32]).n(), Some([2, 2, 2, 2]));
    assert_eq!(t.minted(), [4, 0, 2, 5], "the folded subsidy read at the checkpoint, the mint above the fold on its row");

    let back = super::round_trip(&t).expect("the epoch rows read back");
    assert_eq!(back.entries(), t.entries());
    assert_eq!(back.audit().expect("a restored fold conserves"), before);

    let again = t.fold(place(7), a(8)).expect("a fold over epoch rows folds");
    assert_eq!(again.rows, 3, "one epoch row per identity, the mint above joining");
    assert_eq!(t.audit().expect("still conserves"), before);

    let mut broken = Treasury::new();
    broken.genesis([1; 32], [10, 0, 0, 0], a(1));
    // A credit with no row: the carriers pass what the journal issued.
    broken.credit([1; 32], &Mu::counts([1, 0, 0, 0]));
    assert_eq!(broken.fold(place(1), a(2)), Err(REFUSE_FOLD_INEXACT));
    assert_eq!(broken.entries().len(), 1, "a ledger that does not conserve writes nothing");
}

/// The anchor of act `k`, its hash the act's own byte.
fn anchor(k: u64) -> Anchor {
    at(k, [u8::try_from(k).expect("the test's heights are small literals, each one byte"); 32])
}

/// A LEDGER THAT PAID EVERY PROOF AND BURNED: a genesis allocation and a
/// bond; a subsidy (no proof); POUW for a solve; POWR minted; POWC paid as
/// one use, a transfer's two halves; a burn; then a POUW mint above `K = 7`,
/// past the fold's head.
fn paid_every_proof() -> Treasury {
    let mut t = Treasury::new();
    t.genesis([1; 32], [10, 10, 10, 10], anchor(1));
    t.enroll([1; 32], [2, 2, 2, 2], anchor(1));
    t.settle_subsidy([2; 32], [4, 0, 2, 0], anchor(2));
    t.settle_solve([2; 32], [0, 3, 0, 1], anchor(3));
    t.mint_to([3; 32], [1, 1, 0, 0], anchor(4), Some(Arm::Powr));
    assert_eq!(t.settle_use([1; 32], [2; 32], [3, 0, 0, 0], anchor(5)), [3, 0, 0, 0]);
    assert_eq!(t.burn([2; 32], [1, 0, 0, 0], anchor(6)), [1, 0, 0, 0]);
    t.settle_solve([3; 32], [0, 0, 0, 5], anchor(8));
    t
}

/// MU-ECONOMY §6.1, §0 — A FOLD REMOVES NOTHING FROM WHAT EACH PROOF PAID:
/// what POUW, POWC, POWR and POWV paid and burned reads the same after a
/// fold as before it, the folded prefix's payments read at the checkpoint.
#[test]
fn fold_keeps_what_each_proof_paid() {
    let mut t = paid_every_proof();
    let before = t.economy(&[9; 32]).expect("the economy reads");
    assert_eq!(before.paid[Arm::Pouw.index()], [0, 3, 0, 6], "the two solves");
    assert_eq!(before.paid[Arm::Powc.index()], [3, 0, 0, 0], "the use, one transfer");
    assert_eq!(before.paid[Arm::Powr.index()], [1, 1, 0, 0]);
    t.fold(place(7), anchor(9)).expect("a conserving prefix folds");
    let after = t.economy(&[9; 32]).expect("the economy reads after the fold");
    assert_eq!(after.paid, before.paid, "each proof's payments, the folded ones read at the checkpoint");
    assert_eq!(after.burned, before.burned);
}

/// §6 — GENESIS STAYS SEPARABLE ACROSS A FOLD: what was earned (the mint
/// rows, every proof and the subsidy) and what was issued read the same
/// after the fold, and issued less earned is the genesis allocation.
#[test]
fn fold_keeps_genesis_apart_from_the_earned() {
    let mut t = paid_every_proof();
    let (minted, issued) = (t.minted(), t.issued().clone());
    assert_eq!(minted, [5, 4, 2, 6], "subsidy, two solves and POWR");
    t.fold(place(7), anchor(9)).expect("a conserving prefix folds");
    assert_eq!(t.minted(), minted, "the earned supply, the folded mints read at the checkpoint");
    assert_eq!(t.issued(), &issued, "issuance only rises, and a fold moves none");
    assert_eq!(t.issued().draw(&t.minted()), Ok(Mu::counts([10, 10, 10, 10])), "issued less earned is the genesis allocation");
}

/// §6 — A FOLD MOVES NO ISSUANCE AND NO BURN: issued and burned each read
/// the same after a fold, never netted into one another, so the supply is
/// their difference before and after.
#[test]
fn fold_keeps_issued_and_burned_each() {
    let mut t = paid_every_proof();
    let (issued, supply) = (t.issued().clone(), t.supply().expect("the supply reads"));
    t.fold(place(7), anchor(9)).expect("a conserving prefix folds");
    assert_eq!(t.issued(), &issued, "the burn is not netted out of what was issued");
    assert_eq!(t.supply().as_ref(), Some(&supply));
    assert_eq!(t.issued().draw(&supply), Ok(Mu::counts([1, 0, 0, 0])), "what was burned, read apart");
}

/// A SECOND FOLD OVER THE FIRST'S EPOCH ROWS reads the first fold's
/// issuance once: the epoch rows are nets of balances, never issuance.
#[test]
fn second_fold_reads_the_first_folds_issuance_once() {
    let mut t = paid_every_proof();
    let (paid, minted, issued) = (t.economy(&[9; 32]).expect("reads").paid, t.minted(), t.issued().clone());
    t.fold(place(7), anchor(9)).expect("the first fold");
    t.settle_solve([4; 32], [1, 0, 0, 0], anchor(10));
    t.fold(place(11), anchor(12)).expect("the second fold, over the first's epoch rows");
    let mut paid_now = paid;
    paid_now[Arm::Pouw.index()].step(&Mu::counts([1, 0, 0, 0]));
    assert_eq!(t.economy(&[9; 32]).expect("reads").paid, paid_now);
    assert_eq!(t.minted(), minted.add(&Mu::counts([1, 0, 0, 0])));
    assert_eq!(t.issued(), &issued.add(&Mu::counts([1, 0, 0, 0])));
}

/// A CHECKPOINT SOURCE IS A ROW'S OWN COORDINATE `[kind tag, arm byte]`: a
/// kind that issues and burns nothing, a proof on genesis or a slash, or an
/// arm byte naming no arm names no source, so a tape carrying one refuses
/// whole rather than reading it as another source.
#[test]
fn checkpoint_source_at_no_coordinate_is_refused() {
    let mut f = crate::economy::fold::FoldedIssuance::default();
    assert!(f.source_mut([Kind::Stake.tag(), 0]).is_none(), "a stake issues nothing");
    assert!(f.source_mut([Kind::Epoch.tag(), 0]).is_none(), "an epoch row is a net of balances");
    assert!(f.source_mut([Kind::Genesis.tag(), Arm::byte(Some(Arm::Pouw))]).is_none(), "genesis carries no proof");
    assert!(f.source_mut([Kind::Mint.tag(), 5]).is_none(), "no arm has byte 5");
    *f.source_mut([Kind::Mint.tag(), Arm::byte(Some(Arm::Powc))]).expect("a POWC mint is a source") = Mu::counts([1, 0, 0, 0]);
    assert_eq!(f.minted[usize::from(Arm::byte(Some(Arm::Powc)))], [1, 0, 0, 0]);
    assert_eq!(f.sources().len(), 1, "only the source that holds something is written");
}

/// THE CHECKPOINT IS PERSISTED with the carriers: a treasury restored after
/// a fold reads each proof's payments, the earned supply and the issuance
/// as the folded one does, and conserves.
#[test]
fn restored_fold_keeps_its_issuance() {
    let mut t = paid_every_proof();
    t.fold(place(7), anchor(9)).expect("a conserving prefix folds");
    let back = super::round_trip(&t).expect("the folded treasury reads back");
    assert_eq!(back.economy(&[9; 32]).expect("reads").paid, t.economy(&[9; 32]).expect("reads").paid);
    assert_eq!(back.minted(), t.minted());
    assert_eq!(back.issued(), t.issued());
    assert_eq!(back.audit(), t.audit());
}
