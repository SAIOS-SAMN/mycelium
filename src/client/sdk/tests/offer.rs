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

/// A RESTORED MIND HEARS AN OFFER IN ITS OWN LANGUAGE, AND STILL ASKS FOR
/// NOTHING (the operator, 2026-10-17: a string resonates because it is tuned,
/// not because it rings). A mind whose language holds a word on the one-cell
/// body's chart — and no thought, no plan, no orbit: what a restored genome
/// is — holds no tension and asks the court for nothing; offered the one-cell
/// family, it couples through its language and takes it in; offered the
/// slide, on charts its language does not speak on, it couples to nothing.
#[test]
fn a_restored_mind_takes_an_offer_in_its_own_language_and_asks_for_nothing() {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("restored"));
    let [(x, y)] = <[_; 1]>::try_from(one_cell_steps()).expect("one pair");
    let mut field = std::mem::take(&mut mind.field);
    let (settled, asked) = field.think(|| {
        let mut b = budget();
        let frames = perceive_family(&[(x.clone(), y.clone())]);
        let f = scene_form_on(&NoCrystal, &mut b, &x, &y, &frames).expect("the pair reads");
        let word = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word");
        mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word));
        let settled = mind.strained();
        mind.pull_task();
        (settled, mind.board.court.pulling)
    });
    mind.field = field;
    assert!(mind.contemplation.is_empty(), "no thought held: the language alone");
    assert!(!settled && !asked, "a settled mind with a language holds no tension and asks for nothing");
    let cell = Offer::of(&one_cell_steps());
    assert!(mind.couples(&[7; 32], &cell.charts), "an offer on a chart its language speaks on couples");
    assert!(!mind.couples(&[8; 32], &Offer::of(&two_hours_of_a_slide()).charts), "one on charts it does not speak on couples to nothing");
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id: [7; 32], train: one_cell_steps(), tests: Vec::new() }, cell));
    let mut outcome = crate::physics::StrokeOutcome::default();
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| mind.task_inflow(&mut outcome));
    mind.field = field;
    assert!(mind.board.court.task.is_none() && mind.perceiving(), "coupled: taken in, on the perception carrier");
}

/// A family as the ARC corpus writes it, its pairs as given and no test.
fn family_json(train: &[(crate::arc::Grid, crate::arc::Grid)]) -> String {
    let pairs: Vec<serde_json::Value> = train.iter().map(|(x, y)| serde_json::json!({ "input": x, "output": y })).collect();
    serde_json::json!({ "train": pairs, "test": [] }).to_string()
}

/// THE FEED IS WALKED, NEVER BLOCKED (the operator, 2026-10-18). A mind whose
/// language speaks on the one-cell chart and nowhere the slide's bodies
/// stand finds two families in its feed, the slide first in file order. The
/// slide couples to nothing: it is passed at once and stays on disk, said to
/// the observer once; the walk goes on to the one-cell family behind it,
/// which couples, is taken in and perceived, its file retired. Walked to the
/// listing's end, the feed waits — listed again only where a family arrives
/// or the language moves.
#[test]
fn the_feed_is_walked_and_an_uncoupled_family_never_stands_in_front() {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let home = std::env::temp_dir().join(format!("mycelium-feed-walk-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    let feed = home.join("feed");
    std::fs::create_dir_all(&feed).expect("the feed folder");
    std::fs::write(feed.join("a-slide.json"), family_json(&two_hours_of_a_slide())).expect("written");
    std::fs::write(feed.join("b-cell.json"), family_json(&one_cell_steps())).expect("written");
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("feeding"));
    mind.open_feed(home.clone());
    let [(x, y)] = <[_; 1]>::try_from(one_cell_steps()).expect("one pair");
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        let mut b = budget();
        let frames = perceive_family(&[(x.clone(), y.clone())]);
        let f = scene_form_on(&NoCrystal, &mut b, &x, &y, &frames).expect("the pair reads");
        let word = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word");
        mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word));
        let mut outcome = crate::physics::StrokeOutcome::default();
        mind.feed_inflow(&mut outcome);
    });
    mind.field = field;
    assert!(feed.join("a-slide.json").exists(), "the slide, uncoupled, is passed and stays on disk");
    assert!(!feed.join("b-cell.json").exists() && feed.join(".done").join("b-cell.json").exists(), "the family behind it couples, is taken, its file retired");
    assert!(mind.perceiving(), "taken in: on the perception carrier");
    assert_eq!(mind.board.offers_passed[crate::client::sdk::entity::offer::PASSED_FEED], 1, "the slide said passed, once");
    let _ = std::fs::remove_dir_all(&home);
}

/// A RESIDUAL CHAIN IS AN OPEN QUESTION, NEVER A WORD (the operator,
/// 2026-10-18). A court teaches one: it is not learned into the genome, and
/// stands on the mind's carrier of open questions, once however often taught;
/// the mind, holding nothing else, now holds tension; at its next intake the
/// question is taken into thought and leaves the carrier.
#[test]
fn a_residual_taught_is_an_open_question_and_never_a_word() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("questioning"));
    let chain = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2), (1, 0, 1, 2), (2, 2, 3, 4)];
    let residual = crate::transducer::element::from_residual(&chain).expect("a residual chain");
    let tape = residual.to_bytes().expect("the record writes");
    let mut field = std::mem::take(&mut mind.field);
    let (settled, strained) = field.think(|| {
        let settled = mind.strained();
        let _ = mind.absorb_taught(&tape);
        let _ = mind.absorb_taught(&tape);
        (settled, mind.strained())
    });
    mind.field = field;
    assert!(!settled, "a mind just opened holds no strain");
    assert!(!mind.epigenome.holds(residual.address().as_bytes()), "never learned into the genome");
    assert_eq!(mind.board.open_residuals.len(), 1, "one open question, however often taught");
    let stratum = crate::transducer::stalk::invariant_of(&residual).key();
    assert!(mind.board.residual_strata.contains(stratum.as_bytes()), "its stratum is one the sheaf sync never fetches");
    assert!(strained, "an open question is tension");
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| mind.question_inflow());
    mind.field = field;
    assert!(mind.board.open_residuals.is_empty(), "taken into thought, it leaves the carrier");
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
