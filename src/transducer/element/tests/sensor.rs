// Law 11, 16.4 — THE SENSOR: a carried compartment is exact or refused, and a
// body a versor shrinks is carried as a body.

use super::common::q;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::cga;
use crate::transducer::element::*;

fn qn(n: i64, d: i64) -> crate::core::Q {
    crate::core::Q::new(num_bigint::BigInt::from(n), num_bigint::BigInt::from(d))
}

/// The (3,4,5) rotor on the sensor plane turns the unit square off the
/// lattice: its corners land between the base's vertices, so no block of
/// compartments is its image and the sensor refuses to carry it (law 11), at
/// the origin and away from it alike. The 90° turn keeps the lattice
/// and carries one compartment onto one compartment, the one its centre lands in.
#[test]
fn carry_compartment_refuses_pythagorean_tilt_keeps_turn_90() {
    let tilt = LetterId::pythagorean_5((0, 1)).expect("the (3,4,5) rotor on the sensor plane");
    let v = versor_of_word(&Word::from_letters(&[(tilt, false)])).expect("a base action");
    assert_eq!(carry_compartment(&v, 0, 0, 0), None, "a tilted image is no compartment");
    assert_eq!(carry_compartment(&v, 2, 3, 0), None, "wherever the compartment stands");
    let turn_90 = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    let w = versor_of_word(&Word::from_letters(&[(turn_90, false)])).expect("a base action");
    let image = carry_compartment(&w, 2, 3, 0).expect("an isometry carries a compartment onto a compartment");
    let [(i, j, frame)] = image.as_slice() else {
        panic!("one compartment, exactly: {image:?}");
    };
    assert_eq!(*frame, 0, "in its own frame");
    assert_eq!(
        site_of(&carry_center(&w, 2, 3, 0).expect("the centre lands")),
        Some((*i, *j, *frame)),
        "the compartment its centre lands in"
    );
}

/// A dilation by two carries a compartment onto its 2×2 block, in its own
/// frame; a translation along `e₃` carries it onto one compartment of the next
/// frame — a compartment moved to another frame is a compartment, a compartment split
/// across frames is not.
#[test]
fn carry_compartment_dilates_onto_block() {
    let d = cga::dilator(&q(2));
    assert_eq!(
        carry_compartment(&d, 1, 2, 0),
        Some(vec![(2, 4, 0), (2, 5, 0), (3, 4, 0), (3, 5, 0)]),
        "the square [1,2)×[2,3) doubled about the origin is the block [2,4)×[4,6)"
    );
    let along = cga::at_three::translator(&q(0), &q(0), &q(1));
    assert_eq!(carry_compartment(&along, 1, 2, 0), Some(vec![(1, 2, 1)]), "one compartment, one frame on");
}

/// A shrink carries one compartment onto no compartment — its image is a quarter of
/// a square, which no block of compartments tiles — and a 2×2 block onto one
/// compartment only AS A BODY: `versor_between` finds the shrink from the two
/// shapes and the region tower's covariance verifies it — the body's
/// reading, never the compartment's.
#[test]
fn carry_compartment_refuses_shrink_except_as_body() {
    use crate::transducer::blade::{shape_of, versor_between};
    let half = cga::dilator(&qn(1, 2));
    assert_eq!(carry_compartment(&half, 0, 0, 0), None, "a fractional image is no compartment");
    assert_eq!(carry_compartment(&half, 4, 4, 0), None, "even where its centre lands in one");
    let block = shape_of(&[(0i64, 0i64, 0i64), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
    let one = shape_of(&[(0i64, 0i64, 0i64)]);
    let v = versor_between(&block, &one, &mut super::common::budget()).expect("a shrink by one half carries the block onto the compartment");
    for s in &block.blade.support {
        assert_eq!(carry_compartment_site(&v, s), None, "no compartment of the block is carried alone: {s:?}");
    }
}

/// A versor turning `e₃` into the plane carries the unit square to a
/// square standing across the frames, which is no compartment of the sensor
/// (16.4): refused, wherever the compartment stands.
#[test]
fn carry_compartment_refuses_across_frames() {
    let turn = LetterId::turn_90((0, 3)).expect("the 90° turn on the row–frame plane");
    let v = versor_of_word(&Word::from_letters(&[(turn, false)])).expect("a base action");
    assert_eq!(carry_compartment(&v, 0, 0, 0), None, "a square across the frames is no compartment");
    assert_eq!(carry_compartment(&v, 3, 1, 2), None, "wherever it stands");
}
