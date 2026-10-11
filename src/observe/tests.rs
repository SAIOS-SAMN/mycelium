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
    let planes = Planes {
        n: 3,
        read: 7,
        coupled: 1,
        counts: vec![(0b00011, 5), (0b11000, 2)],
        parabolic: vec![(1, 0, 4), (2, 1, 1)],
    };
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
    // head (the gap is signed), and a voice before and after a persist.
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
    let voice_clock = Clock::Voice(VoiceClock {
        k: 7,
        heard: 39,
        persisted: None,
        radius: 4,
        point: Some(crate::chronometry::tuniversal::TPoint { h: 39, lanes: [1, 2, 3, 4], k: 5, ..Default::default() }),
    });
    let all = vec![
        Reading::Planes(planes),
        Reading::Loss(loss),
        Reading::Memory(memory),
        Reading::Court(Box::new(court)),
        Reading::Clock(court_clock),
        Reading::Clock(voice_clock),
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
    let push = encode_push(&vk(7), &all).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(7), all.clone())));
    let entries: Vec<(Vec<u8>, Reading)> =
        all.iter().map(|r| (Source::Voice(vk(7)).key(r.tag()), r.clone())).collect();
    let reply = encode_reply(&entries, usize::MAX).expect("the reply writes");
    assert_eq!(decode_reply(&reply), Some((7, entries.clone())));
    // A reply that cannot hold them all says how many were held.
    let short = encode_reply(&entries, 64).expect("the reply writes");
    let (total, sent) = decode_reply(&short).expect("a short reply still decodes");
    assert_eq!(total, 7);
    assert!(sent.len() < 7, "the budget is honoured");
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
    let all = vec![
        Reading::Replication(Replication { signers: vec![(vk(3), axes), (vk(4), [0; REP_AXES])], unattributed }),
        Reading::Adjudication(Adjudication {
            ratified: 5,
            stood: 9,
            awaiting: 1,
            deferred: 2,
            refused: vec![(0, 3), (2, 4)],
            obstructed: [1, 0, 3, 2, 0, 1],
        }),
    ];
    let push = encode_push(&vk(8), &all).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(8), all)));
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
    for s in [Source::Court(vk(1)), Source::Voice(vk(2))] {
        assert_eq!(Source::of_key(&s.key(TAG_LOSS)), Some((s, TAG_LOSS)));
    }
    assert_eq!(Source::of_key(&[9; 34]), None, "an unknown source kind");
}

/// THE PLANE CENSUS FOLDS PLANE BY PLANE, in the larger algebra, and never
/// adds one plane's count to another's (law 5).
#[test]
fn the_plane_census_folds_plane_by_plane() {
    use crate::transducer::cga::Pole;
    let mut a = Planes { n: 3, read: 2, coupled: 0, counts: vec![(0b00011, 2)], parabolic: vec![(1, 0, 2)] };
    let b = Planes {
        n: 4,
        read: 3,
        coupled: 1,
        counts: vec![(0b00011, 1), (0b10001, 3)],
        parabolic: vec![(1, 0, 1), (4, 1, 5)],
    };
    a.absorb(&b);
    assert_eq!(a.n, 4);
    assert_eq!((a.read, a.coupled), (5, 1));
    assert_eq!(a.count(0b00011), 3);
    assert_eq!(a.count(0b10001), 3);
    assert_eq!(a.count(0b00101), 0, "a plane never turned reads zero");
    assert_eq!(a.count_parabolic(1, Pole::Infinity), 3, "a parabolic plane folds onto its own");
    assert_eq!(a.count_parabolic(4, Pole::Origin), 5);
    assert_eq!(a.count_parabolic(1, Pole::Origin), 0, "and never onto its other pole");
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

/// THE CENSUS CLASSIFIES ITSELF, typed and read off its counts: nothing read;
/// every turn on the sensor's plane — FLATTENED, five other harmonic planes
/// declared and never turned; and a turn on a second plane of the base,
/// turning on two of the six. The transitions are noted by the engine's own
/// census move on rotors of the letters' chart.
#[test]
fn the_census_classifies_what_it_turned_on() {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let _ = cga::take_plane_tally();
    assert_eq!(Planes::of(&cga::take_plane_tally()).turning(), Turning::Unread);
    let still = cga::Mv::scalar(q(1, 1));
    let on_grid = Rotor::scalar(q(3, 5)).add(&Rotor::blade(0b0011, q(4, 5)));
    cga::note_transition(&on_grid, &still);
    let flat = Planes::of(&cga::take_plane_tally());
    assert_eq!(flat.turning(), Turning::Flattened { other: 5 }, "every turn on e1e2");
    cga::note_transition(&on_grid, &still);
    let on_03 = Rotor::scalar(q(3, 5)).add(&Rotor::blade(0b1001, q(4, 5)));
    cga::note_transition(&on_03, &still);
    let two = Planes::of(&cga::take_plane_tally());
    assert_eq!(two.turning(), Turning::Turns { turned: 2, of: 6 }, "a turn off the sensor's plane");
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

/// PL1 — THE CIRCUITS TRAVEL WHOLE, every arm of a step by name: read with
/// its ray, class, modes and composite; unread; on the cone. A later
/// percept's circuits stand, and the three owed fields are named by row.
#[test]
fn the_circuits_read_back_whole_step_by_step() {
    use crate::network::nerve::coord::BivectorKind;
    use crate::transducer::sentence::ModesRead;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let read = |kind, modes| Carried::Read {
        ray: vec![(0, q(1, 1)), (0b00011, q(-7, 3))],
        kind: Some(kind),
        modes,
        after: None,
    };
    // AT1 — a face is an overlap of two charts, never a slot number: the
    // `n`th distinct overlap the fixture names, the Frame's chart to a member
    // on chart `[n]` at frame `n`.
    let ov = |n: u64| crate::network::nerve::coord::Overlap {
        from: crate::network::nerve::coord::ChartAt::Frame,
        to: crate::network::nerve::coord::ChartAt::at(
            crate::network::nerve::coord::VertexAt::Frame,
            crate::transducer::stalk::Invariant::counts(&[n]),
            n,
        ),
    };
    let circuits = Circuits {
        circuits: vec![
            Circuit {
                chord: ov(4),
                test: Some(CircuitTest {
                    arms: vec![(TestArm::OtherPairs, ArmRead::Reads(2)), (TestArm::Coset, ArmRead::Empty)],
                    class: CircuitClass::C,
                    offers: vec![Offer {
                        arm: TestArm::ConstructedFibre,
                        face: ov(9),
                        constructor: vec![(0b00001, q(1, 1)), (0b10000, q(-1, 2))],
                        flat: false,
                        holonomy: vec![(0, q(3, 1)), (0b00011, q(4, 1))],
                        modes: ModesRead::Split(vec![(BivectorKind::Elliptic, 1, None)]),
                    }],
                }),
                transitions: vec![
                    CircuitTransition {
                        face: ov(1),
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Elliptic, ModesRead::Split(vec![(BivectorKind::Elliptic, 1, Some(4))])),
                    },
                    CircuitTransition {
                        face: ov(9),
                        forward: false,
                        refracted: true,
                        carried: read(
                            BivectorKind::Hyperbolic,
                            ModesRead::Undecomposed(cga::Undecomposed::Irrational(2)),
                        ),
                    },
                    CircuitTransition { face: ov(2), forward: true, refracted: false, carried: Carried::Cone },
                ],
            },
            Circuit {
                chord: ov(7),
                test: Some(CircuitTest {
                    arms: vec![
                        (TestArm::OtherCircuits, ArmRead::Routes { agree: 1, disagree: 2 }),
                        (TestArm::Cone, ArmRead::Permanent),
                    ],
                    class: CircuitClass::Curved,
                    offers: Vec::new(),
                }),
                transitions: vec![
                    CircuitTransition { face: ov(3), forward: false, refracted: false, carried: Carried::Unread },
                    CircuitTransition {
                        face: ov(5),
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Trivial, ModesRead::Unread),
                    },
                    CircuitTransition {
                        face: ov(6),
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Parabolic, ModesRead::Undecomposed(cga::Undecomposed::Defective)),
                    },
                ],
            },
        ],
    };
    let push = encode_push(&vk(3), &[Reading::Circuit(circuits.clone())]).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(3), vec![Reading::Circuit(circuits.clone())])));
    assert_eq!(circuits.circuits[0].first_refraction(), Some(1), "the refracting step is named");
    assert_eq!(circuits.circuits[1].first_refraction(), None);
    let mut earlier = Reading::Circuit(Circuits::default());
    assert!(earlier.absorb(&Reading::Circuit(circuits.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Circuit(circuits), "the later percept's circuits stand");
    assert_eq!(
        CIRCUIT_OWED.map(|(field, _)| field),
        ["constructors composed", "round carried", "lane drawn"],
        "the fields no producer reads are owed by their rows, never zero"
    );
}

/// CO1 — THE CELLS TRAVEL WHOLE: every closure, every dual arm, a product
/// with its seal blade, each nucleus's mirrors; a later percept's stand.
#[test]
fn the_cells_read_back_whole() {
    use crate::transducer::cga::PairRegime;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let cells = Cells {
        mirror_planes: 5,
        mirror_group: Some(CellClosure::Affine),
        cells: vec![
            CellEntry {
                grade: 1,
                round: CellRound::HeldZ2,
                permutes: CellPermutes::Moves,
                coherence: CellCoherence::Closes,
                capacity: Some(3),
                orbit: CellOrbit::Unbounded,
                signature: Some(((1, 1, 0), 2)),
                pairs: vec![PairRegime::Boost],
                coxeter: Some(CellClosure::Hyperbolic),
                polars: Some(CellClosure::InfiniteByPair),
                dual: Some(CellDual::Frame),
                products: vec![CellProduct {
                    order: vec![1, 0],
                    grades: vec![0, 2],
                    det: Some(1),
                    classed: false,
                    seal: vec![(0b00011, q(-7, 3))],
                }],
                classes: 0,
                mirrors: vec![vec![(1, q(0, 1), q(1, 4), PairRegime::Finite(3))], vec![]],
                enclosure: Some(CellEnclosure::Round { bodies: [1, 2, 0, 1, 0], within: Some(1) }),
                // CV1b — every arm of the refusals by content travels.
                refusals: CellRefusals {
                    circuits: CircuitUnread::ALL.to_vec(),
                    modes: vec![
                        ModesUnread::NotVersor,
                        ModesUnread::Irrational(3),
                        ModesUnread::Defective,
                        ModesUnread::Unbounded(2),
                    ],
                    capacity: Some(CapacityRefused::Spent { product: 12 }),
                    zero_meet: true,
                    nuclei: false,
                    unordered: 2,
                    orbit: Some(OrbitUnread::NoRay { constructor: 1 }),
                },
                constructors: Some(2),
                degeneracy: Some(1),
                // 78.26(5) — every ring axis travels, each its own value.
                rings: crate::observe::Rings {
                    coherent: 1,
                    resonant: 2,
                    rosette: 3,
                    parabolic: 4,
                    boost: 5,
                    harmonic: 6,
                    luminal: 7,
                    holonomic: 8,
                    d: 9,
                    c: 10,
                },
            },
            CellEntry {
                grade: 2,
                round: CellRound::Carried,
                permutes: CellPermutes::NoCircuit,
                coherence: CellCoherence::Unread,
                capacity: None,
                orbit: CellOrbit::Closed,
                signature: None,
                pairs: vec![PairRegime::Rosette, PairRegime::Finite(6), PairRegime::Parabolic],
                coxeter: Some(CellClosure::FiniteByCheck),
                polars: None,
                dual: Some(CellDual::Redundant),
                products: vec![CellProduct { order: vec![0, 1, 2], grades: vec![1, 3], det: Some(-1), classed: true, seal: vec![] }],
                classes: 1,
                mirrors: vec![],
                enclosure: Some(CellEnclosure::Flat),
                refusals: CellRefusals {
                    capacity: Some(CapacityRefused::Spent { product: 7 }),
                    nuclei: true,
                    orbit: Some(OrbitUnread::CarriedNoRay),
                    ..CellRefusals::default()
                },
                constructors: None,
                degeneracy: None,
                rings: crate::observe::Rings::default(),
            },
        ],
        bracket: Some(CellBracket { held: 2, lost: 5, unread: 1 }),
        cap: 25,
    };
    let push = encode_push(&vk(4), &[Reading::Cells(cells.clone())]).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(4), vec![Reading::Cells(cells.clone())])));
    let mut earlier = Reading::Cells(Cells::default());
    assert!(earlier.absorb(&Reading::Cells(cells.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Cells(cells), "the later percept's cells stand");
}

/// LC1 — THE CURVE TRAVELS WHOLE, A LATER ONE STANDS, AND A FRESH MIND READS
/// ZERO HONESTLY: every field measured and zero, the four owed named by row.
#[test]
fn the_learning_curve_reads_back_whole_and_a_fresh_mind_reads_zero() {
    let curve = Learning {
        k: 41,
        strata: vec![(vec![1, 2], 3), (vec![1, 2, 5], 1)],
        radius: 3,
        // CS2 — every arm of the pivot reading travels.
        pivots: crate::transducer::sentence::PivotTally {
            reached: 1,
            derivable: 2,
            demanded: 3,
            adjoin: 4,
            starved: 5,
            unread_fibre: 6,
            unread_algebra: 7,
        },
        // 13.12 — every arm of the merges travels.
        merges: crate::transducer::bubble::merge::MergeTally { merged: 8, stayed: 9, unnamed: 10 },
        // TP1 — every field of the meet graph travels, the cycle rank signed.
        meet: Box::new(crate::transducer::bubble::MeetGraph {
            vertices: 11,
            thoughts: 12,
            components: 13,
            edges: [1, 2, 3, 4, 5, 6],
            cycle_rank: -7,
            rings: [7, 8, 9, 10, 11, 12],
            orbits: [13, 14, 15, 16, 17, 18],
            decoherence: crate::transducer::bubble::Decoherence {
                decohered: [[19, 20], [25, 26]],
                classed: [[21, 22], [27, 28]],
                rejoined: [[23, 24], [29, 30]],
            },
            advanced: 31,
            sources: 32,
            transport: crate::transducer::bubble::TransportTally {
                fixed: [33, 34],
                collapsed: 35,
                collapsed_filled: 36,
                held: 37,
                classified: 38,
                unsectioned: 39,
                ungauged: 40,
                unclassed: 41,
            },
            docket: crate::transducer::bubble::DocketReading {
                pending: 42,
                open: 43,
                owed: 44,
                unwritten: 45,
                answered: 46,
                taken: 47,
                lapsed: 300,
            },
            // TC1 — the nerve's shape and every door, a rank lost negative.
            core: crate::transducer::bubble::CoreReading {
                free_vertices: 48,
                isolated: 49,
                free_faces: [50, 51],
                core: 52,
                h1: -3,
                unfaced: 53,
                malformed: 66,
            },
            shed: crate::transducer::bubble::ShedTally {
                removed: [54, 55, 56],
                in_core: [57, 58, 59],
                rank_lost: [60, -61, 62],
            },
            // TC1's fix — the truncation, a Ȟ¹ movement negative.
            truncation: crate::transducer::bubble::Truncation {
                truncated: 67,
                held_by_class: 68,
                glued: 69,
                deferred: 70,
                rank_moved: -71,
                malformed: 73,
            },
            // FX1 — the flux, every window and count, one optional unread.
            flux: crate::transducer::bubble::FluxTally {
                closes: 74,
                orbit: crate::transducer::bubble::FluxWindow { traversed: 75, zero: 76, held: 77 },
                aperture: Some(crate::transducer::bubble::FluxWindow { traversed: 78, zero: 79, held: 80 }),
                gone: 81,
                unfilamented: 82,
                credited: 83,
                acyclic: 84,
                unfiled: 85,
                unsealed: 86,
                unnamed: 87,
                extent: [[88, 89, 90, 91], [92, 93, 94, 95], [96, 97, 98, 99], [100, 101, 102, 103], [104, 105, 106, 107]],
                extent_n: [108, 109, 110, 111, 112],
                dialect: (113, Some(114), None),
            },
            // RM1 — the mind's remainders, one unplaced.
            remainders: crate::transducer::cech::RemainderReading {
                by_ell: vec![(3, 115)],
                classes: 116,
                across: 117,
                unplaced: 118,
                refilled: 119,
                refused: 120,
                past_carrier: 159,
                misspoken: 160,
                unsited: 123,
                unrecorded: 121,
                unpriced: 0,
                unminted: 124,
                unoffered: 0,
                unmerged: 125,
                coincident: 122,
                // RM1's receptor, every count distinct.
                nested: 126,
                discordant: 127,
                calluses: 128,
                unspelled: 129,
                untopped: 130,
                callus_unminted: 132,
                receive_malformed: 133,
                receive_unrecorded: 134,
                answered_skipped: 135,
                refill_acts: 136,
                refill_refused: 137,
                refill_unshaped: 138,
                refill_unlocated: 139,
                refill_unspelled: 140,
                refill_unopen: 141,
                refill_unsited: 142,
                refill_unfilled: 143,
            },
            // STD1 — the sums the contemplation refused.
            overflowed: 158,
            // N4 — perception across scenes, every count distinct.
            resonance: crate::transducer::bubble::ResonanceTally {
                circuits: 200,
                uncarried: 201,
                not_reciprocal: [202, 203, 204],
                unbounded: 205,
                cells: 206,
                closes: 207,
                open: 208,
                cells_unread: 209,
                resonant: 210,
                rosette: 211,
                parabolic: 212,
                boost: 213,
                holonomic: 214,
                unperturbed: 215,
                flat: 216,
                curved: 217,
                edges: 218,
                several: 219,
                kept_congruent: 220,
                kept_class: 221,
                predicted: 222,
                phase_slips: 223,
                off_orbit: 224,
                deformed: 227,
                unfactored: 228,
                apart: 229,
                scene_motions: 230,
                cosets: 231,
                symmetries: 232,
                appeared: 233,
                vanished: 235,
                unread_lines: 234,
                consolidated: 236,
                monocular: 237,
                collapsed: 238,
                unbalanced: 239,
                destabilizing: 240,
                tilted: 251,
                stability_unread: 252,
                reflexive: 241,
                nonrigid: 242,
                unsucceeded: 243,
                torqued: 244,
                incoherent: 246,
                demanded: 225,
                undemanded: 226,
                grounds: [247, 248, 249, 250],
            },
        }),
        // PR1 — the voice's probes, a residual class counted twice.
        probes: crate::observe::ProbeTally {
            closed: 144,
            residual: 145,
            residual_unread: 146,
            nothing: 147,
            gone: 148,
            unclassed: 149,
            settled: [150, 151, 152, 153],
            remaining: [154, 155, 156, 157],
            classes: vec![(vec![1, 2, 3], 2), (vec![4], 1)],
        },
        // 78.47 — the reasons by kind travel whole.
        reasons: crate::observe::Reasons {
            refused: [159, 160, 161, 162, 163, 164],
            undischarged: [165, 166, 167, 168, 169, 170],
            perturbed: [171, 172, 173, 174, 175, 176, 177],
        },
    };
    let push = encode_push(&vk(8), &[Reading::Learning(curve.clone())]).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(8), vec![Reading::Learning(curve.clone())])));
    let mut earlier = Reading::Learning(Learning { k: 3, ..Learning::default() });
    assert!(earlier.absorb(&Reading::Learning(curve.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Learning(curve), "the curve at the later K stands");

    let fresh = Learning::of(
        &crate::client::sdk::Epigenome::default(),
        0,
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );
    assert_eq!(
        fresh.strata.len(),
        0,
        "a mind holding nothing has learned nothing, and says so"
    );
    assert_eq!(
        OWED.map(|(field, _)| field),
        [
            "cover",
            "warm-cold delta",
            "distinct perspectives per class",
            "mass per lane per layer per grade",
        ],
        "the fields no producer reads are owed by their rows, never zero; the coboundary's agreement is PG1's"
    );
}

/// VO1, SP1, PG1 — THE LIVE FLEET'S READINGS TRAVEL WHOLE AND SUM PER AXIS: a
/// voice's viscosity pair by pair (a new pair filed in order, the ledger's
/// entropy the later), its spend reader by reader, its progress grade by
/// grade — never across axes (law 5).
#[test]
fn viscosity_spend_and_progress_read_back_whole_and_sum_per_axis() {
    let pair = |lane, planes, arms| ViscosityPair { lane, planes, arms, refused: 1, unread: 0, no_class: 2 };
    let v = Viscosity { pairs: vec![pair(0, (3, 5), [1, 2, 0, 0]), pair(3, (0, 1), [0, 0, 4, 1])], entropy: [0, 0, 0, 1] };
    let s = Spend { percepts: 1, reads: [10, 2, 30, 4, 50], unbounded: [0, 1, 0, 0, 3] };
    let g = GradeProgress { grade: 2, cells: 3, round_kept: 1, held_z2: 1, refused: 1, finite: 1, affine: 1, infinite: 0, dual_frames: 2 };
    let p = Progress {
        percepts: 1,
        cycles_curved: 7,
        contact_cycles: 5,
        contact_cycles_curved: 2,
        contact_cycles_unread: 1,
        contact_cycles_static: 1,
        contact_cycles_coset: [0, 1, 1],
        grades: vec![g.clone()],
    };
    // PH5 — a court's phase travels whole beside them.
    let phase = Phase { windows: 3, h0: 986, h1: 989, held: 1, sections: 1, strata: 2, installed: 1, ..Phase::default() };
    let all = [
        Reading::Viscosity(v.clone()),
        Reading::Spend(s.clone()),
        Reading::Progress(p.clone()),
        Reading::Phase(phase.clone()),
    ];
    let push = encode_push(&vk(9), &all).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(9), all.to_vec())));

    let mut held = Reading::Viscosity(v.clone());
    let later = Viscosity { pairs: vec![pair(0, (1, 2), [1, 0, 0, 0]), pair(0, (3, 5), [1, 0, 0, 0])], entropy: [0, 0, 0, 2] };
    assert!(held.absorb(&Reading::Viscosity(later)));
    let Reading::Viscosity(held) = held else { panic!("one kind folds into its own kind") };
    assert_eq!(held.pairs.iter().map(|q| (q.lane, q.planes, q.arms)).collect::<Vec<_>>(), [
        (0, (1, 2), [1, 0, 0, 0]),
        (0, (3, 5), [2, 2, 0, 0]),
        (3, (0, 1), [0, 0, 4, 1])
    ]);
    assert_eq!(held.entropy, [0, 0, 0, 2], "the ledger's entropy is cumulative: the later stands");

    let mut spent = Reading::Spend(s.clone());
    assert!(spent.absorb(&Reading::Spend(s)));
    assert_eq!(spent, Reading::Spend(Spend { percepts: 2, reads: [20, 4, 60, 8, 100], unbounded: [0, 2, 0, 0, 6] }));

    let mut progressed = Reading::Progress(p.clone());
    let other = Progress { percepts: 1, grades: vec![GradeProgress { grade: 1, cells: 1, ..GradeProgress::default() }, g], ..Progress::default() };
    assert!(progressed.absorb(&Reading::Progress(other)));
    let Reading::Progress(progressed) = progressed else { panic!("one kind folds into its own kind") };
    assert_eq!((progressed.percepts, progressed.contact_cycles), (2, 5));
    assert_eq!(progressed.grades.iter().map(|g| (g.grade, g.cells, g.round_kept)).collect::<Vec<_>>(), [(1, 1, 0), (2, 6, 2)]);
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
    let all = [Reading::Carriage(c)];
    let push = encode_push(&vk(4), &all).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(4), all.to_vec())));
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

/// LD1, 78.24 — A LEDGER'S NODES ARE MEASURED BY THE ACT KIND THAT SEALED
/// THEM, and a node citing a word carries the same bytes whatever the size
/// of the word: the citation is the word's fiber and admitting stamp, never
/// a copy of it.
/// THE FACULTIES AND THE ACTS TRAVEL WHOLE (2026-09-29): a mind's `Senses`
/// and a court's `Acts` go out in a push and come back as they were, every
/// counter in its faculty; a reading of either kind later stands.
#[test]
fn the_senses_and_the_acts_travel_whole() {
    let mut senses = Senses { k: 7, ..Senses::default() };
    for (i, x) in senses.sight.iter_mut().chain(senses.cognition.iter_mut()).chain(senses.carriers.iter_mut()).enumerate() {
        *x = 300 + i as u64;
    }
    senses.speech = [1, 2, 3, 4, 5, 39];
    senses.reasons = [40, 41, 42, 43, 44, 45, 46, 47, 48];
    senses.derivation = [50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63];
    senses.flux = [70, 71, 72, 73, 74, 75, 76, 77];
    senses.sentences = [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111];
    senses.unread = [130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161];
    senses.citation = [170, 171, 172, 173];
    senses.encounter = [180, 181, 182, 183, 184];
    senses.constructions = [190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205];
    senses.harmonics = [210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228];
    senses.said = Some(crate::transducer::sentence::Reasons {
        words: vec![crate::transducer::sentence::reasons::WordReason { generators: 2, adjoined: false, charts: 3, frames: 4 }],
        loops: vec![crate::transducer::sentence::reasons::LoopReason {
            curved: true,
            axes: vec![crate::transducer::sentence::Axis::Hue, crate::transducer::sentence::Axis::Placement],
        }],
        open: vec![crate::transducer::sentence::reasons::OpenReason { kind: 3, measure: [5, 0, 0, 1], pieces: 2 }],
        unsaid: 6,
        measure: [7, 8, 9, 10],
    });
    senses.hearing = [6, 7, 8, 30, 31, 32, 33, 34, 35, 36];
    senses.language = [9, 10, 29];
    senses.understanding = [14, 15, 16, 17, 38];
    senses.solving = [18, 19, 20, 21, 26, 27, 28, 58];
    senses.paid = [22, 23, 24, 25];
    {
        use crate::economy::modes::{Mode, Modes};
        use crate::transducer::sentence::presentation::GaugeMode;
        let mut m = Modes::new();
        m.add_mode(Mode::Hue(GaugeMode::Move(3, 0)), num_rational::BigRational::new(5.into(), 2.into()));
        m.add_mode(Mode::Hue(GaugeMode::Cycle(vec![1, 2])), num_rational::BigRational::from_integer(1.into()));
        senses.paid_modes = Some(m);
    }
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
    let all = vec![Reading::Senses(Box::new(senses.clone())), Reading::Acts(Box::new(acts.clone()))];
    let push = encode_push(&vk(8), &all).expect("the push writes");
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(8), all)));
    assert_eq!(senses.read("hearing", "words admitted"), Some(8), "read by its own name");
    assert_eq!(senses.read("solving", "refused"), Some(20));
    assert_eq!(senses.read("solving", "refuted at home"), Some(27));
    assert_eq!(senses.read("solving", "unread"), Some(58), "78.153c — unread, apart from refused");
    assert_eq!(senses.read("sight", "no such"), None, "a name the table does not hold reads nothing");
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
    holds(&HEARING, &["pages", "words heard", "words admitted"]);
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

/// 78.153c — A MIND'S VERDICTS BY THE REPLY'S CODE: unread is a retry and
/// never a refusal, a refusal is counted by its kind, a code this build does
/// not know is counted last; and each verdict says itself in one line.
#[test]
fn a_minds_verdicts_are_kept_by_code_and_said_one_line_each() {
    use crate::anastome::dialogue::{SolveRefusal, SOLVE_BOUNTY, SOLVE_NOTHING, SOLVE_UNREAD};
    let unread = SolveRefusal { reason: b"not judged yet".to_vec(), kind: 4 };
    let no_work = SolveRefusal { reason: b"no work".to_vec(), kind: 4 };
    let unsigned = SolveRefusal { reason: b"unsigned".to_vec(), kind: 3 };
    let mut t = SolveTally::default();
    t.note(SOLVE_UNREAD, &[0; 4], Some(&unread));
    t.note(SOLVE_UNREAD, &[0; 4], Some(&unread));
    t.note(SOLVE_NOTHING, &[0; 4], Some(&no_work));
    t.note(SOLVE_NOTHING, &[0; 4], Some(&unsigned));
    t.note(SOLVE_BOUNTY, &[1, 0, 2, 0], None);
    t.note(9, &[0; 4], None);
    assert_eq!(t.by_code, [2, 0, 1, 2, 1], "nothing 2, floor 0, bounty 1, unread 2, unknown 1");
    assert_eq!(t.unread, 2, "unread apart");
    assert_eq!(t.refused, [0, 0, 0, 1, 1, 0, 0], "the two refusals by kind, I and N; no unread among them");
    assert_eq!((t.first, t.unruled, t.paid), (1, 1, [1, 0, 2, 0]));
    assert_eq!(
        t.line(),
        "solves nothing 2 floor 0 bounty 1 unread 2 unknown 1 · refused (P D C I N ? x) [0, 0, 0, 1, 1, 0, 0] unruled 1 · kept home unminted 0 refuted 0 withheld 0"
    );
    let task = [0xab; 32];
    assert_eq!(
        SolveTally::verdict_line(&task, SOLVE_UNREAD, Some(&unread)),
        "solve verdict · task abababab… · code unread (3) · kind N · not judged yet"
    );
    assert_eq!(SolveTally::verdict_line(&task, SOLVE_BOUNTY, None), "solve verdict · task abababab… · code bounty (2) · kind none");
    assert_eq!(
        SolveTally::verdict_line(&task, 7, Some(&SolveRefusal { reason: Vec::new(), kind: 9 })),
        "solve verdict · task abababab… · code unknown code (7) · kind unknown kind · "
    );
}

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
