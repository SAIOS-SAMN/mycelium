// 20 — the body: what lifts to an invertible blade, what has volume, and
// what joins by adjacency within one hue.

use super::common::*;
use crate::transducer::blade::*;

/// Topology on the closed-cell complex: a square has no hole, a ring
/// one hole, two apart bars two components; the moduli carry both
/// numbers first, and a dilation keeps them.
#[test]
fn topology_counts_components_and_holes_exactly() {
    assert_eq!(topology(&rect(0, 0, 2, 2)), (1, 0));
    let ring: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)];
    assert_eq!(topology(&ring), (1, 1));
    let two: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (5, 5), (5, 6)];
    assert_eq!(topology(&two), (2, 0));
    let m = moduli(&shape_of(&ring).blade);
    assert_eq!((m[0], m[1], m[2]), (1, 1, 8));
    assert_eq!(moduli(&shape_of(&dilate(&ring, 2)).blade), m, "a dilated ring reads the same moduli");
    // The fourth-order axis separates bodies the inertia cannot: an
    // L-tetromino and a T-tetromino... share a cell count; the moduli
    // differ.
    let l4: Vec<(i64, i64)> = vec![(0, 0), (1, 0), (2, 0), (2, 1)];
    let s4: Vec<(i64, i64)> = vec![(0, 1), (0, 2), (1, 0), (1, 1)];
    assert_ne!(moduli(&shape_of(&l4).blade), moduli(&shape_of(&s4).blade));
}

/// A 2×2 square lifts to a blade with the exact centroid (½,½), positive
/// extent, and an invertible spatial sphere — the body HAS volume.
#[test]
fn a_square_lifts_to_an_invertible_body() {
    let b = lift(&cells(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]), 0);
    assert_eq!(b.centroid(), Some((qn(1, 2), qn(1, 2))));
    assert!(b.radius2() > q(0), "the body has volume (off the null cone)");
    assert!(b.is_invertible());
    assert_eq!(b.shell, q(4));
    assert_eq!(b.gauge, vec![(1u8, q(4))]);
}

/// A lone cell still HAS volume (r² = 1/6) — no body is a null point.
#[test]
fn a_lone_cell_has_volume() {
    let b = lift(&cells(&[(3, 5, 7)]), 0);
    assert_eq!(b.radius2(), qn(1, 6));
    assert!(b.is_invertible());
}

/// A BODY IS ONE HUE (chapter 20, ruled 2026-09-09): two touching cells of
/// different hues are TWO bodies sharing a boundary — the contact face a
/// pair form carries with its edge transport — two touching cells of one
/// hue are one body, and a gap splits. (Read hue-blind, every touching
/// scene was one blob no versor carried, and every such percept a
/// deformation demand.)
#[test]
fn components_join_by_adjacency_and_one_hue() {
    let two = components(&cells(&[(0, 0, 1), (0, 1, 2)]));
    assert_eq!(two.len(), 2, "adjacent cells of two hues are two bodies sharing a boundary");
    let one = components(&cells(&[(0, 0, 1), (0, 1, 1)]));
    assert_eq!(one.len(), 1, "adjacent cells of one hue are one body");
    let apart = components(&cells(&[(0, 0, 1), (0, 5, 1)]));
    assert_eq!(apart.len(), 2, "a gap separates bodies");
}

/// A translated square is the SAME body up to rigid motion: identical
/// invariant signature (the correspond key), positions aside.
#[test]
fn a_translate_shares_the_invariant() {
    let a = lift(&cells(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]), 0);
    let b = lift(&cells(&[(4, 7, 1), (4, 8, 1), (5, 7, 1), (5, 8, 1)]), 1);
    assert_eq!(a.invariant(), b.invariant(), "a translate is the same orbit");
}
