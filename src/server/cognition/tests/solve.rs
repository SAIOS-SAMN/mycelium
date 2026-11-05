// THE SOLVE, JUDGED BY ITS ACTION AND PAID BY ITS DISCHARGE (the actions'
// physics, 2026-09-29): the court reconstructs a solve's rule on the withheld
// pairs, prices what it discharges per lane, mints it on the task's first
// closure only, and refuses by the reconstruction's reason and kind.

use super::fixtures::*;
use crate::cognition::*;
use crate::transducer::family::{cipher_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::as_element_form;

/// A canvas of two halves, 0 and 5, both ambient, enclosing and held in
/// place, so the ground is a vortex of 0 and 5; a body of 1 standing on the
/// 0 half moves one column along. The withheld pair is a third such move.
fn halves() -> (Vec<TrainPair>, Vec<TrainPair>) {
    let train = vec![
        (digits(&["00000555", "01000555", "00000555"]), digits(&["00000555", "00100555", "00000555"])),
        (digits(&["00000555", "00000555", "00100555"]), digits(&["00000555", "00000555", "00010555"])),
    ];
    let withheld = vec![(digits(&["00000555", "00010555", "00000555"]), digits(&["00000555", "00001555", "00000555"]))];
    (train, withheld)
}

/// TRACK 26 S2 — A CLAIM STATES THE GROUND IT WAS SPOKEN UNDER, AND THE
/// COURT RECONSTRUCTS UNDER IT: the task's ground is a vortex; the rule is
/// read, minted and spoken under the candidate 0, the claim states 0, the
/// court reads 0 as a candidate of the task (`stated_ground`) and judges
/// the withheld pair under it, solved. The court reads only the ground the
/// claim states.
#[test]
fn claim_states_ground_and_court_reconstructs_under_ground() {
    use crate::transducer::family::transduce::Ground;
    let (train, withheld) = halves();
    assert_eq!(crate::transducer::raster::ground_of(&train), Ground::Vortex(vec![0, 5]), "the task's ground is a vortex");
    let spoken = crate::transducer::raster::on_ground(Ground::One(0), || {
        let rule = crate::transducer::family::extract_rule(&train, &mut budget()).expect("the move extracts on 0");
        crate::transducer::family::mint_sentence(&rule, &train, &NoCrystal, 0).expect("and mints on 0")
    });
    let stated = crate::transducer::raster::stated(&Ground::One(0));
    let ground = stated_ground(&train, stated.as_deref()).expect("0 is a candidate of the task");
    assert_eq!(ground, Ground::One(0), "reconstructed under the stated ground");
    let mut k = Court::new();
    let verdict = crate::transducer::raster::on_ground(ground, || k.judge_solve(&spoken, &withheld));
    assert!(matches!(verdict, SolveVerdict::Solved { .. }), "the claim spoken under 0 is solved under 0: {verdict:?}");
}

/// TRACK 26 S2 — A CLAIM UNDER A GROUND THAT IS NO CANDIDATE IS REFUSED BY
/// NAME: 1 is a body of the task, never its ground; 0 and 5 are its
/// candidates; a stated ground of two symbols is no symbol of the task; a
/// claim stating none is reconstructed under no ground.
#[test]
fn claim_under_a_non_candidate_ground_is_refused() {
    use crate::transducer::family::transduce::Ground;
    let (train, _) = halves();
    assert_eq!(stated_ground(&train, Some(&[1])), Err(StatedGroundRefusal::NoCandidate), "a body is no candidate");
    assert_eq!(stated_ground(&train, Some(&[9])), Err(StatedGroundRefusal::NoCandidate), "a symbol the task never shows");
    assert_eq!(stated_ground(&train, Some(&[0, 5])), Err(StatedGroundRefusal::NotOneSymbol), "two symbols are no one ground");
    assert_eq!(stated_ground(&train, Some(&[5])), Ok(Ground::One(5)), "each candidate is reconstructed under");
    assert_eq!(stated_ground(&train, None), Ok(Ground::None), "none stated, none read");
}

/// TRACK 26 S5 — SPEECH STATES ITS GROUND, AND THE COURT RECONSTRUCTS UNDER
/// IT (MU-SENSING §6.2): the task's ground is a vortex of 0 and 5; the
/// sentence is read and minted under the candidate 0. The speech act
/// carries each sentence's stated ground exactly. Bound to the task's
/// family, the court hears the sentence stating 0 under 0, and it stands; a
/// sentence stating a body (1) or two symbols is refused by name, and no
/// ground of the court's own is read in its place.
#[test]
fn speech_states_its_ground_and_court_reconstructs_under_it() {
    use crate::anastome::dialogue::{decode_speech, encode_speech_spoken, Spoken, SPEECH_REFUSED, SPEECH_STOOD};
    use crate::transducer::family::transduce::Ground;
    let (train, _) = halves();
    let sentence = crate::transducer::raster::on_ground(Ground::One(0), || {
        let rule = crate::transducer::family::extract_rule(&train, &mut budget()).expect("the move extracts on 0");
        crate::transducer::family::mint_sentence(&rule, &train, &NoCrystal, 0).expect("and mints on 0")
    });
    let task = crate::registry::citation(
        &crate::registry::task_address(&crate::transducer::family::Task { train: train.clone(), test: Vec::new() }).expect("a task identity"),
    );
    let act = |ground: Option<Vec<u8>>| {
        let tape = sentence.to_bytes().expect("the record writes");
        let said = Spoken { tape, standing: crate::transducer::stalk::Standing::Stood.byte(), contraction: None, agreement: None, ground };
        decode_speech(&encode_speech_spoken(&[1; 32], task, 0, &[said]).expect("the speech frames")).expect("the speech reads")
    };
    assert_eq!(act(Some(vec![0])).grounds, vec![Some(vec![0])], "the stated ground rides the act");
    assert_eq!(act(Some(vec![0, 5])).grounds, vec![Some(vec![0, 5])], "at its own width");
    assert_eq!(act(None).grounds, vec![None], "none stated, none carried");
    let hear = |ground: Option<Vec<u8>>| {
        let mut k = Court::new();
        k.bind_task_family(task, train.clone());
        k.perceive_speech_under([1; 32], &task, std::slice::from_ref(&sentence), &[ground])
    };
    assert_eq!(hear(Some(vec![0])), vec![SPEECH_STOOD], "stated 0, reconstructed under 0: it stands");
    assert_eq!(hear(Some(vec![1])), vec![SPEECH_REFUSED], "a body of the task is no ground");
    assert_eq!(hear(Some(vec![0, 5])), vec![SPEECH_REFUSED], "two symbols are no one ground");
    assert_eq!(StatedGroundRefusal::NoCandidate.reason(), crate::court::verdict::REFUSE_GROUND_NO_CANDIDATE);
    assert_eq!(StatedGroundRefusal::NotOneSymbol.reason(), crate::court::verdict::REFUSE_GROUND_NOT_ONE_SYMBOL);
}

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
/// reconstruction's, clean, and the price is what the rule discharges there — not
/// zero, and not a sample compared.
#[test]
fn judge_solve_prices_closing_rule_by_discharge() {
    let mut k = Court::new();
    let rule = rule_over(&carried());
    match k.judge_solve(&rule, &carried()) {
        SolveVerdict::Solved { pay, modes } => {
            assert!(!pay.is_zero(), "it discharges something: {pay}");
            // μ IS A SHARE OF THE FRAME (78.170 O1): the pair is one 1×7
            // frame, so the deflation's S, G and T (the flux over the mass it
            // carried) are each over 7 sites; H, the shell count, stands.
            let form = crate::transducer::sentence::from_element_form(&rule).expect("a form");
            let counts = crate::transducer::family::form_deflation_read(&form, &carried(), &k, &crate::transducer::alphabet::Word::identity()).pay;
            assert_eq!(pay, crate::economy::Mu::counts(counts).over([7, 7, 7, 1]).expect("non-zero wholes"), "the deflation over the frame's 1·7 sites");
            // 78.162 — the slide the rule carried is held as its own mode on
            // T, a null flux with an exact positive amount, never a count of
            // lanes.
            assert!(modes.on_lane(crate::economy::modes::T).next().is_some(), "the slide is a T mode: {}", modes.print());
        }
        other => panic!("the reconstruction closes every pair: {other:?}"),
    }
}

/// A rule that does not carry a withheld pair is REFUSED by the reconstruction,
/// with its reason and kind — the coordinate the voice learns from.
#[test]
fn judge_solve_refuses_rule_leaving_pair_open() {
    let mut k = Court::new();
    let rule = rule_over(&carried());
    match k.judge_solve(&rule, &recoloured()) {
        SolveVerdict::Refused(r) => assert!(!r.reason.is_empty(), "the refusal names its reason"),
        other => panic!("the recoloured site stays open: {other:?}"),
    }
    assert!(matches!(k.judge_solve(&rule, &[]), SolveVerdict::Unverifiable), "no withheld answer, no verdict");
}

/// 78.56 II — THE DETECTOR IS A DIRICHLET BOUNDARY. The sensor's channels
/// are the laboratory's anchor: one task and the same task with its palette
/// permuted (1→7, 2→4, 3→9) are different colours, read as different samples,
/// and the rule over each is its own word — so every mind that sees channel
/// 1 names channel 1 the same. (78.53 relabelled the palette at the sensor;
/// 78.56 II supersedes it.) Read through the frame-fixing perspective
/// (`raster::frame_fixed_on_ground`, ROADMAP track 26 S2: a perspective a
/// sensor may take after intake).
#[test]
fn frame_fixed_on_ground_reads_permuted_palette_as_other_scene() {
    let read = |a: u8, b: u8, c: u8| {
        crate::transducer::raster::frame_fixed_on_ground(crate::transducer::family::task_of_lines(
            vec![(vec![vec![a, a, 0, b, 0, 0, 0]], vec![vec![0, 0, a, a, 0, b, 0]])],
            vec![(vec![vec![b, 0, a, a, 0, c, 0]], vec![vec![0, 0, b, 0, a, a, c]])],
        ))
    };
    let task = read(1, 2, 3);
    let permuted = read(7, 4, 9);
    assert_ne!(task.train, permuted.train, "two scenes at the detector");
    let (rule, again) = (rule_over(&task.train), rule_over(&permuted.train));
    assert_ne!(rule.address(), again.address(), "two colours, two words");
    let mut k = Court::new();
    assert!(matches!(k.judge_solve(&rule, &task.train), SolveVerdict::Solved { .. }), "each rule closes its own pair");
    assert!(matches!(k.judge_solve(&again, &permuted.train), SolveVerdict::Solved { .. }));
    // AND THE SAME CHANNELS READ THE SAME WORD, whatever mind reads them.
    let same = read(1, 2, 3);
    assert_eq!(rule_over(&same.train).address(), rule.address(), "one channel, one name");
}

/// 78.54, 78.56 — THE FRAME IS A GAUGE WHERE THE TASK CARRIES NO POLAR
/// VECTOR. A task that recolours in place (its momentum vanishes), turned by
/// 90°, 180° and 270°, and mirrored, is one task: through the sensor every
/// reading is the same samples, so the rule mints one word and the court rules
/// one verdict at one price. A task whose bodies move one way carries
/// momentum, and its orientation is physical: turned, it reads differently.
/// Repainted, any task is other colours at the detector (78.56 II).
#[test]
fn judge_solve_reads_reframed_task_as_one_verdict() {
    crate::transducer::raster::stand_on(0);
    use crate::transducer::spelled::SectionSample;
    type Pairs = Vec<(SectionSample, SectionSample)>;
    type Reframing<'a> = &'a dyn Fn(&SectionSample) -> SectionSample;
    let base: (Pairs, Pairs) = (
        vec![(digits(&["1100", "0020", "0000"]), digits(&["3300", "0040", "0000"]))],
        vec![(digits(&["0110", "2000", "0000"]), digits(&["0330", "4000", "0000"]))],
    );
    let moving: (Pairs, Pairs) = (
        vec![(digits(&["1100", "0020", "0000"]), digits(&["0011", "0200", "0000"]))],
        vec![(digits(&["0110", "2000", "0000"]), digits(&["1100", "0002", "0000"]))],
    );
    // Every sample of the task turned by 90°, clockwise, and mirrored across
    // its diagonal; and the palette permuted (1→6, 2→7, 3→8, 4→9).
    // The fixture's expectations are written as a source writes its lines
    // (the speech end, `SectionSample::raster`), then read at intake.
    let turn = |g: &SectionSample| -> SectionSample {
        let ([h, w], l) = (g.extents(), g.raster());
        (0..w).map(|c| (0..h).rev().map(|r| l[r][c]).collect()).collect()
    };
    let mirror = |g: &SectionSample| -> SectionSample {
        let ([h, w], l) = (g.extents(), g.raster());
        (0..w).map(|c| (0..h).map(|r| l[r][c]).collect()).collect()
    };
    let paint = |g: &SectionSample| -> SectionSample { g.mapped(|h| match h { 1 => 6, 2 => 7, 3 => 8, 4 => 9, x => x }) };
    let each = |ps: &Pairs, f: Reframing<'_>| -> Pairs { ps.iter().map(|(a, b)| (f(a), f(b))).collect() };
    // Read through the frame-fixing perspective (ROADMAP track 26, S2).
    let read = |train: &Pairs, test: &Pairs| {
        let test = test.iter().map(|(x, y)| (x.clone(), Some(y.clone()))).collect();
        crate::transducer::raster::frame_fixed_on_ground(crate::transducer::family::Task { train: train.clone(), test })
    };
    let one = read(&base.0, &base.1);
    let turn180 = |g: &SectionSample| turn(&turn(g));
    let turn270 = |g: &SectionSample| turn(&turn(&turn(g)));
    let turned_mirror = |g: &SectionSample| turn(&mirror(g));
    let frames: Vec<Reframing<'_>> = vec![&turn, &turn180, &turn270, &mirror, &turned_mirror];
    let mut k = Court::new();
    let rule = rule_over(&one.train);
    let SolveVerdict::Solved { pay, .. } = k.judge_solve(&rule, &one.train) else { panic!("the rule closes its own pair") };
    for (i, f) in frames.iter().enumerate() {
        let other = read(&each(&base.0, *f), &each(&base.1, *f));
        assert_eq!((&other.train, &other.test), (&one.train, &one.test), "frame {i}: one scene through the sensor");
        let again = rule_over(&other.train);
        assert_eq!(again.address(), rule.address(), "frame {i}: one word");
        assert!(matches!(k.judge_solve(&again, &other.train), SolveVerdict::Solved { pay: ref p, .. } if *p == pay), "frame {i}: one verdict, one price");
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
fn pay_solve_mints_first_closure_only() {
    let mut k = Court::new();
    let rule = rule_over(&carried());
    let SolveVerdict::Solved { pay, .. } = k.judge_solve(&rule, &carried()) else { panic!("solved") };
    let (task, voice) = ([5u8; 32], [9u8; 32]);
    let first = k.pay_solve(&task, voice, &rule, pay.clone(), true);
    assert_eq!(first, pay, "the first closure is paid its discharge");
    assert_eq!(k.treasury.balance_mu(&voice), pay, "minted to the solver");
    let again = k.pay_solve(&task, voice, &rule, pay.clone(), false);
    assert_eq!(again, [0; 4], "a closure again pays nothing");
    assert_eq!(k.treasury.balance_mu(&voice), pay, "and the balance stands");
}

/// 78.153c — A NO-WORK VERDICT IS COUNTED, under "no work": a rule reconstructed
/// clean on a withheld pair it has nothing to do on discharges nothing, and
/// the court's acts say so — the slot the table lacked, where every 78.149
/// no-work verdict was dropped. A refusal and an unverifiable solve each
/// land on its own slot beside it.
#[test]
fn judge_solve_counts_no_work_verdict() {
    use crate::observe::{Judged, JUDGED};
    let mut k = Court::new();
    let rule = rule_over(&carried());
    let idle = vec![(digits(&["0000000"]), digits(&["0000000"]))];
    let verdict = k.judge_solve(&rule, &idle);
    assert!(matches!(verdict, SolveVerdict::NoWork), "nothing to carry, nothing discharged: {verdict:?}");
    assert!(matches!(k.judge_solve(&rule, &recoloured()), SolveVerdict::Refused(_)));
    assert!(matches!(k.judge_solve(&rule, &[]), SolveVerdict::Unverifiable));
    let judged = k.acts_reading().judged;
    let at = |name: &str| JUDGED.iter().position(|n| *n == name).expect("a slot of the table");
    assert_eq!(judged[at("no work")], 1, "the no-work verdict, counted");
    assert_eq!(judged[Judged::NoWork.slot()], 1);
    assert_eq!(judged[at("unverifiable")], 1);
    // `judge_solve` asks again while the host's stroke runs out before the
    // reconstruction rules; each such asking is counted "unread", the rest exactly.
    let unread = judged[Judged::Unread.slot()];
    assert_eq!(judged.iter().sum::<u64>() - unread, 3, "three verdicts, three counts: none dropped");
    assert!(k.acts_reading().lines()[3].contains("no work 1"), "and the console's line names it");
}

/// NO WORK, NO ACT (ruled 2026-10-08: never give μ away for the
/// possibility of falsification): a solve paid nothing releases no escrow,
/// mints nothing, dockets and offers nothing — the zero is read before any
/// of it, not after.
#[test]
fn pay_solve_acts_on_nothing_at_zero_discharge() {
    let mut k = Court::new();
    let rule = rule_over(&carried());
    let (task, voice) = ([5u8; 32], [9u8; 32]);
    let paid = k.pay_solve(&task, voice, &rule, crate::economy::Mu::zero(), true);
    assert_eq!(paid, [0; 4], "nothing paid");
    assert_eq!(k.treasury.balance_mu(&voice), [0; 4], "nothing minted");
    assert!(k.take_offers().is_empty(), "nothing docketed or offered");
}
