// 78.79 — THE INTERPRETER FOR INCONGRUENCE (`family/incongruence.rs`): a change
// read by its constructors — shared, accreted, extinguished, recoloured — its
// accretion read as light and its extinction as carving; readings held
// together as a manifold, contrasted, and its core judged by replay.

use crate::transducer::family::incongruence::{contrast, decided, difference, manifold, moments, recolours_to, relative, reproduces};
use num_rational::BigRational;

/// ONE CHANGE BY ITS CONSTRUCTORS: a single cell grows a ray of three to the
/// frame's edge and the cell itself is recoloured — one shared, three
/// accreted all on light, one run of three leaving by the edge, one recolour.
#[test]
fn difference_reads_constructors_and_light() {
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
fn manifold_holds_readings_core_judged_by_replay() {
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

/// 78.80 — A BODY BY ITS MOMENTS. A bar of three and an L of three are not
/// congruent but carry one volume; the bar's spread is 1+1+4 = 6 and the L's
/// 1+1+2 = 4, so their densities 9/6 and 9/4 tell them apart without a
/// boundary. A lone compartment's density is refused. Recolours decided by
/// volume: two bodies of mass 3 go to hue 2, one of mass 1 to hue 5 — leave
/// one out, the second mass-3 reading is predicted by the first and the
/// replay reproduces it; the lone mass-1 reading has no other to decide by.
#[test]
fn moments_read_density_and_decided_replays() {
    let bar = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)];
    let ell = vec![(4, 4, 1), (4, 5, 1), (5, 4, 1)];
    let (mb, ml) = (moments(&bar), moments(&ell));
    assert_eq!((mb.mass, mb.spread), (3, 6));
    assert_eq!((ml.mass, ml.spread), (3, 4));
    assert_eq!(mb.density(), Some(BigRational::new(3.into(), 2.into())));
    assert_eq!(ml.density(), Some(BigRational::new(9.into(), 4.into())));
    assert_eq!(moments(&[(2, 2, 7)]).density(), None, "a point has no spread to be dense over");
    assert_eq!(mb.hues, vec![(1, 3)]);
    let readings = vec![(3u64, vec![(1u8, 2u8)]), (3, vec![(1, 2)]), (1, vec![(1, 5)])];
    let p = decided(&readings);
    assert_eq!(p[1], Some(Some(vec![(1, 2)])));
    assert_eq!(p[2], None, "no other reading of mass 1");
    let ell_out: Vec<_> = ell.iter().map(|&(r, c, _)| (r, c, 2)).collect();
    assert!(recolours_to(&[(1, 2)], &ell, &ell_out));
    assert!(!recolours_to(&[(1, 5)], &ell, &ell_out));
    let clash = vec![(3u64, vec![(1u8, 2u8)]), (3, vec![(1, 4)]), (3, vec![(1, 2)])];
    assert_eq!(decided(&clash)[2], Some(None), "the key's others disagree: refused");
}
