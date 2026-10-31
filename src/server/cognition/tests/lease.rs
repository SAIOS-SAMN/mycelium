// MU-ECONOMY §5.2 — THE COMPUTE LEASE: a payer holds μ against a provider's
// capacity over named jobs; each job the provider's verified solve closes
// pays the provider that solve's measured discharge out of the hold, capped
// by what is left; the rest returns to the payer when the lease ends.

use super::common::*;
use crate::anastome::dialogue::lease::{CloseLease, LeaseAct, LeaseRefusal};
use crate::cognition::*;
use crate::transducer::family::{cipher_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::as_element_form;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// A rule element for `pay_solve` to settle (the replay that measured its
/// discharge is the caller's).
fn a_rule() -> crate::transducer::element::Element {
    let family: Vec<TrainPair> = vec![(digits(&["1100200"]), digits(&["0011002"]))];
    let [(x, y)] = family.as_slice() else { panic!("one pair") };
    let f = cipher_form(x, y);
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("every body is spoken");
    as_element_form(&f, mag).expect("a tape")
}

/// A court holding `fund` on the payer's balance, and a lease of `mu` the
/// payer signs for `provider` over `jobs` for `window` heights.
fn court_and_lease(fund: [u64; 4], mu: [u64; 4], provider: [u8; 32], jobs: Vec<[u8; 32]>, window: u64) -> (Court, ed25519_dalek::SigningKey, LeaseAct) {
    let mut k = Court::new();
    let payer = key(1);
    let who = payer.verifying_key().to_bytes();
    k.fund(who, fund);
    let act = LeaseAct { payer: who, provider, mu, window, jobs, nonce: [2; 32] };
    (k, payer, act)
}

/// A LEASE HOLDS ITS VALUE AND PAYS VERIFIED JOBS, AND ONLY THEM: opening it
/// moves `mu` from the payer's balance into the hold; a verified solve of a
/// leased job pays the provider that solve's discharge out of the hold; a
/// second solve of the same job, and a solve by another voice, pay nothing
/// from it; and the ledger's invariant holds throughout.
#[test]
fn pay_lease_pays_each_job_once() {
    let (provider, other) = ([9u8; 32], [8u8; 32]);
    let (job_a, job_b) = ([7u8; 32], [6u8; 32]);
    let (mut k, payer, act) = court_and_lease([50, 40, 0, 30], [30, 20, 0, 10], provider, vec![job_a, job_b], 1_000);
    let who = act.payer;
    let (id, held) = k.open_lease(&act.clone().sign(&payer).expect("signs")).expect("the lease opens");
    assert_eq!(held, [30, 20, 0, 10]);
    assert_eq!(k.treasury().balance_mu(&who), [20, 20, 0, 20], "the hold left the payer's balance");
    assert!(k.treasury().audit().is_ok(), "held value is supply");

    let rule = a_rule();
    let discharge = [12, 25, 0, 4];
    let _ = k.pay_solve(&job_a, other, &rule, discharge, false);
    assert_eq!(k.treasury().balance_mu(&other), [0; 4], "another voice's solve is not the lease's");
    let _ = k.pay_solve(&job_a, provider, &rule, discharge, false);
    // Lane G asked 25 of a hold of 20: capped at what was held.
    assert_eq!(k.treasury().balance_mu(&provider), [12, 20, 0, 4], "the job's discharge, capped by the hold");
    let _ = k.pay_solve(&job_a, provider, &rule, discharge, false);
    assert_eq!(k.treasury().balance_mu(&provider), [12, 20, 0, 4], "a job is paid once");
    assert!(k.leases().iter().any(|l| l.id == id && l.jobs == 1), "one job left");
    assert!(k.treasury().audit().is_ok(), "the payment is a transfer: supply unchanged");
}

/// WHAT A LEASE HAS LEFT RETURNS TO ITS PAYER: on the payer's close, and on
/// the window's passing at the court stroke.
#[test]
fn close_lease_returns_remainder_as_lapse_does() {
    let provider = [9u8; 32];
    let (mut k, payer, act) = court_and_lease([50, 40, 0, 30], [30, 20, 0, 10], provider, vec![[7; 32]], 1_000);
    let who = act.payer;
    let opened = act.sign(&payer).expect("signs");
    let (id, _) = k.open_lease(&opened).expect("the lease opens");
    let close = CloseLease { payer: who, lease: id, nonce: [3; 32] }.sign(&payer);
    assert_eq!(k.close_lease(&close), Ok((id, [30, 20, 0, 10])), "the whole hold returns");
    assert_eq!(k.treasury().balance_mu(&who), [50, 40, 0, 30], "the payer holds what it held before");
    assert!(k.leases().is_empty());
    assert_eq!(k.close_lease(&close), Err(LeaseRefusal::Spent), "a resend moves nothing");
    assert_eq!(k.open_lease(&opened), Err(LeaseRefusal::Spent), "the closed lease's own signed open, resent, opens nothing");
    assert_eq!(k.treasury().balance_mu(&who), [50, 40, 0, 30], "and holds nothing again");

    // A window of one height: passed once the court moves past it.
    let lapsing = LeaseAct { payer: who, provider, mu: [5, 0, 0, 0], window: 1, jobs: vec![[6; 32]], nonce: [4; 32] };
    let _ = k.open_lease(&lapsing.sign(&payer).expect("signs")).expect("the lease opens");
    assert_eq!(k.treasury().balance_mu(&who), [45, 40, 0, 30]);
    let mut steps = 0;
    while !k.leases().is_empty() && steps < 8 {
        k.append_step(crate::chronometry::ActKind::Settled, [1], who);
        let _ = k.lapse_leases();
        steps += 1;
    }
    assert!(k.leases().is_empty(), "the window passed and the lease ended");
    assert_eq!(k.treasury().balance_mu(&who), [50, 40, 0, 30], "its hold returned");
    assert!(k.treasury().audit().is_ok());
}

/// A LEASE IS REFUSED BY NAME: unsigned, short, with no job, to oneself, or
/// naming a job already leased to that provider — and a refused lease holds
/// nothing.
#[test]
fn open_lease_refuses_unsigned_short_or_repeated_lease() {
    let provider = [9u8; 32];
    let (mut k, payer, act) = court_and_lease([10, 10, 10, 10], [5, 0, 0, 0], provider, vec![[7; 32]], 100);
    let who = act.payer;
    let forged = act.clone().sign(&key(4)).expect("signs");
    assert_eq!(k.open_lease(&forged), Err(LeaseRefusal::Unsigned));
    let short = LeaseAct { mu: [11, 0, 0, 0], nonce: [5; 32], ..act.clone() };
    assert_eq!(k.open_lease(&short.sign(&payer).expect("signs")), Err(LeaseRefusal::Short));
    let none = LeaseAct { jobs: vec![], nonce: [6; 32], ..act.clone() };
    assert_eq!(k.open_lease(&none.sign(&payer).expect("signs")), Err(LeaseRefusal::Malformed));
    let oneself = LeaseAct { provider: who, nonce: [7; 32], ..act.clone() };
    assert_eq!(k.open_lease(&oneself.sign(&payer).expect("signs")), Err(LeaseRefusal::Oneself));
    assert!(k.open_lease(&act.clone().sign(&payer).expect("signs")).is_ok());
    let again = LeaseAct { nonce: [8; 32], ..act };
    assert_eq!(k.open_lease(&again.sign(&payer).expect("signs")), Err(LeaseRefusal::Leased), "one lease per provider and job");
    assert_eq!(k.treasury().balance_mu(&who), [5, 10, 10, 10], "only the lease that opened holds");
}
