// 42.6 as amended, L12 — THE PRODUCT COVER: which lane an axis carries
// content on, which lanes a class occupies, and that the four covers are
// independent.

use super::common::*;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, identity_versor};
use crate::transducer::sentence::*;

/// 42.6's table, read off `Axis::lane`. Every row is asserted, so a change
/// to the assignment has to change this test and the doctrine together.
#[test]
fn the_axis_to_lane_table_is_42_6s() {
    for p in 0..6u8 {
        for axis in [Axis::Orientation(p), Axis::Unit(p), Axis::Prime5(p), Axis::Prime13(p)] {
            assert_eq!(axis.lane(), 3, "{} is holonomic on plane {p}", axis.label());
        }
    }
    assert_eq!(Axis::OffPlane.lane(), 3, "the off-plane invariants are holonomic");
    assert_eq!(Axis::Hue.lane(), 1, "the hue cycle type is gauge");
    assert_eq!(Axis::Open.lane(), 1, "an opened axis widens the fibre, which is gauge");
    assert_eq!(Axis::Placement.lane(), 0, "the placement residue is spatial");
    assert_eq!(Axis::Dilation.lane(), 0, "the dilation residue is spatial");
    // NO AXIS IS TEMPORAL. 16.16 reads 𝒯 as the extent held in receipts,
    // and `measure_lanes` writes lane 2 as zero for that reason. This is
    // the assertion that would fail if an axis were ever assigned there.
    for p in 0..6u8 {
        for axis in [
            Axis::Orientation(p),
            Axis::Unit(p),
            Axis::Prime5(p),
            Axis::Prime13(p),
            Axis::Hue,
            Axis::Placement,
            Axis::Dilation,
            Axis::OffPlane,
            Axis::Open,
        ] {
            assert_ne!(axis.lane(), 2, "{} is not temporal", axis.label());
        }
    }
}

/// A rotor with no residue occupies the holonomic lane and no other; a
/// placement occupies the spatial lane and no other; a hue cycle occupies
/// gauge and no other. The lanes are read as four flags and never summed
/// (law 5).
#[test]
fn a_class_occupies_the_lanes_its_coordinates_name() {
    let turn = ClassVector::of_word(&order4()).expect("a quarter turn is a versor");
    assert_eq!(
        turn.lanes(),
        [false, false, false, true],
        "a residue-free rotation is holonomic content alone"
    );

    let placed = ClassVector::read(
        &Word::identity(),
        element::translator_versor(0, 3),
        CycleType::Identity,
    )
    .expect("a placed identity");
    assert_eq!(
        placed.lanes(),
        [true, false, false, false],
        "a placement with no rotation is spatial content alone"
    );

    let hued = ClassVector::read(&Word::identity(), identity_versor(), CycleType::of(&[(1, 2), (2, 1)]))
        .expect("a hued identity");
    assert_eq!(
        hued.lanes(),
        [false, true, false, false],
        "a hue cycle with no rotation is gauge content alone"
    );

    // A SCREW, not a turn about a point: the translation runs ALONG the
    // rotation's axis, where `(I − R)p = t` has no solution. A translation
    // in the turning plane — `translator_versor(0, 3)` — is a base point
    // and occupies no placement coordinate, which is what this test
    // asserted before and what `occupies` now reads correctly.
    let q3 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(3));
    let z = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let screw = crate::transducer::cga::translator(&z, &z, &q3);
    let both = ClassVector::read(&order4(), screw, CycleType::Identity)
        .expect("a placed turn");
    assert_eq!(
        both.lanes(),
        [true, false, false, true],
        "a turn carried along a translation is content on TWO covers, not on one of rank two"
    );
}

/// The chart on a lane is that lane's OCCUPIED AXES, and the four key
/// spaces are disjoint: a placement on 𝒮 and a hue on 𝒢 never collide,
/// and 𝒽 keeps the rotor chart it always had.
#[test]
fn a_lane_chart_is_that_lanes_axes_and_the_four_are_disjoint() {
    let placed = ClassVector::read(
        &Word::identity(),
        element::translator_versor(0, 3),
        CycleType::Identity,
    )
    .expect("a placed identity");
    let hued = ClassVector::read(&Word::identity(), identity_versor(), CycleType::of(&[(1, 2), (2, 1)]))
        .expect("a hued identity");

    let s = lane_chart(&placed, 0).expect("the placement occupies 𝒮");
    let g = lane_chart(&hued, 1).expect("the hue occupies 𝒢");
    assert!(lane_chart(&placed, 1).is_none(), "a placement occupies nothing on 𝒢");
    assert!(lane_chart(&hued, 0).is_none(), "a hue occupies nothing on 𝒮");
    assert!(lane_chart(&placed, 2).is_none(), "nothing occupies 𝒯");
    assert!(lane_chart(&hued, 2).is_none(), "nothing occupies 𝒯");
    assert_ne!(s.key(), g.key(), "the two covers' keys are disjoint");
    assert!(
        matches!(s.0.as_slice(), [crate::transducer::stalk::ChartAxis::Count(t), ..] if *t == LANE_TAG),
        "the lane tag leads, so 𝒮's key cannot be read as a versor chart"
    );
    assert!(matches!(g.0.as_slice(), [crate::transducer::stalk::ChartAxis::Count(t), ..] if *t == LANE_TAG + 1), "and 𝒢's is its own");

    // The holonomic chart is the rotor's, unchanged: what `versor_chart`
    // always answered IS this lane's chart, so nothing filed under it moves.
    let turn = order4();
    assert_eq!(
        versor_chart(&turn).map(|c| c.key()),
        crate::transducer::sentence::versor_chart_of(&turn.rotor()).map(|c| c.key()),
        "𝒽's chart is the rotor chart and did not move"
    );
    assert!(
        versor_chart(&turn).is_some_and(|c| matches!(c.0.as_slice(), [crate::transducer::stalk::ChartAxis::Count(0), ..])),
        "and it keeps its leading 0, disjoint from every lane tag"
    );
}

/// 12.6, 3.9 — A BASE POINT IS NOT AN OBSTRUCTION, AND A SCREW IS.
///
/// `occupies(Axis::Placement)` read `translator_of(carriage) != identity`,
/// so an elliptic rotation centred anywhere but the origin published a
/// placement demand. 3.9's move for that axis is the inverse translator: it
/// moves the frame to the origin and closes no curvature, so the loop spent
/// itself on a discharge that cannot happen and never reached the rest of
/// the fan. `classify` had meanwhile solved `(I − R)p = t` for the SAME
/// carriage and filed the loop `Congruent` — two readers, one carriage,
/// opposite answers (law 12).
#[test]
fn a_turns_base_point_occupies_no_placement_and_a_screw_does() {
    use crate::transducer::sentence::Axis;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    // In the turning plane: `(I − R)p = t` solves, so the translation is
    // where the turn stands.
    let about_a_point =
        ClassVector::read(&order4(), element::translator_versor(0, 3), CycleType::Identity)
            .expect("a turn about a point");
    assert!(
        !about_a_point.occupies(Axis::Placement),
        "a base point is a choice of origin: {:?}",
        about_a_point.coordinates()
    );
    assert_eq!(
        about_a_point.lanes(),
        [false, false, false, true],
        "content on the nerve alone, not on the atlas as well"
    );

    // Along the axis: no `p` fixes it, the seam walks, and that IS an
    // obstruction with a translation word that can zero it.
    let screw = crate::transducer::cga::translator(&q(0), &q(0), &q(3));
    let walking = ClassVector::read(&order4(), screw, CycleType::Identity).expect("a screw");
    assert!(
        walking.occupies(Axis::Placement),
        "a walk occupies placement: {:?}",
        walking.coordinates()
    );
    assert_eq!(walking.lanes(), [true, false, false, true], "content on two covers");

    // And the two are DIFFERENT CLASSES now: the placement status is a
    // coordinate of the key, so a base point and a walk no longer share an
    // address while reading identically on every other axis.
    assert_ne!(
        about_a_point.class_key(),
        walking.class_key(),
        "a turn about a point and a screw are not one class"
    );
}
