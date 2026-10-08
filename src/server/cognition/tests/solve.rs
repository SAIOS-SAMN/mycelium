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

/// 78.56 II — THE DETECTOR IS A DIRICHLET BOUNDARY. The sensor's channels
/// are the laboratory's anchor: one task and the same task with its palette
/// permuted (1→7, 2→4, 3→9) are different colours, read as different grids,
/// and the rule over each is its own word — so every mind that sees channel
/// 1 names channel 1 the same. (78.53 relabelled the palette at the sensor;
/// 78.56 II supersedes it.)
#[test]
fn a_task_and_its_palette_permuted_are_two_scenes_at_the_detector() {
    let json = |a: u8, b: u8, c: u8| {
        format!(
            r#"{{"train":[{{"input":[[{a},{a},0,{b},0,0,0]],"output":[[0,0,{a},{a},0,{b},0]]}}],
                "test":[{{"input":[[{b},0,{a},{a},0,{c},0]],"output":[[0,0,{b},0,{a},{a},{c}]]}}]}}"#
        )
    };
    let task = crate::arc::parse_task(&json(1, 2, 3)).expect("the task reads");
    let permuted = crate::arc::parse_task(&json(7, 4, 9)).expect("the permuted task reads");
    assert_ne!(task.train, permuted.train, "two scenes at the detector");
    let (rule, again) = (rule_over(&task.train), rule_over(&permuted.train));
    assert_ne!(rule.address(), again.address(), "two colours, two words");
    let mut k = Kernel::new();
    assert!(matches!(k.judge_solve(&rule, &task.train), SolveVerdict::Solved { .. }), "each rule closes its own pair");
    assert!(matches!(k.judge_solve(&again, &permuted.train), SolveVerdict::Solved { .. }));
    // AND THE SAME CHANNELS READ THE SAME WORD, whatever mind reads them.
    let same = crate::arc::parse_task(&json(1, 2, 3)).expect("the task reads");
    assert_eq!(rule_over(&same.train).address(), rule.address(), "one channel, one name");
}

/// 78.54, 78.56 — THE FRAME IS A GAUGE WHERE THE TASK CARRIES NO POLAR
/// VECTOR. A task that recolours in place (its momentum vanishes), turned by
/// 90°, 180° and 270°, and mirrored, is one task: through the sensor every
/// reading is the same grids, so the rule mints one word and the court rules
/// one verdict at one price. A task whose bodies move one way carries
/// momentum, and its orientation is physical: turned, it reads differently.
/// Repainted, any task is other colours at the detector (78.56 II).
#[test]
fn a_task_in_every_frame_reads_one_word_and_one_verdict() {
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
        vec![(digits(&["1100", "0020", "0000"]), digits(&["3300", "0040", "0000"]))],
        vec![(digits(&["0110", "2000", "0000"]), digits(&["0330", "4000", "0000"]))],
    );
    let moving: (Pairs, Pairs) = (
        vec![(digits(&["1100", "0020", "0000"]), digits(&["0011", "0200", "0000"]))],
        vec![(digits(&["0110", "2000", "0000"]), digits(&["1100", "0002", "0000"]))],
    );
    // Every grid of the task turned by 90°, clockwise, and mirrored across
    // its diagonal; and the palette permuted (1→6, 2→7, 3→8, 4→9).
    let turn = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        let (h, w) = (g.len(), g[0].len());
        (0..w).map(|c| (0..h).rev().map(|r| g[r][c]).collect()).collect()
    };
    let mirror = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        let (h, w) = (g.len(), g[0].len());
        (0..w).map(|c| (0..h).map(|r| g[r][c]).collect()).collect()
    };
    let paint = |g: &Vec<Vec<u8>>| -> Vec<Vec<u8>> {
        g.iter().map(|row| row.iter().map(|&h| match h { 1 => 6, 2 => 7, 3 => 8, 4 => 9, x => x }).collect()).collect()
    };
    let each = |ps: &Pairs, f: Frame<'_>| -> Pairs { ps.iter().map(|(a, b)| (f(a), f(b))).collect() };
    let read = |train: &Pairs, test: &Pairs| crate::arc::parse_task(&json(train, test)).expect("the task reads");
    let one = read(&base.0, &base.1);
    let turn180 = |g: &Vec<Vec<u8>>| turn(&turn(g));
    let turn270 = |g: &Vec<Vec<u8>>| turn(&turn(&turn(g)));
    let turned_mirror = |g: &Vec<Vec<u8>>| turn(&mirror(g));
    let frames: Vec<Frame<'_>> = vec![&turn, &turn180, &turn270, &mirror, &turned_mirror];
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
    // A TASK WITH A POLAR VECTOR: its bodies move one way, so its frame is
    // physical — turned, it is not the same scene.
    let drift = read(&moving.0, &moving.1);
    let turned_drift = read(&each(&moving.0, &turn), &each(&moving.1, &turn));
    assert_ne!(drift.train, turned_drift.train, "momentum breaks the symmetry: the orientation is observable");
    // REPAINTED, a task is other colours at the detector (78.56 II).
    let repainted = read(&each(&base.0, &paint), &each(&base.1, &paint));
    assert_ne!(one.train, repainted.train, "the channels are an absolute boundary");
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

/// NO WORK, NO ACT (the operator, 2026-10-08: never give μ away for the
/// possibility of falsification): a solve paid nothing releases no escrow,
/// mints nothing, dockets and offers nothing — the zero is read before any
/// of it, not after.
#[test]
fn a_solve_that_discharges_nothing_acts_on_nothing() {
    let mut k = Kernel::new();
    let rule = rule_over(&carried());
    let (task, voice) = ([5u8; 32], [9u8; 32]);
    let paid = k.pay_solve(&task, voice, &rule, [0; 4], true);
    assert_eq!(paid, [0; 4], "nothing paid");
    assert_eq!(k.treasury.balance_mu(&voice), [0; 4], "nothing minted");
    assert!(k.take_offers().is_empty(), "nothing docketed or offered");
}
