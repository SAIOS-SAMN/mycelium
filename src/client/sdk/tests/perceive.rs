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
/// stopped (ruled 2026-10-18). The slide taken as a feed's
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
    let sent = STOPS.iter().position(|s| *s == Stop::Sent.name()).expect("the solve sent is a named stop");
    assert_eq!(feed[sent], 0, "a feed's family carries no test: it sends no solve");
}

/// A WAITING SCENE RESTS UNTIL ITS OWN LANGUAGE MOVES (ruled
/// 2026-10-20: 461 re-reads in an hour on one voice). The
/// language where the slide stands — its charts and the words filed on
/// them — does not move when the mind gains a word on a chart the slide's
/// bodies do not stand on, so the slide is not read again; it moves when a
/// word is filed on the slide's own chart.
#[test]
fn a_waiting_scene_rests_until_a_word_lands_where_it_stands() {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let word_of = |x: &crate::arc::Grid, y: &crate::arc::Grid| {
        let mut b = budget();
        let frames = perceive_family(&[(x.clone(), y.clone())]);
        let f = scene_form_on(&NoCrystal, &mut b, x, y, &frames).expect("the pair reads");
        crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word")
    };
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("resting"));
    let slide: Vec<(crate::arc::Grid, crate::arc::Grid)> = two_hours_of_a_slide().to_vec();
    let before = mind.scene_language(&slide);
    // A word on a chart elsewhere: one cell stepping a column.
    let at = |c: usize| -> crate::arc::Grid { (0..3).map(|r| (0..3).map(|x| u8::from(r == 1 && x == c)).collect()).collect() };
    mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word_of(&at(0), &at(1))));
    assert_eq!(mind.scene_language(&slide), before, "a word elsewhere leaves the slide's language where it was: it rests");
    // A word on the slide's own chart: its first hour, said as a word.
    let [(x, y), _] = two_hours_of_a_slide();
    mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word_of(&x, &y)));
    assert_ne!(mind.scene_language(&slide), before, "a word where the slide stands moves its language: it is read again");
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

/// THE GROUND STATE HEARS ITS LANGUAGE'S CHARTS (the ground-state
/// ruling, 2026-10-18, extended to hearing 2026-10-23): a
/// mind at rest — no thought, no orbit, a language that holds a word on the
/// one-cell body's chart — takes up a hearing whose evidence scene stands on
/// that chart, and lets one standing only on charts its language does not
/// speak on wait, as an empty field couples to nothing else.
#[test]
fn a_mind_at_rest_hears_what_is_said_on_its_languages_charts() {
    use crate::client::sdk::entity::converge::Evident;
    use crate::client::sdk::entity::offer::Offer;
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let at = |c: usize| -> crate::arc::Grid { (0..3).map(|r| (0..3).map(|x| u8::from(r == 1 && x == c)).collect()).collect() };
    let cell = vec![(at(0), at(1))];
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("at rest"));
    let mut field = std::mem::take(&mut mind.field);
    let (elsewhere, on_its_chart) = field.think(|| {
        let mut b = budget();
        let (x, y) = &cell[0];
        let frames = perceive_family(&cell);
        let f = scene_form_on(&NoCrystal, &mut b, x, y, &frames).expect("the pair reads");
        let word = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word");
        mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word));
        let speaker = [3u8; 32];
        mind.perceive_hearing_on(speaker, b"elsewhere".to_vec(), Vec::new(), Evident::True, Offer::of(&two_hours_of_a_slide()).charts);
        let elsewhere = mind.board.hearing;
        mind.perceive_hearing_on(speaker, b"on its chart".to_vec(), Vec::new(), Evident::True, Offer::of(&cell).charts);
        (elsewhere, mind.board.hearing)
    });
    mind.field = field;
    assert!(!elsewhere, "a word said on charts its language does not speak on waits");
    assert!(on_its_chart, "a word said on a chart its language speaks on is heard at rest");
}
