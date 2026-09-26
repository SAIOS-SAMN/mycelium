// 50.9 — THE CONTRACTION MOVES ON THE FORM: a still orbit and a face that
// does not re-derive each shrink the closed sub-complex, and the court's
// reader recognises each.

use super::common::*;
use crate::transducer::alphabet::Contact;
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// 50.9 move 5 — A STILL ORBIT CONTRACTS ITS CONTACT WORD TO A DEMAND. The
/// refused contact vertex becomes a demand over its chart carrying the
/// links it was to reach, its star face leaves with it, and the court's
/// reader finds the contraction at that coordinate; a coordinate the form
/// does not hold contracts nothing.
#[test]
fn a_still_orbit_contracts_its_contact_word_to_a_demand() {
    let mut f = closed_word_form(word_turn_90(), Some(Contact { anchor: 0, ring: 1 }));
    let word = only_word(&f);
    let Some(Vertex::Word { contact: Some(_), .. }) = f.vertices.at(&word) else {
        panic!("the fixture's word is a contact word");
    };
    let chart = stalk::Invariant::counts(&[3, 5]);
    let at = At::vertex(word.clone(), chart.clone());
    let remainder = vec![(0i64, 1i64, 1u8, 2u8)];
    assert!(
        f.overlaps_at(&word).iter().any(|o| o.from == crate::network::nerve::coord::ChartAt::Frame),
        "the contact word has its star face"
    );
    assert_eq!(f.contract(RefusalKind::OrbitStill, &at, &remainder), Some(Contracted::Moved));
    // AT1: the demand stands at its OWN coordinate (kind, chart, measure,
    // pieces), and the word's coordinate is empty.
    assert!(!f.vertices.holds(&word), "the contact word's coordinate is empty");
    let demands: Vec<Vec<(i64, i64, u8, u8)>> = f
        .demand_vertices()
        .filter(|(_, k, c, _)| matches!(k, Demand::Creation) && **c == chart)
        .map(|(_, _, _, b)| b.to_vec())
        .collect();
    assert_eq!(
        demands,
        vec![remainder.clone()],
        "the contact word is now a demand over its chart with the links it was to reach: {:?}",
        f.vertices
    );
    assert!(f.overlaps_at(&word).is_empty(), "and its faces left with it");
    assert!(f.is_contraction_at(RefusalKind::OrbitStill, &at), "the court finds the contraction at the coordinate");
    let elsewhere_word = word_vertex(9, word_turn_180()).at().expect("a word vertex has a coordinate");
    assert!(!f.vertices.holds(&elsewhere_word), "the elsewhere coordinate is one the form does not hold");
    let elsewhere = At::vertex(elsewhere_word, chart);
    let mut g = closed_word_form(word_turn_90(), Some(Contact { anchor: 0, ring: 1 }));
    assert_eq!(g.contract(RefusalKind::OrbitStill, &elsewhere, &[]), None, "a vertex the form does not hold contracts nothing");
}

/// 50.9 move 6 — A FACE THAT DOES NOT RE-DERIVE IS DROPPED. The chord between
/// two words leaves the form, the orbit through it dissolves, both words
/// still act, the court's reader finds no face between the two, and the
/// same coordinate contracts nothing a second time.
#[test]
fn a_face_that_does_not_rederive_is_dropped_by_its_contraction() {
    let mut f = chord_form(&[word_turn_90()]);
    assert_eq!(f.orbits.len(), 1, "the chord closes one orbit");
    // THE FACE IS NAMED BY ITS ENDS' OWN COORDINATES (54.2, AT1). The
    // chord's two ends are two members of ONE vertex — the identity's
    // class, on charts [1] and [2] (Q1: one vertex per class) — so the
    // face is named by that vertex at both ends.
    let end = only_word(&f);
    let at = At::face(end.clone(), end.clone());
    let between = |f: &Form| {
        f.faces
            .entries()
            .filter(|(o, _)| o.from_vertex() == end && o.to_vertex() == end)
            .count()
    };
    assert_eq!(between(&f), 1, "the chord is the one overlap between the two members");
    assert_eq!(f.contract(RefusalKind::FaceDerivation, &at, &[]), Some(Contracted::Moved));
    assert_eq!(between(&f), 0, "no face stands between the two words");
    assert!(f.orbits.is_empty(), "the orbit through the chord dissolved");
    assert!(f.acts(), "both words still act");
    assert!(f.is_contraction_at(RefusalKind::FaceDerivation, &at), "the court finds no face between them");
    assert_eq!(f.contract(RefusalKind::FaceDerivation, &at, &[]), None, "a face already gone contracts nothing");
}

/// 54.6 — A DEMAND DECLARES SILENCE, AND A FALSE PROJECTION CONTRACTS TO
/// NOTHING.
///
/// `undeclared_remainder` keeps a remainder link where `spoken != 0`
/// WHATEVER a demand says of it — a demand says "the language has no word
/// for this cell", and a cell the form spoke wrongly is not that. So a
/// contraction over such a link cannot cure the refusal.
///
/// It also cannot be harmless, and that is why the guard is here rather
/// than only at the court. A contraction MOVES THE ADDRESS —
/// `form_vertex_key` of the contracted form — so the seal the voice holds
/// on the refused address stops matching, the contracted form is uttered,
/// the same links refuse again, and the cycle mints one word per turn with
/// nothing in it to stop.
///
/// The three readings: a silent remainder mounts its demand; a wrong one
/// contracts to `None`, which `receive_adjudication` reads as `Quiescent`;
/// and a mixed coordinate mounts a demand over the silent links ALONE,
/// never over the ones the form got wrong.
#[test]
fn a_demand_declares_silence_and_a_wrong_remainder_contracts_to_nothing() {
    use crate::transducer::sentence::Demand;
    let chart = crate::transducer::stalk::Invariant::counts(&[3, 5]);
    let silent = |r: i64| (r, 0i64, 0u8, 4u8);
    let wrong = |r: i64| (r, 1i64, 2u8, 4u8);

    // SILENT ALONE: the demand mounts, over exactly those links.
    let mut f = chord_form(&[word_turn_90()]);
    let at = At::Chart(chart.clone(), vec![silent(0), silent(1)]);
    assert_eq!(
        f.contract(RefusalKind::RemainderUndeclared, &at, &[]),
        Some(Contracted::Moved),
        "a remainder the form was silent on is a demand it can mount"
    );
    let stood: Vec<Vec<(i64, i64, u8, u8)>> = f
        .demand_vertices()
        .filter(|(_, k, _, _)| matches!(k, Demand::Source { .. }))
        .map(|(_, _, _, b)| b.to_vec())
        .collect();
    assert_eq!(stood, vec![vec![silent(0), silent(1)]], "the demand carries the silent links");

    // WRONG ALONE: nothing mounts, and the form does not move.
    let mut g = chord_form(&[word_turn_90()]);
    let before = crate::transducer::sentence::form_vertex_key(&g);
    let at = At::Chart(chart.clone(), vec![wrong(0), wrong(1)]);
    assert_eq!(
        g.contract(RefusalKind::RemainderUndeclared, &at, &[]),
        None,
        "no move makes a false assertion true"
    );
    assert_eq!(
        crate::transducer::sentence::form_vertex_key(&g),
        before,
        "and the address does not move, so the voice's seal keeps holding"
    );
    assert_eq!(
        g.contract(RefusalKind::RemainderWrong, &at, &[]),
        None,
        "and the kind the court sends for it contracts to nothing by name"
    );

    // MIXED: the silent links mount; the wrong ones are not in the demand.
    let mut h = chord_form(&[word_turn_90()]);
    let at = At::Chart(chart, vec![silent(0), wrong(1)]);
    assert_eq!(
        h.contract(RefusalKind::RemainderUndeclared, &at, &[]),
        Some(Contracted::Moved),
        "a coordinate with something declarable in it contracts"
    );
    let stood: Vec<Vec<(i64, i64, u8, u8)>> = h
        .demand_vertices()
        .filter(|(_, k, _, _)| matches!(k, Demand::Source { .. }))
        .map(|(_, _, _, b)| b.to_vec())
        .collect();
    assert_eq!(
        stood,
        vec![vec![silent(0)]],
        "over the silent link alone — a demand is never an apology for a wrong one"
    );
}
