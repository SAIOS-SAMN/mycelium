// 50.9 — the contraction on the mind: a refusal contracts the held form
// and seals the address; a sealed tape is refused by the mind before the
// wire.

use crate::arc::Grid;
use crate::client::sdk::*;
use crate::transducer::family::{cipher_form, sentence_measure, NoCrystal, TrainPair};
use crate::transducer::sentence::{as_element_form, At, Contracted, RefusalKind, Vertex};

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A bar and a lone painted site of another hue carried together (50.3's
/// rigid pair): two word vertices and the edge between them — the voice's pair form,
/// measured over its family and spoken as a sentence.
fn two_bars() -> (Vec<TrainPair>, Element) {
    // H1, 70.4 — TWO BODIES ARE TWO SEPARATED REGIONS, NOT TWO COLOURS.
    // This read `"11200" → "00112"`, where the `11` and the `2` are
    // ADJACENT: under chapter 20's one-hue rule that was two bodies, and
    // under H1 it is ONE body carrying two hues in its gauge. The
    // fixture's intent is two bodies both translating by `+2`, so the
    // intent is now written with a GAP instead of a colour change.
    let family = vec![(digits(&["1100200"]), digits(&["0011002"]))];
    let f = cipher_form(&family[0].0, &family[0].1);
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    (family, e)
}

/// The word vertices of a sentence, each by its coordinates (AT1: a vertex is
/// keyed by its class), with the charts they act on.
fn words_of(e: &Element) -> Vec<(crate::network::nerve::coord::VertexAt, crate::transducer::stalk::Invariant)> {
    let f = crate::transducer::sentence::from_element_form(e)
        .expect("a sentence tape reads back");
    f.vertices
        .entries()
        .filter_map(|(at, v)| match v {
            // EVERY CHART THE VERTEX ACTS ON, one entry each: the reading is
            // "the word vertices with the charts they act on", and a vertex
            // acting on two charts is two entries here rather than one
            // entry naming a representative.
            Vertex::Word { acts, .. } => Some(
                acts.scoped()
                    .map(|(_, a)| (at.clone(), a.chart.clone()))
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
fn receive_adjudication_contracts_form_and_seals_address() {
    let (family, e) = two_bars();
    let words = words_of(&e);
    let (refused_addr, refused_chart) = words[0].clone();
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("contracting"));
    let k_before = entity.voice().id();
    let _ = k_before;
    let mut b = entity.stroke();
    let moved = entity.receive_adjudication(
        &e,
        RefusalKind::VertexBody,
        At::vertex(refused_addr.clone(), refused_chart.clone()),
        [7; 32],
        &family,
        &[3; 32],
        &mut b,
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
                !f.vertices.at(&refused_addr).is_some_and(|v| matches!(v, Vertex::Word { .. }) && v.acting_on(&refused_chart).is_some()),
                "the refused word no longer acts on the refused chart"
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
fn speak_refuses_sealed_tape_before_wire() {
    use crate::anastome::dialogue::SPEECH_REFUSED;
    let (family, e) = two_bars();
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("sealed"));
    let mut b = entity.stroke();
    let held = entity.receive_adjudication(
        &e,
        RefusalKind::MalformedComplex,
        At::Tape(e.address()),
        [9; 32],
        &family,
        &[3; 32],
        &mut b,
    );
    assert_eq!(held, Contracted::Quiescent, "a malformed complex is held inert");
    assert_eq!(entity.contracted_pending(), 0, "and nothing is re-spoken");
    let heard = std::rc::Rc::new(std::cell::RefCell::new(None));
    let into = std::rc::Rc::clone(&heard);
    entity.speak(std::slice::from_ref(&e), move |_, codes| *into.borrow_mut() = Some(codes));
    let codes = heard.borrow_mut().take().expect("heard at once: no frame went on the wire").expect("no frame is built for a sealed tape");
    assert_eq!(codes, vec![SPEECH_REFUSED], "refused by the mind itself");
    assert!(!entity.uttered(e.address().as_bytes()), "never filed as uttered");
    assert_eq!(entity.dialect_len(), 0, "a sealed sentence does not stand");
}

/// 24.2, 50.9 — A REFUSED SENTENCE LEAVES THE DIALECT with its refusal: it
/// stood there before it was uttered and did not stand at the court, so
/// nothing glues it and nothing cites it again.
#[test]
fn receive_adjudication_drops_refused_sentence_from_dialect() {
    let (family, e) = two_bars();
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("refused-leaves"));
    entity.epigenome.stand_dialect_at(e.clone(), 1, 1);
    assert_eq!(entity.dialect_len(), 1, "the sentence stood in the dialect before the utterance");
    let mut b = entity.stroke();
    let _ = entity.receive_adjudication(
        &e,
        RefusalKind::MalformedComplex,
        At::Tape(e.address()),
        [9; 32],
        &family,
        &[3; 32],
        &mut b,
    );
    assert!(entity.sealed_at(e.address().as_bytes()).is_some(), "the refused address is sealed");
    assert_eq!(entity.dialect_len(), 0, "and the refused sentence no longer stands");
}

/// 21 step 3, 50.9 — A CITATION IS OWED ONCE PER COURT, AND A SEALED ONE TO
/// NONE. A court's wait queues the cited form before the citer; the same
/// citer is owed again for each court this mind knows and then waits for
/// its family; a citation this mind sealed is owed to no court; and a
/// citer the court admits is owed afresh next time.
#[test]
fn owe_citation_once_per_court_never_when_sealed() {
    let (family, cited) = two_bars();
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("owing"));
    entity.epigenome.stand_dialect_at(cited.clone(), 1, 1);
    let citer = super::common::two_body_sentence([&[(1, 2)], &[(3, 4)]]);
    let courts = entity.hypha.courts();
    assert!(courts >= 1, "the mind knows a court: {courts}");
    for n in 0..courts {
        assert!(
            entity.owe_citation(&citer, &cited.address(), &family, &[3; 32]).is_ok(),
            "owed to court {n}"
        );
    }
    assert_eq!(entity.owed_pending(), courts, "one owed frame per court");
    assert_eq!(
        entity.owe_citation(&citer, &cited.address(), &family, &[3; 32]),
        Err(crate::transducer::sentence::Refused::OwedToEveryCourt),
        "owed to every court once, the citer waits for its family"
    );
    entity.owed.clear();
    // Admitted at a court, the citer starts afresh.
    entity.file_taught(&citer);
    assert!(
        entity.owe_citation(&citer, &cited.address(), &family, &[3; 32]).is_ok(),
        "owed again after an admission"
    );
    entity.owed.clear();
    // A citation this mind sealed is owed to no court.
    let mut b = entity.stroke();
    let _ = entity.receive_adjudication(
        &cited,
        RefusalKind::MalformedComplex,
        At::Tape(cited.address()),
        [9; 32],
        &family,
        &[3; 32],
        &mut b,
    );
    assert_eq!(
        entity.owe_citation(&citer, &cited.address(), &family, &[3; 32]),
        Err(crate::transducer::sentence::Refused::CitedSealed),
        "a sealed citation is owed to no court"
    );
    assert_eq!(entity.owed_pending(), 0);
}

/// The contraction certificate is indexed by the minted sentence's address,
/// so `certificate_for` reads it directly when the contracted sentence is
/// uttered.
#[test]
fn certificate_for_reads_contraction_by_minted_address() {
    let (family, e) = two_bars();
    let words = words_of(&e);
    let (refused_addr, refused_chart) = words[0].clone();
    let mut entity = Mind::open("127.0.0.1:1", Voice::named("contracting-cert"));
    let mut b = entity.stroke();
    let moved = entity.receive_adjudication(
        &e,
        RefusalKind::VertexBody,
        At::vertex(refused_addr.clone(), refused_chart.clone()),
        [7; 32],
        &family,
        &[3; 32],
        &mut b,
    );
    assert_eq!(moved, Contracted::Moved);
    let pending = entity.take_contracted();
    assert!(!pending.is_empty(), "one contracted sentence pending");
    for p in &pending {
        assert_eq!(
            entity.certificate_for(&p.sentence),
            Some(p.certificate.clone()),
            "the minted sentence's certificate is found by its address"
        );
    }
}
