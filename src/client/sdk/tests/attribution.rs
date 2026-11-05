// THE ATTRIBUTION READING (ruled 2026-10-22) — every court task that
// lands on a mind is kept by its whole citation: taken in or let go, and
// where its comprehension stopped.

use super::fixtures::*;
use crate::client::sdk::mind::offer::Offer;
use crate::client::sdk::mind::perceive::{SceneKind, Scene, Stop};
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
        mind.perceive_scene(Scene { id: id.to_vec(), train: two_hours_of_a_slide().to_vec(), tests: Vec::new(), reading: SceneKind::Task });
        mind.settle_perception();
    });
    mind.field = field;
    let t = mind.board.tasks.get(&id).copied().expect("the task is kept by its citation");
    assert!(t.taken, "taken in");
    assert_eq!(t.reading, SceneKind::Task);
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
    mind.board.court.task = Some((crate::anastome::dialogue::TaskSection { id, train, tests: Vec::new() }, offer));
    mind.note_task(&id, SceneKind::Task, |_| {});
    mind.lapse_offers();
    assert_eq!(mind.board.tasks.get(&id).map(|t| t.stop), Some(None), "one return crossed: still waiting");
    mind.lapse_offers();
    let t = mind.board.tasks.get(&id).copied().expect("kept by its citation");
    assert!(!t.taken, "never taken in");
    assert_eq!(t.stop, Some(Stop::Lapsed), "let go uncoupled");
}

/// MU-DERIVATIONS (the readers), 78.170 O1 — A TASK TAKEN IN IS READ TWICE,
/// SIDE BY SIDE: its count, what `family_magnitude` draws on the mind's
/// metabolism, and the exact share of the frames its pairs change
/// (`family_share`), both on the mind's observe reading. The slide's two
/// hours couple (the scene waits on comprehension) and are taken in on the
/// live intake (`task_inflow`); the share reads whole and is no count.
#[test]
fn intake_reads_family_share_beside_magnitude() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("fed"));
    let id = [8u8; 32];
    let train = two_hours_of_a_slide().to_vec();
    let share = crate::transducer::family::transduce::family_share(train.iter().map(|(x, y)| (x.sample(), y.sample()))).expect("every pair's record reads whole");
    assert!(share > num_rational::BigRational::from_integer(0.into()), "the slide changes a share of each frame");
    let offer = Offer::of(&train);
    let section = crate::anastome::dialogue::TaskSection { id, train, tests: Vec::new() };
    mind.comprehending.insert(id, (section.clone(), crate::transducer::address::Address::from(Vec::new())));
    mind.board.court.task = Some((section, offer));
    let before = crate::observe::mind();
    let mut outcome = crate::physics::StrokeOutcome::default();
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| mind.task_inflow(&mut outcome));
    mind.field = field;
    assert!(mind.board.court.task.is_none(), "taken in");
    let after = crate::observe::mind();
    assert!(after.fed.iter().zip(&before.fed).zip(&outcome.fed).all(|((a, b), m)| *a >= b + m), "the count reached the reading, lane by lane");
    assert!(after.fed_share >= before.fed_share + share, "the exact share reached the reading beside it");
}
