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

/// A SETTLED MIND WITH ITS CORPUS HOLDING ASKS FOR NOTHING; A STRAINED ONE
/// HOLDS TENSION. A mind just opened holds no demand, no ringing orbit, no
/// reservoir; with an item standing on its corpus it is in equilibrium, and
/// the winding's ask for a scene (`pull_task`) posts nothing. The slide's two
/// hours perceived, the relation between them never returns and stands on
/// its plan: the mind now holds tension.
#[test]
fn a_settled_mind_asks_for_nothing_and_a_strained_one_holds_tension() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("settled"));
    mind.place_question(b"a question placed", &[(0, 0, 1, 2)]);
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
    assert!(!asked, "in equilibrium, its corpus holding, it asks for nothing");
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
/// is — holds no tension and, its corpus holding a question, asks the court
/// for nothing; offered the one-cell
/// family, it couples through its language and takes it in; offered the
/// slide, on charts its language does not speak on, it couples to nothing.
#[test]
fn a_restored_mind_takes_an_offer_in_its_own_language_and_asks_for_nothing() {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("restored"));
    mind.place_question(b"a question placed", &[(0, 0, 1, 2)]);
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

/// A family of two cells whose hues `a` and `b` swap where they stand: a hue
/// map that is a cycle, so its 𝒢 orbit stands (an injection seals at once,
/// its order one).
fn two_cells_swapped(a: u8, b: u8) -> Vec<(crate::arc::Grid, crate::arc::Grid)> {
    let at = |left: u8, right: u8| -> crate::arc::Grid { vec![vec![0, 0, 0, 0], vec![0, left, right, 0], vec![0, 0, 0, 0]] };
    vec![(at(a, b), at(b, a))]
}

/// A feed folder holding these families, each at its file.
fn feed_of(name: &str, families: &[(&str, Vec<(crate::arc::Grid, crate::arc::Grid)>)]) -> std::path::PathBuf {
    let home = std::env::temp_dir().join(format!("mycelium-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(home.join("feed")).expect("the feed folder");
    for (file, train) in families {
        std::fs::write(home.join("feed").join(file), family_json(train)).expect("written");
    }
    home
}

/// THE CORPUS IS MET WHERE THE MIND'S ORBITS STAND, never along a file's line
/// (the operator, 2026-10-18). Two families wait in the feed, two cells
/// swapping hues 3 and 4 first in file order and two swapping 1 and 2 after
/// it. A mind whose field holds no orbit and whose language speaks on no
/// chart is never frozen (the curiosity law, 2026-10-23): it meets exactly one
/// — the least corpus chart — and takes it in, the other undisturbed. A mind
/// whose field rings a 𝒢 orbit on the support {1, 2} meets the family whose
/// hue fibre moves those hues — at that orbit's own coordinate — and takes it
/// in, its file retired; the other lies elsewhere and stays.
#[test]
fn the_corpus_is_met_where_the_minds_orbits_stand() {
    use crate::transducer::cech::Thread;
    let families = [("a-three.json", two_cells_swapped(3, 4)), ("b-one.json", two_cells_swapped(1, 2))];
    let still = feed_of("focus-still", &families);
    let mut quiet = Entity::open("127.0.0.1:1", Kernel::named("quiet"));
    quiet.open_feed(still.clone());
    let mut field = std::mem::take(&mut quiet.field);
    field.think(|| quiet.corpus_inflow(&mut crate::physics::StrokeOutcome::default()));
    quiet.field = field;
    assert_eq!(quiet.board.corpus.size().0, 1, "both placed, one met and taken off the corpus");
    let stayed = [still.join("feed/a-three.json").exists(), still.join("feed/b-one.json").exists()];
    assert_eq!(stayed.iter().filter(|s| **s).count(), 1, "no orbit, no language: the least chart met, the other undisturbed");
    assert!(quiet.perceiving(), "never frozen: the one met is on the perception carrier");
    let home = feed_of("focus-ringing", &families);
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("ringing"));
    mind.open_feed(home.clone());
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        let rang = crate::transducer::mind::with_field(|f| f.orbits.admit_hues(&[(1, 2), (2, 1)], 1, &Thread::Free).is_some());
        assert!(matches!(rang, Ok(true)), "a 𝒢 orbit on the support {{1, 2}}");
        mind.corpus_inflow(&mut crate::physics::StrokeOutcome::default());
    });
    mind.field = field;
    assert!(!home.join("feed/b-one.json").exists() && home.join("feed/.done/b-one.json").exists(), "the family on the orbit's coordinate is met and taken in");
    assert!(home.join("feed/a-three.json").exists(), "the other lies elsewhere on the manifold, undisturbed");
    assert!(mind.perceiving(), "taken in: on the perception carrier");
    let _ = std::fs::remove_dir_all(&still);
    let _ = std::fs::remove_dir_all(&home);
}

/// THE GROUND STATE RESONATES WITH ITS LANGUAGE (the auditor's ruling,
/// 2026-10-18). A mind restored with a word on the swapped cells' charts and
/// nothing else — no thought, no orbit in motion, none ringing — is at rest,
/// not deaf: of the swap and the slide waiting in its feed, it meets the one
/// its language speaks on, takes it in and leaves the slide undisturbed. The
/// perception's admission excites the field: the swap's 𝒢 cycle stands in it
/// (a cycle, so its orbit does not seal at once), and the field's orbits —
/// not the language — are the focus from then on.
#[test]
fn a_mind_at_rest_meets_what_its_language_speaks_on_and_the_admission_moves_its_field() {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let [(x, y)] = <[_; 1]>::try_from(two_cells_swapped(1, 2)).expect("one pair");
    let home = feed_of("ground-state", &[("a-slide.json", two_hours_of_a_slide().to_vec()), ("b-cell.json", two_cells_swapped(1, 2))]);
    // The word is read on a field of its own: what the restored mind holds is
    // the word, never the field its reading moved.
    let mut elsewhere = Entity::open("127.0.0.1:1", Kernel::named("elsewhere")).field;
    let word = elsewhere.think(|| {
        let mut b = budget();
        let frames = perceive_family(&[(x.clone(), y.clone())]);
        let f = scene_form_on(&NoCrystal, &mut b, &x, &y, &frames).expect("the pair reads");
        crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a word")
    });
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("ground"));
    mind.open_feed(home.clone());
    mind.epigenome.moduli_admit(&crate::transducer::element::Held::of(word));
    let mut field = std::mem::take(&mut mind.field);
    let (quiet, excited) = field.think(|| {
        let quiet = mind.contemplation.is_empty() && matches!(crate::transducer::mind::with_field(|f| f.orbits.focus().is_empty()), Ok(true));
        mind.corpus_inflow(&mut crate::physics::StrokeOutcome::default());
        mind.settle_perception();
        let excited = matches!(crate::transducer::mind::with_field(|f| !f.orbits.focus().is_empty()), Ok(true));
        (quiet, excited)
    });
    mind.field = field;
    assert!(quiet, "the ground state: no orbit in motion or ringing, no thought");
    assert!(home.join("feed/.done/b-cell.json").exists(), "the family on its language's chart is met sympathetically and taken in");
    assert!(home.join("feed/a-slide.json").exists(), "the slide, on charts its language does not speak on, stays undisturbed");
    assert!(excited, "the admission excites the field: its orbits are the focus now");
    let _ = std::fs::remove_dir_all(&home);
}

/// THE CORPUS CURIOSITY LAW (the auditor's ruling, 2026-10-23): where the
/// ground focus meets nothing, the vacuum couples to the nearest stratum — the
/// corpus charts sharing the deepest exact prefix of axes with a chart the
/// language speaks on, ties all held in key order; where none shares a leading
/// axis, or the language speaks on no chart, the least corpus chart alone.
#[test]
fn the_ground_state_couples_to_the_nearest_stratum_when_its_language_meets_nothing() {
    use crate::client::sdk::entity::focus::curiosity;
    use crate::transducer::stalk::Invariant;
    let coord = |ns: &[u64]| {
        let mut c = vec![b'c'];
        c.extend_from_slice(Invariant::counts(ns).key().as_bytes());
        c
    };
    let held = |k: &crate::network::nerve::Keyed<()>| k.scoped().map(|(c, ())| c).collect::<Vec<_>>();
    let language = [Invariant::counts(&[2, 1, 0])];
    // depth 2, depth 1, depth 0, depth 2 — in no particular order of arrival.
    let corpus = [coord(&[2, 1, 5]), coord(&[2, 9]), coord(&[7]), coord(&[2, 1, 6])];
    let mut nearest = vec![coord(&[2, 1, 5]), coord(&[2, 1, 6])];
    nearest.sort();
    assert_eq!(held(&curiosity(&language, corpus.iter().cloned())), nearest, "the deepest shared prefix, its ties all held");
    let disjoint = [Invariant::counts(&[9])];
    let mut all = corpus.to_vec();
    all.sort();
    let least = vec![all[0].clone()];
    assert_eq!(held(&curiosity(&disjoint, all.iter().cloned())), least, "no shared axis: the least chart alone");
    assert_eq!(held(&curiosity(&[], all.iter().cloned())), least, "a language on no chart: the least chart alone");
}

/// A RESIDUAL CHAIN IS AN OPEN QUESTION, NEVER A WORD (the operator,
/// 2026-10-18). A court teaches one: it is not learned into the genome, and
/// stands on the mind's carrier of open questions, once however often taught;
/// the mind, holding nothing else, now holds tension; placed on the corpus,
/// it is met where the mind's field rings on its hue fibre's support, taken
/// into thought, and leaves the carrier.
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
    assert_eq!(mind.board.corpus.size().0, 1, "placed on the corpus at its coordinates");
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        // A cycle on the support {1, 3} — the hues the residual's map moves.
        let _ = crate::transducer::mind::with_field(|f| f.orbits.admit_hues(&[(1, 3), (3, 1)], 1, &crate::transducer::cech::Thread::Free));
        mind.corpus_inflow(&mut crate::physics::StrokeOutcome::default());
    });
    mind.field = field;
    assert!(mind.board.open_residuals.is_empty(), "met where the field rings, taken into thought, it leaves the carrier");
}

/// AN OPEN QUESTION IS A CHAIN. The residual's tag also opens a word with no
/// chain (a rewrite, a letters word, a bare action): it has no scene to be
/// met at, and read as a question it stood open forever. Only an element
/// that carries a chain is one.
#[test]
fn only_a_chain_under_the_residual_tag_is_an_open_question() {
    use crate::client::sdk::epigenome::is_residual;
    let bare = crate::transducer::element::Element::identity();
    assert!(
        matches!(bare.address().as_bytes(), [tag, ..] if *tag == crate::transducer::address::TAG_RESIDUAL),
        "the chain-free word stands under the residual's tag"
    );
    assert!(!is_residual(&bare), "a word with no chain is no open question");
    let chain = vec![(0i64, 0i64, 1u8, 2u8), (2, 2, 3, 4)];
    let residual = crate::transducer::element::from_residual(&chain).expect("a residual chain");
    assert!(is_residual(&residual), "a chain under the tag is one");
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

/// AN UNCOUPLED TASK LAPSES AT THE COURT'S NEXT FINAL, IN THE GROUND STATE OR
/// OUT OF IT (ruled 2026-10-24). A mind
/// with no orbit in motion has no return to wait across, nor has one whose
/// orbits stand and never return. The task landed under final `h` 10 waits
/// while no final is carried, and while the one carried is the one it landed
/// under; the next final past it lets it go — kept as lapsed by its citation,
/// passed — with no return in between. Every stroke it waited is counted.
#[test]
fn an_uncoupled_task_lapses_at_the_courts_next_final_in_the_ground_state() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("ground lapse"));
    lapses_at_the_next_final(&mut mind);
}

/// The same, a 𝒢 orbit standing on the field — its orbits in focus, no
/// ground state — and the one-cell task on no chart the mind holds.
#[test]
fn an_uncoupled_task_lapses_at_the_courts_next_final_in_a_field_in_motion() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("moving lapse"));
    let mut field = std::mem::take(&mut mind.field);
    let in_focus = field.think(|| {
        let _ = crate::transducer::mind::with_field(|f| f.orbits.admit_hues(&[(1, 2), (2, 1)], 1, &crate::transducer::cech::Thread::Free));
        matches!(crate::transducer::mind::with_field(|f| !f.orbits.focus().is_empty()), Ok(true))
    });
    mind.field = field;
    assert!(in_focus, "an orbit in focus: not the ground state");
    lapses_at_the_next_final(&mut mind);
}

fn lapses_at_the_next_final(mind: &mut Entity) {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    use crate::client::sdk::entity::perceive::Stop;
    let id = [6u8; 32];
    let train = one_cell_steps();
    let offer = Offer { landed: Some(10), ..Offer::of(&train) };
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id, train, tests: Vec::new() }, offer));
    mind.note_task(&id, Reading::Task, |_| {});
    // A final heard on the hypha's carrier, read back off its view once the
    // carrier has published it.
    let hear = |mind: &mut Entity, h: u64| {
        let p = TPoint { origin: Origin::default(), h, lanes: [1, 0, 0, 0], k: 1 };
        mind.ask(move |s| s.hear_point(Some(p)), |_, ()| {});
        let mut polls = 0u32;
        while mind.hypha.tuniversal().map(|p| p.h) != Some(h) && polls < (1 << 24) {
            std::thread::yield_now();
            polls += 1;
        }
        assert_eq!(mind.hypha.tuniversal().map(|p| p.h), Some(h), "the final is carried");
    };
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| mind.lapse_standing_task());
    assert!(mind.board.court.task.is_some(), "no final carried, no aperture measured: it waits");
    hear(mind, 10);
    field.think(|| mind.lapse_standing_task());
    assert!(mind.board.court.task.as_ref().is_some_and(|(_, o)| o.waited == 2 && !o.crossed), "the final it landed under: it waits, two strokes counted, no return crossed");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 0);
    hear(mind, 11);
    field.think(|| mind.lapse_standing_task());
    mind.field = field;
    assert!(mind.board.court.task.is_none(), "the court's next final: it lapses");
    assert_eq!(mind.board.offers_passed[PASSED_TASK], 1, "passed, said to the observer");
    assert_eq!(mind.board.tasks.get(&id).and_then(|t| t.stop), Some(Stop::Lapsed), "kept as let go uncoupled");
}

/// A SETTLED MIND WHOSE CORPUS HOLDS NO ITEM ASKS (ruled 2026-10-24). A mind just
/// opened, settled, its corpus empty and its reserve full, asks the court for
/// a task; with an item placed on its corpus, the same settled mind asks for
/// nothing. A task standing on the board is never asked past.
#[test]
fn a_settled_mind_on_an_empty_corpus_asks_and_one_whose_corpus_holds_does_not() {
    let mut vacuum = Entity::open("127.0.0.1:1", Kernel::named("vacuum"));
    let mut field = std::mem::take(&mut vacuum.field);
    let (settled, asked) = field.think(|| {
        let settled = vacuum.strained();
        vacuum.pull_task();
        (settled, vacuum.board.court.pulling)
    });
    vacuum.field = field;
    assert!(!settled && vacuum.board.corpus.size().0 == 0, "settled, the corpus empty");
    assert!(asked, "the corpus empty: it asks");
    let mut held = Entity::open("127.0.0.1:1", Kernel::named("holding"));
    held.place_question(b"a question placed", &[(0, 0, 1, 2)]);
    let mut field = std::mem::take(&mut held.field);
    field.think(|| held.pull_task());
    held.field = field;
    assert!(!held.board.court.pulling, "its corpus holding: equilibrium, no ask");
    let mut offered = Entity::open("127.0.0.1:1", Kernel::named("offered"));
    let train = one_cell_steps();
    let offer = Offer::of(&train);
    offered.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id: [3; 32], train, tests: Vec::new() }, offer));
    let mut field = std::mem::take(&mut offered.field);
    field.think(|| offered.pull_task());
    offered.field = field;
    assert!(!offered.board.court.pulling, "a task on the board is not asked past");
}

/// A MIND WITH A COURT'S TASK ON ITS BOARD OWES ITS OWN STROKES (ruled
/// 2026-10-24); one that holds nothing owes none and may block.
#[test]
fn a_task_on_the_board_is_owed_strokes_and_a_quiet_mind_is_quiescent() {
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("quiescent"));
    assert!(!mind.owes_strokes(), "nothing held: quiescent");
    let train = one_cell_steps();
    let offer = Offer::of(&train);
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id: [4; 32], train, tests: Vec::new() }, offer));
    assert!(mind.owes_strokes(), "a task waiting: owed strokes");
}
