use super::super::*;
use super::common::*;

/// Every reading written by its codec reads back as itself, and a query
/// reply carries them whole.
#[test]
fn encode_reply_round_trips_every_reading() {
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
    let court = Court {
        k: 12,
        basis: 3,
        treasury: 2,
        served: vec![(0x03, 9), (0x39, 1)],
        // The digest asks, by the voice each named.
        digest_askers: vec![([1; 32], 7), ([2; 32], 3)],
        // TP1 — the glue's reopened rings, by what each cut, travel.
        reopened: [4, 5],
        // RM1 — the court's remainders by ℓ, ascending, and its torsion words.
        remainders: crate::transducer::cech::RemainderReading {
            by_ell: vec![(2, 3), (4, 1)],
            classes: 2,
            across: 1,
            unplaced: 0,
            refilled: 0,
            refused: 1,
            unsited: 0,
            unrecorded: 0,
            unpriced: 2,
            unminted: 4,
            unoffered: 5,
            unmerged: 0,
            coincident: 3,
            ..Default::default()
        },
        torsion_words: 6,
        // STD1 — the room's refusals and the court's, each by name.
        room_broken: 7,
        room_overflowed: 8,
        overflowed: 9,
        // The fold's round by outcome, each in its own slot.
        folds: [10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        catchup: [[16, 17, 18, 19, 20, 26], [21, 22, 23, 24, 25, 27]],
        // The serving side of a catch-up page, each axis in its own slot.
        serve: [26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
        // 78.152 — nothing final yet travels as nothing final.
        tuniversal: TStanding::default(),
    };
    // ACT1 — a court on the clock with a peer above and a peer below its
    // head (the gap is signed).
    let court_clock = Clock::Court(Box::new(CourtClock {
        k: 13,
        head: [40, 1, 0, 2, 0, 12, 3],
        genesis: Some(([9; 32], 2)),
        peers: vec![(party(1), 37, 3), (party(2), 44, -4)],
        persisted: Some((38, 11)),
        folded: None,
        unframed: 5,
        unstamped: [1, 0, 2, 0, 3],
        rulings: [[3, 1, 1, 1], [0; 4], [2, 2, 0, 0], [1, 0, 1, 0]],
        // F5 — each reason in its own slot travels.
        ruling_refusals: [[1, 0, 0], [0; 3], [0; 3], [0, 2, 3]],
        orbits: Orbits { withheld: 2, distance: vec![(party(1), [-3, 0, 5, -1])] },
    }));
    let all = vec![
        Reading::Memory(memory),
        Reading::Court(Box::new(court)),
        Reading::Clock(court_clock),
        // EC1 — a court's payments by proof of work, each arm on its own
        // lanes.
        Reading::Economy(Box::new(Economy {
            paid: [[5, 0, 2, 0], [0, 1, 0, 0], [0; 4], [1, 0, 0, 2]],
            burned: [[0; 4], [0; 4], [0, 0, 0, 3], [0; 4]],
            overflowed: 7,
            // E3 — each identity's standing travels with it.
            accounts: vec![crate::observe::Account {
                who: party(9),
                balance: [3, 1, 0, 2],
                bond: [12, 3, 0, 20],
                held: [1, 0, 0, 0],
            }],
            // What moved by kind, and the tuition paid to the court.
            flows: std::array::from_fn(|i| crate::observe::Flow { rows: i as u64, mu: [i as u64, 0, 1, 2] }),
            tuition: crate::observe::Flow { rows: 4, mu: [5, 6, 7, 8] },
        })),
    ];
    reads_back(&all);
    let entries: Vec<(Vec<u8>, Reading)> = all.iter().map(|r| (Source::Court(party(7)).key(r.tag()), r.clone())).collect();
    // A reply that cannot hold them all says how many were held.
    let short = encode_reply(&entries, 64).expect("the reply writes");
    let (total, sent) = decode_reply(&short).expect("a short reply still decodes");
    assert_eq!(total, 4);
    assert!(sent.len() < 4, "the budget is honoured");
    // A trailing byte is no act.
    let mut long = encode_query(&[1, 2]).expect("the query writes");
    long.push(0);
    assert_eq!(decode_act(&long), None);
    assert_eq!(decode_act(&encode_query(&[1, 2]).expect("the query writes")), Some(ObserveAct::Query(vec![1, 2])));
}

/// CV5 — replication per signer and adjudication by kind travel whole.
#[test]
fn encode_reply_round_trips_replication_and_adjudication() {
    let mut axes = [0u64; REP_AXES];
    axes[replication::BACKFILL] = 7;
    axes[replication::STALE] = 2;
    axes[replication::BACKFILL_BUDGET] = 3;
    axes[replication::LAG] = 540;
    let mut unattributed = [0u64; REP_AXES];
    unattributed[replication::DIGEST_UNREADABLE] = 1;
    reads_back(&[
        Reading::Replication(Box::new(Replication { signers: vec![(party(3), axes), (party(4), [0; REP_AXES])], unattributed })),
        Reading::Adjudication(Adjudication {
            ratified: 5,
            stood: 9,
            awaiting: 1,
            deferred: 2,
            refused: vec![(0, 3), (2, 4)],
            obstructed: [1, 0, 3, 2, 0, 1],
            body_faults: [0, 1, 2, 3, 4, 5, 6, 7],
            body_lifted: [8, 9],
        }),
    ]);
    assert_eq!(REPLICATION_AXES.len(), REP_AXES, "the axes and the counters agree");
    assert_eq!(REPLICATION_AXES[replication::LAG].0, "lag", "the last axis is the lag, a level");
}

/// The replication counters keep each signer apart, and a frame that named
/// no signer apart from all of them; the verdicts count by kind.
#[test]
fn replication_counts_per_signer_and_verdicts_by_kind() {
    let signer = party(0xb1);
    let before = replication();
    let held = |r: &Replication| r.signers.iter().find(|(k, _)| *k == signer).map(|(_, a)| a[replication::BACKFILL]);
    note_replication(Some(signer), replication::BACKFILL, 4);
    note_replication(Some(signer), replication::BACKFILL, 1);
    note_replication(None, replication::DIGEST_SIGNATURE, 1);
    let after = replication();
    assert_eq!(held(&after), Some(held(&before).map_or(5, |b| b + 5)));
    assert!(after.unattributed[replication::DIGEST_SIGNATURE] > before.unattributed[replication::DIGEST_SIGNATURE]);
    note_replication(Some(signer), 99, 1);
    assert_eq!(held(&replication()), held(&after), "an axis past the list counts nothing");
    let a0 = adjudication();
    note_verdict(Verdict::Ratified);
    note_verdict(Verdict::Refused(6));
    let a1 = adjudication();
    assert_eq!(a1.ratified, a0.ratified + 1);
    let refused = |a: &Adjudication| a.refused.iter().find(|(k, _)| *k == 6).map_or(0, |(_, n)| *n);
    assert_eq!(refused(&a1), refused(&a0) + 1, "a refusal counts under its own kind");
}

/// A key names its source and what the reading answers.
#[test]
fn source_of_key_names_source_and_tag() {
    let s = Source::Court(party(1));
    assert_eq!(Source::of_key(&s.key(TAG_MEMORY)), Some((s, TAG_MEMORY)));
    assert_eq!(Source::of_key(&[9; 34]), None, "an unknown source kind");
}

/// A process's store folds what it produced and holds what it replaces as
/// it stands; a query reads one source by its prefix.
#[test]
fn publish_folds_readings_and_replace_holds_them() {
    let sample = |k: u64, allocated: u64| Memory {
        first_k: k,
        first_allocated: allocated,
        first_resident: 1,
        last_k: k,
        last_allocated: allocated,
        last_resident: 1,
        peak_resident: 1,
        samples: 1,
    };
    let own = Source::Court(party(0xa1));
    assert!(publish(own, Reading::Memory(sample(1, 10))));
    assert!(publish(own, Reading::Memory(sample(2, 20))));
    let held = readings(&own.prefix());
    assert_eq!(held.len(), 1);
    let Reading::Memory(m) = &held[0].1 else { panic!("a memory reading") };
    assert_eq!((m.first_k, m.last_k, m.samples), (1, 2, 2), "the producer's readings fold");
    let replaced = Source::Court(party(0xa2));
    assert!(replace(replaced, Reading::Memory(sample(5, 50))));
    assert!(replace(replaced, Reading::Memory(sample(5, 50))));
    let Reading::Memory(m) = &readings(&replaced.prefix())[0].1 else { panic!("a memory reading") };
    assert_eq!(m.samples, 1, "a replaced reading stands, it is not folded twice");
}
