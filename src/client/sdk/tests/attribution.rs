// THE ATTRIBUTION READING (ruled 2026-10-22) — every court task that
// lands on a mind is kept by its whole citation: taken in or let go, and
// where its comprehension stopped.

use super::common::*;
use crate::client::sdk::entity::offer::Offer;
use crate::client::sdk::entity::perceive::{Reading, Scene, Stop};
use crate::client::sdk::*;

/// A TASK TAKEN IN IS KEPT BY ITS CITATION, TAKEN, WITH ITS STOP: the slide's
/// two hours perceived as a court task stand under the task's citation, taken,
/// stopped where the task's comprehension stopped — never left unattributed.
#[test]
fn perceive_scene_keeps_task_by_citation_with_stop() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("attributed"));
    let id = [7u8; 32];
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        mind.perceive_scene(Scene { id: id.to_vec(), train: two_hours_of_a_slide().to_vec(), tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
    });
    mind.field = field;
    let t = mind.board.tasks.get(&id).copied().expect("the task is kept by its citation");
    assert!(t.taken, "taken in");
    assert_eq!(t.reading, Reading::Task);
    assert!(t.stop.is_some(), "its comprehension stopped somewhere, named: {t:?}");
    assert!(mind.board.tasks.get(&[6u8; 32]).is_none(), "a citation never landed is not kept");
}

/// A TASK THAT COUPLES TO NOTHING IS KEPT AS LET GO: landed on a mind that
/// holds nothing, it crosses one return and lapses at the next — kept by its
/// citation, untaken, `lapsed uncoupled`.
#[test]
fn lapse_offers_keeps_uncoupled_task_as_lapsed() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("lapsing"));
    let id = [5u8; 32];
    let train = two_hours_of_a_slide().to_vec();
    let offer = Offer::of(&train);
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id, train, tests: Vec::new() }, offer));
    mind.note_task(&id, Reading::Task, |_| {});
    mind.lapse_offers();
    assert_eq!(mind.board.tasks.get(&id).map(|t| t.stop), Some(None), "one return crossed: still waiting");
    mind.lapse_offers();
    let t = mind.board.tasks.get(&id).copied().expect("kept by its citation");
    assert!(!t.taken, "never taken in");
    assert_eq!(t.stop, Some(Stop::Lapsed), "let go uncoupled");
}
