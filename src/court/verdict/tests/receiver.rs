// 50.2 G, 23.1 steps 3–4 and 7 (L7) — the receiver on the court's copy of
// the family: a word vertex carries its body or the sentence is refused; a
// carried edge re-derives on the union of its bodies or it is refused; a
// remainder a demand names is valid while an undeclared one is not; and a
// tape with no vertex that acts is a chain.

// `LetterId` and `Word` were imported for the tampered-edge fixture that
// T1 removed (60.5): a 90° turn written over a carried edge, which
// needed an edge to write over. They come back with the fixture, when T2
// gives two bodies a reason to relate.
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
    let words = rule.form.vertices.entries().filter(|(_, v)| matches!(v, Vertex::Word { .. })).count();
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
    let demands: Vec<_> = f.demand_vertices().map(|(at, ..)| at).collect();
    for at in &demands {
        f.vertices.drop_at(at);
    }
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
        !f.vertices.entries().any(|(_, v)| matches!(v, Vertex::Word { .. })),
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

/// 50.3, 23.1 step 4 — AND WHAT T1 TOOK FROM IT (60.5, ruled 2026-09-22).
///
/// This read: a bar and a lamina of two hues carried together are a rigid
/// pair whose edge is the identity of the stabilizer; the honest edge
/// re-derives on the union, and an edge naming a 90° turn the union
/// did not make is refused.
///
/// `11200 -> 00112` is a HUE SEAM: the hue-1 body at cells 0,1 and the
/// hue-2 body at cell 2 are side by side and SHARE NO CELL. The face
/// between them existed only because `ring_of_support` dilated each by one
/// column until they collided, and with the stencil gone the form is two
/// vertices on two Frame-star faces carrying no edge at all.
///
/// THE COVERAGE THIS TEST PROVIDED IS GONE AND IS NOT REPLACED HERE. The
/// court's `REFUSE_EDGE` path — an edge that names a turn the union did
/// not make — is machinery that still exists and is still correct, and it
/// now has NO FIXTURE, because every fixture that produced a carried edge
/// produced it from adjacency. The corpus holds two genuine cover meets
/// and neither is in a unit fixture. Building one means a deliberate
/// replication or merge case, which is T2's ground, and the gap is
/// recorded in the owed ledger rather than left for a green suite to hide.
///
/// What stands here is the collapse itself, asserted so that T2's arrival
/// is visible at this site: when body-to-body incidence returns from the
/// tether, this fixture regains an edge and this test is restored.
#[test]
fn a_hue_seam_carries_no_edge_without_the_stencil() {
    // H1, 70.4 — two bodies are two SEPARATED regions, not two touching
    // colours; the gap restores the intent of two bodies at `+2`.
    let family = vec![(digits(&["1100200"]), digits(&["0011002"]))];
    let f = cipher_form(&family[0].0, &family[0].1);
    assert!(
        !f.faces.entries().any(|(_, face)| face.has_edge()),
        "two disjoint bodies share no cell, so no face carries an edge: {:?}",
        f.faces
    );
    assert!(
        f.faces.entries().all(|(o, _)| {
            o.from_vertex() == crate::network::nerve::coord::VertexAt::Frame
                || o.to_vertex() == crate::network::nerve::coord::VertexAt::Frame
        }),
        "what is left is the Frame star and nothing between the bodies: {:?}",
        f.faces
    );
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    // THE TAPE IS STILL A SENTENCE. A form whose bodies are unrelated is
    // not a broken form — the relation is absent and the absence is the
    // reading (rule 11) — so the court replays it clean.
    assert_eq!(
        replay_form_with(&e, &|_| None, Some(&family), &NoCrystal),
        None,
        "a form with no edge to re-derive replays clean"
    );

    // THE SECTION HALF OF THIS TEST IS GONE WITH THE EDGE (T1). What stood
    // below re-derived a TAMPERED edge against the frames a face claims —
    // `edges_rederive_frames` with `face.pairs` set to every frame, to a
    // frame the face does not stand on, and to the one it does — and every
    // one of those assertions needed an edge to tamper. The fixture's two
    // bodies are disjoint, so there is none.
    //
    // WHAT IS NOW UNTESTED, named so a green suite cannot hide it:
    // `edges_rederive_frames`' section restriction, and `REFUSE_EDGE` in
    // both its whole-replay and its per-move form. The organs are intact
    // and unchanged; they have no input. Restoring them needs a fixture
    // whose bodies genuinely share support — a replication or a merge,
    // which is T2's ground — and the owed ledger carries the row.
    //
    // The reasoning the section half recorded is kept here because it is
    // the reason the carrier exists at all:
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
    // defect and the form is refused for it by the orbit-holonomy check too,
    // so asserting a whole replay here would pass with the section ignored.
    // `edges_rederive_frames` is the organ that changed, and it is what was
    // asked (§3.2).
    //
    // It is still asked HERE, on the honest form, which is the one half of
    // the reading that survives: a form with no edge is re-derived and
    // refuses nothing. That is not the tampering assertion and it is not
    // offered as one.
    let rederives = |f: &crate::transducer::sentence::Form| {
        crate::verdict::body::edges_rederive_frames(f, &family).map(|r| r.reason)
    };
    assert_eq!(
        rederives(&f),
        None,
        "a form carrying no edge re-derives clean rather than refusing"
    );
}

/// 51.8, LAW 12 — THE COURT PERCEIVES EACH FRAME ONCE. A turned L, a
/// shifted bar and a shifted dot are two word vertices — the 90° turn,
/// and the translation acting on two charts — with two faces between them
/// where the bar and the dot land beside the L. The replay reads each frame
/// from one perception of it: the vertex replay reads the sensor once per
/// grid of the family, and so does the edge replay, whatever the number of
/// vertices, faces and member pairs asked about that frame.
#[test]
fn the_court_perceives_each_frame_once_for_every_vertex_and_face() {
    use crate::transducer::family::{reset_sensor_reads, sensor_reads};
    // H1, 70.4 — the three objects are SEPARATED rather than abutting in
    // different hues. This read `"110000"/"033000"`, where the L's foot at
    // (2,1) is 4-adjacent to the bar at (3,1): under chapter 20 that was
    // two bodies, and under H1 the L, the bar and the 5 chain into ONE.
    // The motions are unchanged — the L 90°-turns, the bar and the 5
    // shift — only the gaps are.
    let x = ["100000", "100000", "110000", "000000", "033000", "000005"];
    let y = ["111000", "100000", "000000", "033000", "000000", "500000"];
    let family = vec![(digits(&x), digits(&y))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the turned L and the two shifts extract");
    let words: Vec<crate::network::nerve::coord::VertexAt> = rule
        .form
        .vertices
        .entries()
        .filter_map(|(at, v)| matches!(v, Vertex::Word { contact: None, .. }).then_some(at))
        .collect();
    assert_eq!(words.len(), 2, "the 90° turn and the translation: {:?}", rule.form.vertices);
    let between_words = rule
        .form
        .faces
        .entries()
        .filter(|(o, _)| words.contains(&o.from_vertex()) && words.contains(&o.to_vertex()))
        .count();
    // T1 — "BESIDE" IS NOT "OVERLAPPING" (60.5, ruled 2026-09-22). This
    // asserted two faces where the bar and the dot land BESIDE the L, and
    // beside is what `ring_of_support` manufactured: the three bodies are
    // disjoint and share no cell, so with the stencil gone nothing relates
    // two word vertices and the faces that remain are the Frame star.
    //
    // THE CLAIM OF THIS TEST IS NOT THE FACE COUNT. It is law 12 — the
    // court perceives each frame ONCE, whatever the number of vertices,
    // faces and member pairs asked about that frame — and that assertion
    // is below and still runs. The count is a precondition, and it is kept
    // rather than deleted so the tether's arrival (T2) is visible here: a
    // fixture whose bodies genuinely relate will raise it off zero.
    assert_eq!(
        between_words, 0,
        "three disjoint bodies relate through no face without the stencil: {:?}",
        rule.form.faces
    );
    let cells: u64 = family
        .iter()
        .map(|(x, y)| (x.len() * x[0].len() + y.len() * y[0].len()) as u64)
        .sum();
    reset_sensor_reads();
    let _ = crate::verdict::body::vertices_carry_frames(&rule.form, &family);
    assert_eq!(sensor_reads(), cells, "the vertex replay reads each grid of the family once");
    reset_sensor_reads();
    let _ = crate::verdict::body::edges_rederive_frames(&rule.form, &family);
    assert_eq!(sensor_reads(), cells, "and so does the edge replay");
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

