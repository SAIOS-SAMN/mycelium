// 78.177 (ruled 2026-11-06), MU-CLIENT §5.6 (E2) — A DEMAND CLOSED AT
// ANOTHER PERSPECTIVE PAYS THE CLOSER: a listing's demand vertex, answered by
// a probe citing a form this court holds, closes where the court's own
// speech of that form reproduces the demand's after side, and the closer is
// minted the cited form's discharge on the listing's family as POWC. The
// listing's own speaker closing it is paid nothing (5.7(d)).

use super::fixtures::*;
use crate::cognition::*;
use crate::transducer::element::Element;

fn digits_family() -> Vec<crate::transducer::family::TrainPair> {
    crate::transducer::raster::stand_on(0);
    vec![(digits(&["1100", "0000"]), digits(&["0011", "0000"]))]
}

/// The scene the listing and the question are bound to.
const SCENE: [u8; 32] = [6; 32];

/// The slide by `(r, c)`, spelled as a connection: the heard word.
fn slide(r: i64, c: i64) -> Element {
    let w = crate::transducer::alphabet::Word::identity();
    crate::transducer::element::residue::from_connection(&w, &crate::transducer::element::translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// A court holding the scene's family, with a closed form for it glued
/// into its basis by two fixture voices (§3.4's wire-identifier exception:
/// nothing here is about 47.3's witness). Answers the form's address.
fn court_holding_a_form() -> (Court, crate::transducer::address::Address) {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    let family = digits_family();
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the slide's rule");
    let sentence = mint_sentence(&rule, &family, &NoCrystal, 0).expect("its sentence");
    let mut k = Court::new();
    k.bind_task_family(SCENE, family);
    let _ = k.perceive_sentence([1; 32], &SCENE, &sentence);
    let _ = k.perceive_sentence([2; 32], &SCENE, &sentence);
    k.glue_local();
    let held = k.basis().into_iter().find(|h| h.form().is_some_and(|f| f.is_closed())).expect("a closed form glued into the basis");
    (k, held.address())
}

/// THE ASKER LISTS ITS QUESTION: the heard word's bodies on the scene
/// (`listed::heard_bodies`) stood as demand vertices of a host sentence,
/// put on the book as `asker`'s and bound to the scene. Answers the
/// listing's address and the probe target of its first demand.
fn list_question(k: &mut Court, asker: [u8; 32]) -> (crate::transducer::address::Address, Vec<u8>) {
    let family = digits_family();
    let demands = crate::client::sdk::mind::listed::heard_bodies(&slide(0, 2), &family);
    assert!(!demands.is_empty(), "the heard word's section on its scene names bodies");
    let host = sentence_of(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 1, 1), (1, 0, 1), (1, 1, 1)]).expect("a host sentence");
    let mut f = crate::transducer::sentence::from_element_form(&host).expect("its form");
    for (kind, chart, bodies) in demands {
        f.put_vertex(crate::transducer::family::demand_vertex(kind, &chart, bodies)).expect("a demand has a coordinate");
    }
    f.reindex();
    let listing = crate::transducer::sentence::as_element_form(&f, host.magnitude()).expect("a sentence tape");
    let at = listing.address();
    let read = crate::transducer::sentence::from_element_form(&listing).expect("its tape decodes");
    let (vertex, ..) = read.demand_vertices().next().expect("the demand rides the tape");
    let target = crate::transducer::sentence::demand_at(at.as_bytes(), &vertex);
    k.book_list(listing, asker);
    k.note_discovered(at.as_bytes(), asker);
    k.bind_listing_task(at.as_bytes(), SCENE);
    (at, target)
}

/// What `who` was minted as POWC.
fn powc_minted(k: &Court, who: [u8; 32]) -> usize {
    k.treasury.entries().iter().filter(|e| e.who == who && e.kind == crate::economy::Kind::Mint && e.arm == Some(crate::economy::Arm::Powc)).count()
}

/// The probe `closer` speaks at `target`, citing the held form.
fn probe(k: &mut Court, closer: [u8; 32], target: &[u8], cited: &crate::transducer::address::Address) -> crate::anastome::dialogue::ProbeReading {
    let cite = crate::anastome::dialogue::CiteWire { addr: cited.clone(), standing: 0, transport: crate::transducer::element::identity_versor(), body: Vec::new() };
    k.settle_probe(closer, target, Vec::new(), std::slice::from_ref(&cite), false)
}

/// E2'S GUARD — A DEMAND CLOSED BY ANOTHER MIND'S CITED FORM PAYS THE
/// CLOSER. The listing's demand vertex is closed by a probe citing a form
/// the court holds: the court speaks the form it holds on the demand's own
/// bodies, the closure is read, and the closer is minted POWC; the
/// listing's speaker is minted nothing.
#[test]
fn demand_closed_by_another_minds_cited_form_pays_the_closer() {
    let (asker, closer) = ([3; 32], [4; 32]);
    let (mut k, cited) = court_holding_a_form();
    let (_, target) = list_question(&mut k, asker);
    let reading = probe(&mut k, closer, &target, &cited);
    assert_eq!(reading.closure, crate::anastome::dialogue::ProbeClosure::Closed, "the held form reproduces the demand's after side");
    assert_eq!(powc_minted(&k, closer), 1, "the closer is minted the citation, once");
    assert_eq!(powc_minted(&k, asker), 0, "the listing's speaker is minted nothing");
}

/// 78.177 — A HELD QUESTION CLOSED BY ANOTHER MIND PAYS THE CLOSER. The
/// asker's question, raised on a heard word that carried its evidence,
/// stands as the word's bodies on its scene; another mind reaching it on
/// the listing closes it with a form the court holds, and is paid; the
/// asker is paid nothing.
#[test]
fn held_question_closed_by_another_mind_pays_the_closer() {
    let (asker, closer) = ([5; 32], [7; 32]);
    let (mut k, cited) = court_holding_a_form();
    let (_, target) = list_question(&mut k, asker);
    let before = k.treasury.balance_mu(&asker);
    let _ = probe(&mut k, closer, &target, &cited);
    assert!(powc_minted(&k, closer) > 0, "the closer is paid the convergence");
    assert_eq!(k.treasury.balance_mu(&asker), before, "the asker is paid nothing");
}

/// 5.7(d) — A MIND CLOSING ITS OWN LISTED QUESTION IS PAID NOTHING: the
/// same closure, probed by the listing's own speaker, mints no POWC.
#[test]
fn a_mind_closing_its_own_listed_question_is_paid_nothing() {
    let asker = [8; 32];
    let (mut k, cited) = court_holding_a_form();
    let (_, target) = list_question(&mut k, asker);
    let reading = probe(&mut k, asker, &target, &cited);
    assert_eq!(reading.closure, crate::anastome::dialogue::ProbeClosure::Closed, "the closure itself stands");
    assert_eq!(powc_minted(&k, asker), 0, "no self-pay");
}
