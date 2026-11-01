// MU-DERIVATIONS D4.3 — a grid on the wire is bounded by the bytes its tape
// carries, never by a chosen width: a wide grid within its bytes reads back,
// and a shape that is no rectangle is refused on both sides.

use crate::anastome::dialogue::carrier::{push_grid, read_grid};

/// A GRID WIDER THAN THE FORMER CAP READS BACK: 1 × 2000 within its bytes.
#[test]
fn read_grid_admits_wide_grid_within_bytes() {
    let g = vec![vec![3u8; 2000]];
    let mut tape = Vec::new();
    push_grid(&mut tape, &g).expect("a rectangle");
    let mut p = 0;
    assert_eq!(read_grid(&tape, &mut p), Some(g));
    assert_eq!(p, tape.len(), "read whole");
}

/// ROWS WITHOUT COLUMNS ARE NO GRID: `h = 65 535, w = 0` from a four-byte
/// header is refused before any row is made; a grid of more cells than the
/// tape carries is refused; the empty `0 × 0` reads back.
#[test]
fn read_grid_refuses_rows_without_columns() {
    let mut header = u16::MAX.to_le_bytes().to_vec();
    header.extend_from_slice(&0u16.to_le_bytes());
    assert_eq!(read_grid(&header, &mut 0), None);
    let mut short = 2u16.to_le_bytes().to_vec();
    short.extend_from_slice(&2u16.to_le_bytes());
    short.extend_from_slice(&[1, 2, 3]);
    assert_eq!(read_grid(&short, &mut 0), None, "four cells, three bytes");
    let mut empty = Vec::new();
    push_grid(&mut empty, &[]).expect("the empty grid");
    assert_eq!(read_grid(&empty, &mut 0), Some(Vec::new()));
}

/// A RAGGED GRID IS REFUSED, AS IS ONE WITH ROWS BUT NO COLUMNS: neither
/// would read back as itself.
#[test]
fn push_grid_refuses_ragged_rows() {
    assert_eq!(push_grid(&mut Vec::new(), &[vec![1, 2], vec![3]]), None);
    assert_eq!(push_grid(&mut Vec::new(), &[vec![], vec![]]), None);
}
