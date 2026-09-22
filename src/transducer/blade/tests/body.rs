// 20 — the body: what lifts to an invertible blade, what has volume, and
// what joins by adjacency within one hue.

use super::common::*;
use crate::transducer::blade::*;

/// Topology on the cube complex: a square has no hole, a ring one hole,
/// two apart bars two components, and a section no cavity; the moduli
/// carry the numbers first, and a dilation keeps them.
#[test]
fn topology_counts_components_and_holes_exactly() {
    assert_eq!(topology(&rect(0, 0, 2, 2)), (1, 0, 0));
    let ring = at_frame_0(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
    assert_eq!(topology(&ring), (1, 1, 0));
    let two = at_frame_0(&[(0, 0), (0, 1), (5, 5), (5, 6)]);
    assert_eq!(topology(&two), (2, 0, 0));
    let m = moduli(&shape_of(&ring).blade);
    use crate::transducer::stalk::ChartAxis::Count;
    assert_eq!((m[0].clone(), m[1].clone(), m[2].clone(), m[3].clone()), (Count(1), Count(1), Count(0), Count(8)));
    assert_eq!(moduli(&shape_of(&dilate(&ring, 2)).blade), m, "a dilated ring reads the same moduli");
    // The fourth-order axis separates bodies the inertia cannot: an
    // L-tetromino and a T-tetromino... share a lamina count; the moduli
    // differ.
    let l4 = at_frame_0(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
    let s4 = at_frame_0(&[(0, 1), (0, 2), (1, 0), (1, 1)]);
    assert_ne!(moduli(&shape_of(&l4).blade), moduli(&shape_of(&s4).blade));
}

/// A 2×2 square lifts to a blade with the exact centroid (½,½,0), positive
/// extent, and an invertible spatial sphere — the body HAS volume.
#[test]
fn a_square_lifts_to_an_invertible_body() {
    let b = lift(&laminae(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]));
    assert_eq!(b.centroid(), Some((qn(1, 2), qn(1, 2), q(0))));
    assert!(b.radius2() > q(0), "the body has volume (off the null cone)");
    assert!(b.is_invertible());
    assert_eq!(b.shell, q(4));
    assert_eq!(b.gauge, vec![(1u8, q(4))]);
}

/// A lone lamina still HAS volume (r² = 1/6) — no body is a null point.
#[test]
fn a_lone_lamina_has_volume() {
    let b = lift(&laminae(&[(3, 5, 7)]));
    assert_eq!(b.radius2(), qn(1, 6));
    assert!(b.is_invertible());
}

/// H1, 70.4 — A BODY IS A MAXIMAL HUE-BLIND CONNECTED REGION. Adjacency
/// joins whatever hues the laminae carry; a gap splits; the hue rides in
/// the gauge.
///
/// THIS REPLACES CHAPTER 20 (2026-09-09), AND ITS FEAR IS RECORDED
/// BECAUSE IT WAS MEASURED RATHER THAN ARGUED AWAY. The old test read:
/// "read hue-blind, every touching scene would be one blob no versor
/// carried, and every such percept a deformation demand." Measured over
/// 40 tasks and 128 scenes, hue-blind reading gives 732 regions — 5.7
/// objects per scene, not one blob — and 598 of them are monochrome
/// anyway. What merges is exactly the polychrome object that chapter 20
/// was shattering into 7.8 pieces.
///
/// The cost of the old rule, on the same corpus: 1646 bodies for 732
/// objects, 64.0% of them single cells, and a "contact graph" whose
/// component count was exactly the region count — because it was the
/// INTERNAL gauge structure of single objects, never contact at all.
#[test]
fn components_join_by_adjacency_whatever_the_hue() {
    let one = components(&laminae(&[(0, 0, 1), (0, 1, 2)]));
    assert_eq!(one.len(), 1, "adjacent laminae of two hues are ONE body, in two colours");
    let same = components(&laminae(&[(0, 0, 1), (0, 1, 1)]));
    assert_eq!(same.len(), 1, "adjacent laminae of one hue are one body");
    let apart = components(&laminae(&[(0, 0, 1), (0, 5, 1)]));
    assert_eq!(apart.len(), 2, "a gap separates bodies");
    let across = components(&[(0, 0, 0, 1), (0, 0, 1, 1)]);
    assert_eq!(across.len(), 1, "one site in two consecutive frames is one body (51.3)");
    // THE HUE IS NOT LOST, IT MOVES TO THE FIBER. `Blade.gauge` carries it
    // as sorted `(hue, count)` relations — which is where every reader
    // downstream of perception already looked for it.
    let body = lift(&laminae(&[(0, 0, 1), (0, 1, 2)]));
    assert_eq!(body.gauge.len(), 2, "both hues carried as gauge: {:?}", body.gauge);
    // AND THE DIAGONAL DOES NOT JOIN. H2 gives it to contact; if it
    // entered here it would merge the bodies it is meant to relate.
    let corner = components(&[(0, 0, 0, 1), (1, 1, 0, 1)]);
    assert_eq!(corner.len(), 2, "a corner meeting is contact, never one body");
}

/// A translated square is the SAME body up to rigid motion: identical
/// invariant signature (the correspond key), sites aside — and so is the
/// same square in another frame.
#[test]
fn a_translate_shares_the_invariant() {
    let a = lift(&laminae(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)]));
    let b = lift(&[(4, 7, 1, 1), (4, 8, 1, 1), (5, 7, 1, 1), (5, 8, 1, 1)]);
    assert_eq!(a.invariant(), b.invariant(), "a translate is the same orbit");
}

/// A body across two frames lifts with its centroid in ℚ³ and the six
/// moments on {e₀, e₁, e₃}; the variance along e₃ is carried on Izz and
/// the off-plane harmonic is readable.
#[test]
fn volumetric_blade_3d_lift_computes_six_moments_and_3d_centroid() {
    let comp3 = vec![
        (0, 0, 0, 1u8),
        (0, 2, 0, 1u8),
        (0, 0, 2, 1u8),
        (0, 2, 2, 1u8),
    ];
    let b = lift(&comp3);
    assert_eq!(b.shell, q(4));
    let c3 = b.centroid().expect("3D centroid");
    // Row centroid = 0, Col centroid = 1, Frame centroid = 1
    assert_eq!(c3.0, q(0));
    assert_eq!(c3.1, q(1));
    assert_eq!(c3.2, q(1));

    // Inertia tensor has 6 components: [Ixx, Iyy, Ixy, Izz, Ixz, Iyz]
    assert_eq!(b.inertia.len(), 6);
    // Across frames, Izz carries the offsets along e₃ and nothing of a
    // lamina's own thickness (16.4): four laminae one frame from the
    // centroid.
    assert_eq!(b.inertia[3], q(4), "Izz is the offsets alone");

    // Deviatoric harmonic on plane (0, 1) and off-plane (0, 3)
    let h01 = b.harmonic_on_plane((0, 1), false).expect("plane 0,1");
    let h03 = b.harmonic_on_plane((0, 3), false).expect("plane 0,3");
    assert_ne!(h01, [q(0), q(0)]);
    assert_ne!(h03, [q(0), q(0)]);
}

#[test]
fn rotor_and_placement_between_measures_rotation_and_translation() {
    let s1 = shape_of(&at_frame_0(&[(0, 0), (0, 1), (0, 2)]));
    let s2 = shape_of(&at_frame_0(&[(3, 5), (4, 5), (5, 5)])); // rotated by 90 degrees and translated
    let (rotor, trans) = rotor_and_placement_between(&s1, &s2).expect("carried");
    assert!(!trans.is_zero());
    assert!(!rotor.is_identity());
    let r = rotor_between_shapes(&s1, &s2).expect("rotor exists");
    assert_eq!(r, rotor);
}
