use super::super::*;
use super::common::*;

/// Reset, branch and superpose cite the point they act on.
#[test]
fn ops_cites_point_on_reset_branch_and_merge() {
    let q = |n: i64| crate::core::Q::from_integer(n.into());
    let p = TPoint { origin: ORIGIN, h: 30, lanes: [5, 1, 0, 2].into(), k: 9 };
    let r = ops::Reset::at(p.clone()).expect("an epoch past");
    assert_eq!((r.epoch, r.cites), (147, ops::citation(&p)));
    let (x, y) = (ops::Branch::of(p.clone(), [1; 32]), ops::Branch::of(p.clone(), [2; 32]));
    assert_ne!(x.origin, y.origin, "two branches of one point are two universes");
    let a = TPoint { origin: x.origin, h: 4, lanes: [1, 0, 0, 0].into(), k: 1 };
    let b = TPoint { origin: y.origin, h: 6, lanes: [0, 3, 0, 0].into(), k: 2 };
    let s = ops::Superposed::of(a.clone(), b.clone());
    assert_eq!((s.lanes.clone(), s.k), ([q(-1), q(3), q(0), q(0)], 1));
    let by = TPoint { h: 7, ..p.clone() };
    assert_eq!(s.merged_by(by.clone(), [ops::citation(&a), ops::citation(&b)]), Some(by.clone()));
    assert_eq!(s.merged_by(by, [ops::citation(&a), ops::citation(&a)]), None, "a merge cites both");
    assert_eq!(s.merged_by(TPoint { h: 5, ..p.clone() }, [ops::citation(&a), ops::citation(&b)]), None);
    assert_eq!(p.unpack().hh, q(2));
}
