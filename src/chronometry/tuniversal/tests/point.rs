use super::super::*;
use super::common::*;

/// A point prints in one lowercase writing and parses back exactly, and its wire form takes back whole.
#[test]
fn tpoint_print_round_trips() {
    let p = TPoint { origin: ORIGIN, h: 40_961, lanes: [3, u64::MAX, 0, 17], k: 812 };
    let text = p.print();
    assert_eq!(
        text,
        format!("{}.00000092/000000000000a001:0000000000000003:ffffffffffffffff:0000000000000000:0000000000000011:000000000000032c", "6a".repeat(32))
    );
    assert_eq!(TPoint::parse(&text), Some(p));
    assert_eq!(TPoint::parse(&text.to_uppercase()), None, "one writing: lowercase");
    assert_eq!(TPoint::parse(&format!("{text}:0000000000000000")), None, "a seventh field is no point");
    assert_eq!(TPoint::parse(&text.replacen("/000000000000a001", "/a001", 1)), None, "a short field is no point");
    let mut wire = Vec::new();
    p.put(&mut wire);
    let mut at = 0;
    assert_eq!((TPoint::take(&wire, &mut at), at), (Some(p), TPoint::LEN));
}
