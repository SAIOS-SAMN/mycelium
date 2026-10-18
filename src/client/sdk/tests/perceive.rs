// MU-CLIENT §3.1, §4.12, §7.1 R1/R3/R4 — A SCENE IS PERCEPTION, AND ITS
// ESCAPING THOUGHT IS BOUND: a scene taken in is perceived off the field, on
// the perception carrier, and lands under its citation; the slide between
// its hours escapes onto the docket, and the one demand taker binds it — a
// close of a thought of that scene, which K moves by.

use super::common::*;
use crate::client::sdk::entity::perceive::{Reading, Scene};
use crate::client::sdk::*;

/// THE SCENE, PERCEIVED OFF THE FIELD, LANDS UNDER ITS CITATION; ITS ESCAPING
/// THOUGHT IS BOUND WHERE IT LANDS. Two hours of a slide are taken in as one
/// scene; the perception carrier reads them (R1) and the winding admits the
/// reading: the hours are held under the scene's citation and meet there, and
/// the field lets them go as a slide's motion is let go (MU-CLIENT §4.13) —
/// while the motion between them, which never returns, stands docketed under
/// that citation, and the demand taker binds it (R3), by construction where
/// the language holds no word for it. The binding is a return of a thought of
/// that scene: K moves by it.
#[test]
fn a_scene_perceived_off_the_field_lands_and_its_escaping_thought_is_bound() {
    let id = [9u8; 32];
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("perceiving"));
    let [first, second] = two_hours_of_a_slide();
    let mut field = std::mem::take(&mut mind.field);
    let (bound, returned, k) = field.think(|| {
        mind.perceive_scene(Scene { id: id.to_vec(), train: vec![first, second], tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
        let mut b = budget();
        let bound = mind.take_demands(&mut b);
        let returned = mind.act_on_returns(&mut b, bound.clone(), true);
        (bound, returned, mind.own_k())
    });
    mind.field = field;
    assert!(bound.contains(&id.to_vec()), "the slide between its hours escaped under its citation and was bound: a close of a thought of this scene");
    assert!(returned > 0, "the binding is a return");
    assert_eq!(k, returned, "K moved by exactly the returns");
}

/// MU-CLIENT §7.1 — EVERY SCENE STOPS EXACTLY ONCE, where its comprehension
/// stopped (the auditor's ruling, 2026-10-18). The slide taken as a feed's
/// family and as a court's task each stops at one step of the path to a
/// solve, read on the row of what it was taken as; the feed's family, which
/// carries no test, never reaches a solve.
#[test]
fn every_scene_taken_in_stops_once_on_its_own_reading() {
    use crate::client::sdk::entity::perceive::{Stop, STOPS};
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("stopping"));
    let [first, second] = two_hours_of_a_slide();
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        mind.perceive_scene(Scene { id: vec![5u8; 32], train: vec![first.clone(), second.clone()], tests: Vec::new(), reading: Reading::Feed });
        mind.settle_perception();
        mind.perceive_scene(Scene { id: vec![6u8; 32], train: vec![first, second], tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
    });
    mind.field = field;
    let total = |row: &[u64; STOPS.len()]| row.iter().sum::<u64>();
    let [task, feed, again] = mind.board.stopped;
    assert_eq!((total(&task), total(&feed), total(&again)), (1, 1, 0), "each scene stops once, on its own reading");
    let sent = STOPS.len() - 1;
    assert_eq!(feed[sent], 0, "a feed's family carries no test: it sends no solve");
    assert!(matches!(STOPS.get(sent), Some(&"sent")), "the last stop is the solve sent ({:?})", Stop::Sent);
}

/// MU-CLIENT §4.13, §7.1 — HEARINGS WAIT BY THEIR COORDINATES, AND ONE THAT
/// COUPLES TO NOTHING IS NOT READ. A hearing of no word the mind knows and
/// none that lands on its field never goes to the perception carrier; the
/// same thing said again by the same mind waits once, its latest saying in
/// the place of the first, and a different thing waits beside it.
#[test]
fn a_hearing_said_again_waits_once() {
    use crate::client::sdk::entity::converge::Evident;
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("hearing"));
    let speaker = [3u8; 32];
    mind.perceive_hearing(speaker, b"first".to_vec(), Vec::new(), Evident::Unbound);
    assert!(!mind.board.hearing, "coupled to nothing, it is not read");
    mind.perceive_hearing(speaker, b"again".to_vec(), Vec::new(), Evident::Unbound);
    mind.perceive_hearing(speaker, b"again".to_vec(), Vec::new(), Evident::Unbound);
    assert_eq!(mind.hearings_waiting(), 2, "said again before it is read, it waits once");
    mind.perceive_hearing(speaker, b"other".to_vec(), Vec::new(), Evident::Unbound);
    assert_eq!(mind.hearings_waiting(), 3, "a different thing waits beside it");
}

/// MU-CLIENT §4.13 — A HEARING WAITS AT MOST ACROSS ONE RETURN, the return
/// being the event and no clock read. Three hearings, coupled to nothing,
/// wait; the return that closes the traversal they were heard in carries them
/// across; one more is heard, and one said again; at the next return the two
/// left lapse unread and the two heard since cross in their place. The same
/// thing said again is heard in the traversal it was said in.
#[test]
fn a_hearing_unread_across_a_return_lapses() {
    use crate::client::sdk::entity::converge::Evident;
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("lapsing"));
    let speaker = [4u8; 32];
    for said in [&b"zero"[..], b"one", b"two"] {
        mind.perceive_hearing(speaker, said.to_vec(), Vec::new(), Evident::Unbound);
    }
    let mut b = budget();
    assert!(mind.act_on_returns(&mut b, vec![b"a scene".to_vec()], false) > 0, "a binding is a return");
    assert_eq!(mind.board.hearings_crossed.len(), 3, "the return closing their traversal carries them across");
    mind.perceive_hearing(speaker, b"since".to_vec(), Vec::new(), Evident::Unbound);
    mind.perceive_hearing(speaker, b"two".to_vec(), Vec::new(), Evident::Unbound);
    assert_eq!((mind.board.hearings.len(), mind.board.hearings_crossed.len()), (2, 2), "said again, it is heard in this traversal");
    assert!(mind.act_on_returns(&mut b, vec![b"a scene".to_vec()], false) > 0, "and again");
    assert_eq!(mind.board.hearings_lapsed, 2, "unread across a return, the two left lapse");
    assert_eq!(mind.board.hearings_crossed.len(), 2, "the two heard since cross in its place");
}
