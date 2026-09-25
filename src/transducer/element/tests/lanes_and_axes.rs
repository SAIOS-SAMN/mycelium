// 18.3 — the four lanes and the germinated axes: what a germ measures, and
// that the action is a versor and never a matrix.

use crate::transducer::element::*;
use crate::transducer::germinate::germinate;
use super::common::*;

/// ONE SEAT PER OPENED AXIS, HOWEVER OFTEN IT IS OPENED.
///
/// `germinated` has no remover anywhere in the tree and its live writer is
/// ungated: it fires once per stroke per address while a held orbit stands
/// unclosed on a Twisted chart. So the same versor could take a new seat
/// on every stroke, each push re-minting the address, and the list is
/// copied by `compose`, by `solution` and by the court on every ruling.
/// A versor is sixteen exact rationals.
///
/// An axis already holding this transport IS this axis, and a versor and
/// its negation are one transport (1.5), so the comparison is projective.
#[test]
fn opening_one_axis_twice_is_one_axis() {
    let mut e = Element::identity();
    let v = translator_versor(3, 4);
    let first = e.open_axis(v.clone()).expect("a seat");
    assert_eq!(e.germinated().len(), 1, "the axis is opened");
    let again = e.open_axis(v.clone()).expect("the seat it has");
    assert_eq!(again, first, "and opening it again answers the seat it has");
    assert_eq!(e.germinated().len(), 1, "with no second seat for one transport");

    // The negation is the same transport, so it takes no seat of its own.
    let neg = v.scale(&num_rational::BigRational::from_integer(num_bigint::BigInt::from(-1)));
    assert_eq!(e.open_axis(neg), Some(first), "a versor and its negation are one axis");
    assert_eq!(e.germinated().len(), 1);

    // A DIFFERENT transport is a different axis, or the dedup would be
    // erasing coordinates rather than bounding a carrier.
    let other = translator_versor(1, 0);
    let second = e.open_axis(other).expect("a second seat");
    assert_ne!(second, first, "another transport opens another axis");
    assert_eq!(e.germinated().len(), 2);
    assert_eq!(e.opened_axes(), vec![first, second]);
}

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
    let germ = from_canonical(&g);
    // A hue transition on a standing lamina measures as GAUGE content
    // alongside the residual defect (the four-lane projection).
    assert_eq!(germ.lanes(), [false, true, false, false], "a recolour is 𝒢 content; temporal and holonomic are empty until standing or punctured");
    let mag = germ.magnitude();
    assert!(mag[2] == 0 && mag[1] >= 1 && mag[0] == 0, "gauge only: no spatial, no temporal extent yet");
    assert_eq!(mag[3], 0, "holonomic volume is empty until sections or torsion");
    // A residual has no group inverse (its block is antisymmetric/singular);
    // its reverse is the reverse coboundary (from_residual of b→a), not a
    // versor's — the group reverse() correctly declines.
    assert!(germ.reverse().is_none(), "no versor reverse for a coboundary");
    let reverse = from_canonical(&germinate(&b, &a).unwrap());
    assert_ne!(germ.address(), reverse.address(), "the reverse description is a distinct germ");
}

/// THE CANCELING TALLY IS NOT IDENTITY: L→bar's bidirectional hue swap cancels
/// the tally to zero — but the CHAIN keeps it a real, non-identity germ with
/// exact magnitude, and its reverse is a distinct word.
#[test]
fn a_bidirectional_swap_does_not_collapse_to_zero() {
    let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)]; // L
    let b = [(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]; // bar
    let l_to_bar = from_canonical(&germinate(&a, &b).unwrap());
    // One lamina vacates and one appears: SPATIAL content (mass moved)
    // alongside the two-link residual defect — measured, not defaulted.
    assert_eq!(l_to_bar.magnitude(), [2, 0, 0, 0], "two moved laminae, exact — not a canceled 0; the fibre lane is empty until punctured");
    assert_eq!(l_to_bar.lanes(), [true, false, false, false], "a real spatial germ");
    assert!(!l_to_bar.is_identity(), "the chain decides — a canceled tally is not identity");
    assert!(!l_to_bar.substance().is_identity(), "and its substance is real");
    let bar_to_l = from_canonical(&germinate(&b, &a).unwrap());
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
    germ.open_axis(translator_versor(0, 1)).expect("a seat");
    let both = spatial.compose(&germ).expect("the composite forms");
    assert_eq!(both.axes(), vec![FIBER_SPATIAL, FIBER_GERMINATED], "both axes present");
    assert!(same_versor(
        &both.versor().expect("S"),
        &spatial.versor().expect("S")
    ));
    assert_eq!(both.max_axis(), Some(FIBER_GERMINATED));
    assert!(!both.same_action(&spatial), "a germinated axis is a new generator, not a new digest");
    spatial.open_axis(translator_versor(2, 0)).expect("a seat");
    assert_eq!(spatial.opened_axes(), vec![FIBER_GERMINATED]);
}

/// STD1 — A WORD WHOSE LARGEST AXIS SEAT IS THE CARRIER'S LAST HAS NO SEAT
/// LEFT: a new transport is refused, never pinned onto the last seat, and
/// the word is as it was. A transport it already seats still answers.
#[test]
fn an_axis_past_the_last_seat_is_refused() {
    let mut e = Element::identity();
    let last = translator_versor(1, 1);
    e.action.germinated.push((u16::MAX, last.clone()));
    assert_eq!(e.open_axis(translator_versor(2, 3)), None, "no seat past the last");
    assert_eq!(e.germinated().len(), 1, "the word is as it was");
    assert_eq!(e.open_axis(last), Some(u16::MAX), "the seat it has still answers");
}

/// A CHAIN MEASURES THREE LANES AND NEVER CLAIMS THE FOURTH.
///
/// 𝒯 is temporal extent — the scene-state span in receipts (51.9) — and a
/// chain is a set of links with no clock in it. `measure_lanes` reporting
/// zero there is a reading of what a chain knows, and the three lanes beside
/// it are its own. This binds the two halves: the chain does not invent a
/// span, and whatever the chain holds, 𝒯 stays the mint's to supply.
#[test]
fn a_chain_measures_three_lanes_and_never_the_span() {
    // Laminae appearing and vanishing are spatial; a hue change is gauge.
    let appears = measure_lanes(&[(0, 0, 0, 1), (1, 1, 0, 1)]);
    assert!(appears[0] > 0, "laminae that appear are spatial");
    assert_eq!(appears[2], 0, "and none of them is a span");

    let recoloured = measure_lanes(&[(0, 0, 1, 2), (0, 1, 1, 2)]);
    assert!(recoloured[1] > 0, "a hue change is gauge");
    assert_eq!(recoloured[2], 0, "and none of it is a span");

    // However large or long the chain, 𝒯 does not move: there is no clock in
    // a set of links to read one from.
    let long: Vec<(i64, i64, u8, u8)> = (0..64).map(|i| (i, 0, 0, 1)).collect();
    assert_eq!(measure_lanes(&long)[2], 0, "a longer chain is not an older one");
    assert_eq!(measure_lanes(&[])[2], 0, "and an empty one is not either");
}

#[test]
fn reduction_magnitude_has_law_12_parity_with_axes_for_offplane_and_letters() {
    use crate::transducer::alphabet::{LetterId, Word};
    // An off-plane rotor on plane (2, 3)
    let w = Word::from_letters(&[(LetterId::quarter((2, 3)).expect("offplane quarter"), false)]);
    let mut e = Element::identity();
    e.action.rotor = Some(w.rotor());
    assert_eq!(e.axes(), vec![FIBER_SPATIAL], "axes reports occupied spatial stratum");
    assert_eq!(e.reduction_magnitude()[0], 1, "reduction_magnitude reports spatial=1 in exact parity with axes");

    // Letters with sigma permutation from a connection
    let id = Word::identity();
    let remainder = vec![(0, 0, 1u8, 2u8)];
    let conn = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &remainder, [0, 1, 0, 0])
        .expect("connection spells");
    assert_eq!(conn.axes(), vec![FIBER_GAUGE], "axes reports occupied gauge stratum");
    assert_eq!(conn.reduction_magnitude()[1], 1, "reduction_magnitude reports gauge=1 in exact parity with axes");
}
