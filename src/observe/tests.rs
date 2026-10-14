// §8.1 — the observer's readings travel whole and fold axis by axis.

use super::*;

fn vk(b: u8) -> [u8; 32] {
    [b; 32]
}

/// A court's readings held under its key, written as a query reply and read
/// back: every one as itself.
fn reads_back(all: &[Reading]) {
    let entries: Vec<(Vec<u8>, Reading)> = all.iter().map(|r| (Source::Court(vk(7)).key(r.tag()), r.clone())).collect();
    let reply = encode_reply(&entries, usize::MAX).expect("the reply writes");
    let total = u32::try_from(entries.len()).expect("a few readings");
    assert_eq!(decode_reply(&reply), Some((total, entries)));
}

/// Every reading written by its codec reads back as itself, and a query
/// reply carries them whole.
#[test]
fn every_reading_reads_back_as_itself_through_the_act() {
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
        folds: [10, 11, 12, 13, 14, 15, 16, 17],
        catchup: [[16, 17, 18, 19, 20], [21, 22, 23, 24, 25]],
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
        peers: vec![(vk(1), 37, 3), (vk(2), 44, -4)],
        persisted: Some((38, 11)),
        folded: None,
        unframed: 5,
        unstamped: [1, 0, 2, 0, 3],
        rulings: [[3, 1, 1, 1], [0; 4], [2, 2, 0, 0], [1, 0, 1, 0]],
        // F5 — each reason in its own slot travels.
        ruling_refusals: [[1, 0, 0], [0; 3], [0; 3], [0, 2, 3]],
        orbits: Orbits { withheld: 2, distance: vec![(vk(1), [-3, 0, 5, -1])] },
    }));
    let all = vec![
        Reading::Memory(memory),
        Reading::Court(Box::new(court)),
        Reading::Clock(court_clock),
        // EC1 — a court's payments by proof of work, each arm on its own
        // lanes.
        Reading::Economy(Box::new(Economy {
            paid: [[5, 0, 2, 0], [0, 1, 0, 0], [0; 4]],
            burned: [[0; 4], [0; 4], [0, 0, 0, 3]],
            overflowed: 7,
            // E3 — each identity's standing travels with it.
            accounts: vec![crate::observe::Account {
                who: vk(9),
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
    let entries: Vec<(Vec<u8>, Reading)> = all.iter().map(|r| (Source::Court(vk(7)).key(r.tag()), r.clone())).collect();
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
fn replication_and_adjudication_read_back_as_themselves() {
    let mut axes = [0u64; REP_AXES];
    axes[rep::BACKFILL] = 7;
    axes[rep::STALE] = 2;
    axes[rep::BACKFILL_BUDGET] = 3;
    axes[rep::LAG] = 540;
    let mut unattributed = [0u64; REP_AXES];
    unattributed[rep::DIGEST_UNREADABLE] = 1;
    reads_back(&[
        Reading::Replication(Box::new(Replication { signers: vec![(vk(3), axes), (vk(4), [0; REP_AXES])], unattributed })),
        Reading::Adjudication(Adjudication {
            ratified: 5,
            stood: 9,
            awaiting: 1,
            deferred: 2,
            refused: vec![(0, 3), (2, 4)],
            obstructed: [1, 0, 3, 2, 0, 1],
        }),
    ]);
    assert_eq!(REPLICATION_AXES.len(), REP_AXES, "the axes and the counters agree");
    assert_eq!(REPLICATION_AXES[rep::LAG].0, "lag", "the last axis is the lag, a level");
}

/// The replication counters keep each signer apart, and a frame that named
/// no signer apart from all of them; the verdicts count by kind.
#[test]
fn replication_counts_per_signer_and_verdicts_by_kind() {
    let signer = vk(0xb1);
    let before = replication();
    let held = |r: &Replication| r.signers.iter().find(|(k, _)| *k == signer).map(|(_, a)| a[rep::BACKFILL]);
    note_replication(Some(signer), rep::BACKFILL, 4);
    note_replication(Some(signer), rep::BACKFILL, 1);
    note_replication(None, rep::DIGEST_SIGNATURE, 1);
    let after = replication();
    assert_eq!(held(&after), Some(held(&before).map_or(5, |b| b + 5)));
    assert!(after.unattributed[rep::DIGEST_SIGNATURE] > before.unattributed[rep::DIGEST_SIGNATURE]);
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
fn a_key_names_its_source_and_its_reading() {
    let s = Source::Court(vk(1));
    assert_eq!(Source::of_key(&s.key(TAG_MEMORY)), Some((s, TAG_MEMORY)));
    assert_eq!(Source::of_key(&[9; 34]), None, "an unknown source kind");
}

/// A process's store folds what it produced and holds what it replaces as
/// it stands; a query reads one source by its prefix.
#[test]
fn the_store_folds_its_own_readings_and_holds_replaced_ones_as_they_stand() {
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
    let own = Source::Court(vk(0xa1));
    assert!(publish(own, Reading::Memory(sample(1, 10))));
    assert!(publish(own, Reading::Memory(sample(2, 20))));
    let held = readings(&own.prefix());
    assert_eq!(held.len(), 1);
    let Reading::Memory(m) = &held[0].1 else { panic!("a memory reading") };
    assert_eq!((m.first_k, m.last_k, m.samples), (1, 2, 2), "the producer's readings fold");
    let replaced = Source::Court(vk(0xa2));
    assert!(replace(replaced, Reading::Memory(sample(5, 50))));
    assert!(replace(replaced, Reading::Memory(sample(5, 50))));
    let Reading::Memory(m) = &readings(&replaced.prefix())[0].1 else { panic!("a memory reading") };
    assert_eq!(m.samples, 1, "a replaced reading stands, it is not folded twice");
}

/// PH5 — a court's phase travels whole, and the later window stands.
#[test]
fn a_courts_phase_reads_back_whole() {
    let phase = Phase { windows: 3, h0: 986, h1: 989, held: 1, sections: 1, strata: 2, installed: 1, ..Phase::default() };
    reads_back(&[Reading::Phase(phase.clone())]);
    let mut earlier = Reading::Phase(Phase::default());
    assert!(earlier.absorb(&Reading::Phase(phase.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Phase(phase), "the later window stands");
}

/// LD1 — THE CARRIAGE TRAVELS WHOLE, each record filed under its carrier and
/// kind in order, its largest kept.
#[test]
fn the_carriage_reads_back_whole_filed_by_carrier_and_kind() {
    let mut c = Carriage::default();
    c.add(CARRIER_FRAME, 0x0d, 40);
    c.add(CARRIER_DELTA, 12, 900);
    c.add(CARRIER_DELTA, 12, 100);
    c.add(CARRIER_AWARD, 0, 120);
    assert_eq!(
        c.rows.iter().map(|r| (r.carrier, r.kind, r.records, r.bytes, r.largest)).collect::<Vec<_>>(),
        [(CARRIER_AWARD, 0, 1, 120, 120), (CARRIER_DELTA, 12, 2, 1000, 900), (CARRIER_FRAME, 0x0d, 1, 40, 40)]
    );
    reads_back(&[Reading::Carriage(c)]);
}

/// LD0 step 4 — AN AWARD IS A FIELD OF ITS WORD'S ROW, measured at what the
/// field carries: the voices, μ⃗, the stamp and the receipt, never the
/// address. An award on a 70 000-byte word weighs what one on a 3-byte word
/// does, and the treasury's own snapshot carries no award at all.
#[test]
fn an_award_is_measured_flat_in_its_word() {
    use crate::transducer::address::Address;
    let mut t = crate::economy::Treasury::new();
    let a = (crate::chronometry::Stamp { h: 1, vk: [0; 32], k: 1, i: 0 }, [1u8; 32]);
    t.escrow_award(&Address::from_bytes([7u8; 3]), vec![[2; 32]], [4, 0, 0, 0], a);
    t.escrow_award(&Address::from_bytes(vec![9u8; 70_000]), vec![[2; 32]], [4, 0, 0, 0], a);
    let mut c = Carriage::default();
    t.carriage(&mut c);
    // The field: its tag, the voice count, one voice, μ⃗, the stamp, the receipt.
    let field = 1 + 4 + 32 + 32 + crate::chronometry::Stamp::LEN as u64 + 32;
    let [award] = c.rows.as_slice() else { panic!("one carrier, one kind: {:?}", c.rows) };
    assert_eq!((award.records, award.largest, award.bytes), (2, field, 2 * field), "flat in the word's size");
    let mut tries = Vec::new();
    t.put_tries(&mut tries);
    let (stakes, balances, held) = crate::economy::Treasury::take_tries(&tries, &mut 0).expect("the tries read back");
    let back = crate::economy::Treasury::from_parts(stakes, balances, held, t.entries().to_vec());
    assert!(back.escrow_of(&[7u8; 3]).is_none(), "the award rides its word's node, not the treasury's tries");
}

/// A COURT'S ACTS TRAVEL WHOLE (2026-09-29): every counter in its table.
#[test]
fn the_acts_travel_whole() {
    let acts = Acts {
        k: 9,
        reserve: [1, 2, 3, 4],
        drawn: [5, 6, 7, 8],
        restored: [9, 10, 11, 12],
        refused: [13, 0, 1, 0],
        syllabus: [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26],
        fetch: [21, 22, 39, 49, 40, 41, 42, 43, 44, 45, 46, 47, 48, 65, 66, 67, 68, 69, 70],
        judged: [23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 59, 60, 61, 62, 63, 64],
        paid: [33, 34, 35, 36],
        colours: 37,
        colours_derived: 38,
        deferred: vec![crate::observe::Deferred { head: [1, 2, 3, 4, 5, 6, 7, 8], kind: 2, mag: [39, 40, 41, 42], short: [0, 43, 0, 0], tries: 44, learner: Some([9; 8]) }],
        provisional: [50, 51, 52, 53, 54, 55, 56, 57],
    };
    reads_back(&[Reading::Acts(Box::new(acts))]);
}

/// EVERY NAME A PRODUCER COUNTS UNDER IS ITS TABLE'S: `count` is silent on a
/// name the table does not hold, so each name the producers use is pinned
/// here, and a renamed slot fails this test rather than counting nothing.
#[test]
fn every_counted_name_is_its_table_s() {
    fn holds<const N: usize>(table: &[&str; N], names: &[&str]) {
        for n in names {
            let mut slots = [0u64; N];
            count(&mut slots, table, n);
            assert_eq!(slots.iter().sum::<u64>(), 1, "{n} is a slot of its table");
        }
    }
    holds(
        &SYLLABUS,
        &["pages", "words served", "words missed", "ended: order", "ended: frame", "ended: count", "deferred: reserve"],
    );
    holds(
        &FETCH,
        &[
            "served to courts",
            "served to voices",
            "deferred: reserve",
            "deferred: learner short",
            "words: refracted (held, Γ)",
            "words: conjugate, carriage apart",
            "words: unspellable conjugator",
            "words: rotor per vertex δγ",
            "words: chart only",
            "words: unanchored",
            "bytes served",
            "bytes refracted, whole",
            "bytes refracted, as (held, Γ)",
        ],
    );
    holds(
        &PROVISIONAL,
        &[
            "letters docketed",
            "letters heard again (converged as a party)",
            "offers sent to witnesses",
            "witness reached: installed",
            "witness reached: inexact",
            "witness reached: awaiting a vantage",
            "witness refused",
            "witness unmeasured",
        ],
    );
    holds(
        &JUDGED,
        &[
            "first closures",
            "closures again",
            "refused P",
            "refused D",
            "refused C",
            "refused I",
            "refused N",
            "refused ?",
            "not a rule",
            "unverifiable",
            "no work",
            "unread",
            "unsigned",
            "not an ancient",
            "no such task",
            "no rule carried",
        ],
    );
}

/// 78.153c — EVERY VERDICT THE COURT OR ITS DOOR COUNTS HAS ITS OWN SLOT:
/// `Judged::slot` is a bijection onto `JUDGED`, in its order, and a verdict
/// counted lands exactly there — so no name is dropped as "no work" was.
#[test]
fn every_judged_verdict_is_one_slot_of_the_table() {
    let mut seen = [false; JUDGED.len()];
    for (i, j) in Judged::ALL.iter().enumerate() {
        assert_eq!(j.slot(), i, "{j:?} sits in the table's order");
        assert_eq!(j.name(), JUDGED[i], "{j:?} reads by the table's name");
        assert!(!seen[i], "{j:?} shares no slot");
        seen[i] = true;
        let mut tally = ActsTally::default();
        tally.judge(*j);
        let mut want = [0u64; JUDGED.len()];
        want[i] = 1;
        assert_eq!(tally.judged, want, "{j:?} counted on its slot alone");
    }
    assert!(seen.iter().all(|&s| s), "every slot has its verdict");
    assert_eq!(Judged::NoWork.name(), "no work");
    assert_eq!(Judged::Unread.name(), "unread");
}

/// LD1, 78.24 — A LEDGER'S NODES ARE MEASURED BY THE ACT KIND THAT SEALED
/// THEM, and a node citing a word carries the same bytes whatever the size
/// of the word: the citation is the word's fiber and admitting stamp, never
/// a copy of it.
#[test]
fn a_ledgers_nodes_are_measured_by_act_kind_and_flat_in_the_word() {
    use crate::chronometry::{ActKind, Cite, ReceiptChain, Stamp};
    let small = crate::transducer::element::from_canonical(
        &crate::transducer::germinate::germinate(&[(0, 0, 1u8)], &[(0, 0, 2u8)]).expect("a small word"),
    );
    let pairs: Vec<(i64, i64, u8)> = (0..40).map(|i| (i, i % 7, 1u8)).collect();
    let after: Vec<(i64, i64, u8)> = (0..40).map(|i| (i, i % 7, 2u8)).collect();
    let large = crate::transducer::element::from_canonical(
        &crate::transducer::germinate::germinate(&pairs, &after).expect("a larger word"),
    );
    assert!(large.address().len() > small.address().len(), "two words of different sizes");
    let at = Stamp { h: 2, vk: [5; 32], k: 2, i: 0 };
    let chart = crate::transducer::stalk::Invariant::counts(&[1, 0, 0, 1]);
    let cite = |_: &crate::transducer::element::Element| Cite::Word { chart: chart.clone(), class: None, at };
    // Genesis first: it names no predecessor, so the two listing nodes after
    // it are the same shape apart from the word each cites. (78.94: kinds a
    // court seals; the stood and learned kinds of the voices' chains are
    // sealed by nothing.)
    let mut chain = ReceiptChain::with_vk([3; 32]);
    chain.append(ActKind::Attest, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::List, [0; 4], vec![cite(&small)], [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::List, [0; 4], vec![cite(&large)], [0; 32], Vec::new()).expect("appends");
    let mut c = Carriage::default();
    c.of_chain(&chain);
    let listed = c.rows.iter().find(|r| r.kind == ActKind::List.to_byte()).expect("the listing nodes");
    assert_eq!(listed.records, 2);
    assert_eq!(listed.largest * 2, listed.bytes, "both carry the same bytes, whatever the word they cite");
    assert!(c.rows.iter().any(|r| r.kind == ActKind::Attest.to_byte() && r.records == 1), "and the attest node by its own kind");
}

/// 78.152 — A COURT'S TUNIVERSAL STANDING TRAVELS EXACTLY: three keepers
/// exchange readings, the observer reads one court's keepers off them, and
/// the Court reading carrying that standing — and a hand-built residual
/// with signed lanes at their carrier's ends — reads back as itself.
#[test]
fn a_courts_tuniversal_standing_reads_back_exactly() {
    use crate::chronometry::tuniversal::{Edge, Keepers, Origin, Residual, TPoint, Tick};
    use ed25519_dalek::SigningKey;
    let origin = Origin { genesis: [0x6a; 32], epoch: 146 };
    let keys = [1u8, 2, 3].map(|n| SigningKey::from_bytes(&[n; 32]));
    let vks = keys.clone().map(|k| k.verifying_key().to_bytes());
    let mut courts = vks.map(|v| Keepers::new(origin, v));
    let tick = |h, lanes, k| Tick { h, lanes, k };
    let hear = |courts: &mut [Keepers; 3], to: usize, from: usize| {
        let ring = courts[from].own_ring();
        courts[to].admit(vks[from], ring).expect("the ring admits");
    };
    assert_eq!(courts[0].read_own(&keys[0], tick(10, [4, 0, 0, 0], 1)), Some(true));
    hear(&mut courts, 1, 0);
    assert_eq!(courts[1].read_own(&keys[1], tick(12, [0, 5, 0, 0], 3)), Some(true));
    hear(&mut courts, 2, 0);
    hear(&mut courts, 2, 1);
    assert_eq!(courts[2].read_own(&keys[2], tick(14, [0, 0, 6, 0], 2)), Some(true));
    // c's view: a final at 10 on b's citation, b at 12 on c's, c cited by
    // nobody; the door's point the greatest final.
    let seen = keepers_seen(&courts[2]);
    let a10 = TPoint { origin, h: 10, lanes: [4, 0, 0, 0], k: 1 };
    let b12 = TPoint { origin, h: 12, lanes: [0, 5, 0, 0], k: 3 };
    let of = |v: [u8; 32]| seen.iter().find(|s| s.key == v).cloned().expect("the keeper is seen");
    assert_eq!(seen.len(), 3);
    assert_eq!(seen.iter().filter(|s| s.own).map(|s| s.key).collect::<Vec<_>>(), [vks[2]]);
    assert_eq!(of(vks[0]), KeeperSeen { key: vks[0], own: false, newest: Some(tick(10, [4, 0, 0, 0], 1)), final_of: Some(a10) });
    assert_eq!(of(vks[1]), KeeperSeen { key: vks[1], own: false, newest: Some(tick(12, [0, 5, 0, 0], 3)), final_of: Some(b12) });
    assert_eq!(of(vks[2]), KeeperSeen { key: vks[2], own: true, newest: Some(tick(14, [0, 0, 6, 0], 2)), final_of: None });
    assert_eq!(courts[2].final_point(), Some(b12));
    let read = TStanding { final_point: courts[2].final_point(), advances: 2, keepers: seen, residual: courts[2].residual() };
    // A residual measured on all three edges, signed, at the carrier's ends.
    let built = TStanding {
        final_point: Some(TPoint { origin, h: u64::MAX, lanes: [u64::MAX, 0, 1, 2], k: 7 }),
        advances: u64::MAX,
        keepers: vec![KeeperSeen { key: vk(4), own: true, newest: None, final_of: None }],
        residual: Residual {
            edges: vec![
                Edge { from: vk(4), to: vk(5), h: 20, delta: [-3, 5, 0, i128::MIN] },
                Edge { from: vk(5), to: vk(6), h: 21, delta: [i128::MAX, -1, 2, 0] },
                Edge { from: vk(4), to: vk(6), h: 20, delta: [1, 1, -1, 0] },
            ],
            holonomy: Some([-4, 3, 3, -9]),
        },
    };
    for standing in [read, built, TStanding::default()] {
        let court = Reading::Court(Box::new(Court { k: 5, tuniversal: standing.clone(), ..Default::default() }));
        let mut wire = Vec::new();
        court.put(&mut wire).expect("the court writes");
        let mut at = 0;
        assert_eq!(Reading::take(&wire, &mut at), Some(court.clone()));
        assert_eq!(at, wire.len(), "read whole");
        // Another version of the standing is refused, never misread.
        let mut alone = Vec::new();
        standing.put(&mut alone).expect("the standing writes");
        let mut at = 0;
        assert_eq!(TStanding::take(&alone, &mut at), Some(standing));
        alone[0] = TSTANDING_VERSION + 1;
        let mut at = 0;
        assert_eq!(TStanding::take(&alone, &mut at), None);
    }
}

/// MU-CLIENT §5.3, §5.5 — A MIND'S READING AND A COURT'S AGREEMENTS travel
/// whole, their deviations exact rationals lane by lane, and a mind's
/// reading stands under its own source.
#[test]
fn a_minds_reading_and_a_courts_agreements_read_back_exact() {
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let mind = Mind {
        k: 9,
        heard: 4,
        understood: 3,
        returned: 2,
        priced: [5, 1, 2, 1],
        demanded: 2,
        constructed: [1, 0, 1, 0, 1, 2, 1],
        flex: [2, 1, 0, 3, 1, 1, 7, 2, 4],
        bodies: [4, 1, 2],
        perspective: [9, 3, 4, 1, 1, 6, 3, 1],
        disagree: [1, 0, 2, 0, 3, 1],
        asks: [5, 2, 1, 1],
        seats: [7, 2, 1, 3, 0],
        mutual: [4, 1],
        lexicon: 12,
        grown: 5,
        vocabulary: 40,
        remembered: 17,
        answers: [6, 1, 2],
        unanswered: 4,
        awaiting: 3,
        vantages: [5, 2, 7],
        repeats: [4, 9],
        laws: [3, 1, 0, 2],
        locks: vec![(1, 1, 3), (2, 1, 1)],
        drifts: 2,
        shears: 5,
        escapes: 1,
        held: 3,
        agreed: 2,
        composed: 8,
        shared: 1,
        closed: [q(1, 2), q(0, 1), q(0, 1), q(3, 7)],
        open: [q(-1, 3), q(2, 1), q(5, 4), q(0, 1)],
        thoughts: 6,
        unions: 1,
        waiting: 0,
    };
    let agreements = Agreements {
        pairs: vec![Agreed { speaker: vk(1), hearer: vk(2), agreed: 3, whole: 2, deviation: [q(1, 2), q(0, 1), q(0, 1), q(0, 1)] }],
    };
    reads_back(&[Reading::Mind(Box::new(mind.clone())), Reading::Agreements(Box::new(agreements))]);
    let key = Source::Mind(vk(3)).key(TAG_MIND);
    assert_eq!(Source::of_key(&key), Some((Source::Mind(vk(3)), TAG_MIND)), "a mind's reading stands under the mind");
    assert!(mind.lines().iter().any(|l| l.contains("deviation closed 1/2:0:0:3/7")), "the deviation is printed exact");
}
