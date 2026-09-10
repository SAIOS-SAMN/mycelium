// 21 step 3, 24.1 — THE CRYSTAL AT A BODY'S CHART: the closed forms a
// perceived body's chart reaches, which is what the glue asks for.

use crate::arc::Grid;
use crate::client::sdk::*;
use crate::transducer::family::{pair_form, sentence_measure, Crystal, NoCrystal};
use crate::transducer::sentence::{as_element_form, from_element_form, Vertex};

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A closed form and one of the body charts its word vertices act on.
fn closed_form_and_body_chart() -> (Element, crate::transducer::stalk::Invariant) {
    let family = vec![(digits(&["11200"]), digits(&["00112"]))];
    let f = pair_form(&family[0].0, &family[0].1, None);
    let mag = sentence_measure(&f, &family, &NoCrystal).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    let chart = from_element_form(&e)
        .expect("the tape reads back")
        .vertices
        .iter()
        .find_map(|v| match v {
            Vertex::Word { class, .. } => Some(class.clone()),
            _ => None,
        })
        .expect("a word vertex carries the chart of the body it acts on");
    (e, chart)
}

/// 21 step 3: "Crystal is indexed by the section coordinate (moduli)". The
/// glue asks `on_class(body moduli)` and a stood sentence was filed under
/// `chart_of_form` — `[2, vertices, faces, loops, …]` against a body's
/// `[components, …harmonics…]`, two coordinate systems that meet only at
/// the empty root. The crystal answered the empty vector to every question
/// the glue could ask, so no rule ever cited a held form and no citation
/// was ever paid (26, 27): the fleet's "comprehending 0 of 2 listings".
#[test]
fn a_stood_form_is_found_at_the_chart_of_the_body_it_carries() {
    let (e, chart) = closed_form_and_body_chart();
    let f = from_element_form(&e).expect("the tape reads back");
    assert!(
        f.is_closed(),
        "the fixture's form is closed, so the crystal may return it at all"
    );
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e.clone(), 1, 1)
        .expect("the mind stands what it spoke");
    let found = Crystal::on_body(&epi, &chart);
    assert!(
        found.iter().any(|(held, _, _)| held.address() == e.address()),
        "a mind holding a closed form finds it at the chart of the body that form carries"
    );
}

/// 21 step 3 END TO END on the live crystal: a mind that stood a closed
/// form cites it when it extracts a rule over a family that form closes
/// part of. The same claim `glued_form_replays_descent_through_the_court_genome`
/// makes of a stub crystal whose `on_class` ignores the chart it is asked
/// at — made here of the epigenome the fleet runs.
#[test]
fn a_mind_that_stood_a_form_cites_it_when_it_extracts_a_rule() {
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
    ];
    let held = pair_form(&family[0].0, &family[0].1, None);
    let mag = sentence_measure(&held, &family[..1], &NoCrystal).expect("the body is spoken");
    let e = as_element_form(&held, mag).expect("a tape");
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e.clone(), 1, 1).expect("stood");

    let rule = crate::transducer::family::extract_rule_with(&family, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    assert!(
        rule.form
            .vertices
            .iter()
            .any(|v| matches!(v, Vertex::Glued { address, .. } if *address == e.address())),
        "the rule cites the form the mind stood, by address"
    );
}

/// The read is at the coordinate the listing supplies and is a keyed read
/// of that coordinate alone (24.3, law 3): a chart the mind holds no body
/// on answers nothing, so the crystal never becomes a scan of everything
/// held.
#[test]
fn a_chart_the_mind_holds_no_body_on_answers_nothing() {
    let (e, chart) = closed_form_and_body_chart();
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e, 1, 1).expect("stood");
    let mut other = chart.0.clone();
    other.push(u64::MAX);
    let elsewhere = crate::transducer::stalk::Invariant(other);
    assert!(
        Crystal::on_body(&epi, &elsewhere).is_empty(),
        "the crystal answers at the coordinate asked and nowhere else"
    );
}
