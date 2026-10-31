// E3 — THE SYNTHETIC ECONOMY: a payment moves μ⃗ between two balances and
// a hold sets it aside against a key until it is released or refunded; both
// equations of §6 hold through every movement, a refused movement writes no
// row, and the held carrier survives the treasury's persisted form.

use super::at;
use crate::economy::{Kind, Treasury, Unpaid};

const A: [u8; 32] = [1; 32];
const B: [u8; 32] = [2; 32];

/// A treasury where `A` was allocated `[10, 4, 0, 6]` at genesis.
fn funded() -> Treasury {
    let mut t = Treasury::new();
    t.genesis(A, [10, 4, 0, 6], at(1, [9; 32]));
    t
}

/// A PAYMENT IS WHOLE OR REFUSED, AND CONSERVES. The payee holds what the
/// payer gave, per lane; a payer short on any lane moves nothing and is told
/// which lanes; a payment to oneself or of nothing is no movement. The
/// supply equals the carriers before and after.
#[test]
fn pay_moves_between_balances_or_refuses() {
    let mut t = funded();
    let supply = t.audit().expect("the genesis conserves");
    t.pay(A, B, [3, 1, 0, 2], at(2, [8; 32])).expect("A holds it");
    assert_eq!((t.balance_mu(&A), t.balance_mu(&B)), ([7, 3, 0, 4], [3, 1, 0, 2]));
    assert_eq!(t.audit(), Ok(supply), "a payment issues nothing and burns nothing");
    let rows = t.entries().len();
    assert_eq!(t.pay(B, A, [4, 0, 0, 1], at(3, [7; 32])), Err(Unpaid::Short([1, 0, 0, 0])), "short on S by one");
    assert_eq!(t.pay(A, A, [1, 0, 0, 0], at(3, [7; 32])), Err(Unpaid::Oneself));
    assert_eq!(t.pay(A, B, [0; 4], at(3, [7; 32])), Err(Unpaid::Nothing));
    assert_eq!(t.entries().len(), rows, "a refused payment writes no row");
    assert_eq!(t.balance_mu(&B), [3, 1, 0, 2], "and moves nothing");
}

/// A HOLD IS THE PAYER'S UNTIL IT MOVES. Set aside, it leaves the payer's
/// balance and the audit still holds, counting it as a carrier; released to
/// a payee it is paid on as one transfer; refunded it returns whole. A key
/// already held refuses a second hold; a key with nothing held releases
/// nothing.
#[test]
fn hold_releases_to_payee_or_refunds_and_conserves() {
    let mut t = funded();
    let supply = t.audit().expect("the genesis conserves");
    t.hold(A, b"task", [5, 2, 0, 1], at(2, [8; 32])).expect("A holds it");
    assert_eq!(t.balance_mu(&A), [5, 2, 0, 5], "the hold left the balance");
    assert_eq!(t.audit(), Ok(supply), "and the held carrier is counted");
    assert_eq!(t.hold(A, b"task", [1, 0, 0, 0], at(3, [7; 32])), Err(Unpaid::Taken));
    // THE ECONOMY READS IT, typed: A's standing with what it holds.
    let e = t.economy(&[0; 32]).expect("the reading adds up");
    let [a] = e.accounts.as_slice() else { panic!("one identity: {:?}", e.accounts) };
    assert_eq!((a.who, a.balance, a.held), (A, [5, 2, 0, 5], [5, 2, 0, 1]));
    assert_eq!(t.held_of(b"task").map(|h| (h.payer, h.mu)), Some((A, [5, 2, 0, 1])));
    assert_eq!(t.release_held(b"task", B, at(3, [7; 32])), Ok([5, 2, 0, 1]));
    assert_eq!((t.balance_mu(&A), t.balance_mu(&B)), ([5, 2, 0, 5], [5, 2, 0, 1]), "paid on to B");
    assert!(t.held_of(b"task").is_none(), "the hold is gone");
    assert_eq!(t.release_held(b"task", B, at(4, [6; 32])), Err(Unpaid::Unheld));
    assert_eq!(t.audit(), Ok(supply));
    // Refunded: back to the payer, whole.
    t.hold(A, b"bounty", [2, 0, 0, 2], at(4, [6; 32])).expect("A holds it");
    assert_eq!(t.refund_held(b"bounty", at(5, [5; 32])), Ok([2, 0, 0, 2]));
    assert_eq!(t.balance_mu(&A), [5, 2, 0, 5], "the refund returned it");
    assert_eq!(t.audit(), Ok(supply));
    // A short payer holds nothing.
    assert_eq!(t.hold(B, b"more", [6, 0, 0, 0], at(5, [5; 32])), Err(Unpaid::Short([1, 0, 0, 0])));
}

/// THE FOLD READS THE MOVEMENTS EXACTLY. A release is one transfer pair and
/// a hold or refund nets within its payer, so the ledger's prefix folds to
/// epoch rows that conserve — the payee's net is what it was paid.
#[test]
fn nets_of_nets_payments_and_holds() {
    let mut t = funded();
    t.pay(A, B, [1, 1, 0, 1], at(2, [8; 32])).expect("paid");
    t.hold(A, b"k", [2, 0, 0, 0], at(3, [7; 32])).expect("held");
    t.release_held(b"k", B, at(4, [6; 32])).expect("released");
    let nets = crate::economy::nets_of(t.entries()).expect("the rows net exactly");
    assert_eq!(nets.rows, vec![(A, [7, 3, 0, 5]), (B, [3, 1, 0, 1])], "each identity's net is its balance");
    assert!(t.entries().iter().any(|e| e.kind == Kind::Escrow), "the hold stands as its row");
}

/// THE HELD CARRIER IS PERSISTED: a hold standing when the treasury is
/// written reads back whole — payer, amount and act — and the audit of the
/// restored treasury holds.
#[test]
fn hold_survives_treasury_round_trip() {
    let mut t = funded();
    t.hold(A, b"open", [3, 1, 0, 2], at(2, [8; 32])).expect("held");
    let back = super::round_trip(&t).expect("the treasury reads back");
    assert_eq!(back.held_of(b"open"), t.held_of(b"open"), "the hold, whole");
    assert_eq!(back.audit(), t.audit(), "and the restored ledger conserves");
}
