// 50.2 G, 23.1 steps 3–4 and 7 (L7) — the receiver on the court's copy of
// the family: a word vertex carries its body or the sentence is refused; a
// carried edge re-derives on the union of its bodies or it is refused; a
// remainder a demand names is valid while an undeclared one is not; and a
// tape with no vertex that acts is a chain.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::Element;
use crate::transducer::family::{extract_rule, mint_sentence, cipher_form, sentence_measure, Grid, NoCrystal};
use crate::transducer::sentence::{as_element_form, Vertex};
use crate::verdict::*;

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A bar recoloured and a lamina created beside it: one word vertex, one
/// `Creation` demand, spoken as one sentence (50.5).
fn a_recolour_with_a_creation() -> (Vec<(Grid, Grid)>, Element) {
    let family = vec![(digits(&["110", "000"]), digits(&["220", "003"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the bar's recolour extracts");
    let words = rule.form.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    let demands = rule.form.demand_vertices().count();
    assert_eq!((words, demands), (1, 1), "one word and one demand ride the sentence");
    let e = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence with a demand is spoken whole");
    (family, e)
}

/// 50.5: valid, not closed. The created lamina is the remainder the demand
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
/// pair it leaves a lamina on: `REFUSE_NOT_CLOSED`.
#[test]
fn an_undeclared_remainder_refuses_the_sentence_as_not_closed() {
    let (family, e) = a_recolour_with_a_creation();
    let mut f =
        crate::transducer::sentence::from_element_form(&e).expect("the tape reads back");
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
    let f = cipher_form(&family[0].0, &family[0].1);
    assert!(
        !f.vertices.iter().any(|v| matches!(v, Vertex::Word { .. })),
        "no body pair the eyes admit"
    );
    assert!(f.demand_vertices().next().is_some(), "the deformation rides as a demand");
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("the demands measure what they ask");
    let e = as_element_form(&f, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        Some(REFUSE_NOT_A_SENTENCE)
    );
}

/// 50.3, 23.1 step 4: a bar and a lamina of two hues carried together are
/// a rigid pair whose edge is the identity of the stabilizer; the honest
/// edge re-derives on the union of the bodies, and an edge that names a
/// quarter turn the union did not make is refused. Two shapes, so each
/// chart holds one body and the edge is read on the pair, not on sections.
#[test]
fn a_carried_edge_rederives_on_the_union_or_is_refused() {
    let family = vec![(digits(&["11200"]), digits(&["00112"]))];
    let f = cipher_form(&family[0].0, &family[0].1);
    assert!(f.faces.iter().any(|face| face.has_edge()), "the rigid pair carries an edge");
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        None,
        "the identity edge re-derives on the union"
    );
    let quarter = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("the sensor plane"), false)]);
    let mut g = f.clone();
    // EVERY MEMBER PAIR'S EDGE is tampered, so the court's refusal is not a
    // function of which pair it happened to read.
    for face in &mut g.faces {
        let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
        for k in pairs {
            if let Some(edge) = face.at.get_mut(&k).and_then(|p| p.edge.as_mut()) {
                edge.word = quarter.clone();
            }
        }
    }
    let tampered = as_element_form(&g, mag).expect("a tape");
    assert_eq!(
        replay_form_with(&tampered, &|_| None, Some(&family), &NoCrystal),
        Some(REFUSE_EDGE)
    );

    // A FACE IS ASKED FOR THE PAIRS ITS WORD WAS MEASURED ON, and the
    // section travels on the face across the tape.
    //
    // 21 step 2 keeps every word the bundle spoke between two ends as a
    // face of its own, each holding on the pairs it was spoken on. The
    // extractor measured those lists and filed them in `Form.sections`
    // under a key built from the two ends and the word — a second key
    // family in a carrier whose only reader asks at `TransportClass::key()`
    // — and nothing read them. Nothing COULD: `Form::regauge` rewrites
    // every face's word at extraction step 7 and `rebase_named_faces`
    // rewrites the star faces' and turns a `Vertex::Word` into a
    // `Vertex::Rebased`, after which neither half of that key can be
    // rebuilt. So each section face was re-derived against every pair of
    // the family, including the ones it never claimed.
    //
    // The tampered edge above is the case in one line, and the READING IS
    // TAKEN AT THE MOVE, not at the verdict: a tampered edge is a real
    // defect and the form is refused for it by the loop-holonomy check too,
    // so asserting a whole replay here would pass with the section ignored.
    // `edges_rederive_frames` is the organ that changed, and it is what is
    // asked (§3.2).
    let section = |f: &crate::transducer::sentence::Form, ps: Option<Vec<u16>>| {
        let mut s = f.clone();
        let ps = ps.map(crate::transducer::sentence::section_of_frames);
        for face in &mut s.faces {
            face.pairs = ps.clone();
        }
        // Through the tape, because the court replays what arrives on the
        // wire and a section that does not travel is not a section.
        let e = as_element_form(&s, mag).expect("a tape");
        crate::transducer::sentence::from_element_form(&e).expect("the tape reads back")
    };
    let rederives = |f: &crate::transducer::sentence::Form| {
        crate::verdict::body::edges_rederive_frames(f, &family).map(|r| r.reason)
    };
    assert_eq!(
        rederives(&section(&g, None)),
        Some(REFUSE_EDGE),
        "a face claiming every frame is re-derived at every frame"
    );
    assert_eq!(
        rederives(&section(&g, Some(vec![1]))),
        None,
        "a face measured at frame 1 is asked nothing in a one-frame family"
    );
    assert_eq!(
        rederives(&section(&g, Some(vec![0]))),
        Some(REFUSE_EDGE),
        "and naming the frame it does stand on brings the re-derivation back"
    );
    // The honest form is unaffected either way: a section restricts which
    // pairs are asked, never what the answer is.
    assert_eq!(
        rederives(&section(&f, Some(vec![0]))),
        None,
        "the identity edge re-derives on the pair it is measured on"
    );
}

/// MEMBER REPLAY IS SCOPED TO ACTIVE CHART SECTIONS.
///
/// A rule vertex carrying member charts from different pairs (Chart A from
/// pair 0, Chart B from pair 1) is replayed per member on the pairs that
/// member's section names: Member B is active only on pair 1, so pair 0
/// replays Member A alone, and a body of Chart B present in pair 0's input
/// is not a body this vertex claims. A section that defaulted an unmeasured
/// chart to every pair would replay Member B on pair 0 against that body
/// and refuse the sentence at a claim it never made.
#[test]
fn member_replay_is_scoped_to_active_chart_sections() {
    let family = vec![
        (digits(&["1100", "1100", "3000"]), digits(&["2200", "2200", "3000"])),
        (digits(&["3000", "0000", "0000"]), digits(&["0000", "0000", "0040"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("multi-member rule extracts");
    let e = mint_sentence(&rule, &family, &NoCrystal, 0).expect("sentence mints");
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        None,
        "court replays multi-member rule when input scene has bodies of the other chart"
    );
}

