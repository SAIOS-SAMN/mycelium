// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §3.6; ROADMAP track 26 S4d, stage 3 — a coordinate along the
// plane is named by the plane axis it is read along, and keeps its byte.

use super::super::clause::{put_pred, take_pred, Coord, Order, PlaneAxis, Pred, Tie};

/// A COORDINATE ALONG THE PLANE IS NAMED BY ITS ROLE AND KEEPS ITS BYTE:
/// every coordinate along the plane round-trips through the clause codec,
/// and a rank written with the byte its planar name was written with (`1`
/// to `6`) reads back as the extent, the least and the greatest coordinate
/// along the plane's first and second axis, so no clause or word address
/// moves.
#[test]
fn coord_round_trips_and_reads_its_planar_byte_by_role() {
    use PlaneAxis::{First, Second};
    let along = [
        Coord::Extent(First),
        Coord::Extent(Second),
        Coord::Least(First),
        Coord::Least(Second),
        Coord::Greatest(First),
        Coord::Greatest(Second),
    ];
    for (byte, coord) in (1u8..).zip(along) {
        let rank = Pred::extremum(coord, Order::Greatest, Tie::Hold);
        let mut written = Vec::new();
        put_pred(&mut written, &rank);
        let mut p = 0;
        assert_eq!((take_pred(&written, &mut p, 0), p), (Some(rank.clone()), written.len()), "{coord:?} round-trips");
        // A rank's tape: its tag, the coordinate's byte, the order
        // (`Greatest`), the place (0, four bytes) and the tie (`Hold`).
        let mut planar = vec![4, byte, 0];
        planar.extend_from_slice(&0u32.to_le_bytes());
        planar.push(0);
        assert_eq!(take_pred(&planar, &mut 0, 0), Some(rank), "byte {byte} reads as {coord:?}");
    }
}
