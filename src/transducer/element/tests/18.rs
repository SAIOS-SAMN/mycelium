// 18.3 — the four lanes and the germinated axes: what a germ measures, and
// that the action is a versor and never a matrix.

use crate::transducer::element::*;
use crate::transducer::germinate::germinate;
use super::common::*;

// ─── THE GERM CARRIER: centered coboundary chains (post-excision) ───────

/// THE RESIDUAL GERMINATES: a pivot that no motor closes becomes a first-class
/// germ on the residual lane — the description that used to be discarded. Its
/// magnitude is nonzero there, so the POW++ gate admits it.
#[test]
fn a_residual_germinates_as_a_coboundary_germ() {
    // (0,1): 2→9 with the rest fixed — not a rigid motion; a pure residual.
    let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
    let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)];
    let g = germinate(&a, &b).unwrap();
    let germ = from_residual(&g);
    // A hue transition on a standing cell measures as GAUGE content
    // alongside the residual defect (the four-lane projection).
    assert_eq!(germ.lanes(), [false, true, false, false], "a recolour is 𝒢 content; temporal and holonomic are empty until standing or punctured");
    let mag = germ.magnitude();
    assert!(mag[2] == 0 && mag[1] >= 1 && mag[0] == 0, "gauge only: no spatial, no temporal extent yet");
    assert_eq!(mag[3], 0, "holonomic volume is empty until sections or torsion");
    // A residual has no group inverse (its block is antisymmetric/singular);
    // its solution is the reverse coboundary (from_residual of b→a), not a
    // matrix inverse — the group solution() correctly declines.
    assert!(germ.solution().is_none(), "no single-matrix inverse for a coboundary");
    let reverse = from_residual(&germinate(&b, &a).unwrap());
    assert_ne!(germ.address(), reverse.address(), "the reverse description is a distinct germ");
}

/// THE CANCELING TALLY IS NOT IDENTITY: L→bar's bidirectional hue swap cancels
/// the tally to zero — but the CHAIN keeps it a real, non-identity germ with
/// exact magnitude, and its reverse is a distinct word.
#[test]
fn a_bidirectional_swap_does_not_collapse_to_zero() {
    let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)]; // L
    let b = [(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]; // bar
    let l_to_bar = from_residual(&germinate(&a, &b).unwrap());
    // One cell vacates and one appears: SPATIAL content (mass moved)
    // alongside the two-cell residual defect — measured, not defaulted.
    assert_eq!(l_to_bar.magnitude(), [2, 0, 0, 0], "two moved cells, exact — not a canceled 0; the fibre lane is empty until punctured");
    assert_eq!(l_to_bar.lanes(), [true, false, false, false], "a real spatial germ");
    assert!(!l_to_bar.is_identity(), "the chain decides — a canceled tally is not identity");
    assert!(!l_to_bar.substance().is_identity(), "and its substance is real");
    let bar_to_l = from_residual(&germinate(&b, &a).unwrap());
    assert_ne!(l_to_bar.address(), bar_to_l.address(), "distinct descriptions, distinct address");
    // The two descriptions share one base geometry: a versor carries one
    // support onto the other, and the reversed gauge is a second section
    // of the same word, never a second card.
    assert!(l_to_bar.carriage(&bar_to_l).is_some(), "one base geometry, two sections");
}

/// Multi-axial: distinct axes compose INDEPENDENTLY (no cross-talk, no cap on
/// how many axes germinate). A germinated fiber extends without disturbing others.
#[test]
fn axes_are_independent_and_extensible() {
    let mut spatial = elem_from_versor(translator_versor(1, 0));
    let mut germ = Element::identity();
    germ.open_axis(translator_versor(0, 1));
    let both = spatial.compose(&germ);
    assert_eq!(both.axes(), vec![FIBER_SPATIAL, FIBER_GERMINATED], "both axes present");
    assert!(same_versor(
        &both.versor().expect("S"),
        &spatial.versor().expect("S")
    ));
    assert_eq!(both.max_axis(), Some(FIBER_GERMINATED));
    assert!(!both.same_action(&spatial), "a germinated axis is a new generator, not a new digest");
    spatial.open_axis(translator_versor(2, 0));
    assert_eq!(spatial.opened_axes(), vec![FIBER_GERMINATED]);
}

/// The gate this chapter owes: the element's action is a versor and never a
/// matrix. It read one file when the carrier was one file; it reads every
/// production file of `element/` now, and the test module is a sibling
/// directory rather than a `#[cfg(test)]` tail, so the split is what the
/// scan walks.
#[test]
fn no_qmatrix_in_cognition() {
    let production = [
        ("carrier.rs", include_str!("../carrier.rs")),
        ("gauge.rs", include_str!("../gauge.rs")),
        ("letters.rs", include_str!("../letters.rs")),
        ("mod.rs", include_str!("../mod.rs")),
        ("residual.rs", include_str!("../residual.rs")),
        ("sensor.rs", include_str!("../sensor.rs")),
        ("tape.rs", include_str!("../tape.rs")),
        ("vertex.rs", include_str!("../vertex.rs")),
    ];
    for (name, src) in production {
        assert!(
            !src.contains("fibers: Vec") && !src.contains("as_operator") && !src.contains("realize("),
            "the Element's action is a versor, never a matrix — {name}"
        );
    }
}
