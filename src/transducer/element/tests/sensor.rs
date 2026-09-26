// Law 11, 16.4 — THE SENSOR: a carried lamina is exact or refused, and a
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
/// laminae is its image and the sensor refuses to carry it (law 11), at
/// the origin and away from it alike. The quarter turn keeps the lattice
/// and carries one lamina onto one lamina, the one its centre lands in.
#[test]
fn the_pythagorean_rotor_carries_a_lamina_onto_no_lamina_and_the_quarter_turn_onto_one() {
    let tilt = LetterId::pythagorean_5((0, 1)).expect("the (3,4,5) rotor on the sensor plane");
    let v = versor_of_word(&Word::from_letters(&[(tilt, false)])).expect("a base action");
    assert_eq!(carry_lamina(&v, 0, 0, 0), None, "a tilted image is no lamina");
    assert_eq!(carry_lamina(&v, 2, 3, 0), None, "wherever the lamina stands");
    let quarter = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    let w = versor_of_word(&Word::from_letters(&[(quarter, false)])).expect("a base action");
    let image = carry_lamina(&w, 2, 3, 0).expect("an isometry carries a lamina onto a lamina");
    let [(i, j, frame)] = image.as_slice() else {
        panic!("one lamina, exactly: {image:?}");
    };
    assert_eq!(*frame, 0, "in its own frame");
    assert_eq!(
        site_of(&carry_center(&w, 2, 3, 0).expect("the centre lands")),
        Some((*i, *j, *frame)),
        "the lamina its centre lands in"
    );
}

/// A dilation by two carries a lamina onto its 2×2 block, in its own
/// frame; a translation along `e₃` carries it onto one lamina of the next
/// frame — a lamina moved to another frame is a lamina, a lamina split
/// across frames is not.
#[test]
fn a_dilation_by_two_carries_a_lamina_onto_its_block() {
    let d = cga::dilator(&q(2));
    assert_eq!(
        carry_lamina(&d, 1, 2, 0),
        Some(vec![(2, 4, 0), (2, 5, 0), (3, 4, 0), (3, 5, 0)]),
        "the square [1,2)×[2,3) doubled about the origin is the block [2,4)×[4,6)"
    );
    let along = cga::at_three::translator(&q(0), &q(0), &q(1));
    assert_eq!(carry_lamina(&along, 1, 2, 0), Some(vec![(1, 2, 1)]), "one lamina, one frame on");
}

/// A shrink carries one lamina onto no lamina — its image is a quarter of
/// a square, which no block of laminae tiles — and a 2×2 block onto one
/// lamina only AS A BODY: `versor_between` finds the shrink from the two
/// shapes and the region tower's covariance verifies it — the body's
/// reading, never the lamina's.
#[test]
fn a_shrink_carries_one_lamina_to_none_and_a_block_as_a_body_to_one() {
    use crate::transducer::blade::{carries_by_covariance, shape_of, versor_between, Tower};
    let half = cga::dilator(&qn(1, 2));
    assert_eq!(carry_lamina(&half, 0, 0, 0), None, "a fractional image is no lamina");
    assert_eq!(carry_lamina(&half, 4, 4, 0), None, "even where its centre lands in one");
    let block = shape_of(&[(0i64, 0i64, 0i64), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
    let one = shape_of(&[(0i64, 0i64, 0i64)]);
    let v = versor_between(&block, &one).expect("a shrink by one half carries the block onto the lamina");
    assert!(carries_by_covariance(&v, &block.blade, &one.blade, Tower::Region), "the region's tower is carried");
    for s in &block.blade.support {
        assert_eq!(carry_lamina_site(&v, s), None, "no lamina of the block is carried alone: {s:?}");
    }
}

/// A versor turning `e₃` into the plane carries the unit square to a
/// square standing across the frames, which is no lamina of the sensor
/// (16.4): refused, wherever the lamina stands.
#[test]
fn a_lamina_carried_across_the_frames_is_refused() {
    let turn = LetterId::quarter((0, 3)).expect("the quarter turn on the row–frame plane");
    let v = versor_of_word(&Word::from_letters(&[(turn, false)])).expect("a base action");
    assert_eq!(carry_lamina(&v, 0, 0, 0), None, "a square across the frames is no lamina");
    assert_eq!(carry_lamina(&v, 3, 1, 2), None, "wherever it stands");
}
