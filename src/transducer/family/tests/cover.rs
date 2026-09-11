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
use crate::network::nerve::ROOT;
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

/// WHAT `flat N / standing 0` ACTUALLY MEASURES — and it is not the scene.
///
/// EVERY face of a pair form carries `face_of(i, j)`, which is the
/// COBOUNDARY of the two vertices' placements: `w_j ∘ w_i⁻¹`. A cycle of
/// those telescopes —
///
///   (w₂w₁⁻¹)(w₃w₂⁻¹)…(w₁wₙ⁻¹) = 1
///
/// — so every loop of such a form is flat BY CONSTRUCTION, whatever the
/// scene was. The cover's meets are pushed as `FormFace::new(i, j, t.word)`
/// with `t = face_of(i, j)`, and the star and cell faces come from the same
/// reader, so no face of a pair form is anything else.
///
/// The corpus reading `loops flat 3444 filed 0 standing 0` at 1<<20 is
/// therefore a tautology and not a measurement: the construction cannot
/// produce a non-flat loop, so 3 444 flat ones say only that 3 444 cycles
/// were walked. A palindrome — `x x⁻¹ = 1`, 1-D backtracking — is all a
/// coboundary can be.
///
/// THE CONTRAST IS ALREADY IN THE TREE, one file over:
/// `relator.rs` builds a form whose CHORD carries an independently ruled
/// word rather than a coboundary, and that form files a class of period 2.
/// A non-trivial holonomy needs transports that are not the coboundary of
/// one global assignment — three charts each ruled on its own, `[x, y] =
/// xyx⁻¹y⁻¹ ≠ 1` — which is what L17's depth 4 proves and what the cover
/// does not yet build.
#[test]
fn every_loop_of_a_pair_form_is_flat_because_every_face_is_a_coboundary() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    assert!(!f.loops.is_empty(), "the fixture walks cycles at all");
    for lp in &f.loops {
        assert!(
            lp.holonomy.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()),
            "a cycle of coboundaries telescopes: {:?}",
            lp.holonomy
        );
    }
    // And so the form files nothing and stands nothing — not because the
    // notched block has no curvature, but because this construction has
    // nowhere to put any.
    assert_eq!(
        f.torsion.fiber(ROOT).count(),
        0,
        "no cell is filled, and a coboundary could not have filled one"
    );
}

/// 50.8 CASE 3 — WHERE A CELL IS FILLED it is filed on the form's own
/// torsion carrier, keyed by the holonomy's class, with the period the
/// rotation closes at and the chord it closes on. Nothing else is a filled
/// cell, and a form that filled none carries none.
#[test]
fn a_filled_cell_is_a_filed_loop_at_its_own_period() {
    let (a, b) = notched();
    let f = measured(&a, &b);
    for (class, filed) in f.torsion.fiber(ROOT) {
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
    let (Some(input), Some(image)) = (ia.first(), ib.first()) else {
        panic!("the notched fixture perceives one body on each side");
    };
    fn keys<T>(k: &crate::network::nerve::Keyed<T>) -> Vec<Vec<u8>> {
        k.fiber(ROOT).map(|(a, _)| a).collect()
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
        f.torsion.fiber(ROOT).map(|(k, _)| k).collect()
    };
    assert_eq!(keys(&one), keys(&two), "the same filed classes, in the same key order");
    assert_eq!(one.vertices.len(), two.vertices.len(), "the same vertices");
    assert_eq!(one.faces.len(), two.faces.len(), "the same faces");
    assert_eq!(one.loops.len(), two.loops.len(), "the same loops");
}
