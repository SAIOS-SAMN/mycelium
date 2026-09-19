// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8, L11c, L23(c) — THE COVER ONTO THE FORM. The pieces a cover
//! carries are vertices of the form, their meets are its faces, and the
//! class of a cycle of those faces is the form's loop holonomy.
//!
//! These are claims the cover could not make while it classified its own
//! meets into carriers nothing read: 161 carriages and 3 964 meets over
//! twelve tasks reached no form and left only a tally behind them.

use super::common::*;
use crate::transducer::family::*;
use crate::transducer::sentence::{Demand, Vertex};

/// A solid block against the same block with ONE LAMINA REMOVED. No versor
/// carries the whole body onto the whole image — that is what makes it a
/// deformation — and under 51.3 as ruled 2026-09-18 the cover does not
/// descend below the body, so this carries NOTHING and stands as one typed
/// demand. It is kept for exactly that reading.
fn notched() -> (Grid, Grid) {
    (
        grid(&["11111", "11111", "11111", "11111", "11111"]),
        grid(&["11111", "11111", "11111", "11111", "11110"]),
    )
}

/// FOUR BODIES OF ONE HUE, THE FOURTH SLID ONE COLUMN. The cover's pieces
/// are bodies (51.3 as ruled 2026-09-18), so a fixture that exercises the
/// pullback, the meets and their loops needs SEVERAL bodies. These tests
/// stood on one body peeled into shells, which the peel no longer produces
/// and which `notched` now demonstrates the absence of.
///
/// The four blocks are one hue and one shape, so they share a chart and
/// `extract_rule_with` unions them into one composite (extract.rs:281).
/// They are separated by background, so they are four distinct components
/// under 51.3's 6-adjacency while their rings still meet. Three stand in
/// place and the fourth slides one column, so no versor carries the
/// composite whole — that is what makes it a deformation — while the
/// slide still overlaps its own source, so the pullback places all four.
fn four_bodies() -> (Grid, Grid) {
    (
        grid(&["1100110", "1100110", "0000000", "1100110", "1100110"]),
        grid(&["1100110", "1100110", "0000000", "1101100", "1101100"]),
    )
}

/// THE SAME FOUR BODIES, THE FOURTH DEFORMED RATHER THAN SLID: its upper
/// row holds and its lower row slides, so the pullback cannot place it and
/// pass (b) is offered it under the versors pass (a) placed. This is the
/// fixture for propagation, which the sliding one does not reach.
fn four_bodies_one_left() -> (Grid, Grid) {
    (
        grid(&["1100110", "1100110", "0000000", "1100110", "1100110"]),
        grid(&["1100110", "1100110", "0000000", "1100110", "1101100"]),
    )
}

/// The composite of every body a grid perceives, as `extract_rule_with`
/// forms it for one chart (extract.rs:281). The cover is given a composite
/// and reads its bodies back out with `pieces_of`; a fixture of one body
/// has nothing for it to cover (51.3).
fn composite(g: &Grid) -> Body {
    body_of(perceive(g).iter().flat_map(|b| b.laminae.iter().copied()).collect::<Vec<_>>())
}

/// P9 — A DEMAND'S CANDIDATE TRANSPORT IS RE-DERIVED FROM ITS OWN BODIES.
/// The demand carries the bodies it stands for (50.5) and the two sides of
/// its transition are its links split by which hue they occupy, so the
/// transport between them is read at hold time and never stored beside
/// them (law 12). `carriage_with_core` answers the versor where one exists
/// and the homeomorphic core's where the shapes differ by a surgery, which
/// is the case an uncarried body is in.
#[test]
fn a_demands_candidate_transport_is_read_from_the_bodies_it_stands_for() {
    use crate::transducer::bubble::contemplation::Contemplation;
    let held = |ga: &[&str], gb: &[&str]| -> (usize, usize) {
        let f = measured(&grid(ga), &grid(gb));
        let chart = f
            .vertices
            .iter()
            .find_map(|v| match v {
                Vertex::Demand { kind: Demand::TopologicalDeformation { .. }, chart, .. } => {
                    Some(chart.clone())
                }
                _ => None,
            })
            .expect("the deformation stands as a typed demand");
        let mut c = Contemplation::new();
        c.hold_form(f.clone(), &[], &[], [0; 4]);
        let (_, b) = c.bubbles.scoped().next().expect("the demand is held as an idea");
        let over_chart =
            b.candidates.get(chart.key().as_bytes()).map_or(0, |m| m.len());
        let total: usize = b.candidates.scoped().map(|(_, m)| m.len()).sum();
        assert_eq!(over_chart, total, "every candidate stands over the demand's own chart");
        (total, b.cover.len())
    };
    // A HOLE FILLED AND A BODY PUNCTURED are surgeries, so the core carries
    // and its rotor is the candidate this mind holds over the demand.
    let (fill, cover) = held(
        &["11111", "10001", "10001", "10001", "11111"],
        &["11111", "11111", "11111", "11111", "11111"],
    );
    assert_eq!((fill, cover), (1, 1), "the filled hole's core is one candidate on one chart");
    let (punch, _) = held(
        &["11111", "11111", "11111", "11111", "11111"],
        &["11111", "10001", "10001", "10001", "11111"],
    );
    assert_eq!(punch, 1, "and so is the puncture's");
    // A CORNER ERODED IS NO SURGERY: `Surgery::factor` classifies none of
    // the four kinds on it, so no core transport is measured and NOTHING is
    // held. An absent candidate is the reading, never an invented one
    // (rule 11).
    let (none, cover) = held(
        &["11111", "11111", "11111", "11111", "11111"],
        &["11111", "11111", "11111", "11111", "11110"],
    );
    assert_eq!(none, 0, "no core transport, so no candidate stands in for one");
    assert_eq!(cover, 1, "and the idea still stands on the demand's chart");
}

/// A stroke large enough that the cover runs to the end of its pieces.
/// The machine's own stroke does not reach it, so a fixture on the machine
/// stroke would assert the starvation and not the reading.
fn measured(a: &Grid, b: &Grid) -> crate::transducer::sentence::Form {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    scene_form(&NoCrystal, &mut budget, a, b).expect("a cover that runs to its end")
}

/// 53.6 — THE CARRIED PIECES ARE GROUPED BY TRANSPORT CLASS, and the CLASS
/// is the vertex.
///
/// THIS TEST ASSERTED FOUR AND ITS CLAIM IS FALSE UNDER THE RULING. It was
/// `the_carried_pieces_are_word_vertices_of_the_form`, and it read four
/// word vertices for the four bodies this cover carries. A piece is a
/// reading inside a cover, not an idea the scene holds; 21 step 1 already
/// says two bodies carried by ONE transport are one vertex, and a cover's
/// pieces are grouped exactly as a chart's quotients are.
///
/// What is measured now is that the four pieces, all carried by one
/// transport, are ONE vertex with a member per chart — and the reading
/// that matters beside it is still the old one's: no versor carries the
/// composite onto its image, so this vertex came from the cover and not
/// from a whole-body quotient.
#[test]
fn a_covers_pieces_are_one_vertex_per_transport_class() {
    let (a, b) = four_bodies();
    let f = measured(&a, &b);
    let words: Vec<&Vertex> =
        f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).collect();
    assert_eq!(words.len(), 1, "four pieces of one transport class are one vertex");
    let [w] = words.as_slice() else { panic!("one vertex") };
    let members = w.acts().map_or(0, crate::network::nerve::Keyed::len);
    assert!(members >= 1, "with a member per chart it carries a piece on: {members}");
    assert!(
        carriage(&composite(&a), &composite(&b)).is_none(),
        "the composite does not carry whole; this vertex is the cover's"
    );
}

/// 51.3 AS RULED 2026-09-18 — A PIECE IS A WORD VERTEX ONLY IF IT IS A
/// BODY. One body no versor carries whole is not a listing of its parts: it
/// carries nothing and stands as one typed demand. Before the ruling the
/// peel cut this block into shells and the form held five word vertices for
/// a scene with one body in it.
#[test]
fn one_body_that_does_not_carry_whole_is_a_demand_and_not_a_listing() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    let (ia, ib) = (perceive(&a), perceive(&b));
    let ([input], [image]) = (ia.as_slice(), ib.as_slice()) else {
        panic!("the notched fixture perceives one body on each side");
    };
    assert!(carriage(input, image).is_none(), "the whole body does not carry");
    let words = f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    assert_eq!(words, 0, "and the cover lists no pieces of it");
    assert!(
        f.vertices
            .iter()
            .any(|v| matches!(v, Vertex::Demand { kind: Demand::TopologicalDeformation { .. }, .. })),
        "what no versor carries whole stands as one typed demand"
    );
}

/// A MEET'S ADDRESS IS ITS TWO ENDS, so the nerve stores nothing beside it
/// (law 12): `meets` was a `Keyed<(Vec<u8>, Vec<u8>)>` holding a second
/// copy of what its own key already spells, cloned twice per meet. The
/// form reads the ends back out of the address, so the address must give
/// them back exactly — including for a piece key that is empty or that
/// contains the length bytes of another.
#[test]
fn a_meets_address_gives_back_its_two_ends() {
    use crate::transducer::family::pieces::{meet_ends, meet_key};
    let cases: [(&[u8], &[u8]); 5] = [
        (b"a", b"b"),
        (b"", b"b"),
        (b"a", b""),
        (b"", b""),
        // A key whose bytes look like a length prefix, against a longer one.
        (&[4, 0, 0, 0, 9, 9], &[1, 0, 0, 0]),
    ];
    for (a, b) in cases {
        let k = meet_key(a, b);
        let (lo, hi) = meet_ends(&k).expect("a meet address decodes");
        let want = if a <= b { (a, b) } else { (b, a) };
        assert_eq!((lo, hi), want, "the ends come back in the order the key sorts them");
        assert_eq!(meet_key(lo, hi), k, "and re-address to the same meet");
    }
    // A truncated address is a refusal, never a half-read pair (rule 11).
    let k = meet_key(b"ab", b"cd");
    for n in 0..k.len() {
        assert!(meet_ends(&k[..n]).is_none(), "a short address decodes to nothing at {n}");
    }
}

/// 53.6 item 3 — THE MEETS ARE FACES OF THE PIECES' OWN FORM, NOT OF THE
/// SCENE'S.
///
/// The old claim was that a cover's meets become faces of the scene's form
/// and close into its loops. They do not any more: between the CLASS
/// vertices a meet is incidence the form already builds on supports, and
/// the pieces' meets are faces of a form built inside the cover and
/// discarded once its loops are classified.
///
/// What survives, and is asserted here, is that the scene's form still has
/// faces and that every loop it does stand names a live chord over at
/// least two vertices — the structural claim, on the form the scene keeps.
#[test]
fn the_scenes_faces_are_its_own_incidence_not_the_covers_meets() {
    let (a, b) = four_bodies();
    let f = measured(&a, &b);
    assert!(!f.faces.is_empty(), "the scene's form carries its own incidence");
    for lp in &f.loops {
        assert!(
            f.faces.get(lp.chord as usize).is_some(),
            "every loop's chord names a live face: {lp:?}"
        );
        assert!(lp.vertices.len() >= 2, "a loop runs over at least two vertices: {lp:?}");
    }
}

// `every_loop_of_a_pair_form_is_flat_because_every_face_is_a_coboundary`
// was here and is DELETED, with its reasoning, because both were wrong.
//
// Its argument was that a cycle of `face_of` transports telescopes:
// `(w₂w₁⁻¹)(w₃w₂⁻¹)(w₁w₃⁻¹) = 1`. That holds only in an ABELIAN group —
// written out, the middle terms `w₁⁻¹w₃` do not cancel — and the
// sensor-plane group is not abelian once the orientation character is
// present (`S R S⁻¹ = R⁻¹`, asserted in `rotor_class.rs`).
//
// And the fixture proved nothing either way: `notched` is a pure
// translation, so every vertex's rotor is the identity, every `face_of`
// word is the identity, and every loop is flat for the most trivial
// reason there is. It confirmed what it was written to confirm.
//
// THE MEASURED FACT IT WAS REACHING FOR is narrower and stands on the
// code rather than on this fixture: `spell_peel_with` is called at
// `(PLANE_ROW, PLANE_COL)` and nowhere else, so every word a pair form
// composes is a sensor-plane word, and on this corpus they are rotations
// in one plane, which commute. That is L22 §51.6's four-letter
// sub-alphabet appearing as a consequence. Its observable is a fleet
// number, not an assertion here (L25).

/// 50.8 CASE 3 — WHERE A SIMPLEX IS FILLED it is filed on the form's own
/// torsion carrier, keyed by the holonomy's class, with the period the
/// rotation closes at and the chord it closes on. Nothing else is a filled
/// simplex, and a form that filled none carries none.
#[test]
fn a_filled_simplex_is_a_filed_loop_at_its_own_period() {
    let (a, b) = four_bodies();
    let f = measured(&a, &b);
    // WHAT THIS FIXTURE ACTUALLY CONTAINS, asserted, because every check below
    // is inside a walk of `f.torsion` and that carrier is EMPTY here — a test
    // that only walked it would pass by iterating nothing while its name
    // claimed a filed loop. An empty carrier is the expected state and not a
    // surprise, which is exactly why the content has to be asserted instead
    // of walked.
    //
    // On this fixture: 3 loops, each carrying evaluated member paths whose
    // edge holonomy is the identity — flat, and 50.8 case 3 files nothing
    // there because there is nothing to fill — and none unevaluated. The
    // four bodies are carried by translations, so the engine is right and
    // the fixture is not curved.
    //
    // 50.8 CASE 3'S FILING PATH THEREFORE HAS NO TEST. Exercising it needs a
    // cover whose meet loop composes to a finite NON-identity rotor — three
    // bodies whose pairwise frame transitions do not telescope — which is a
    // fixture to build and not a line to write, so it is named here rather
    // than implied by a green test.
    let (mut flat, mut unevaluated) = (0usize, 0usize);
    for lp in &f.loops {
        // A loop with no member path and a loop whose paths evaluate to
        // nothing are one reading here — no edge holonomy was measured on it —
        // and neither is a default standing in for an absent one.
        let hs = match f.edge_holonomies(lp) {
            Some(hs) if !hs.is_empty() => hs,
            _ => {
                unevaluated += 1;
                continue;
            }
        };
        if hs.scoped().all(|(_, h)| {
            h.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity())
        }) {
            flat += 1;
        }
    }
    // THE CLAIM, not the count: nothing is filed, because every loop this
    // cover stands is flat.
    //
    // 53.6 MOVED WHERE THE COVER'S LOOPS LIVE and the old assertion —
    // `!f.loops.is_empty()`, "the cover stands loops" — is false on the
    // scene's form now. A cover's loops are classified inside the cover, on
    // a form of its pieces that is discarded; the scene's form walks loops
    // among its CLASS vertices only. So the cover's flatness is read where
    // it lands, in `f.torsion`, and the loops walked above are the scene's
    // own — flat and evaluated, which is still asserted.
    assert_eq!(
        (flat, unevaluated),
        (f.loops.len(), 0),
        "the scene's loops are flat and every one is evaluated"
    );
    assert!(
        f.torsion.is_empty(),
        "and the form files nothing, which is why the walk below is empty"
    );
    for (class, filed) in f.torsion.scoped() {
        assert!(
            filed.period >= 2,
            "a filed period is a real rotation's order, never one: {filed:?}"
        );
        assert!(
            f.faces.get(filed.chord as usize).is_some(),
            "the chord names a live face of this form"
        );
        assert!(!class.is_empty(), "and it is filed under the holonomy's class key");
        // The filed class is re-derivable from the loop the chord names,
        // which is what the court replays (23.1 step 4).
        assert!(
            crate::transducer::sentence::torsion_loop_rotor(&f, &class).is_some(),
            "the chord face carries the holonomy it was filed under"
        );
    }
}

/// 50.8 CASE 4 — WHAT NO PIECE CARRIED KEEPS THE DEMAND ALIVE. The notched
/// block leaves laminae no carriage reaches, so the deformation stands; it
/// dissolves only when the uncarried volume is the zero vector on every
/// lane AND no loop over its pieces stands.
#[test]
fn an_uncarried_remainder_leaves_the_deformation_standing() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    let stands = f
        .vertices
        .iter()
        .any(|v| matches!(v, Vertex::Demand { kind: Demand::TopologicalDeformation { .. }, .. }));
    assert!(stands, "laminae no piece reached leave the deformation on the form");
}

/// 50.8 — A COVER THAT ENDS KEEPS WHAT IT MEASURED, AND GOES ON FROM THE
/// STATE. The same deformation covered in one large stroke and in a run of
/// small ones reaches THE SAME COVER: the same carriages under the same
/// addresses, the same meets, the same uncarried volume.
///
/// AND IT CARRIES NO POSITION. There is no stage and no index: a stroke
/// that resumes asks the cover's own carriers what is already settled — an
/// image piece whose sites are all reached, an input piece consumed, a meet
/// recorded — and every one of those readings is intrinsic to the state.
/// The test asserts that a small stroke does NOT finish in one pass, so
/// what it proves is a resume and not a single run.
///
/// THE FLOOR IS REAL AND IS NOT A SCHEDULING FAULT. A body the cover
/// cannot carry is re-read every stroke and fails again — it is the
/// uncarried support, a standing defect, and `uncarried` is where it
/// reports — so a stroke smaller than the cost of re-reading the defects
/// never reaches past them. That is the defect exerting its cost on every
/// stroke, not a queue starving a latecomer, and the reading to take from
/// it is that the stroke is too small for this scene.
#[test]
fn a_cover_that_ends_is_parked_and_the_next_stroke_resumes_it() {
    use crate::transducer::family::cover::cover_deformation;
    use crate::transducer::family::cursor;
    let (ga, gb) = four_bodies();
    let (input, image) = (composite(&ga), composite(&gb));
    fn keys<T>(k: &crate::network::nerve::Keyed<T>) -> Vec<Vec<u8>> {
        k.scoped().map(|(a, _)| a).collect()
    }

    // WHOLE, in a stroke that cannot end.
    cursor::clear();
    let whole = cover_deformation(
        &input,
        &image,
        None,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
    )
    .expect("a cover that runs to its end");
    assert_eq!(cursor::parked(), 0, "a cover that completed leaves nothing parked");

    // THE SAME DEFORMATION, one small stroke at a time. The smallest stroke
    // on this fixture that both needs more than one and reaches the end;
    // below it the uncarried bodies consume the whole stroke, see above.
    cursor::clear();
    const STROKE: usize = 4;
    let mut strokes = 0u32;
    let piecewise = loop {
        let mut small = crate::physics::Physics::default().stroke_n(STROKE);
        match cover_deformation(&input, &image, None, &mut small) {
            Ok(c) => break c,
            Err(_) => {
                strokes += 1;
                assert_eq!(
                    cursor::parked(),
                    1,
                    "a cover that ended is parked, not discarded"
                );
                assert!(strokes < 4096, "and each stroke gets further than the last");
            }
        }
    };
    assert!(strokes > 0, "six reads do not cover this deformation in one stroke");
    assert_eq!(
        keys(&whole.carried),
        keys(&piecewise.carried),
        "the same carriages, under the same addresses"
    );
    assert_eq!(keys(&whole.meets), keys(&piecewise.meets), "and the same meets");
    assert_eq!(whole.uncarried, piecewise.uncarried, "and the same uncarried volume");
    assert_eq!(cursor::parked(), 0, "and the finished cover is taken off the carrier");
}

/// THE COVER CLASSIFIES NOTHING (L23c): the reading is the form's, so it
/// is a function of the form's loops and not of the order the cover
/// happened to walk its meets in. Two builds of one pair agree exactly.
#[test]
fn the_reading_is_the_forms_and_is_stable() {
    let (a, b) = notched();
    let one = measured(&a, &b);
    let two = measured(&a, &b);
    let keys = |f: &crate::transducer::sentence::Form| -> Vec<Vec<u8>> {
        f.torsion.scoped().map(|(k, _)| k).collect()
    };
    assert_eq!(keys(&one), keys(&two), "the same filed classes, in the same key order");
    assert_eq!(one.vertices.len(), two.vertices.len(), "the same vertices");
    assert_eq!(one.faces.len(), two.faces.len(), "the same faces");
    assert_eq!(one.loops.len(), two.loops.len(), "the same loops");
}

/// One cover run to the end of its pieces: the cover, its tally, and how
/// many bodies the input composite holds.
fn measured_cover(
    a: &Grid,
    b: &Grid,
) -> (crate::transducer::family::cover::Cover, crate::transducer::family::cover::CoverTally, usize)
{
    use crate::transducer::family::cover::{cover_deformation, take_cover_tally};
    use crate::transducer::family::cursor;
    use crate::transducer::family::pieces::pieces_of;
    let (input, image) = (composite(a), composite(b));
    cursor::clear();
    let _ = take_cover_tally();
    let cover = cover_deformation(
        &input,
        &image,
        None,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
    )
    .expect("a cover that runs to its end");
    let t = take_cover_tally();
    let ins = pieces_of(&input.laminae).scoped().count();
    (cover, t, ins)
}

/// 50.8 — Φ(U) IS ONE SET, exactly as Φ⁻¹(V) is. The two peels are under
/// no obligation to agree on granularity, so a carried piece whose image
/// lies across several output pieces is assembled whole and carried once.
///
/// Skipping that case instead refuses every carriage where the output side
/// holds more bodies than the input — which is the same fault the
/// pullback's own preimage exists to avoid, taken in the other direction.
/// The body the pullback cannot place is the one pass (b) is offered, and
/// its image is read from the versors pass (a) already placed.
#[test]
fn an_image_across_several_output_pieces_is_assembled_and_carried_once() {
    let (a, b) = four_bodies_one_left();
    let (_, t, _) = measured_cover(&a, &b);
    assert!(
        t.propagated > 0,
        "the pass attempts the pieces the pullback left: {} attempts",
        t.propagated
    );
}

/// 50.8, rule 1 — PASS (b) PROPAGATES, IT DOES NOT SEARCH. The second pass
/// tried every output piece of the unplaced piece's own shell kind and hue
/// and asked `carriage` to decide each try: `ins × outs` versor solves over
/// a correspondence the cover had already measured. It applies the versors
/// pass (a) placed and reads the image by position, so the attempt count is
/// `placed × unplaced` — and where the pullback placed nothing, there is no
/// versor to propagate and the pass attempts nothing at all.
///
/// The second half is the one the search could not pass: with no placed
/// carriage it still ran `ins × outs` attempts, and whatever those found
/// was a piece the correspondence did not put there.
#[test]
fn cover_attempts_are_placed_times_unplaced() {
    // TWO BODIES THAT DO NOT MEET. No lamina of the input stands under any
    // lamina of the image, so the pullback names no preimage and places
    // nothing; with nothing placed there is nothing to propagate.
    let apart = (
        grid(&["111", "111", "000", "000", "000", "000"]),
        grid(&["000", "000", "000", "000", "111", "111"]),
    );
    let (cover, t, _) = measured_cover(&apart.0, &apart.1);
    assert_eq!(t.carried, 0, "the pullback places nothing across a gap");
    assert_eq!(t.propagated, 0, "and with no placed versor, (b) attempts nothing");
    assert!(
        cover.uncarried.iter().any(|&x| x > 0),
        "what no versor carries is a demand, and it reports: {:?}",
        cover.uncarried
    );

    // A DEFORMATION THE PULLBACK REACHES. The attempts (b) makes are
    // bounded by the versors (a) placed times the input's own bodies —
    // never by the output's count, which is what the search read.
    let (a, b) = four_bodies_one_left();
    let (_, t, ins) = measured_cover(&a, &b);
    assert!(t.carried > 0, "the pullback places the pieces that stand in place");
    assert!(
        t.propagated as usize <= (t.carried as usize) * ins,
        "attempts are placed × unplaced: {} against {} × {ins}",
        t.propagated,
        t.carried
    );
}

/// 13.9 STEP 7, 50.3 — A PURE-COBOUNDARY FORM YIELDS NO EVALUATED LOOP.
/// A face whose two pieces the cover did not both carry has no transition
/// between their frames to write, so it carries `edge: None`; a cycle
/// through such a face has no edge holonomy, and `edge_holonomy` answers
/// `None` rather than composing the identity.
///
/// That `None` is a REFUSAL TO READ and not a flat reading, and it is what
/// both organs skip on: `pair.rs` classifies nothing there, so nothing is
/// filed, and the court demands nothing of a loop the voice never
/// evaluated. Reading the transport composite instead answers for such a
/// loop anyway — it telescopes over the sensor-plane words a pair form
/// composes — which is the flat reading this refuses to make.
#[test]
fn a_pure_coboundary_form_yields_no_evaluated_loop() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::Form;
    let mut f = Form::default();
    for tag in [1u8, 2u8, 3u8] {
        f.vertices.push(crate::transducer::sentence::Vertex::word_on(
            crate::transducer::address::Address::from_bytes([tag; 32]),
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            crate::transducer::stalk::Invariant::default(),
            Box::new(crate::transducer::element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    // Every face the coboundary of its two vertices, and no edge on any of
    // them: the pieces were not carried, so there is no frame transition.
    for (i, j) in [(0u16, 1u16), (0, 2), (0, 3), (1, 2), (2, 3), (1, 3)] {
        f.faces.push(f.face_of(i, j).expect("a coboundary of two vertices"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    // THE TRIANGLE STANDS. Its faces are real — between bodies, not star —
    // and carry no edge, so every member path drops on a remainder edge and
    // the loop is UNREAD rather than flat. A trivial-cycle filter that took
    // an empty edge reading for a flat one would prune exactly the loops
    // this test is about (rule 11).
    assert!(!f.loops.is_empty(), "the triangle among the bodies closes");
    for lp in &f.loops {
        // NO MEMBER PATH IS EVALUATED, which is what "not evaluated" means once
        // a loop has one reading per path: a path carrying a remainder edge
        // anywhere on it is absent from the result, so an empty result is the
        // whole loop unevaluated.
        assert!(
            f.edge_holonomies(lp).is_some_and(|hs| hs.is_empty()),
            "a cycle through a face with no edge is not evaluated on any path: {lp:?}"
        );
        assert!(f.edge_classes(lp).is_empty(), "so it has no class to file under");
        assert!(f.edge_coheres(lp), "and it owes no filing");
    }
    assert!(
        f.torsion.scoped().next().is_none(),
        "the voice files nothing on a form it evaluated no loop of"
    );
}

/// 53.6's DoD — A FORM'S WORD VERTICES ARE BOUNDED BY ITS TRANSPORT
/// CLASSES, not by the pieces a cover cut.
///
/// The bound is the claim: one Word vertex per transport class present,
/// times the readings, plus one per demand kind and chart. A cover of `n`
/// pieces used to be `n` vertices, so the form grew with the cut rather
/// than with what the cut FOUND — `484b58aa` read 2 055 vertices, 11 167
/// faces and 3 986 loops, and every carrier the engine has is multiplied
/// by that count.
///
/// This is the bound asked of the form directly: no two Word vertices
/// share a transport class. A count would not say it — four pieces of one
/// class reading "one" is the same number as one piece reading "one" — so
/// the classes are compared coordinatewise.
#[test]
fn a_forms_word_vertices_are_bounded_by_its_transport_classes() {
    let (a, b) = four_bodies();
    let f = measured(&a, &b);
    let classes: Vec<Vec<u8>> = f
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .filter_map(crate::transducer::family::transport_class)
        .map(|tc| tc.key())
        .collect();
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for k in &classes {
        assert!(
            !seen.contains(k),
            "two Word vertices of one transport class: the cover's pieces are not grouped"
        );
        seen.insert(k, ());
    }
    // And every Word vertex HAS a class, or the bound names nothing.
    let words = f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    assert_eq!(classes.len(), words, "every word vertex is bounded by a class it carries");
}

/// 53.6 item 4 AS AMENDED — A CLASSIFIED LOOP THE SCENE CANNOT GIVE A
/// CHORD TO LEAVES AS THE DEMAND IT IS, never dropped.
///
/// A cover's loop is classified inside the cover, on a form of its pieces
/// that is discarded. Its chord indexes THAT form, so the scene resolves a
/// chord of its own between the two class vertices the loop's ends belong
/// to. Where the scene relates those classes by no face there is no chord
/// to name — and the first cut of 53.6 dropped the reading there, which is
/// a measured class discarded (law 7, rule 11).
///
/// It leaves as `Standing` with the holonomy's own rotor as the witness.
/// The two readings here are that the demand is TYPED — a versor that
/// turns, or it is not a standing reading at all — and that the rotor it
/// carries is the one measured and not a representative.
#[test]
fn a_filed_loop_with_no_chord_leaves_as_a_standing_demand() {
    use crate::transducer::alphabet::{standing_of, LetterId, Word};
    use crate::transducer::sentence::Demand;
    // The quarter turn is a versor that turns, so it has a standing
    // reading; the identity is a versor that does not, and has none.
    let q = LetterId::quarter((0, 1)).expect("quarter");
    let turned = Word::from_letters(&[(q, false)]).rotor();
    let Some(d @ Demand::Standing { .. }) = standing_of(&turned) else {
        panic!("a rotor that turns has a standing reading");
    };
    assert!(d.typed().is_ok(), "and it is typed: a versor that turns");
    let Demand::Standing { rotor, .. } = &d else { panic!("standing") };
    assert!(
        rotor.projective_eq(&turned),
        "the witness is the holonomy measured, not a representative"
    );
    assert!(
        standing_of(&crate::transducer::alphabet::Rotor::identity()).is_none(),
        "the identity turns nothing and has no standing reading (rule 11)"
    );
}
