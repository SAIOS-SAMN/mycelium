// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Nesting as a fibration of levels where the family is invariant (ROADMAP
//! track 26 S7 stage 4; MU-SENSING §3.6–§3.7): an axis the family's nesting
//! takes apart is a level, its base the coarse phase and its fibre over each
//! base point the block's own field; an axis kept whole is no level.

use super::super::*;
use super::fixtures::{degrees, sample_of};
use crate::transducer::family::transduce::{nesting_of, Factoring, Member, Taken};

/// The symbols of each body a lift holds, in the lift's order.
fn symbols(l: &Lifted) -> Vec<Vec<u8>> {
    l.bodies.iter().map(|b| b.symbol.clone()).collect()
}

/// AN AXIS TAKEN APART LIFTS AS A FIBRATION OF LEVELS. The family's axis of
/// extent 8 is taken apart as three blocks of two sites, each but the last
/// followed by its separator (`3·3 − 1`), on every pair. Each member is
/// lifted with one level on that axis: the base is the coarse phase, 0°,
/// 120° and 240° on the axis's own plane, and over each base point the
/// fibre is the block's own field, two sites lifted with their own atlas,
/// the separator left out. The input's fibres hold `1 2`, `3 4`, `5 6`, and
/// the output's `2 1`, `4 3`, `6 5`.
#[test]
fn axis_taken_apart_lifts_as_a_fibration_of_levels() {
    let (x, y) = (sample_of(&[8], "12934956"), sample_of(&[8], "21943965"));
    let nesting = nesting_of(&[(x.clone(), y.clone())]);
    assert!(matches!(nesting[0].taken, Taken::Apart { .. }), "the family takes the axis apart");
    for (sample, member, blocks) in [(&x, Member::Input, ["12", "34", "56"]), (&y, Member::Output, ["21", "43", "65"])] {
        let fibred = Fibred::of(sample, &nesting, 0, member).expect("lifted as a fibration");
        assert_eq!(fibred.levels.len(), 1, "one level, on the one axis taken apart");
        let level = &fibred.levels[0];
        assert_eq!((level.axis, level.factoring), (0, Factoring::separated(8, 3, 3).expect("3·3 − 1")));
        let base: Vec<_> = level.base.iter().map(|t| (t.degrees().clone(), t.winding())).collect();
        assert_eq!(base, [(degrees(0, 1), 0.into()), (degrees(120, 1), 0.into()), (degrees(240, 1), 0.into())], "the coarse phase is the base");
        assert!(level.base.iter().all(|t| t.plane() == &fibred.whole.planes[0]), "on the axis's own plane");
        assert_eq!(level.fibres.len(), 3, "one fibre over each base point");
        for (fibre, block) in level.fibres.iter().zip(blocks) {
            let own = Lifted::of(&sample_of(&[2], block)).expect("the block lifts");
            assert_eq!(symbols(fibre), symbols(&own), "the fibre holds the block's bodies");
            assert_eq!(fibre.bodies, own.bodies, "lifted with its own atlas, at the block's own extent");
        }
    }
}

/// AN AXIS KEPT WHOLE IS NO LEVEL. A family that one pair refuses every
/// factoring of keeps the axis one phase: the sample lifts with no level,
/// its whole lift the plain lift.
#[test]
fn axis_kept_whole_is_no_level() {
    let (x, y) = (sample_of(&[4], "1234"), sample_of(&[6], "123456"));
    let nesting = nesting_of(&[(x.clone(), y)]);
    assert!(nesting.iter().all(|n| !matches!(n.taken, Taken::Apart { .. })), "nothing is taken apart");
    let fibred = Fibred::of(&x, &nesting, 0, Member::Input).expect("lifted");
    assert!(fibred.levels.is_empty(), "no level: the axis is one phase");
    assert_eq!(fibred.whole.bodies, Lifted::of(&x).expect("lifts").bodies);
}
