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
        orbits: Orbits { withheld: 0, distance: vec![(vk(2), [1, -2, 0, 7])] },
    });
    let all = vec![
        Reading::Planes(planes),
        Reading::Loss(loss),
        Reading::Memory(memory),
        Reading::Court(court),
        Reading::Clock(court_clock),
        Reading::Clock(voice_clock),
        // EC1 — a court's payments by proof of work, each arm on its own
        // lanes.
        Reading::Economy(Economy { paid: [[5, 0, 2, 0], [0, 1, 0, 0], [0; 4]], burned: [[0; 4], [0; 4], [0, 0, 0, 3]] }),
    ];
    let push = encode_push(&vk(7), &all);
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(7), all.clone())));
    let entries: Vec<(Vec<u8>, Reading)> =
        all.iter().map(|r| (Source::Voice(vk(7)).key(r.tag()), r.clone())).collect();
    let reply = encode_reply(&entries, usize::MAX);
    assert_eq!(decode_reply(&reply), Some((7, entries.clone())));
    // A reply that cannot hold them all says how many were held.
    let (total, sent) = decode_reply(&encode_reply(&entries, 64)).expect("a short reply still decodes");
    assert_eq!(total, 7);
    assert!(sent.len() < 7, "the budget is honoured");
    // A trailing byte is no act.
    let mut long = encode_query(&[1, 2]);
    long.push(0);
    assert_eq!(decode_act(&long), None);
    assert_eq!(decode_act(&encode_query(&[1, 2])), Some(ObserveAct::Query(vec![1, 2])));
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
    let push = encode_push(&vk(8), &all);
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
    let circuits = Circuits {
        circuits: vec![
            Circuit {
                chord: 4,
                test: Some(CircuitTest {
                    arms: vec![(TestArm::OtherPairs, ArmRead::Reads(2)), (TestArm::Coset, ArmRead::Empty)],
                    class: CircuitClass::C,
                    offers: vec![Offer {
                        arm: TestArm::ConstructedFibre,
                        face: 9,
                        constructor: vec![(0b00001, q(1, 1)), (0b10000, q(-1, 2))],
                        flat: false,
                        holonomy: vec![(0, q(3, 1)), (0b00011, q(4, 1))],
                        modes: ModesRead::Split(vec![(BivectorKind::Elliptic, 1, None)]),
                    }],
                }),
                steps: vec![
                    CircuitStep {
                        face: 1,
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Elliptic, ModesRead::Split(vec![(BivectorKind::Elliptic, 1, Some(4))])),
                    },
                    CircuitStep {
                        face: 9,
                        forward: false,
                        refracted: true,
                        carried: read(
                            BivectorKind::Hyperbolic,
                            ModesRead::Undecomposed(cga::Undecomposed::Irrational(2)),
                        ),
                    },
                    CircuitStep { face: 2, forward: true, refracted: false, carried: Carried::Cone },
                ],
            },
            Circuit {
                chord: 7,
                test: Some(CircuitTest {
                    arms: vec![
                        (TestArm::OtherCircuits, ArmRead::Routes { agree: 1, disagree: 2 }),
                        (TestArm::Cone, ArmRead::Permanent),
                    ],
                    class: CircuitClass::Curved,
                    offers: Vec::new(),
                }),
                steps: vec![
                    CircuitStep { face: 3, forward: false, refracted: false, carried: Carried::Unread },
                    CircuitStep {
                        face: 5,
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Trivial, ModesRead::Unread),
                    },
                    CircuitStep {
                        face: 6,
                        forward: true,
                        refracted: false,
                        carried: read(BivectorKind::Parabolic, ModesRead::Undecomposed(cga::Undecomposed::Defective)),
                    },
                ],
            },
        ],
    };
    let push = encode_push(&vk(3), &[Reading::Circuit(circuits.clone())]);
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
                rings: crate::observe::Rings::default(),
            },
        ],
        bracket: Some(CellBracket { held: 2, lost: 5, unread: 1 }),
        cap: 25,
    };
    let push = encode_push(&vk(4), &[Reading::Cells(cells.clone())]);
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
        adjoined_by_closure: 1,
        installed: 4,
        closing: 2,
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
                uncited: 131,
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
    };
    let push = encode_push(&vk(8), &[Reading::Learning(curve.clone())]);
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
        (fresh.strata.len(), fresh.adjoined_by_closure, fresh.installed, fresh.closing),
        (0, 0, 0, 0),
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
    let push = encode_push(&vk(9), &all);
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
    let push = encode_push(&vk(4), &all);
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
    let (stakes, balances) = crate::economy::Treasury::take_tries(&tries, &mut 0).expect("the tries read back");
    let back = crate::economy::Treasury::from_parts(stakes, balances, t.entries().to_vec());
    assert!(back.escrow_of(&[7u8; 3]).is_none(), "the award rides its word's node, not the treasury's tries");
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
    // Genesis first: it names no predecessor, so the two learned nodes after
    // it are the same shape apart from the word each cites.
    let mut chain = ReceiptChain::with_vk([3; 32]);
    chain.append(ActKind::Stood, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::Learned, [0; 4], vec![cite(&small)], [0; 32], Vec::new()).expect("appends");
    chain.append(ActKind::Learned, [0; 4], vec![cite(&large)], [0; 32], Vec::new()).expect("appends");
    let mut c = Carriage::default();
    c.of_chain(&chain);
    let learned = c.rows.iter().find(|r| r.kind == ActKind::Learned.to_byte()).expect("the learned nodes");
    assert_eq!(learned.records, 2);
    assert_eq!(learned.largest * 2, learned.bytes, "both carry the same bytes, whatever the word they cite");
    assert!(c.rows.iter().any(|r| r.kind == ActKind::Stood.to_byte() && r.records == 1), "and the stood node by its own kind");
}
