// MU-CLIENT §4.4(4), §4.12, §3.1, 78.167 D1 — THE WINDING CARRIER: what the
// mind says leaves at its returns; a scene waiting on comprehension lapses
// with its last thought; a meaning given whole is derived once the words
// beside it are known.

use super::fixtures::*;
use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};

/// MU-CLIENT §4.4(4), §7.1 R2 — STOOD NOW, SAID AT THE RETURN OF THE THOUGHT
/// IT SPEAKS FOR, AND AT NO OTHER. Two fresh sentences spoken under two
/// scenes stand in the dialect at once — the mind's own act — and nothing
/// reaches the carrier facing the court. Said again before its return, a
/// sentence is the one thing held, never two. A thought of the first scene
/// comes round by the field's own motion (`bind_a_thought`: two hours of a
/// slide, the escaping relation bound by the demand taker); the winding reads
/// that return, K moves by exactly it, and the first scene's speech is posted —
/// the second's, whose scene no thought of has returned, is still held.
#[test]
fn speak_with_posts_only_at_thought_return() {
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let e2 = two_body_sentence([&[(5, 6), (5, 6)], &[(7, 8), (7, 8)]]);
    let (one, two) = ([1u8; 32], [2u8; 32]);
    let mut e = Mind::open("127.0.0.1:1", Voice::named("winding"));
    e.speak_with(std::slice::from_ref(&e1), &[], &one, |_, _| {});
    e.speak_with(std::slice::from_ref(&e2), &[], &two, |_, _| {});
    assert_eq!(e.dialect_len(), 2, "both stand at once");
    assert_eq!((e.hypha.in_flight(), e.held_for_returns(), e.own_k()), (0, 2, 0), "nothing posted: each waits for its thought's return");
    // Said again before the return, it is the one thing held, never two
    // (MU-CLIENT §4.13: held by its coordinates, never by a count).
    e.speak_with(std::slice::from_ref(&e1), &[], &one, |_, _| {});
    assert_eq!(e.held_for_returns(), 2, "one speech held per sentence, its latest saying");
    let returned = bind_a_thought(&mut e, &one);
    assert!(returned > 0, "a thought of the first scene came round");
    assert_eq!(e.own_k(), returned, "K moved by exactly the returns");
    assert_eq!(e.hypha.in_flight(), 1, "the first scene's speech posted at its thought's return");
    assert_eq!(e.held_for_returns(), 1, "the second scene's still waits for a thought of its own");
}

/// The scene a task's pairs make: one bar carried a column along.
fn scene() -> (crate::transducer::family::SectionSample, crate::transducer::family::SectionSample) {
    (vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]].into(), vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 0]].into())
}

fn envelope(id: [u8; 32]) -> crate::anastome::dialogue::TaskSection {
    crate::anastome::dialogue::TaskSection { id, train: vec![scene()], tests: Vec::new() }
}

/// A SCENE WAITING ON COMPREHENSION STANDS WITH ITS THOUGHTS. With no thought
/// perceived in it, it lapses at the mind's return; with one standing and the
/// mind's language unmoved since it was read, it waits as it was — nothing
/// new to comprehend it by.
#[test]
fn comprehend_waiting_lapses_scene_with_last_thought() {
    let id = [7u8; 32];
    let mut alone = Mind::open("127.0.0.1:1", Voice::named("alone"));
    let root = alone.scene_language(&envelope(id).train);
    alone.comprehending.insert(id, (envelope(id), root));
    alone.comprehend_waiting();
    assert!(alone.comprehending.get(&id).is_none(), "no thought stands on it: it lapses with them");

    let mut held = Mind::open("127.0.0.1:1", Voice::named("held"));
    let (a, b) = scene();
    let frames = crate::transducer::family::perceive_family(&[(a.clone(), b.clone())]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let f = crate::transducer::family::scene_form_on(&held.language, &mut budget, &a, &b, &frames).expect("the scene reads");
    held.contemplation.hold_perceived(f, &frames, &Vec::new(), &[], [0; 4], &id, &mut budget).expect("held");
    assert!(held.contemplation.holds_family(&id), "a thought stands on the scene");
    // Its own language — the words on the charts it stands on — as it was
    // last read (`scene_language`).
    let root = held.scene_language(&envelope(id).train);
    held.comprehending.insert(id, (envelope(id), root));
    held.comprehend_waiting();
    assert!(held.comprehending.get(&id).is_some(), "it waits beside its thought");
}

/// 68.5, 78.7 VS1 — ONE STROKE'S LEDGER IS TAKEN INTO THE MIND'S OBSERVE
/// READING. Two cauterizations and a viscous bend on 𝒽 stand on this
/// thread's ledger; one winding takes the ledger whole at its booking, so the
/// mind's reading carries them (entropy and the lane's viscosity, never
/// summed with another lane) and the ledger the stroke leaves stands taken.
#[test]
fn stroke_reads_metabolism_into_observe() {
    use crate::physics::metabolism;
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("ledger"));
    let before = crate::observe::mind();
    metabolism::note_cauterized();
    metabolism::note_cauterized();
    metabolism::note_viscous(3);
    let stop = std::sync::atomic::AtomicBool::new(false);
    let _ = mind.wind(&stop);
    let after = crate::observe::mind();
    assert!(after.entropy >= before.entropy + 2, "the stroke's entropy reached the reading");
    assert!(after.viscous[3] > before.viscous[3], "the viscous bend on 𝒽 reached the reading, on its lane");
    assert_eq!(metabolism::metabolism().viscous, [0; 4], "the ledger was taken, once, at the stroke's booking");
}

/// 78.167 D1 — A MEANING GIVEN WHOLE IS DERIVED ONCE THE WORDS BESIDE IT ARE
/// KNOWN. A screw — 90° in the e₁e₂ plane carried along e₃ — answered as an
/// atom is known as GIVEN. Knowing the turn alone leaves the slide as its
/// residue: still given. Knowing the slide too, it is reduced again at the
/// mind's return and DERIVED, one rung above the two.
#[test]
fn re_reduce_given_derives_once_factors_known() {
    use crate::transducer::sentence::rung::{held_rung, learn_by, learn_given, re_reduce_given};
    let q = LetterId::turn_90((0, 1)).expect("90° in the e₁e₂ plane");
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let quarter = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &crate::transducer::element::identity_versor()).expect("a motion");
    let screw = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &along).expect("a motion");
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("given"));
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        assert!(learn_given(&screw), "the screw's meaning given whole");
        assert!(learn_by(&quarter, &[]), "the turn known");
        assert_eq!(re_reduce_given(), 0, "the turn alone leaves the slide: still given");
        assert!(learn_by(&along, &[]), "the slide known");
        assert_eq!(re_reduce_given(), 1, "turn and slide known: the screw is derived");
        assert_eq!(held_rung(&screw), Some(2), "one rung above the two it is made of");
    });
    mind.field = field;
}

/// 23.8, MU-CLIENT §4.13 — AN ANSWERED QUOTE ON NON-SHORT LANES DWELLS NOT,
/// AND A DEFERRED QUOTE'S DWELL COUNTS DOWN ONE STROKE PER WINDING. An
/// answered quote (`deficit: None`) records `last_pressure` without starting a
/// dwell on lanes where the mind's reserve is not short, and does not trigger
/// `dwell_elapsed`. A deferred quote (`deficit: Some(d)`) dwells on the
/// deferred lanes across non-short strokes, counts down by one each stroke,
/// fires `dwell_elapsed` on the stroke that reaches zero, and clears
/// `dwell_elapsed` on the stroke after.
#[test]
fn wind_preserves_quote_dwell_across_non_short_strokes() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("dwell"));
    mind.board.court.quoted = Some(YieldQuote {
        height: 5,
        surface: [3, 0, 0, 0],
        k: 0,
        clusters: Vec::new(),
        deficit: None,
    });
    let stop = std::sync::atomic::AtomicBool::new(false);
    let _ = mind.wind(&stop);
    assert_eq!(mind.board.court.pressure(), [3, 0, 0, 0], "the court's surface stands as last_pressure");
    assert_eq!(mind.board.court.dwell_remaining, [0, 0, 0, 0], "an answered quote on non-short lanes starts no dwell");
    assert!(!mind.board.court.dwelling(), "no dwell runs on non-short lanes");
    assert!(!mind.board.court.dwell_elapsed, "no dwell elapsed without a countdown");

    mind.board.court.quoted = Some(YieldQuote {
        height: 5,
        surface: [3, 0, 0, 0],
        k: 0,
        clusters: Vec::new(),
        deficit: Some([3, 0, 0, 0]),
    });
    let _ = mind.wind(&stop);
    assert_eq!(mind.board.court.dwell_remaining, [3, 0, 0, 0], "booking preserves a deferral's dwell on non-short lanes");
    assert!(mind.board.court.dwelling(), "the mind owes strokes while the deferral dwell runs");
    let _ = mind.wind(&stop);
    assert_eq!(mind.board.court.dwell_remaining, [2, 0, 0, 0]);
    assert!(!mind.board.court.dwell_elapsed);
    let _ = mind.wind(&stop);
    assert_eq!(mind.board.court.dwell_remaining, [1, 0, 0, 0]);
    assert!(!mind.board.court.dwell_elapsed);
    let _ = mind.wind(&stop);
    assert_eq!(mind.board.court.dwell_remaining, [0, 0, 0, 0], "the dwell expires on the third stroke after admission");
    assert!(mind.board.court.dwell_elapsed, "dwell_elapsed fires on the stroke that reaches zero");
    assert!(!mind.board.court.dwelling(), "no further strokes are owed once the dwell has run out");
    let _ = mind.wind(&stop);
    assert!(!mind.board.court.dwell_elapsed, "dwell_elapsed clears on the following stroke");
}

/// MU-CLIENT §7.1, #307 — A MIND HONOURS ITS STOP AND HALT INSIDE A STROKE.
/// When stopped or halted, a stroke returns without entering heavy work
/// (demands, pivots, hearings, task pulls), remaining reads 0, and the winding
/// completes promptly.
#[test]
fn wind_honours_stop_and_halt() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("stopping"));
    let stop = std::sync::atomic::AtomicBool::new(true);
    let wound = mind.wind(&stop);
    assert_eq!(wound.moved, [false; 4]);
    assert_eq!(wound.returns, 0);

    // When physics::halt() is called, even with stop=false, the budget is spent
    // and inner loops break.
    let stop_false = std::sync::atomic::AtomicBool::new(false);
    crate::physics::halt();
    assert!(crate::physics::halted());
    let mut budget = mind.stroke();
    assert_eq!(budget.remaining(), 0);
    assert!(!budget.spend());
    let wound2 = mind.wind(&stop_false);
    assert_eq!(wound2.moved, [false; 4]);
    assert_eq!(wound2.returns, 0);

    crate::physics::unhalt_for_test();
}
