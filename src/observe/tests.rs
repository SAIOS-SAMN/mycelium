// §8.1 — the observer's readings travel whole and fold axis by axis.

use super::*;
use crate::transducer::cga;

fn vk(b: u8) -> [u8; 32] {
    [b; 32]
}

/// Every reading written by its codec reads back as itself, and a push and
/// a query reply carry them whole.
#[test]
fn every_reading_reads_back_as_itself_through_the_act() {
    let planes = Planes { n: 3, read: 7, coupled: 1, counts: vec![(0b00011, 5), (0b11000, 2)] };
    let mut loss = Loss::default();
    loss.axes[1] = 4;
    loss.axes[17] = 2;
    let memory = Memory {
        first_k: 1,
        first_allocated: 10,
        first_resident: 20,
        last_k: 9,
        last_allocated: 30,
        last_resident: 40,
        peak_resident: 44,
        samples: 3,
    };
    let court = Court { k: 12, basis: 3, treasury: 2, served: vec![(0x03, 9), (0x39, 1)] };
    let all = vec![
        Reading::Planes(planes),
        Reading::Loss(loss),
        Reading::Memory(memory),
        Reading::Court(court),
    ];
    let push = encode_push(&vk(7), &all);
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(7), all.clone())));
    let entries: Vec<(Vec<u8>, Reading)> =
        all.iter().map(|r| (Source::Voice(vk(7)).key(r.tag()), r.clone())).collect();
    let reply = encode_reply(&entries, usize::MAX);
    assert_eq!(decode_reply(&reply), Some((4, entries.clone())));
    // A reply that cannot hold them all says how many were held.
    let (total, sent) = decode_reply(&encode_reply(&entries, 64)).expect("a short reply still decodes");
    assert_eq!(total, 4);
    assert!(sent.len() < 4, "the budget is honoured");
    // A trailing byte is no act.
    let mut long = encode_query(&[1, 2]);
    long.push(0);
    assert_eq!(decode_act(&long), None);
    assert_eq!(decode_act(&encode_query(&[1, 2])), Some(ObserveAct::Query(vec![1, 2])));
}

/// A key names its source and what the reading answers.
#[test]
fn a_key_names_its_source_and_its_reading() {
    for s in [Source::Court(vk(1)), Source::Voice(vk(2))] {
        assert_eq!(Source::of_key(&s.key(TAG_LOSS)), Some((s, TAG_LOSS)));
    }
    assert_eq!(Source::of_key(&[9; 34]), None, "an unknown source kind");
}

/// THE PLANE CENSUS FOLDS PLANE BY PLANE, in the larger algebra, and never
/// adds one plane's count to another's (law 5).
#[test]
fn the_plane_census_folds_plane_by_plane() {
    let mut a = Planes { n: 3, read: 2, coupled: 0, counts: vec![(0b00011, 2)] };
    let b = Planes { n: 4, read: 3, coupled: 1, counts: vec![(0b00011, 1), (0b10001, 3)] };
    a.absorb(&b);
    assert_eq!(a.n, 4);
    assert_eq!((a.read, a.coupled), (5, 1));
    assert_eq!(a.count(0b00011), 3);
    assert_eq!(a.count(0b10001), 3);
    assert_eq!(a.count(0b00101), 0, "a plane never turned reads zero");
}

/// The census of a tally carries the plane each transition turned, by its
/// own blade, in the algebra it was read in.
#[test]
fn the_census_of_a_tally_carries_each_plane_by_its_blade() {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let _ = cga::take_plane_tally();
    let turn = Rotor::scalar(q(3, 5)).add(&Rotor::blade(0b0011, q(4, 5)));
    cga::note_transition(&turn, &cga::Mv::scalar(q(1, 1)));
    let p = Planes::of(&cga::take_plane_tally());
    assert_eq!(p.read, 1);
    assert_eq!(p.count(cga::E1 | cga::E2), 1, "the grid plane turned");
    assert_eq!(p.counts.len(), 1, "and no other");
}

/// A process's store folds what it produced and holds what it was pushed as
/// it stands; a query reads one source by its prefix.
#[test]
fn the_store_folds_its_own_readings_and_holds_pushed_ones_as_they_stand() {
    let own = Source::Voice(vk(0xa1));
    let mut one = Loss::default();
    one.axes[0] = 1;
    assert!(publish(own, Reading::Loss(one.clone())));
    assert!(publish(own, Reading::Loss(one)));
    let held = readings(&own.prefix());
    assert_eq!(held.len(), 1);
    let Reading::Loss(l) = &held[0].1 else { panic!("a loss reading") };
    assert_eq!(l.axes[0], 2, "the producer's readings fold");
    let pushed = Source::Voice(vk(0xa2));
    let mut totals = Loss::default();
    totals.axes[0] = 5;
    assert!(replace(pushed, Reading::Loss(totals.clone())));
    assert!(replace(pushed, Reading::Loss(totals)));
    let Reading::Loss(l) = &readings(&pushed.prefix())[0].1 else { panic!("a loss reading") };
    assert_eq!(l.axes[0], 5, "a pushed total stands, it is not folded twice");
}
