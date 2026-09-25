// 78.37(4), RM1 — THE MIND'S REMAINDER ON THE WORD'S RECORD: carried through
// a restart whole, the address untouched, and the index rebuilt from it.

use crate::client::sdk::*;
use crate::transducer::element::{Element, Filed, Generator, Remainder, TorsionCycle, FIBER_SPATIAL};

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
    Remainder { class: Box::new(class), at: at.iter().map(|c| c.to_vec()).collect(), answered: None }
}

/// A HOLONOMY cycle turning by `ray` at the period READ off it (PD1): the
/// remainder's ray is its cycle's generator, never a copy on the remainder.
fn cycle_by(ray: crate::transducer::alphabet::Rotor, r: Option<Remainder>) -> TorsionCycle {
    let generator = Generator::Holonomy(Box::new(ray));
    let period = generator.order().expect("a remainder's ray is a finite-order rotor, as a Congruent fill files it");
    TorsionCycle { generator, period, fibre: FIBER_SPATIAL, remainder: r }
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
    let mut w = super::common::two_body_sentence(gauges);
    w.record_cycle(cycle_by(ray, Some(r)));
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
    // The callus stood, then its two remainder words re-filled: the line
    // reads Stood, Settled, Settled.
    let chain = e.session.voice().chain();
    let head = chain.head().expect("the re-fill's act").clone();
    let at = |back: u64| chain.at_place(crate::chronometry::Place { k: head.stamp.k, i: head.stamp.i - back }).expect("an act").clone();
    let stood = at(2);
    assert_eq!(stood.kind, crate::chronometry::ActKind::Stood);
    let words = stood.cites.iter().filter(|c| matches!(c, crate::chronometry::Cite::Word { .. })).count();
    assert_eq!(words, 3, "the callus at its own stamp and the two remainder words: {:?}", stood.cites);
    assert!(
        stood.cites.iter().any(|c| matches!(c, crate::chronometry::Cite::Class { cover: Some(cv), .. } if a.iter().chain(&b).all(|k| cv.contains(k)))),
        "the class, its ray and both chains' charts"
    );
    // RM1 (c), door (a): each remainder word re-fills by a Settled act
    // citing the word, its remainder and the callus; its μ⃗ the pressure
    // answered, ℓ − 1 = 3 on the shell lane.
    for settled in [at(1), head.clone()] {
        assert_eq!(settled.kind, crate::chronometry::ActKind::Settled);
        assert!(
            matches!(
                settled.cites.as_slice(),
                [crate::chronometry::Cite::Word { .. }, crate::chronometry::Cite::Class { .. }, crate::chronometry::Cite::Word { .. }]
            ),
            "the rewrite word, its remainder, the spelling word: {:?}",
            settled.cites
        );
        assert_eq!(settled.mu, [0, 0, 0, 3], "the pressure answered");
    }
    let r = e.meet_graph().remainders;
    assert_eq!((r.refill_acts, r.refill_refused), (2, 0), "{}", r.line());
    for w in [&w1, &w2] {
        let held = e.epigenome.held_element(w.address().as_bytes()).expect("still held");
        let cycle = held.torsion().iter().find(|t| t.remainder.is_some()).expect("its remainder").clone();
        assert!(cycle.remainder.as_ref().is_some_and(|r| r.answered.is_some()), "answered at the act's stamp");
        assert_eq!(cycle.pressure(), Some([0; 4]), "and its pressure reads zero");
    }
    // An answered remainder is evidence, not a question: a third word of the
    // class at a third vantage finds both standing ones answered and raises
    // nothing.
    let c: Chain = crate::transducer::stalk::Invariant::counts(&[1, 7, 3]).prefixes().iter().map(|p| p.key().into_vec()).collect();
    stand_with(&mut e, [&[(1, 9)], &[(3, 9)]], &c, quarter());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.calluses, r.answered_skipped), (1, 2), "the receptor skips the answered: {}", r.line());

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
    // primes reaches — and it turns FOREVER: a ray of infinite order has no
    // period, so it is no holonomy cycle and no remainder can ride one (PD1:
    // the period is read off the generator). The record refuses it by name;
    // no Congruent fill files such a ray (classify files finite orders only).
    let pi17 = || {
        let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
        crate::transducer::alphabet::Rotor::scalar(q(4)).add(&crate::transducer::alphabet::Rotor::blade(0b0011, q(1)))
    };
    assert_eq!(Generator::Holonomy(Box::new(pi17())).order(), None, "an infinite order is no period");
    let mut w = super::common::two_body_sentence([&[(1, 2)], &[(3, 4)]]);
    let mut r = remainder(&[]);
    r.at = a.clone();
    let claimed = TorsionCycle {
        generator: Generator::Holonomy(Box::new(pi17())),
        period: 4,
        fibre: FIBER_SPATIAL,
        remainder: Some(r),
    };
    assert_eq!(w.record_cycle(claimed), Filed::Misspoken, "a period the ray does not have, refused by name");
    assert_eq!(w.remainders().count(), 0, "and no remainder rides it");
}

/// RM1 (c), door (b) — A DEMAND ANSWERED BY CLASS KEY RE-FILLS: a word's
/// open remainder of κ; the class docketed as a Demand and answered by class
/// key (a discharge); at the close the ray is re-spelled in the held
/// constructors, the spelling stood as the mind's own word, and the word
/// re-fills by a Settled act, its remainder answered.
#[test]
fn a_demand_answered_by_class_key_refills_an_open_remainder() {
    let (a, _, _) = chains();
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("answered"));
    let w = stand_with(&mut e, [&[(1, 2)], &[(3, 4)]], &a, quarter());
    let class = *remainder(&[]).class;
    let charts: Vec<crate::transducer::stalk::Invariant> =
        a.iter().filter_map(|k| crate::transducer::stalk::Invariant::from_key(k)).collect();
    e.contemplation.docket_demand(&class, &charts);
    e.contemplation.docket_discharged(class.class_key().as_bytes());
    e.stand_calluses();
    let r = e.meet_graph().remainders;
    assert_eq!((r.refill_acts, r.refill_refused, r.calluses), (1, 0, 0), "{}", r.line());
    let head = e.session.voice().chain().head().expect("the re-fill").clone();
    assert_eq!((head.kind, head.mu), (crate::chronometry::ActKind::Settled, [0, 0, 0, 3]));
    let held = e.epigenome.held_element(w.address().as_bytes()).expect("held");
    assert!(held.remainders().all(|r| r.answered == Some(head.stamp)), "answered at the act's stamp");
}

fn cycle(r: Option<Remainder>) -> TorsionCycle {
    cycle_by(quarter(), r)
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
    let bytes = e.to_bytes().expect("the record writes");
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
