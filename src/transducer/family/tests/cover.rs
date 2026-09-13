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

/// A solid block against the same block with ONE CELL REMOVED. No versor
/// carries the whole body onto the whole image — that is what makes it a
/// deformation — and the peel's shells carry piece by piece.
fn notched() -> (Grid, Grid) {
    (
        grid(&["11111", "11111", "11111", "11111", "11111"]),
        grid(&["11111", "11111", "11111", "11111", "11110"]),
    )
}

/// A stroke large enough that the cover runs to the end of its pieces.
/// The machine's own stroke does not reach it — measured on the corpus,
/// where the cover needs 4 374 reads over twelve tasks and a stroke gives
/// it 16 — so a fixture on the machine stroke would assert the starvation
/// and not the reading.
fn measured(a: &Grid, b: &Grid) -> crate::transducer::sentence::Form {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    pair_form(&NoCrystal, &mut budget, a, b, None).expect("a cover that runs to its end")
}

/// THE CARRIED PIECES ARE VERTICES OF THE FORM (L23c). Before this, a
/// deformation's form held a frame and a demand and nothing else: the
/// cover's carriages were written to `cover.carried` and read by a count.
#[test]
fn the_carried_pieces_are_word_vertices_of_the_form() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    let words = f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    assert_eq!(words, 5, "the five pieces the cover carried are five word vertices");
    // The pairing consumed nothing here — no versor carries the whole body
    // — so every one of them came from the cover.
    assert_eq!(
        pair_scene(&a, &b, None).pairs.len(),
        0,
        "the whole body does not pair; these vertices are the cover's"
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

/// THE MEETS ARE FACES, AND THE FACES MAKE LOOPS. A meet is a 1-simplex of
/// the nerve; the transport across it is `face_of`'s to compute from the
/// two vertices, and a cycle of those faces is what carries a holonomy.
#[test]
fn the_meets_are_faces_and_the_faces_close_into_loops() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    assert!(!f.faces.is_empty(), "the cover's meets became faces");
    assert!(!f.loops.is_empty(), "and the faces close into cycles");
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

/// 50.8 CASE 3 — WHERE A CELL IS FILLED it is filed on the form's own
/// torsion carrier, keyed by the holonomy's class, with the period the
/// rotation closes at and the chord it closes on. Nothing else is a filled
/// cell, and a form that filled none carries none.
#[test]
fn a_filled_cell_is_a_filed_loop_at_its_own_period() {
    let (a, b) = notched();
    let f = measured(&a, &b);
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
/// block leaves cells no carriage reaches, so the deformation stands; it
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
    assert!(stands, "cells no piece reached leave the deformation on the form");
}

/// 50.8 — A COVER THAT ENDS KEEPS WHAT IT MEASURED, AND GOES ON FROM THE
/// STATE. The same deformation covered in one large stroke and in a run of
/// small ones reaches THE SAME COVER: the same carriages under the same
/// addresses, the same meets, the same uncarried volume.
///
/// AND IT CARRIES NO POSITION. There is no stage and no index: a stroke
/// that resumes asks the cover's own carriers what is already settled — an
/// image piece whose cells are all reached, an input piece consumed, a meet
/// recorded — and every one of those readings is intrinsic to the state.
/// The test asserts that a small stroke does NOT finish in one pass, so
/// what it proves is a resume and not a single run.
///
/// THE FLOOR IS REAL AND IS NOT A SCHEDULING FAULT. Measured on this
/// fixture: at a stroke of 6 and above the cover completes across several
/// strokes; at 4 it never does, and parks forever at `placed 5 meets 0`.
/// A piece the cover cannot carry is re-read every stroke and fails again
/// — it is the uncarried support, a standing defect, and `uncarried` is
/// where it reports — so a stroke smaller than the cost of re-reading the
/// defects never reaches past them. That is the defect exerting its cost
/// on every stroke, not a queue starving a latecomer, and the reading to
/// take from it is that the stroke is too small for this scene.
#[test]
fn a_cover_that_ends_is_parked_and_the_next_stroke_resumes_it() {
    use crate::transducer::family::cover::cover_deformation;
    use crate::transducer::family::cursor;
    let (ga, gb) = notched();
    let (ia, ib) = (perceive(&ga), perceive(&gb));
    // The fixture perceives ONE body on each side, which the slice says.
    let ([input], [image]) = (ia.as_slice(), ib.as_slice()) else {
        panic!("the notched fixture perceives one body on each side");
    };
    fn keys<T>(k: &crate::network::nerve::Keyed<T>) -> Vec<Vec<u8>> {
        k.scoped().map(|(a, _)| a).collect()
    }

    // WHOLE, in a stroke that cannot end.
    cursor::clear();
    let whole = cover_deformation(
        input,
        image,
        None,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
    )
    .expect("a cover that runs to its end");
    assert_eq!(cursor::parked(), 0, "a cover that completed leaves nothing parked");

    // THE SAME DEFORMATION, one small stroke at a time.
    cursor::clear();
    // Six: the smallest stroke on this fixture that both needs more than
    // one and reaches the end. Below it the uncarried pieces consume the
    // whole stroke; see the note above.
    const STROKE: usize = 6;
    let mut strokes = 0u32;
    let piecewise = loop {
        let mut small = crate::physics::Physics::default().stroke_n(STROKE);
        match cover_deformation(input, image, None, &mut small) {
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
/// many pieces the input peeled into.
fn measured_cover(
    a: &Grid,
    b: &Grid,
) -> (crate::transducer::family::cover::Cover, crate::transducer::family::cover::CoverTally, usize)
{
    use crate::transducer::family::cover::{cover_deformation, take_cover_tally};
    use crate::transducer::family::cursor;
    use crate::transducer::family::pieces::pieces_of;
    let (ia, ib) = (perceive(a), perceive(b));
    let ([input], [image]) = (ia.as_slice(), ib.as_slice()) else {
        panic!("each fixture perceives one body on each side")
    };
    cursor::clear();
    let _ = take_cover_tally();
    let cover = cover_deformation(
        input,
        image,
        None,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
    )
    .expect("a cover that runs to its end");
    let t = take_cover_tally();
    let ins = pieces_of(&input.cells, None).scoped().count();
    (cover, t, ins)
}

/// 50.8 — Φ(U) IS ONE SET, exactly as Φ⁻¹(V) is. The two peels are under
/// no obligation to agree on granularity, so a carried piece whose image
/// lies across several output pieces is assembled whole and carried once.
///
/// Skipping that case instead refuses every carriage where the output peel
/// is finer than the input peel — which is the same fault the pullback's
/// own preimage exists to avoid, taken in the other direction. It is not
/// rare: on `notched`, where the output peel splits the block into shells,
/// EVERY propagation attempt landed across two output pieces, so the pass
/// made no attempt at all and the skip was silent.
#[test]
fn an_image_across_several_output_pieces_is_assembled_and_carried_once() {
    let (a, b) = notched();
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
    // TWO BODIES THAT DO NOT MEET. No cell of the input stands under any
    // cell of the image, so the pullback names no preimage and places
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
    // bounded by the versors (a) placed times the input's own pieces —
    // never by the output's count, which is what the search read.
    let (a, b) = notched();
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
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
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
