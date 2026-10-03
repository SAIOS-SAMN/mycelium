// 78.79 — THE INTERPRETER FOR INCONGRUENCE (`family/incongruence.rs`): a change
// read by its constructors — shared, accreted, extinguished, recoloured — its
// accretion read as light and its extinction as carving; readings held
// together as a manifold, contrasted, and its core judged by replay.

use crate::transducer::family::incongruence::{contrast, difference, manifold, relative, reproduces};

/// ONE CHANGE BY ITS CONSTRUCTORS: a single cell grows a ray of three to the
/// frame's edge and the cell itself is recoloured — one shared, three
/// accreted all on light, one run of three leaving by the edge, one recolour.
#[test]
fn a_change_is_read_by_its_constructors_and_its_light() {
    let frame = vec![vec![0u8; 4]];
    let input = vec![(0, 0, 1)];
    let output = vec![(0, 0, 2), (0, 1, 2), (0, 2, 2), (0, 3, 2)];
    let d = difference(&input, &output, &frame);
    assert_eq!(d.shared, 1);
    assert_eq!(d.accreted.len(), 3);
    assert_eq!(d.on_light, 3, "every accreted constructor on the ray out of the body");
    assert_eq!(d.runs, vec![(3, true)], "one run of three, leaving by the frame's edge: luminal");
    assert_eq!(d.recoloured, vec![(0, 0, 1, 2)]);
    assert!(!d.kinematic());
}

/// THE MANIFOLD HOLDS ITS READINGS AND IS JUDGED BY THE PLANE. Two bodies,
/// each growing one cell to its right: their relative differences are one,
/// the core is that cell, nothing is an edge, and the core replayed on each
/// input reproduces its output. A third reading growing left instead narrows
/// the core and sharpens an edge — the contrast held, not dissolved.
#[test]
fn a_manifold_holds_its_readings_and_its_core_is_judged_by_replay() {
    let frame = vec![vec![0u8; 8]; 3];
    let a_in = vec![(1, 1, 3)];
    let a_out = vec![(1, 1, 3), (1, 2, 3)];
    let b_in = vec![(0, 5, 3)];
    let b_out = vec![(0, 5, 3), (0, 6, 3)];
    let ra = relative(&a_in, &difference(&a_in, &a_out, &frame)).expect("a reading");
    let rb = relative(&b_in, &difference(&b_in, &b_out, &frame)).expect("a reading");
    assert_eq!(ra, rb, "one rule on two bodies reads as one relative difference");
    let m = manifold(&[(ra.clone(), Vec::new()), (rb.clone(), Vec::new())]);
    assert_eq!((m.core.len(), m.edges), (1, 0));
    assert!(m.moved[1].recognized, "the second reading is the first, recognized");
    assert_eq!(reproduces(&m, &a_in, &a_out), Some(true));
    assert_eq!(reproduces(&m, &b_in, &b_out), Some(true));
    let c_in = vec![(2, 4, 3)];
    let c_out = vec![(2, 4, 3), (2, 3, 3)];
    let rc = relative(&c_in, &difference(&c_in, &c_out, &frame)).expect("a reading");
    let m = manifold(&[(ra.clone(), Vec::new()), (rc.clone(), Vec::new())]);
    assert!(m.core.is_empty() && m.edges == 2, "right against left: no core, two edges");
    assert!(m.moved[1].sharpened, "the contrast sharpened the subject's edges");
    let k = contrast(&ra, &rc);
    assert_eq!((k.shared, k.only_first, k.only_second), (0, 1, 1), "held together, each whole");
}
