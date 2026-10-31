use super::super::*;
use super::common::*;

/// Reset, branch and superpose cite the point they act on.
#[test]
fn ops_reset_branch_superpose_cite_point() {
    let p = TPoint { origin: ORIGIN, h: 30, lanes: [5, 1, 0, 2], k: 9 };
    let r = ops::Reset::at(p).expect("an epoch past");
    assert_eq!((r.epoch, r.cites), (147, ops::citation(&p)));
    let (x, y) = (ops::Branch::of(p, [1; 32]), ops::Branch::of(p, [2; 32]));
    assert_ne!(x.origin, y.origin, "two branches of one point are two universes");
    let a = TPoint { origin: x.origin, h: 4, lanes: [1, 0, 0, 0], k: 1 };
    let b = TPoint { origin: y.origin, h: 6, lanes: [0, 3, 0, 0], k: 2 };
    let s = ops::Superposed::of(a, b);
    assert_eq!(s.residual, [-1, 3, 0, 0, 1]);
    let by = TPoint { h: 7, ..p };
    assert_eq!(s.merged_by(by, [ops::citation(&a), ops::citation(&b)]), Some(by));
    assert_eq!(s.merged_by(by, [ops::citation(&a), ops::citation(&a)]), None, "a merge cites both");
    assert_eq!(s.merged_by(TPoint { h: 5, ..p }, [ops::citation(&a), ops::citation(&b)]), None);
    assert_eq!(p.unpack().hh, 2);
}
