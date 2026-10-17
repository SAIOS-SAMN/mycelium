// MU-CLIENT §4.12, §7.1 — THE OFFER AND THE MIND'S OWN ATTENTION: a mind asks
// only under tension, and takes an offer only where it couples to what its
// field holds; an offer that couples to nothing lapses at its second return.

use super::common::*;
use crate::client::sdk::entity::offer::{Offer, PASSED_TASK};
use crate::client::sdk::entity::perceive::{Reading, Scene};
use crate::client::sdk::*;

/// A family of one cell stepping one column: a body of no chart the slide's
/// bodies stand on.
fn one_cell_steps() -> Vec<(crate::arc::Grid, crate::arc::Grid)> {
    let at = |c: usize| -> crate::arc::Grid { (0..3).map(|r| (0..3).map(|x| u8::from(r == 1 && x == c)).collect()).collect() };
    vec![(at(0), at(1))]
}

/// A SETTLED MIND ASKS FOR NOTHING; A STRAINED ONE HOLDS TENSION. A mind just
/// opened holds no demand, no ringing orbit, no reservoir: it is in
/// equilibrium, and the winding's ask for a scene (`pull_task`) posts
/// nothing, its reserve full or not. The slide's two hours perceived, the
/// relation between them never returns and stands on its plan: the mind now
/// holds tension.
#[test]
fn a_settled_mind_asks_for_nothing_and_a_strained_one_holds_tension() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("settled"));
    let [first, second] = two_hours_of_a_slide();
    let mut field = std::mem::take(&mut mind.field);
    let (settled, asked, strained) = field.think(|| {
        let settled = mind.strained();
        mind.pull_task();
        let asked = mind.board.court.pulling;
        mind.perceive_scene(Scene { id: vec![9; 32], train: vec![first, second], tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
        (settled, asked, mind.strained())
    });
    mind.field = field;
    assert!(!settled, "a mind just opened holds no strain");
    assert!(!asked, "in equilibrium it asks for nothing");
    assert!(strained, "the slide's relation stands unbound: tension");
}

/// AN OFFER COUPLES ONLY TO WHAT THE MIND HOLDS. Before the mind has seen
/// anything, the slide offered couples to nothing. Once its hours are held
/// under their citation, the same scene couples by its citation, the same
/// family under another citation couples by the charts its bodies stand on —
/// charts the mind's thoughts stand on — and a family of one cell, whose body
/// stands on no chart the mind holds, couples to nothing.
#[test]
fn an_offer_couples_only_to_what_the_mind_holds() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("coupling"));
    let train = two_hours_of_a_slide().to_vec();
    let (id, other) = ([9u8; 32], [8u8; 32]);
    let slide = Offer::of(&train);
    let cell = Offer::of(&one_cell_steps());
    assert!(!slide.charts.is_empty() && !cell.charts.is_empty(), "each offer reads to the charts its bodies stand on");
    assert!(!mind.couples(&id, &slide.charts), "a mind that holds nothing couples to nothing");
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        mind.perceive_scene(Scene { id: id.to_vec(), train: train.clone(), tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
    });
    mind.field = field;
    assert!(mind.couples(&id, &[]), "the scene it holds thoughts of couples by its citation");
    assert!(mind.couples(&other, &slide.charts), "the same bodies under another citation couple by their charts");
    assert!(!mind.couples(&other, &cell.charts), "a body on no chart the mind holds couples to nothing");
}

/// AN UNCOUPLED TASK IS NOT TAKEN, AND LAPSES AT ITS SECOND RETURN. The
/// court's task lands on a mind that holds nothing: the intake leaves it where
/// it landed and perceives nothing; the first return carries it across; at
/// the second it still couples to nothing, and is let go — passed, said to
/// the observer — with no refusal sent.
#[test]
fn an_uncoupled_task_is_not_taken_and_lapses_at_its_second_return() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("passing"));
    let train = one_cell_steps();
    let offer = Offer::of(&train);
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id: [7; 32], train, tests: Vec::new() }, offer));
    let mut outcome = crate::physics::StrokeOutcome::default();
    mind.task_inflow(&mut outcome);
    assert!(mind.board.court.task.is_some() && !mind.perceiving(), "uncoupled: left where it landed, nothing perceived");
    let mut b = budget();
    assert!(mind.act_on_returns(&mut b, vec![b"a scene".to_vec()], false) > 0, "a binding is a return");
    assert!(mind.board.court.task.as_ref().is_some_and(|(_, o)| o.crossed), "the first return carries it across");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 0);
    assert!(mind.act_on_returns(&mut b, vec![b"a scene".to_vec()], false) > 0, "and again");
    assert!(mind.board.court.task.is_none(), "uncoupled across a return: it lapses");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 1, "passed, said to the observer");
}
