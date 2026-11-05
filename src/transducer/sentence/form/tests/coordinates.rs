// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §3.6; ROADMAP track 26 S4d, stages 3 and 2d — a coordinate
// along the plane, and a frame act's split and counts, are named by the plane
// axis they are read along, and keep their bytes.

use super::super::clause::{put_act, put_pred, take_act, take_pred, Act, Coord, Order, PlaneAxis, Pred, Tie};

/// A FRAME ACT NAMES ITS PLANE AXES AND KEEPS ITS BYTES: the panes split
/// along each plane axis, and a tiling holding one count per plane axis,
/// round-trip through the clause codec; the byte the split's planar flag was
/// written with (`1`, `0`) reads back as the plane's first and second axis,
/// and the planar pair of counts as the counts in the plane's order, so no
/// clause or word address moves.
#[test]
fn frame_act_round_trips_and_reads_its_planar_bytes_by_axis() {
    let round_trip = |act: &Act| -> Option<Act> {
        let mut written = Vec::new();
        put_act(&mut written, act);
        let mut p = 0;
        let back = take_act(&written, &mut p)?;
        (p == written.len()).then_some(back)
    };
    for (byte, along) in [(1u8, PlaneAxis::First), (0, PlaneAxis::Second)] {
        let panes = Act::Panes { along, gap: true, table: 0b0110, paint: 2 };
        let same = |a: Option<Act>| matches!(a, Some(Act::Panes { along: b, gap: true, table: 0b0110, paint: 2 }) if b == along);
        assert!(same(round_trip(&panes)), "{along:?} round-trips");
        // The panes' tape: the tag, the split's byte, the gap, the table and
        // the paint.
        assert!(same(take_act(&[10, byte, 1, 0b0110, 2], &mut 0)), "byte {byte} reads as {along:?}");
    }
    let tile = Act::Tile { blocks: [1, 2], turns: vec![0, 1] };
    let same = |a: Option<Act>, blocks: [u8; 2], turns: &[u8]| matches!(a, Some(Act::Tile { blocks: b, turns: t }) if b == blocks && t == turns);
    assert!(same(round_trip(&tile), [1, 2], &[0, 1]), "the tiling round-trips");
    // The tiling's tape: the tag, the count along the plane's first axis and
    // along its second, and one turn per block.
    assert!(same(take_act(&[11, 1, 2, 0, 1], &mut 0), [1, 2], &[0, 1]), "one block along the first axis, two along the second");
    assert!(same(take_act(&[11, 2, 1, 4, 5], &mut 0), [2, 1], &[4, 5]), "two blocks along the first axis, one along the second");
}

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
