// MU-CLIENT §4.10, §4.12, §7.1, ROADMAP track 10 — THE SOVEREIGN MIND'S FIVE CLAIMS:
// 1. mind_idles_with_no_escaping_orbit
// 2. task_arrival_moves_no_orbit
// 3. mind_declines_a_scene_its_field_does_not_bind
// 4. mind_acts_at_its_returns
// 5. mind_revises_a_perception_on_a_court_ruling_by_amendment

use super::fixtures::*;
use crate::client::sdk::mind::amend::Amendment;
use crate::client::sdk::mind::offer::{Offer, PASSED_TASK};
use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::cga::multivector::{one, Mv};

/// MU-CLIENT §4.12, §7.1, ROADMAP track 10 — MIND IDLES WITH NO ESCAPING ORBIT.
/// A mind is moved by its own open curvature and escaping orbits, acts at its
/// returns, and rests when its metabolism runs short. A mind just opened with no
/// unclosed demands, no ringing orbits, and no escaping modes idles in equilibrium:
/// one stroke's winding moves no lanes and fires no returns.
#[test]
fn mind_idles_with_no_escaping_orbit() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("quiescent"));
    let stop = std::sync::atomic::AtomicBool::new(false);
    let wound = mind.wind(&stop);
    assert_eq!(wound.returns, 0, "no escaping orbit: no returns fire");
    assert_eq!(wound.moved, [false; 4], "no lanes moved in equilibrium");
    assert_eq!(mind.held_for_returns(), 0, "no escaping thoughts held for return");
    assert_eq!(mind.speaking.len(), 0, "no speech emitted from idle mind");
    assert_eq!(mind.own_k(), 0, "K remains unmoved");
}

/// A family of one cell stepping one column.
fn one_cell_steps() -> Vec<(crate::transducer::family::SectionSample, crate::transducer::family::SectionSample)> {
    let at = |c: usize| -> crate::transducer::family::SectionSample { (0..3).map(|r| (0..3).map(|x| u8::from(r == 1 && x == c)).collect()).collect() };
    vec![(at(0), at(1))]
}

/// MU-CLIENT §4.12, §7.1, ROADMAP track 10 — TASK ARRIVAL MOVES NO ORBIT.
/// What arrives (a scene, a task, a peer's assertion, a verdict) is perception
/// that perturbs the orbits it lands on; it never moves the mind. When a task
/// arrives on the board, task intake takes the task into the perception carrier
/// without advancing the mind's winding K and without firing returns or dispatching speech.
#[test]
fn task_arrival_moves_no_orbit() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("intake"));
    let train = one_cell_steps();
    let cell = Offer::of(&train);
    let id = [42u8; 32];

    // Give language a word on that chart so the task couples sympathetically
    let [(x, y)] = <[_; 1]>::try_from(train.clone()).expect("one pair");
    let mut b = budget();
    let frames = crate::transducer::family::perceive_family(&[(x.clone(), y.clone())]);
    let f = crate::transducer::family::scene_form_on(&crate::transducer::family::NoCrystal, &mut b, &x, &y, &frames).expect("the pair reads");
    let word = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word");
    mind.language.moduli_admit(&crate::transducer::element::Held::of(word));

    mind.board.court.task = Some((crate::anastome::dialogue::TaskSection { id, train, tests: Vec::new() }, cell));

    let k_before = mind.own_k();
    assert_eq!(k_before, 0);

    let mut outcome = crate::physics::StrokeOutcome::default();
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| mind.task_inflow(&mut outcome));
    mind.field = field;

    assert!(mind.board.court.task.is_none(), "task accepted onto perception boundary");
    assert!(mind.perceiving(), "perception carrier active");
    assert_eq!(mind.own_k(), k_before, "task arrival moved no orbit: K is unchanged");
    assert_eq!(mind.held_for_returns(), 0, "nothing held for return on arrival alone");
}

/// MU-CLIENT §3.1, §4.12, §7.1, ROADMAP track 10 — MIND DECLINES A SCENE ITS FIELD DOES NOT BIND.
/// An offer that couples to nothing what the field holds is left unperceived,
/// carries across the return or next court final, and lapses uncoupled — passed,
/// said to the observer, with no refusal sent.
#[test]
fn mind_declines_a_scene_its_field_does_not_bind() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("decliner"));
    let uncoupled_train = one_cell_steps();
    let offer = Offer::of(&uncoupled_train);
    let id = [13u8; 32];

    assert!(!mind.couples(&id, &offer.charts), "an empty mind couples to no novel chart");

    mind.board.court.task = Some((crate::anastome::dialogue::TaskSection { id, train: uncoupled_train, tests: Vec::new() }, offer));
    let mut outcome = crate::physics::StrokeOutcome::default();
    mind.task_inflow(&mut outcome);

    assert!(mind.board.court.task.is_some() && !mind.perceiving(), "uncoupled: not taken onto perception carrier");

    let mut b = budget();
    assert!(mind.act_on_returns(&mut b, vec![b"other scene".to_vec()], false) > 0);
    assert!(mind.board.court.task.as_ref().is_some_and(|(_, o)| o.crossed), "first return marks crossed");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 0);

    assert!(mind.act_on_returns(&mut b, vec![b"other scene".to_vec()], false) > 0);
    assert!(mind.board.court.task.is_none(), "uncoupled across second return: declined and lapsed");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 1, "passed count recorded for observer");
}

/// MU-CLIENT §4.4(4), §4.12, §7.1, ROADMAP track 10 — MIND ACTS AT ITS RETURNS.
/// What was held for a scene's return leaves where a thought of that scene returned,
/// and nowhere else. Speech asserted under a scene stands in the dialect waiting for
/// that thought's orbital return; K moves by exactly the return, and only then is the
/// speech posted to the carrier facing the court.
#[test]
fn mind_acts_at_its_returns() {
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let scene_id = [7u8; 32];
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("returns"));

    mind.speak_with(std::slice::from_ref(&e1), &[], &scene_id, |_, _| {});
    assert_eq!(mind.held_for_returns(), 1, "speech held waiting for thought return");
    assert_eq!(mind.hypha.in_flight(), 0, "nothing posted before return");

    // A thought of that scene comes round
    let returned = bind_a_thought(&mut mind, &scene_id);
    assert!(returned > 0, "thought completed its orbit and returned");
    assert_eq!(mind.own_k(), returned, "K moved by exactly the returns");
    assert_eq!(mind.hypha.in_flight(), 1, "held speech posted at thought's return");
    assert_eq!(mind.held_for_returns(), 0, "speech discharged from held queue");
}

/// MU-CLIENT §4.10, §7.1, ROADMAP track 10 — MIND REVISES A PERCEPTION ON A COURT RULING BY AMENDMENT.
/// Plastic deformation: when experience or a court ruling shows an unclosed remainder,
/// the constructor is re-derived from residue and enters BESIDE the one it amends.
/// The commutator C = g₁ g₂ g̃₁ g̃₂ reads what the amendment was:
/// if identity, old retires by derivability; if non-trivial, held beside at conjugate grade.
#[test]
fn mind_revises_a_perception_on_a_court_ruling_by_amendment() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("amender"));

    // Two rotations in disjoint planes (e₀e₁ and e₂e₃): commute, commutator is identity
    let q1 = LetterId::turn_90((0, 1)).expect("90 deg in e0e1");
    let q2 = LetterId::turn_90((2, 3)).expect("90 deg in e2e3");
    let r1 = Word::from_letters(&[(q1, false)]).rotor().lift();
    let r2 = Word::from_letters(&[(q2, false)]).rotor().lift();

    let amend_comm = mind.revise_perception_by_amendment(&r1, &r2).expect("amendment evaluates");
    assert!(matches!(amend_comm, Amendment::Derivable { .. }), "commuting constructors retire by derivability");

    // Two rotations sharing an axis (e₀e₁ and e₀e₂): non-commuting, non-trivial commutator
    let q3 = LetterId::turn_90((0, 2)).expect("90 deg in e0e2");
    let r3 = Word::from_letters(&[(q3, false)]).rotor().lift();

    let amend_beside = mind.revise_perception_by_amendment(&r1, &r3).expect("amendment evaluates");
    match amend_beside {
        Amendment::HeldBeside { original, amended, commutator } => {
            assert_eq!(original, r1);
            assert_eq!(amended, r3);
            assert!(!commutator.projective_eq(&Mv::scalar(one())), "commutator is non-trivial distinction");
        }
        _ => panic!("expected HeldBeside for non-commuting constructors"),
    }

    assert_eq!(mind.held_amendments().len(), 2, "both amendments held in mind history");
}
