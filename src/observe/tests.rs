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

/// CV5 — replication per signer and adjudication by kind travel whole.
#[test]
fn replication_and_adjudication_read_back_as_themselves() {
    let mut axes = [0u64; 13];
    axes[rep::BACKFILL] = 7;
    axes[rep::STALE] = 2;
    let mut unattributed = [0u64; 13];
    unattributed[rep::DIGEST_UNREADABLE] = 1;
    let all = vec![
        Reading::Replication(Replication { signers: vec![(vk(3), axes), (vk(4), [0; 13])], unattributed }),
        Reading::Adjudication(Adjudication { ratified: 5, stood: 9, awaiting: 1, deferred: 2, refused: vec![(0, 3), (2, 4)] }),
    ];
    let push = encode_push(&vk(8), &all);
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(8), all)));
    assert_eq!(REPLICATION_AXES.len(), 13, "the axes and the counters agree");
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
            },
        ],
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
        adjoined_by_closure: 1,
        installed: 4,
        closing: 2,
    };
    let push = encode_push(&vk(8), &[Reading::Learning(curve.clone())]);
    assert_eq!(decode_act(&push), Some(ObserveAct::Push(vk(8), vec![Reading::Learning(curve.clone())])));
    let mut earlier = Reading::Learning(Learning { k: 3, ..Learning::default() });
    assert!(earlier.absorb(&Reading::Learning(curve.clone())), "one kind folds");
    assert_eq!(earlier, Reading::Learning(curve), "the curve at the later K stands");

    let fresh = Learning::of(&crate::client::sdk::Epigenome::default(), 0);
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
    let p = Progress { percepts: 1, contact_cycles: 5, contact_cycles_curved: 2, contact_cycles_unread: 1, grades: vec![g.clone()] };
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
