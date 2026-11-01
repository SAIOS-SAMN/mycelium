use super::super::*;
use super::common::*;
use crate::core::Q;
use crate::economy::Mu;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// A point prints in one lowercase writing and parses back exactly, and its wire form takes back whole.
#[test]
fn tpoint_print_round_trips() {
    let p = TPoint { origin: ORIGIN, h: 40_961, lanes: [3, u64::MAX, 0, 17].into(), k: 812 };
    let text = p.print();
    assert_eq!(text, format!("{}.00000092/000000000000a001:3/1:ffffffffffffffff/1:0/1:11/1:000000000000032c", "6a".repeat(32)));
    assert_eq!(TPoint::parse(&text), Some(p.clone()));
    assert_eq!(TPoint::parse(&text.to_uppercase()), None, "one writing: lowercase");
    assert_eq!(TPoint::parse(&format!("{text}:0000000000000000")), None, "a seventh field is no point");
    assert_eq!(TPoint::parse(&text.replacen("/000000000000a001", "/a001", 1)), None, "a short field is no point");
    let mut wire = Vec::new();
    p.put(&mut wire);
    let mut at = 0;
    assert_eq!((TPoint::take(&wire, &mut at), at), (Some(p), wire.len()));
}

/// A lane holding a share of a unit prints as its exact rational in lowest
/// terms, `7/30` as `7/1e` and `1/900` as `1/384`; a writing not reduced, with
/// a leading zero or a zero denominator is no point; the wire and the TU-7
/// trailer carry it whole.
#[test]
fn tpoint_print_round_trips_rational_lanes() {
    let lanes = Mu::of([q(7, 30), q(0, 1), q(1, 900), q(2, 1)]).expect("non-negative");
    let p = TPoint { origin: ORIGIN, h: 10, lanes, k: 3 };
    assert_eq!(p.local(), "000000000000000a:7/1e:0/1:1/384:2/1:0000000000000003");
    assert_eq!(TPoint::parse(&p.print()), Some(p.clone()));
    for bad in ["e/3c", "07/1e", "7/0", "7/01e", "-7/1e", "7"] {
        assert_eq!(TPoint::parse(&p.print().replacen("7/1e", bad, 1)), None, "{bad} is not a lane's one writing");
    }
    let mut body = b"act".to_vec();
    p.put_trailer(&mut body).expect("a point under its u16 count");
    assert_eq!(TPoint::take_trailer(&body), Some((p, 3)));
}

/// TEXT ORDER IS NOT NUMERIC ORDER PAST `h`: `1/2` and `1/3` write `1/2` and
/// `1/3`, ordered as text the other way round from their values; the writings
/// compare as the points do.
#[test]
fn compare_writings_agrees_with_point_order() {
    let at = |s: Q| TPoint { origin: ORIGIN, h: 5, lanes: Mu::of([s, q(0, 1), q(0, 1), q(0, 1)]).expect("non-negative"), k: 1 };
    let (half, third, tenth) = (at(q(1, 2)), at(q(1, 3)), at(q(1, 10)));
    assert!(third < half && tenth < third, "the point's order is the lane's value");
    assert!(half.print() < third.print(), "as text, 1/2 sorts before 1/3");
    assert_eq!(TPoint::compare_writings(&half.print(), &third.print()), Some(std::cmp::Ordering::Greater));
    assert_eq!(TPoint::compare_writings(&tenth.print(), &third.print()), Some(std::cmp::Ordering::Less));
    let later = TPoint { h: 6, ..tenth.clone() };
    assert!(tenth.print() < later.print(), "by h, text order is numeric order");
    assert_eq!(TPoint::compare_writings(&half.print(), "no point"), None);
}
