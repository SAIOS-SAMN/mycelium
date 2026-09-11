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
