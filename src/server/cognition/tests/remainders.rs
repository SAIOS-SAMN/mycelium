// 78.37(4), RM1 — THE COURT'S REMAINDERS, READ: its complexes' Congruent
// fills by ℓ, a chart per complex, a class standing in two charts' complexes
// counted across, and no re-fill.

use super::super::*;

/// A quarter turn filed Congruent in the complexes of two charts and a half
/// turn in one: ℓ 2 once and ℓ 4 twice, two classes, the quarter's standing
/// across two charts; nothing re-filled; no basis word carries a torsion.
#[test]
fn the_courts_remainders_are_read_over_its_complexes() {
    use crate::transducer::alphabet::{LetterId, Word};
    let turn = |n: usize| {
        let q = LetterId::quarter((0, 1)).expect("the sensor plane");
        Word::from_letters(&vec![(q, false); n]).rotor()
    };
    let mut k = Kernel::new();
    let (a, b) = (vec![1u8; 4], vec![2u8; 4]);
    let mut c = crate::transducer::cech::Complex::new();
    c.file_congruent(&a, &b, turn(1), 4);
    let mut d = crate::transducer::cech::Complex::new();
    d.file_congruent(&a, &b, turn(1), 4);
    d.file_congruent(&a, [3u8; 4], turn(2), 2);
    k.complexes.insert(b"chart c".as_slice(), c);
    k.complexes.insert(b"chart d".as_slice(), d);
    let (r, torsion_words) = k.remainders();
    assert_eq!(r.by_ell, vec![(2, 1), (4, 2)], "by ℓ, ascending: {}", r.line());
    assert_eq!((r.classes, r.across, r.unplaced, r.refilled), (2, 1, 0, 0), "{}", r.line());
    assert_eq!(torsion_words, 0);
}
