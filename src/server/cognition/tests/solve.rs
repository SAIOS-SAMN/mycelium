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

/// 78.53 — COLOUR IS A GAUGE, READ ONLY AT THE SENSOR. One task and the
/// same task with its palette permuted (1→7, 2→4, 3→9) are one scene: read
/// through the sensor they are the SAME grids, so the voice's rule mints the
/// IDENTICAL word — address and all — and the court's verdict and price are
/// identical. Read without the sensor's fixing, the two rules are two
/// addresses: the gauge was in the bytes.
#[test]
fn a_task_and_its_palette_permuted_read_one_word_and_one_verdict() {
    let json = |a: u8, b: u8, c: u8| {
        format!(
            r#"{{"train":[{{"input":[[{a},{a},0,{b},0,0,0]],"output":[[0,0,{a},{a},0,{b},0]]}}],
                "test":[{{"input":[[{b},0,{a},{a},0,{c},0]],"output":[[0,0,{b},0,{a},{a},{c}]]}}]}}"#
        )
    };
    let task = crate::arc::parse_task(&json(1, 2, 3)).expect("the task reads");
    let permuted = crate::arc::parse_task(&json(7, 4, 9)).expect("the permuted task reads");
    assert_eq!((&task.train, &task.test), (&permuted.train, &permuted.test), "one scene through the sensor");
    let (rule, again) = (rule_over(&task.train), rule_over(&permuted.train));
    assert_eq!(rule.address(), again.address(), "one word");
    let mut k = Kernel::new();
    let verdict = |k: &mut Kernel, r: &Element, t: &crate::arc::Task| match k.judge_solve(r, &t.train) {
        SolveVerdict::Solved { pay } => Some(pay),
        _ => None,
    };
    let v = verdict(&mut k, &rule, &task);
    assert!(v.is_some(), "the rule closes its own pair");
    assert_eq!(v, verdict(&mut k, &again, &permuted), "one verdict, one price");
    // WITHOUT THE SENSOR'S FIXING the palette is in the word's bytes.
    let raw = |a: u8, b: u8| vec![(digits(&[&format!("{a}{a}0{b}000")]), digits(&[&format!("00{a}{a}0{b}0")]))];
    assert_ne!(rule_over(&raw(1, 2)).address(), rule_over(&raw(7, 4)).address(), "two integers, two addresses");
}

/// 78.54 — THE FRAME IS A GAUGE TOO. The task above, turned by 90°, 180°
/// and 270°, mirrored, and with its palette permuted besides, is one task:
/// through the sensor every reading is the same grids, so the rule mints one
/// word and the court rules one verdict at one price.
#[test]
fn a_task_in_every_frame_and_palette_reads_one_word_and_one_verdict() {
    type Pairs = Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>;
    type Frame<'a> = &'a dyn Fn(&Vec<Vec<u8>>) -> Vec<Vec<u8>>;
    let json = |train: &Pairs, test: &Pairs| {
        let pair = |(a, b): &(Vec<Vec<u8>>, Vec<Vec<u8>>)| {
            format!(r#"{{"input":{},"output":{}}}"#, serde_json::to_string(a).expect("a grid"), serde_json::to_string(b).expect("a grid"))
        };
        let list = |ps: &Pairs| ps.iter().map(pair).collect::<Vec<_>>().join(",");
        format!(r#"{{"train":[{}],"test":[{}]}}"#, list(train), list(test))
    };
    let base: (Pairs, Pairs) = (
        vec![(digits(&["1100", "0020", "0000"]), digits(&["0011", "0200", "0000"]))],
        vec![(digits(&["0110", "2000", "0000"]), digits(&["1100", "0002", "0000"]))],
    );
    // Every grid of the task turned by 90°, clockwise, and mirrored across
    // its diagonal; and the palette permuted (1→6, 2→3).
    let turn = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        let (h, w) = (g.len(), g[0].len());
        (0..w).map(|c| (0..h).rev().map(|r| g[r][c]).collect()).collect()
    };
    let mirror = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        let (h, w) = (g.len(), g[0].len());
        (0..w).map(|c| (0..h).map(|r| g[r][c]).collect()).collect()
    };
    let paint = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        g.iter().map(|row| row.iter().map(|&h| match h { 1 => 6, 2 => 3, x => x }).collect()).collect()
    };
    let each = |ps: &Pairs, f: Frame<'_>| -> Pairs { ps.iter().map(|(a, b)| (f(a), f(b))).collect() };
    let read = |train: &Pairs, test: &Pairs| crate::arc::parse_task(&json(train, test)).expect("the task reads");
    let one = read(&base.0, &base.1);
    let turn180 = |g: &Vec<Vec<u8>>| turn(&turn(g));
    let turn270 = |g: &Vec<Vec<u8>>| turn(&turn(&turn(g)));
    let painted = |g: &Vec<Vec<u8>>| paint(&turn(&mirror(g)));
    let frames: Vec<Frame<'_>> = vec![&turn, &turn180, &turn270, &mirror, &painted];
    let mut k = Kernel::new();
    let rule = rule_over(&one.train);
    let SolveVerdict::Solved { pay } = k.judge_solve(&rule, &one.train) else { panic!("the rule closes its own pair") };
    for (i, f) in frames.iter().enumerate() {
        let other = read(&each(&base.0, *f), &each(&base.1, *f));
        assert_eq!((&other.train, &other.test), (&one.train, &one.test), "frame {i}: one scene through the sensor");
        let again = rule_over(&other.train);
        assert_eq!(again.address(), rule.address(), "frame {i}: one word");
        assert!(matches!(k.judge_solve(&again, &other.train), SolveVerdict::Solved { pay: p } if p == pay), "frame {i}: one verdict, one price");
    }
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
