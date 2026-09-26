// 78.37(4), RM1 — THE MIND'S REMAINDERS, READ: the contemplation's Congruent
// fills by ℓ, each at the deepest chart its thoughts share.

use super::super::*;

/// Thoughts on charts: a 90° turn filed Congruent between two thoughts
/// on `c` and between two on `d`, a 180° turn between two on `d`, and a
/// 90° turn between two that share no chart. ℓ 2 once, ℓ 4 three times;
/// two classes, the 90° turn's across two charts; one unplaced; no re-fill.
#[test]
fn the_minds_remainders_are_read_at_the_chart_their_thoughts_share() {
    use crate::transducer::alphabet::{LetterId, Word};
    let turn = |n: usize| {
        let q = LetterId::turn_90((0, 1)).expect("the sensor plane");
        Word::from_letters(&vec![(q, false); n]).rotor()
    };
    let mut c = Contemplation::new();
    let on = |c: &mut Contemplation, i: u8, charts: &[&[u8]]| {
        let mut cover = Keyed::new();
        for ch in charts {
            cover.insert(ch, ());
        }
        c.bubbles.insert(vec![i; 4], Bubble { cover, ..Bubble::default() });
        vec![i; 4]
    };
    let (a, b) = (on(&mut c, 1, &[b"", b"c"]), on(&mut c, 2, &[b"", b"c"]));
    let (x, y, z) = (on(&mut c, 3, &[b"", b"d"]), on(&mut c, 4, &[b"", b"d"]), on(&mut c, 5, &[b"", b"d"]));
    let (u, v) = (on(&mut c, 6, &[b"e"]), on(&mut c, 7, &[b"f"]));
    c.complex.file_congruent(&a, &b, turn(1), 4);
    c.complex.file_congruent(&x, &y, turn(1), 4);
    c.complex.file_congruent(&y, &z, turn(2), 2);
    c.complex.file_congruent(&u, &v, turn(1), 4);
    let r = c.meet_graph().remainders;
    assert_eq!(r.by_ell, vec![(2, 1), (4, 3)], "{}", r.line());
    assert_eq!((r.classes, r.across, r.unplaced, r.refilled), (2, 1, 1, 0), "the shared chart, not the root alone: {}", r.line());
}
