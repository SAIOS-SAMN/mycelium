// D3.2 §C — LIFT: a frame held by its bodies' bands.

use super::grid;
use crate::transducer::family::transduce::*;

/// A BAND IS A MAXIMAL RUN STANDING OVER CONSECUTIVE ROW ARCS, and the bands
/// partition the frame. A `2 × 2` block of hue 1 in the corner of a `3 × 3`
/// ground is three bands — the block, the ground beside it, the ground row
/// below — and two bodies: the block, and the ground's L, whose two bands
/// meet along the row boundary `2/3` over the column arc `[2/3, 1)`.
#[test]
fn enclosed_holds_bands_and_bodies() {
    let e = Enclosed::of(&grid(&["110", "110", "000"])).expect("the frame stands");
    assert_eq!(e.enclosure, Enclosure { rows: 3, cols: 3 });
    assert_eq!(
        e.bands,
        vec![
            Band { rows: (0, 2), cols: (0, 2), hue: 1 },
            Band { rows: (0, 2), cols: (2, 3), hue: 0 },
            Band { rows: (2, 3), cols: (0, 3), hue: 0 },
        ]
    );
    assert_eq!(e.bodies, 2);
    assert_eq!(e.body_of, vec![0, 1, 1]);
}

/// TWO RUNS OF ONE HUE THAT DO NOT TOUCH ARE TWO BODIES, and a run under
/// another of one hue that overlaps it by a column arc of positive length
/// joins it; a diagonal contact is not a contact (4-adjacency).
#[test]
fn enclosed_joins_bands_by_row_boundary_contact() {
    let e = Enclosed::of(&grid(&["10", "01"])).expect("the frame stands");
    assert_eq!(e.bands.len(), 4);
    assert_eq!(e.bodies, 4, "diagonal hues of one value never meet");
    let t = Enclosed::of(&grid(&["111", "010"])).expect("the frame stands");
    let ones: Vec<usize> = t.bands.iter().zip(&t.body_of).filter(|(b, _)| b.hue == 1).map(|(_, &n)| n).collect();
    assert_eq!(ones.len(), 2);
    assert_eq!(ones[0], ones[1], "the T's stem meets its bar");
}

/// A frame with no row or no column has no enclosure, and one whose rows
/// disagree is not the extent it claims.
#[test]
fn enclosed_refuses_empty_and_ragged() {
    assert_eq!(Enclosed::of(&Vec::new()), Err(TransductionRefusal::Empty));
    assert_eq!(Enclosed::of(&vec![Vec::new()]), Err(TransductionRefusal::Empty));
    assert_eq!(Enclosed::of(&grid(&["11", "1"])), Err(TransductionRefusal::Ragged));
}

/// THE CONTENT IS LOSSLESS (O3): two frames that differ at one site, or in
/// one period with the same sites, write different content.
#[test]
fn enclosed_content_separates_frames() {
    let key = |rows: &[&str]| {
        let mut out = Vec::new();
        Enclosed::of(&grid(rows)).expect("the frame stands").put_content(&mut out);
        out
    };
    assert_ne!(key(&["10", "00"]), key(&["01", "00"]));
    assert_ne!(key(&["1"]), key(&["11"]));
    assert_ne!(key(&["1", "1"]), key(&["11"]));
    assert_eq!(key(&["12", "30"]), key(&["12", "30"]));
}
