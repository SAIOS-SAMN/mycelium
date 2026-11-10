// 78.177 (ruled 2026-11-06) — A HELD QUESTION CARRIED TO A COURT ONLY UNDER
// EVIDENCE: a question raised on a heard word that carried its evidence is
// listed as that word's bodies on its scene; one with none stays the
// conversation's (E1).

use crate::client::sdk::*;

fn q(n: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(n.into())
}

/// The scene a word was spoken under, and its family.
const SCENE: [u8; 32] = [6; 32];
fn digits(rows: &[&str]) -> crate::transducer::family::SectionSample {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}
fn family() -> Vec<crate::transducer::family::TrainPair> {
    crate::transducer::raster::stand_on(0);
    vec![(digits(&["1100", "0000"]), digits(&["0011", "0000"]))]
}

/// The slide by `(r, c)`, spelled as a connection: the heard word.
fn slide(r: i64, c: i64) -> crate::transducer::element::Element {
    let w = crate::transducer::alphabet::Word::identity();
    crate::transducer::element::residue::from_connection(&w, &crate::transducer::element::translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// A QUESTION WITHOUT EVIDENCE REACHES NO COURT. Two questions stand: one
/// raised on a word that carried no evidence lists no demand; the other,
/// bearing the heard word's evidence on a scene the mind holds, lists the
/// word's bodies there as demands.
#[test]
fn question_without_evidence_reaches_no_court() {
    let speaker = [7u8; 32];
    let word = slide(0, 2);
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("listing"));
    mind.scenes.hold(&SCENE, &family(), &word);
    let mut field = std::mem::take(&mut mind.field);
    let (bare, evidenced) = field.think(|| {
        let key = |said: &[u8]| {
            let mut k = speaker.to_vec();
            k.extend_from_slice(&crate::client::sdk::mind::chatter::question_key(said, b"vertex"));
            k
        };
        let gain = crate::transducer::cga::dilator(&q(2));
        assert!(mind.question(&speaker, b"bare", b"vertex", &gain).is_some(), "the gain is asked about");
        let bare = mind.listed_question(&key(b"bare"));
        mind.bear_evidence(&key(b"bare"), crate::client::sdk::chatter::Evidence { scene: SCENE.to_vec(), word: word.to_bytes().expect("the record writes") });
        (bare, mind.listed_question(&key(b"bare")))
    });
    mind.field = field;
    assert!(bare.is_empty(), "no evidence: nothing a court could measure is listed");
    assert!(!evidenced.is_empty(), "under evidence the heard word's bodies on its scene are listed");
    assert!(evidenced.iter().all(|(k, _, bodies)| matches!(k, crate::transducer::sentence::Demand::Source { .. }) && !bodies.is_empty()), "each a source demand over bodies of the scene");
}

/// 78.177 §4a(2) — THE QUESTION RIDES A SENTENCE OF THE ASKER'S OWN, CLOSED
/// ON THE SCENE, WITH THE DEMAND ADDED. A mind that spoke no closed sentence
/// on the scene has nothing to list the question by; once it holds one, the
/// question is listed by that sentence carrying the heard word's bodies as
/// demand vertices.
#[test]
fn question_rides_the_askers_own_closed_sentence_on_the_scene() {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    let speaker = [9u8; 32];
    let word = slide(0, 2);
    let rule = extract_rule(&family(), &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the scene's rule");
    let own = mint_sentence(&rule, &family(), &NoCrystal, 0).expect("the asker's own sentence on the scene");
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("riding"));
    mind.scenes.hold(&SCENE, &family(), &word);
    let mut field = std::mem::take(&mut mind.field);
    let (without, with) = field.think(|| {
        let mut key = speaker.to_vec();
        key.extend_from_slice(&crate::client::sdk::mind::chatter::question_key(b"said", b"vertex"));
        let gain = crate::transducer::cga::dilator(&q(2));
        assert!(mind.question(&speaker, b"said", b"vertex", &gain).is_some(), "the gain is asked about");
        mind.bear_evidence(&key, crate::client::sdk::chatter::Evidence { scene: SCENE.to_vec(), word: word.to_bytes().expect("the record writes") });
        let without = mind.question_sentence(&key).is_some();
        mind.scenes.hold(&SCENE, &family(), &own);
        (without, mind.question_sentence(&key))
    });
    mind.field = field;
    assert!(!without, "no closed sentence of its own on the scene: nothing to ride");
    let (sentence, _, scene) = with.expect("listed by its own closed sentence");
    assert_eq!(scene, SCENE.to_vec(), "under the evidence's scene");
    let form = crate::transducer::sentence::from_element_form(&sentence).expect("a sentence tape");
    assert!(form.demand_vertices().next().is_some(), "the heard word's bodies ride it as demand vertices");
}

/// 78.177 §4a(4) — THE CLOSER ASSERTS THE CLOSED TWIST UNDER THE FAMILY:
/// where another mind listed a demand vertex of its question and the
/// closer's cited form reproduces its bodies, the closer asserts the
/// closed twist under the scene's family (`Mind::assert_closed_twist`);
/// the asker is paid nothing.
#[test]
fn closer_asserts_closed_twist_under_the_family() {
    let word = slide(0, 2);
    let mut closer = Mind::open("127.0.0.1:1", Voice::named("closer"));
    closer.scenes.hold(&SCENE, &family(), &word);
    let demands = crate::client::sdk::mind::listed::heard_bodies(&word, &family());
    assert!(!demands.is_empty(), "demands stand on the scene");
    let (_, _, links) = &demands[0];
    let mut field = std::mem::take(&mut closer.field);
    let asserted = field.think(|| {
        closer.assert_closed_twist(&word, links)
    });
    closer.field = field;
    assert!(asserted, "closer asserts the closed twist under the family");
    assert!(closer.speaking.contains(word.address().as_bytes()), "spoken under the evidence's scene");
    let (held_scenes, _) = closer.scenes.held();
    assert!(held_scenes > 0, "the word is held under the scene");
}
