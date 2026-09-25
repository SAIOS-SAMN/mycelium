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

/// Two incomparable chart chains, and a chain nested in the first: each the
/// prefixes of a real invariant, finest first.
type Chain = Vec<Vec<u8>>;

fn chains() -> (Chain, Chain, Chain) {
    use crate::transducer::stalk::Invariant;
    let chain = |inv: &Invariant| inv.prefixes().iter().map(|p| p.key().into_vec()).collect::<Vec<_>>();
    // Two charts that part at their second axis, and the first's parent.
    let (ia, ib) = (Invariant::counts(&[1, 2, 3]), Invariant::counts(&[1, 5, 3]));
    let (a, b) = (chain(&ia), chain(&ib));
    assert!(!a.iter().all(|k| b.contains(k)) && !b.iter().all(|k| a.contains(k)), "the fixture's chains are incomparable");
    let nested = chain(&ia.peel());
    assert!(nested.iter().all(|k| a.contains(k)) && nested.len() < a.len(), "and one nested in the first");
    (a, b, nested)
}

/// A word standing with a remainder of the quarter's class at `at`, its ray
/// `ray`, sealed Stood by the entity and stood in the dialect.
fn stand_with(entity: &mut Entity, gauges: [&[(u8, u8)]; 2], at: &[Vec<u8>], ray: crate::transducer::alphabet::Rotor) -> Element {
    let mut r = remainder(&[]);
    r.at = at.to_vec();
    r.ray = Box::new(ray);
    let mut w = super::common::two_body_sentence(gauges);
    w.record_cycle(cycle(Some(r)));
    let k = entity.seal_word(crate::chronometry::ActKind::Stood, &w);
    let _ = entity.epigenome.stand_dialect_at(w.clone(), k, 0);
    w
}

/// THE RECEPTOR, constructed. One class at two incomparable chains with one
/// ray → one callus, Stood, citing both remainder words and the class. The
/// same at a nested chain → nested, no callus. The same class with a
/// differing ray → discordant, no callus. A ray the held constructors do not
/// spell → unspelled, the class docketed as a Demand over both chains, no
/// callus.
#[test]
fn the_receptor_raises_a_callus_only_for_one_twist_at_two_vantages() {
    let (a, b, nested) = chains();
    let other_quarter = || {
        let q = crate::transducer::alphabet::LetterId::quarter((0, 2)).expect("a plane");
        crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor()
    };

    let mut e = Entity::open("127.0.0.1:1", Kernel::named("callus"));
    let w1 = stand_with(&mut e, [&[(1, 2)], &[(3, 4)]], &a, quarter());
    let w2 = stand_with(&mut e, [&[(5, 6)], &[(7, 8)]], &b, quarter());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.calluses, r.nested, r.discordant, r.unspelled), (1, 0, 0, 0), "{}", r.line());
    let head = e.session.voice().chain().head().expect("the callus's act").clone();
    assert_eq!(head.kind, crate::chronometry::ActKind::Stood);
    let words = head.cites.iter().filter(|c| matches!(c, crate::chronometry::Cite::Word { .. })).count();
    assert_eq!(words, 3, "the callus at its own stamp and the two remainder words: {:?}", head.cites);
    assert!(
        head.cites.iter().any(|c| matches!(c, crate::chronometry::Cite::Class { cover: Some(cv), .. } if a.iter().chain(&b).all(|k| cv.contains(k)))),
        "the class, its ray and both chains' charts"
    );
    let _ = (w1, w2);

    let mut e = Entity::open("127.0.0.1:1", Kernel::named("nested"));
    stand_with(&mut e, [&[(1, 2)], &[(3, 4)]], &a, quarter());
    stand_with(&mut e, [&[(5, 6)], &[(7, 8)]], &nested, quarter());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.calluses, r.nested), (0, 1), "one vantage at a finer depth: {}", r.line());

    let mut e = Entity::open("127.0.0.1:1", Kernel::named("discordant"));
    stand_with(&mut e, [&[(1, 2)], &[(3, 4)]], &a, quarter());
    stand_with(&mut e, [&[(5, 6)], &[(7, 8)]], &b, other_quarter());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.calluses, r.discordant), (0, 1), "one class, two twists, none chosen: {}", r.line());

    // π₁₇: its arc 15 + 8i has norm 17², which no product of the held
    // primes reaches.
    let pi17 = || {
        let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
        crate::transducer::alphabet::Rotor::scalar(q(4)).add(&crate::transducer::alphabet::Rotor::blade(0b0011, q(1)))
    };
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("unspelled"));
    stand_with(&mut e, [&[(1, 2)], &[(3, 4)]], &a, pi17());
    stand_with(&mut e, [&[(5, 6)], &[(7, 8)]], &b, pi17());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.calluses, r.unspelled), (0, 1), "{}", r.line());
    let docket = e.contemplation.docket_cites();
    assert!(
        docket.iter().any(|c| matches!(c, crate::chronometry::Cite::Demand { cover: Some(cv), .. } if a.iter().chain(&b).all(|k| cv.contains(k)))),
        "the class docketed as a Demand over both chains: {docket:?}"
    );
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
    let none = TorsionCycle { period: 0, ..cycle(None) };
    assert_eq!(e.record_cycle(none), Filed::NoPeriod, "a zero period is no cycle, refused by name");
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
