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

/// RM1 (c) — THE COURT RE-FILLS ONLY BY REPLAYING A VOICE'S ACT: its complex
/// holds a Congruent simplex and the Torse rewrite minted over it; a voice's
/// Settled act cites the rewrite, the class with its ray and chain, and a
/// word spelling the ray. Replayed, the simplex reads Filled through the
/// named door (refilled 1), the rewrite's remainder is answered at the act's
/// stamp and its pressure reads zero. A second replay finds nothing open.
#[test]
fn the_court_replays_a_voices_refill_and_its_congruent_simplex_fills() {
    use crate::chronometry::{Cite, KStepReceipt, Place, Stamp};
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::address::Address;
    let word = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("the sensor plane"), false)]);
    let g = word.rotor();
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &g,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class");
    let mut k = Kernel::new();
    let inv = crate::transducer::stalk::Invariant::counts(&[1, 2]);
    let (a, b) = (vec![1u8; 8], vec![2u8; 8]);
    let mut cx = crate::transducer::cech::Complex::new();
    cx.file_congruent(&a, &b, g.clone(), 4);
    let simplex = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(vec![a.as_slice(), b.as_slice()]))
        .as_bytes()
        .to_vec();
    k.complexes.insert(inv.key().as_bytes(), cx);
    let words = vec![Address::from(a.clone()), Address::from(b.clone())];
    let rw = k.mint_torse_at(&inv, &simplex, (Box::new(kappa.clone()), Box::new(g.clone()), 4), &words).expect("minted");
    let rewrite = k.named_word(rw.as_bytes()).expect("the rewrite stands").clone();
    let rw_at = *k.admitted_at.get(rw.as_bytes()).expect("admitted by the court's act");
    // The spelling word: the quarter spelled, acting at the complex's chart.
    let spelling = crate::transducer::sentence::callus_form(Derivation::spelled(word), std::slice::from_ref(&inv)).expect("a form");
    k.basis_insert(spelling.clone(), [9; 32]);
    let sp_at = Stamp::new(3, [9; 32], Place { k: 1, i: 0 });
    k.file_admitted(&spelling.address(), sp_at);
    let chart = vec![inv.key().into_vec()];
    let receipt = KStepReceipt {
        stamp: Stamp::new(5, [7; 32], Place { k: 2, i: 1 }),
        kind: crate::chronometry::ActKind::Settled,
        mu: [0, 0, 0, 3],
        prev: None,
        parents: Vec::new(),
        cites: vec![
            Cite::word(&rewrite, rw_at),
            Cite::Class { class: Box::new(kappa.clone()), rotor: Box::new(g.clone()), cover: Some(chart.clone()) },
            Cite::word(&spelling, sp_at),
        ],
        operator_hash: [0; 32],
        chain_hash: [0; 32],
    };
    k.hear_refill(&receipt);
    assert_eq!(k.refills.applied, 1, "{:?}", k.refills);
    let cx = k.complexes.get(inv.key().as_bytes()).expect("the complex");
    let s = cx.simplex(&simplex).expect("the simplex");
    assert!(matches!(s.fill, crate::transducer::cech::Fill::Filled(..)), "the Congruent simplex reads Filled");
    assert_eq!(cx.refilled(), 1, "through the one fill writer, counted");
    let answered = k.named_word(rw.as_bytes()).expect("the rewrite").clone();
    let cycle = answered.torsion().iter().find(|t| t.remainder.is_some()).expect("its remainder").clone();
    assert_eq!(cycle.remainder.as_ref().and_then(|r| r.answered), Some(receipt.stamp), "answered at the act's stamp");
    assert_eq!(cycle.pressure(), Some([0; 4]), "its pressure reads zero");
    // R1 — the re-fill carries the Congruent's own carriage, κ's.
    assert!(
        matches!(&s.fill, crate::transducer::cech::Fill::Filled(_, c) if crate::transducer::element::same_versor(c, &kappa.carriage)),
        "the placement is not lost"
    );
    k.hear_refill(&receipt);
    assert_eq!((k.refills.applied, k.refills.unopen), (1, 1), "nothing open the second time");

    // R2 — a rewrite whose `after` is no court simplex (the complex of its
    // chart holds none): the act replays whole, the record is answered, and
    // only the write is skipped, counted `unsited`.
    let inv2 = crate::transducer::stalk::Invariant::counts(&[3, 4]);
    let rw2 = k.mint_torse_at(&inv2, b"no simplex here", (Box::new(kappa.clone()), Box::new(g.clone()), 4), &words).expect("minted");
    let rewrite2 = k.named_word(rw2.as_bytes()).expect("stands").clone();
    let rw2_at = *k.admitted_at.get(rw2.as_bytes()).expect("admitted");
    let receipt2 = KStepReceipt {
        stamp: Stamp::new(6, [7; 32], Place { k: 2, i: 2 }),
        cites: vec![
            Cite::word(&rewrite2, rw2_at),
            Cite::Class { class: Box::new(kappa.clone()), rotor: Box::new(g.clone()), cover: Some(vec![inv2.key().into_vec()]) },
            Cite::word(&spelling, sp_at),
        ],
        ..receipt.clone()
    };
    k.hear_refill(&receipt2);
    assert_eq!((k.refills.applied, k.refills.unsited), (2, 1), "{:?}", k.refills);
    let back = k.named_word(rw2.as_bytes()).expect("the rewrite").clone();
    assert!(back.remainders().all(|r| r.answered == Some(receipt2.stamp)), "answered though nothing was written");
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
        answered: None,
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
