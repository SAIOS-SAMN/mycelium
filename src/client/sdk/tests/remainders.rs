// 78.37(4), RM1 — THE MIND'S REMAINDER ON THE WORD'S RECORD: carried through
// a restart whole, the address untouched, and the index rebuilt from it.

use crate::client::sdk::*;
use crate::transducer::element::{Element, Filed, Remainder, TorsionCycle, FIBER_SPATIAL};

fn quarter() -> crate::transducer::alphabet::Rotor {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("the sensor plane");
    crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor()
}

fn remainder(at: &[&[u8]]) -> Remainder {
    let g = quarter();
    let class = crate::transducer::sentence::ClassVector::of_rotor(
        &g,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a quarter turn has a class");
    Remainder { class: Box::new(class), ray: Box::new(g), at: at.iter().map(|c| c.to_vec()).collect() }
}

fn cycle(r: Option<Remainder>) -> TorsionCycle {
    TorsionCycle { sigma: Vec::new(), period: 4, fibre: FIBER_SPATIAL, remainder: r }
}

/// A word's torsion record carries its remainder (κ, g, the residue's chart):
/// the record round-trips it and the address does not move; a cycle filed
/// without one gains it, a differing one is refused. Admitted to the genome
/// it is indexed at chart ‖ κ; crystallized and restored into a fresh
/// epigenome, the record comes back whole and the index is rebuilt from it.
/// A second word whose remainder coincides is not filed over the first.
#[test]
fn a_remainder_rides_the_record_through_a_restart_and_the_index_is_rebuilt() {
    let r = remainder(&[b"", b"c"]);
    let mut e = super::common::two_body_sentence([&[(1, 2)], &[(3, 4)]]);
    let addr = e.address();
    assert_eq!(e.record_cycle(cycle(None)), Filed::New);
    assert_eq!(e.record_cycle(cycle(Some(r.clone()))), Filed::Gained, "a cycle filed without one gains it");
    assert_eq!(e.record_cycle(cycle(Some(remainder(&[b"d"])))), Filed::Refused, "a differing one is refused");
    let bytes = e.to_bytes();
    let back = Element::from_bytes(&bytes, &mut 0).expect("the record reads back");
    assert_eq!(back.remainders().collect::<Vec<_>>(), vec![&r], "the remainder rides the record whole");
    assert_eq!(back.address(), addr, "and the address does not carry it");

    let mut epi = Epigenome::new();
    epi.admit_word(crate::transducer::element::Held::of(e.clone()), 0);
    assert_eq!(epi.remainders_at(&r.at, &r.class), vec![addr.clone()], "indexed at the residue's chart ‖ κ");
    let admitting = crate::chronometry::Stamp::new(1, [7; 32], crate::chronometry::Place::GENESIS);
    let (frozen, refused) = epi.crystallize(|_| Some(admitting));
    assert_eq!(refused.total(), 0);
    let mut booted = Epigenome::new();
    assert!(booted.uncrystallize(&frozen).is_some(), "the genome restores");
    assert_eq!(booted.remainders_at(&r.at, &r.class), vec![addr.clone()], "the index is rebuilt from the record");
    assert_eq!(booted.remainders_len(), 1);

    let mut other = super::common::two_body_sentence([&[(5, 6)], &[(7, 8)]]);
    other.record_cycle(cycle(Some(r.clone())));
    let _ = booted.stand_dialect_at(other, 1, 1);
    assert_eq!(booted.remainders_at(&r.at, &r.class), vec![addr], "one remainder per class at a chart");
    assert_eq!(booted.remainders_coincident, 1, "the coincidence counted");
}
