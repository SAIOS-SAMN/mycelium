// 78.84 — THE BRIDGE (`family/bridge.rs`): a pair's motions, as the
// perception carries them, put through the universal law register.

use super::common::grid;
use crate::transducer::family::bridge::bridge_pair;

/// ONE MOTION, ONE TRANSITION. Two bars slide one site right: the two
/// carriages that are that slide are ONE transition holding both bodies, and
/// the bars' cross carriages are each a transition of its own. Every one is a
/// rigid motion recolouring nothing — conformal, its momentum carried, every
/// pair's product kept, its height advanced, no hue relation — and admitted.
#[test]
fn bridge_pair_admits_shared_slide_as_one_transition() {
    let x = grid(&["220000", "000000", "000220"]);
    let y = grid(&["022000", "000000", "000022"]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let b = bridge_pair(&x, &y, &mut budget);
    assert_eq!((b.bodies, b.carried, b.uncarried), (2, 2, 0));
    assert_eq!(b.several, 1, "the slide carries both bars at once");
    assert_eq!(b.admitted, b.transitions, "every rigid carriage is admitted");
    assert!(b.by_law.iter().all(|&n| n == 0), "no law breached");
}

/// A CHANGE THAT IS NO MOTION ASKS THE LAWS NOTHING. A bar grows a cell: no
/// output body on its chart, no carriage, no transition — the bridge's count
/// of what the register, as written, cannot be asked about.
#[test]
fn bridge_pair_reads_growth_as_no_transition() {
    let x = grid(&["220000"]);
    let y = grid(&["222000"]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let b = bridge_pair(&x, &y, &mut budget);
    assert_eq!((b.bodies, b.carried, b.uncarried, b.transitions), (1, 0, 1, 0));
}
