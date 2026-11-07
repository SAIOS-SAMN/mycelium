// K3 (78.177 §4a(1), ruled 2026-11-06) — A VERIFIED CONVERGENCE IS PAID A
// POWC SHARE OUT OF THE INSTALL'S COMPOSITION PREMIUM, after the base and use
// shares; several split it by their share of the frame; with no premium left
// it is held as a claim, never minted.

use super::{at, word};
use crate::economy::*;

fn powc(t: &Treasury, who: [u8; 32]) -> Mu {
    Mu::sum(t.entries().iter().filter(|e| e.who == who && e.kind == Kind::Mint && e.arm == Some(Arm::Powc)).map(|e| &e.mu))
}

/// K3 — A VERIFIED CONVERGENCE IS PAID FROM THE COMPOSITION PREMIUM. An
/// install of `[8,0,4,0]` whose use share takes `[2,0,0,0]` and whose
/// premium is `[4,0,2,0]`: the converging mind is minted its share of the
/// frame `[3,0,0,0]` as POWC out of the premium, the voice keeps the rest of
/// the premium, and nothing is issued beyond the premium and the share.
#[test]
fn verified_convergence_is_paid_from_the_composition_premium() {
    let (voice, cited, mind) = ([1; 32], [2; 32], [3; 32]);
    let mut t = Treasury::new();
    let addr = [7u8; 32];
    t.escrow_award(&word(addr), vec![voice], [8, 0, 4, 0], at(1, [9; 32]));
    let premium = Mu::counts([4, 0, 2, 0]);
    let taken = t.settle_recursive(&addr, &[(cited, Mu::counts([2, 0, 0, 0]))], &Mu::counts([8, 0, 4, 0]), Some(premium), &[(mind, Mu::counts([3, 0, 0, 0]))], at(2, [9; 32]));
    assert_eq!(taken, [4, 0, 2, 0], "the premium is what the composition took");
    assert_eq!(powc(&t, mind), [3, 0, 0, 0], "the converging mind is paid its share of the frame, as POWC");
    assert_eq!(t.balance_mu(&voice), [1, 0, 2, 0], "the voice keeps the rest of the premium");
    assert_eq!(t.balance_mu(&cited), [2, 0, 0, 0], "the use share is taken out first, untouched");
    assert_eq!(t.supply().expect("supply adds up"), [6, 0, 2, 0], "never added to the mint: shares and premium only");
}

/// K3 — SEVERAL CONVERGING MINDS SPLIT THE PREMIUM BY THEIR SHARE OF THE
/// FRAME. Shares `[3,..]` and `[1,..]` pass a premium of `[2,..]`: they
/// take `3/2` and `1/2`, exactly, and the voice is left nothing of it.
#[test]
fn converging_minds_split_the_premium_by_their_share_of_the_frame() {
    let (voice, a, b) = ([1; 32], [4; 32], [5; 32]);
    let mut t = Treasury::new();
    let addr = [8u8; 32];
    t.escrow_award(&word(addr), vec![voice], [2, 0, 0, 0], at(1, [9; 32]));
    let converged = [(a, Mu::counts([3, 0, 0, 0])), (b, Mu::counts([1, 0, 0, 0]))];
    let _ = t.settle_recursive(&addr, &[], &Mu::counts([2, 0, 0, 0]), Some(Mu::counts([2, 0, 0, 0])), &converged, at(2, [9; 32]));
    let half = |n: i64| crate::core::q::Q::new(n.into(), 2.into());
    let zero = || crate::core::q::Q::from_integer(0.into());
    assert_eq!(powc(&t, a), Mu::of([half(3), zero(), zero(), zero()]).expect("exact"), "three parts of four");
    assert_eq!(powc(&t, b), Mu::of([half(1), zero(), zero(), zero()]).expect("exact"), "one part of four");
    assert!(t.balance_mu(&voice).is_zero(), "the premium went to the converging minds");
}

/// K3 — A CONVERGENCE WITH NO PREMIUM LEFT IS HELD AS A CLAIM. A word's
/// install has no composition premium: its voice takes what is left, the
/// converging mind is minted nothing, and its share stands as a claim.
#[test]
fn convergence_with_no_premium_left_is_held_as_a_claim() {
    let (voice, mind) = ([1; 32], [6; 32]);
    let mut t = Treasury::new();
    let addr = [9u8; 32];
    t.escrow_award(&word(addr), vec![voice], [4, 0, 0, 0], at(1, [9; 32]));
    let _ = t.settle_recursive(&addr, &[], &Mu::counts([4, 0, 0, 0]), None, &[(mind, Mu::counts([4, 0, 0, 0]))], at(2, [9; 32]));
    assert!(powc(&t, mind).is_zero(), "never minted");
    assert_eq!(t.balance_mu(&voice), [4, 0, 0, 0], "the discoverer keeps the whole release");
    let claim = t.escrow_of(converged::convergence_claim(&addr, &mind).as_bytes()).expect("held as a claim");
    assert_eq!(claim.voices, vec![mind]);
    assert_eq!(claim.mu, [4, 0, 0, 0], "at its share of the frame");
}
