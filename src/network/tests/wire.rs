use crate::network::wire::*;

/// A frame decodes to itself on the lane it was encoded on.
#[test]
fn frame_round_trips() {
    let f = WireFrame { lane: Lane::Gauge, body: vec![1, 2, 3, 9] };
    let (g, n) = WireFrame::decode(&f.encode().unwrap()).unwrap();
    assert_eq!(f, g);
    assert_eq!(n, 9);
    assert_eq!(g.lane.byte(), 1);
}

/// A body over the MTU budget is refused by the encoder, never truncated.
#[test]
fn encode_refuses_body_over_budget() {
    let f = WireFrame { lane: Lane::Spatial, body: vec![0u8; BODY_BUDGET + 1] };
    assert!(f.encode().is_none(), "over MTU — refused, never truncated");
}

/// 78.152 TU-7 — a frame carries its speaker's last final point behind
/// its body, its lanes holding shares of a unit, read back exactly by the
/// trailer's count; an older decoder's lane read refuses it.
#[test]
fn encode_at_carries_last_final_point() {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    let q = |n: i64, d: i64| crate::core::Q::new(n.into(), d.into());
    let lanes = crate::economy::Mu::of([q(3, 1), q(0, 1), q(9, 900), q(1, 2)]).expect("non-negative");
    let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes, k: 812 };
    let f = WireFrame { lane: Lane::Temporal, body: vec![0x53, 1, 2] };
    let bytes = f.encode_at(Some(&point)).unwrap();
    assert_eq!(Lane::of(bytes[0]), None, "a pre-78.152 lane read refuses the pointed frame");
    let (g, at, n) = WireFrame::decode_at(&bytes).unwrap();
    assert_eq!((g, at, n), (f.clone(), Some(point.clone()), 5 + 3 + point.to_bytes().len() + 2));
    let (h, none, _) = WireFrame::decode_at(&f.encode().unwrap()).unwrap();
    assert_eq!((h, none), (f, None), "a frame with no point reads with none");
}
