// THE SOLVE, JUDGED BY ITS ACTION AND PAID BY ITS DISCHARGE (the actions'
// physics, 2026-09-29): the court replays a solve's rule on the withheld
// pairs, prices what it discharges per lane, mints it on the task's first
// closure only, and refuses by the replay's reason and kind.

use super::common::*;
use crate::cognition::*;
use crate::transducer::family::{cipher_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::as_element_form;

/// Two separated bodies, both carried two columns along (70.4).
fn carried() -> Vec<TrainPair> {
    vec![(digits(&["1100200"]), digits(&["0011002"]))]
}

/// The same scene with the lone site landing another hue: no motion the
/// rule speaks carries it there.
fn recoloured() -> Vec<TrainPair> {
    vec![(digits(&["1100200"]), digits(&["0011003"]))]
}

/// The voice's rule over a one-pair family, measured and spoken.
fn rule_over(family: &[TrainPair]) -> Element {
    let [(x, y)] = family else { panic!("one pair") };
    let f = cipher_form(x, y);
    let mag = sentence_measure(&f, family, &NoCrystal, 0).expect("every body is spoken");
    as_element_form(&f, mag).expect("a tape")
}

/// A rule that carries the withheld pair closes it: the verdict is the
/// replay's, clean, and the price is what the rule discharges there — not
/// zero, and not a grid compared.
#[test]
fn a_rule_that_closes_the_withheld_pairs_is_solved_and_priced_by_its_discharge() {
    let mut k = Kernel::new();
    let rule = rule_over(&carried());
    match k.judge_solve(&rule, &carried()) {
        SolveVerdict::Solved { pay } => assert!(pay.iter().any(|&m| m > 0), "it discharges something: {pay:?}"),
        other => panic!("the replay closes every pair: {other:?}"),
    }
}

/// A rule that does not carry a withheld pair is REFUSED by the replay,
/// with its reason and kind — the coordinate the voice learns from.
#[test]
fn a_rule_that_leaves_a_pair_open_is_refused_by_the_replay() {
    let mut k = Kernel::new();
    let rule = rule_over(&carried());
    match k.judge_solve(&rule, &recoloured()) {
        SolveVerdict::Refused(r) => assert!(!r.reason.is_empty(), "the refusal names its reason"),
        other => panic!("the recoloured site stays open: {other:?}"),
    }
    assert!(matches!(k.judge_solve(&rule, &[]), SolveVerdict::Unverifiable), "no withheld answer, no verdict");
}

/// POUW: the task's FIRST closure mints the solver what its rule discharged;
/// a closure AGAIN is verified and pays nothing — no floor.
#[test]
fn the_first_closure_mints_the_discharge_and_a_closure_again_pays_nothing() {
    let mut k = Kernel::new();
    let rule = rule_over(&carried());
    let SolveVerdict::Solved { pay } = k.judge_solve(&rule, &carried()) else { panic!("solved") };
    let (task, voice) = ([5u8; 32], [9u8; 32]);
    let first = k.pay_solve(&task, voice, &rule, pay, true);
    assert_eq!(first, pay, "the first closure is paid its discharge");
    assert_eq!(k.treasury.balance_mu(&voice), pay, "minted to the solver");
    let again = k.pay_solve(&task, voice, &rule, pay, false);
    assert_eq!(again, [0; 4], "a closure again pays nothing");
    assert_eq!(k.treasury.balance_mu(&voice), pay, "and the balance stands");
}
