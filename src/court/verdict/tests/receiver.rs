// 50.2 G, 23.1 steps 3–4 and 7 (L7) — the receiver on the court's copy of
// the family: a word vertex carries its body or the sentence is refused; a
// carried edge re-derives on the union of its bodies or it is refused; a
// remainder a demand names is valid while an undeclared one is not; and a
// tape with no vertex that acts is a chain.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::Element;
use crate::transducer::family::{extract_rule, mint_sentence, pair_form, sentence_measure, Grid, NoCrystal};
use crate::transducer::sentence::{as_element_form, from_element_form, Vertex};
use crate::verdict::*;

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A bar recoloured and a cell created beside it: one word vertex, one
/// `Creation` demand, spoken as one sentence (50.5).
fn a_recolour_with_a_creation() -> (Vec<(Grid, Grid)>, Element) {
    let family = vec![(digits(&["110", "000"]), digits(&["220", "003"]))];
    let rule = extract_rule(&family).expect("the bar's recolour extracts");
    let words = rule.form.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    let demands = rule.form.demand_vertices().count();
    assert_eq!((words, demands), (1, 1), "one word and one demand ride the sentence");
    let e = mint_sentence(&rule, &family, &NoCrystal).expect("a sentence with a demand is spoken whole");
    (family, e)
}

/// 50.5: valid, not closed. The created cell is the remainder the demand
/// vertex names, and the court replays the sentence on its pair without
/// refusing it for that remainder.
#[test]
fn a_remainder_a_demand_names_is_valid_and_replays_on_the_family() {
    let (family, e) = a_recolour_with_a_creation();
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        None,
        "the closed sub-complex closes; the demand carries the rest"
    );
}

/// The same sentence with its demand vertex struck out claims to close a
/// pair it leaves a cell on: `REFUSE_NOT_CLOSED`.
#[test]
fn an_undeclared_remainder_refuses_the_sentence_as_not_closed() {
    let (family, e) = a_recolour_with_a_creation();
    let mut f = from_element_form(&e).expect("the tape reads back");
    f.vertices.retain(|v| !matches!(v, Vertex::Demand { .. }));
    f.reindex();
    let tampered = as_element_form(&f, e.magnitude()).expect("a tape");
    assert_eq!(
        replay_form_with(&tampered, &|_| None, Some(&family), &NoCrystal),
        Some(REFUSE_NOT_CLOSED)
    );
}

/// 50.1 step 1 on the court's copy: the word that recoloured the bar to 2
/// does not carry it onto a pair where the bar became 3 — the vertex does
/// not reconstruct its body there, and the sentence is refused before any
/// remainder is read.
#[test]
fn a_vertex_that_does_not_carry_its_body_on_the_family_is_refused() {
    let (_, e) = a_recolour_with_a_creation();
    let other = vec![(digits(&["110", "000"]), digits(&["330", "003"]))];
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&other), &NoCrystal),
        Some(REFUSE_VERTEX_BODY)
    );
}

/// 50.6 (L7): an L the eyes cannot pair with the bar it became is a form
/// of the Frame and one deformation demand — nothing in it acts — and the
/// court refuses the tape as a chain whatever it measures.
#[test]
fn a_tape_with_no_acting_vertex_is_a_chain() {
    let family = vec![(digits(&["110", "100"]), digits(&["111", "000"]))];
    let f = pair_form(&family[0].0, &family[0].1, None);
    assert!(
        !f.vertices.iter().any(|v| matches!(v, Vertex::Word { .. })),
        "no body pair the eyes admit"
    );
    assert!(f.demand_vertices().next().is_some(), "the deformation rides as a demand");
    let mag = sentence_measure(&f, &family, &NoCrystal).expect("the demands measure what they ask");
    let e = as_element_form(&f, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        Some(REFUSE_NOT_A_SENTENCE)
    );
}

/// 50.3, 23.1 step 4: a bar and a cell of two hues carried together are a
/// rigid pair whose edge is the identity of the stabilizer; the honest
/// edge re-derives on the union of the bodies, and an edge that names a
/// quarter turn the union did not make is refused. Two shapes, so each
/// chart holds one body and the edge is read on the pair, not on sections.
#[test]
fn a_carried_edge_rederives_on_the_union_or_is_refused() {
    let family = vec![(digits(&["11200"]), digits(&["00112"]))];
    let f = pair_form(&family[0].0, &family[0].1, None);
    assert!(f.faces.iter().any(|face| face.edge.is_some()), "the rigid pair carries an edge");
    let mag = sentence_measure(&f, &family, &NoCrystal).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        None,
        "the identity edge re-derives on the union"
    );
    let quarter = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("the sensor plane"), false)]);
    let mut g = f.clone();
    for face in &mut g.faces {
        if let Some(edge) = &mut face.edge {
            edge.word = quarter.clone();
        }
    }
    let tampered = as_element_form(&g, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&tampered, &|_| None, Some(&family), &NoCrystal),
        Some(REFUSE_EDGE)
    );
}
