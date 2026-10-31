use crate::network::wire::*;

/// A frame decodes to itself on the lane it was encoded on.
#[test]
fn frame_round_trips_on_its_lane() {
    let f = Frame { lane: Stratum::Gauge, body: vec![1, 2, 3, 9] };
    let (g, n) = Frame::decode(&f.encode().unwrap()).unwrap();
    assert_eq!(f, g);
    assert_eq!(n, 9);
    assert_eq!(g.lane.byte(), 1);
}

/// A body over the MTU budget is refused by the encoder, never truncated.
#[test]
fn encode_refuses_body_over_budget() {
    let f = Frame { lane: Stratum::Spatial, body: vec![0u8; BODY_BUDGET + 1] };
    assert!(f.encode().is_none(), "over MTU — refused, never truncated");
}

/// Frames on distinct lanes multiplex and demultiplex back in order.
#[test]
fn multiplex_demultiplexes_lanes_in_order() {
    let fs = vec![
        Frame { lane: Stratum::Spatial, body: vec![10] },
        Frame { lane: Stratum::Temporal, body: vec![20, 21] },
        Frame { lane: Stratum::Harmonic, body: vec![] },
    ];
    let round = demultiplex(&multiplex(&fs).unwrap()).unwrap();
    assert_eq!(fs, round, "all four lanes remain distinct and ordered");
}

/// 78.152 TU-7 — a frame carries its speaker's last final point behind
/// its body, read back exactly; an older decoder's lane read refuses it.
#[test]
fn encode_at_carries_last_final_point() {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes: [3, 0, 9, 1], k: 812 };
    let f = Frame { lane: Stratum::Temporal, body: vec![0x53, 1, 2] };
    let bytes = f.encode_at(Some(&point)).unwrap();
    assert_eq!(Stratum::of(bytes[0]), None, "a pre-78.152 lane read refuses the pointed frame");
    let (g, at, n) = Frame::decode_at(&bytes).unwrap();
    assert_eq!((g, at, n), (f.clone(), Some(point), 5 + 3 + TPoint::LEN));
    let (h, none, _) = Frame::decode_at(&f.encode().unwrap()).unwrap();
    assert_eq!((h, none), (f, None), "a frame with no point reads with none");
}
