// 50.9 — the contraction on the mind: a refusal contracts the held form
// and seals the address; a sealed tape is refused by the mind before the
// wire.

use crate::arc::Grid;
use crate::client::sdk::*;
use crate::transducer::family::{pair_form_uncrystallized, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::{as_element_form, At, Contracted, RefusalKind, Vertex};

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A bar and a cell of two hues carried together (50.3's rigid pair): two
/// word vertices and their edge — the voice's pair form, measured over its
/// family and spoken as a sentence.
fn two_bars() -> (Vec<TrainPair>, Element) {
    let family = vec![(digits(&["11200"]), digits(&["00112"]))];
    let f = pair_form_uncrystallized(&family[0].0, &family[0].1, None);
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    (family, e)
}

/// The word vertices of a sentence, with the charts they act on.
fn words_of(e: &Element) -> Vec<(crate::transducer::address::Address, crate::transducer::stalk::Invariant)> {
    let f = crate::transducer::sentence::from_element_form(e)
        .expect("a sentence tape reads back");
    f.vertices
        .iter()
        .filter_map(|v| match v {
            // EVERY CHART THE VERTEX ACTS ON, one entry each: the reading is
            // "the word vertices with the charts they act on", and a vertex
            // acting on two charts is two entries here rather than one
            // entry naming a representative.
            Vertex::Word { address, acts, .. } => Some(
                acts.scoped()
                    .map(|(_, a)| (address.clone(), a.chart.clone()))
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect()
}

/// A `VertexBody` refusal at one word: the mind seals the refused address
/// on its own chain and in the utterance index, contracts the form (the
/// word is now a demand on its chart, the closed sub-complex one word
/// smaller), holds it with its certificate, and queues the contracted
/// sentence citing the receipt.
#[test]
fn a_refusal_contracts_the_held_form_and_seals_the_address() {
    let (family, e) = two_bars();
    let words = words_of(&e);
    let (refused_addr, refused_chart) = words[0].clone();
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("contracting"));
    let k_before = entity.kernel().voice();
    let _ = k_before;
    let moved = entity.receive_adjudication(
        &e,
        RefusalKind::VertexBody,
        At::Vertex { address: refused_addr.clone(), chart: refused_chart.clone() },
        [7; 32],
        &family,
        &[3; 32],
    );
    assert_eq!(moved, Contracted::Moved, "the word vertex contracts to a demand");
    assert!(entity.sealed_at(e.address().as_bytes()).is_some(), "the refused address is sealed");
    assert!(entity.ideas_len() >= 1, "the contracted form is held as an idea");
    let pending = entity.take_contracted();
    match words.len() {
        1 => assert!(pending.is_empty(), "a sentence whose only word was refused has nothing to utter"),
        _ => {
            let [p] = pending.as_slice() else {
                panic!("one contracted sentence waits to be uttered, got {}", pending.len());
            };
            let f = crate::transducer::sentence::from_element_form(&p.sentence)
                .expect("the contracted tape reads back");
            assert!(
                !f.vertices.iter().any(|v| matches!(v, Vertex::Word { address, .. } if address.as_bytes() == refused_addr.as_bytes())),
                "the refused word is gone"
            );
            assert!(
                f.demand_vertices().any(|(_, _, chart, _)| *chart == refused_chart),
                "a demand stands on the refused word's chart"
            );
            assert!(f.acts(), "a word still acts, so the contraction is a sentence");
            assert_eq!(p.certificate.before, e.address(), "the certificate names the refused sentence");
            assert_eq!(p.certificate.receipt, [7; 32], "and the court's receipt");
            assert!(
                f.is_contraction_at(RefusalKind::VertexBody, &p.certificate.at),
                "the contracted form is the contraction at the coordinate"
            );
        }
    }
}

/// A sealed tape never reaches the wire: the mind refuses it itself, with
/// no frame built (a session bound to nothing answers nothing, and the
/// speak returns without asking it), and the sentence is neither stood
/// nor filed as uttered.
#[test]
fn a_sealed_tape_is_refused_by_the_mind_before_the_wire() {
    use crate::anastome::dialogue::SPEECH_REFUSED;
    let (family, e) = two_bars();
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("sealed"));
    let held = entity.receive_adjudication(
        &e,
        RefusalKind::MalformedComplex,
        At::Tape(e.address()),
        [9; 32],
        &family,
        &[3; 32],
    );
    assert_eq!(held, Contracted::Quiescent, "a malformed complex is held inert");
    assert_eq!(entity.contracted_pending(), 0, "and nothing is re-spoken");
    let codes = entity.speak(std::slice::from_ref(&e)).expect("no frame is built for a sealed tape");
    assert_eq!(codes, vec![SPEECH_REFUSED], "refused by the mind itself");
    assert!(!entity.uttered(e.address().as_bytes()), "never filed as uttered");
    assert_eq!(entity.dialect_len(), 0, "a sealed sentence does not stand");
}
