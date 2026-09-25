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

/// RM1 — A SIMPLEX CLOSED UP TO ITS CLASS MINTS ITS TORSE REWRITE: `before`
/// the simplex's words, `after` the simplex's own key, the payload σ and ℓ,
/// priced off its own record (ℓ − 1 on the shell lane), and the remainder
/// (κ, g, the complex's chart) on the rewrite's record. The court indexes it
/// at chart ‖ κ; snapshotted and restored, the record comes back with the
/// element and the index is rebuilt from it.
#[test]
fn a_courts_torse_carries_its_remainder_through_a_restart() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::address::Address;
    let g = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("the sensor plane"), false)]).rotor();
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &g,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class");
    let mut k = Kernel::new();
    let inv = crate::transducer::stalk::Invariant::default();
    let words = vec![Address::from(vec![1u8; 8]), Address::from(vec![2u8; 8])];
    let simplex = b"the simplex's own key".to_vec();
    let rw = k
        .mint_torse_at(&inv, &simplex, (Box::new(kappa.clone()), Box::new(g.clone()), 4), &words)
        .expect("the Torse rewrite is minted and offered");
    let key = crate::transducer::element::Remainder {
        class: Box::new(kappa),
        ray: Box::new(g),
        at: vec![inv.key().into_vec()],
    }
    .index_key();
    assert_eq!(k.remainder_at(&key), Some(&rw), "indexed at the complex's chart ‖ κ");
    let (r, _) = k.remainders();
    assert_eq!((r.refused, r.unpriced, r.unminted, r.unoffered), (0, 0, 0, 0), "{}", r.line());
    // A period of one prices nothing: counted by its own name, never folded.
    assert!(k.mint_torse_at(&inv, b"another", (Box::new(crate::transducer::sentence::ClassVector::of_rotor(
        &crate::transducer::alphabet::Rotor::identity(),
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    ).expect("the identity's class")), Box::new(crate::transducer::alphabet::Rotor::identity()), 1), &words).is_none());
    assert_eq!(k.remainders().0.unpriced, 1, "unpriced by name");
    let snap = k.snapshot();
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap), "clean restore");
    assert_eq!(booted.remainder_at(&key), Some(&rw), "the index is rebuilt from the record");
    let word = booted.named_word(rw.as_bytes()).expect("the rewrite stands");
    let back: Vec<_> = word.remainders().collect();
    assert_eq!(back.len(), 1, "its record carries the remainder");
    let claimed = crate::transducer::rewrite::Rewrite::from_element(word).expect("a rewrite").claimed;
    assert_eq!(claimed[crate::transducer::rewrite::LANE_SHELL], 3, "priced off its own record: ℓ − 1");
}
