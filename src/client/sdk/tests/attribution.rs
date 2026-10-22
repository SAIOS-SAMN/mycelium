// THE AUDITOR'S ATTRIBUTION READING (2026-10-22) — every court task that
// lands on a mind is kept by its whole citation: taken in or let go, and
// where its comprehension stopped.

use super::common::*;
use crate::client::sdk::entity::offer::Offer;
use crate::client::sdk::entity::perceive::{Reading, Scene, Stop};
use crate::client::sdk::*;

/// A TASK TAKEN IN IS KEPT BY ITS CITATION, TAKEN, WITH ITS STOP: the slide's
/// two hours perceived as a court task stand under their citation, taken,
/// stopped where their comprehension stopped — never left unattributed.
#[test]
fn a_task_taken_in_is_kept_by_its_citation_with_its_stop() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("attributed"));
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
fn a_task_let_go_uncoupled_is_kept_as_lapsed() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("lapsing"));
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
