// 78.153 — THE COURT'S REPLAYS ON ITS STROKE: a replay the stroke cannot
// finish is unread and resumed, never slashed; the docket stands before the
// word until it is ruled.

use crate::cognition::*;
use crate::transducer::element::Element;
use crate::verdict::Replayed;

fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A recolour read off one pair and minted on it: a good word on `one`; on
/// `wider` it leaves a remainder it does not declare.
fn fixture() -> (Element, Vec<crate::transducer::family::TrainPair>, Vec<crate::transducer::family::TrainPair>) {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    let (x, y) = (grid(&["1"]), grid(&["2"]));
    let one = vec![(x.clone(), y.clone())];
    let rule = extract_rule(&one, &mut stroke(1 << 20)).expect("a recolour is a rule");
    let word = mint_sentence(&rule, &one, &NoCrystal, 0).expect("it closes its own pair");
    let wider = vec![(x, y), (grid(&["100"]), grid(&["203"]))];
    (word, one, wider)
}

/// A court holding `word` on its docket, bound to `family`.
fn docketed(word: &Element, family: Vec<crate::transducer::family::TrainPair>) -> Kernel {
    let mut k = Kernel::new();
    let task = [7u8; 32];
    k.bind_task_family(task, family);
    k.docket_connection(word, [1; 32]);
    k.speech_task.insert(word.address(), Address::from_bytes(task));
    k
}

/// THE COURT'S REPLAY ON STROKES OF TWO READS: unread, unread, … and then
/// exactly the whole replay's verdict. Every unread stroke kept the cursor;
/// none slashed; the ruled replay leaves no cursor behind.
#[test]
fn the_court_resumes_an_unread_replay_and_rules_it_as_the_whole_one() {
    let (word, one, wider) = fixture();
    for family in [one, wider] {
        let whole = docketed(&word, family.clone()).court_replay(&word, &mut stroke(1 << 20));
        let mut k = docketed(&word, family);
        let mut strokes = 0u64;
        let out = loop {
            strokes += 1;
            assert!(strokes <= 4096, "a stroke of two reads moves the replay on");
            match k.court_replay(&word, &mut stroke(2)) {
                Replayed::Unread => {
                    assert_eq!(k.replay_cursors.len(), 1, "the unread replay keeps its cursor");
                    assert_eq!(k.court_slashed, 0, "an unread replay slashes nothing");
                }
                out => break out,
            }
        };
        assert!(strokes > 1, "the replay needs more than one stroke of two reads");
        assert_eq!(out, whole, "resumed, it rules as the whole replay does");
        assert_eq!(k.court_unread, strokes - 1, "every stroke before the ruling was unread");
        assert!(k.replay_cursors.is_empty(), "a ruled replay keeps no cursor");
    }
}

/// DEFECT 1 (lock audit) ON THE DOCKET: a good word whose replay does not
/// fit the court's stroke is never slashed — the docket stands before it,
/// resumes it, and rules it; a word that leaves a remainder on its family is
/// still slashed, once its replay is read.
#[test]
fn the_docket_rules_a_word_its_stroke_could_not_finish() {
    let (word, one, wider) = fixture();
    let addr = word.address();
    let mut ruling = docketed(&word, one);
    let mut strokes = 0;
    while !ruling.ruled_contains(&addr) {
        strokes += 1;
        assert!(strokes <= 4096, "the docket rules the word");
        ruling.court_stroke();
        assert_eq!(ruling.court_slashed, 0, "a good word is never slashed, on any stroke");
    }
    assert!(ruling.replay_cursors.is_empty(), "its replay is ruled and keeps no cursor");

    let mut refusing = docketed(&word, wider);
    let mut strokes = 0;
    while refusing.provisional.contains(&addr) {
        strokes += 1;
        assert!(strokes <= 4096, "the docket rules the word");
        refusing.court_stroke();
    }
    assert_eq!(refusing.court_slashed, 1, "the form that does not close its family is slashed once");
    assert!(!refusing.ruled_contains(&addr), "and never ruled");
}

/// THE WITNESS DOOR AND THE GLUE SHARE THE CURSOR: a solve judged on the
/// act's one stroke is `Unread` — not refused, not paid — and asked again it
/// resumes, to the verdict the whole replay gives.
#[test]
fn a_solve_not_judged_on_its_stroke_is_unread_and_resumed() {
    let (word, one, _) = fixture();
    // The act's stroke is the host's; asked again and again, the solve
    // resumes and is judged — never refused for a spent stroke — and each
    // `Unread` kept the cursor the next asking resumed.
    let solve = |k: &mut Kernel| -> ([u64; 4], u64) {
        let mut asked = 0u64;
        loop {
            asked += 1;
            assert!(asked <= 4096, "asked again, the solve is judged");
            match k.judge_solve_on_stroke(&word, &one) {
                SolveVerdict::Unread => assert_eq!(k.replay_cursors.len(), 1, "the unread solve keeps its cursor"),
                SolveVerdict::Solved { pay, .. } => return (pay, asked),
                other => panic!("never refused for a spent stroke: {other:?}"),
            }
        }
    };
    let mut first = Kernel::new();
    let (pay, asked) = solve(&mut first);
    let (again, asked_again) = solve(&mut Kernel::new());
    assert!(pay.iter().any(|&m| m > 0), "the rule discharges its pair: {pay:?}");
    assert_eq!((pay, asked), (again, asked_again), "one verdict, one price, one number of strokes");
    // 78.153c — EVERY UNREAD ASKING IS COUNTED, under "unread": the askings
    // before the judging one, exactly; the solved one is counted where it is
    // paid, so nothing else is.
    let judged = first.acts_reading().judged;
    assert_eq!(judged[crate::observe::Judged::Unread.slot()], asked - 1, "each unread asking, counted");
    assert_eq!(judged.iter().sum::<u64>(), asked - 1, "and nothing else");
}
