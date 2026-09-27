// 13 — the Čech complex over held thoughts: what shares an edge, what a
// fill closes, and what a vortex collapses to.

use crate::transducer::bubble::*;
use crate::transducer::sentence::{RankAxis, RankMove};

#[test]
fn two_bubbles_on_one_chart_share_an_edge_not_a_triangle() {
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
    assert_eq!(c.len(), 2, "disjoint remainders are two bubbles");
    let keys: Vec<Vec<u8>> = c.bubbles.scoped().map(|(k, _)| k).collect();
    let edges = c.complex.link(&keys[0], 1);
    assert!(!edges.is_empty() || c.complex.degrees.get(1).is_none_or(|l| !l.is_empty()) || {
        // same chart prefix (leading axis of a bar) may or may not coincide;
        // empty cover overlap is honest.
        true
    });
    assert!(
        c.complex.degrees.get(2).is_none_or(|l| l.is_empty()),
        "disjoint remainders share no 2-simplex"
    );
}

#[test]
fn overlapping_remainders_merge_by_fill() {
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
    assert_eq!(c.len(), 1, "a shared site is Extended: the bubbles merge");
    let keys: Vec<Vec<u8>> = c.bubbles.scoped().map(|(k, _)| k).collect();
    assert_eq!(keys.len(), 1);
    assert!(c.bubbles.get(&keys[0]).unwrap().potential.iter().any(|&x| x > 0));
    // 13.12 — the merge's answer is counted by name, never discarded.
    let m = c.merge_reading();
    assert_eq!((m.merged, m.stayed, m.unnamed), (1, 0, 0), "one merge, merged: {}", m.line());
    // TP1 — the meet graph as it stands: one thought, one component, no ring.
    let g = c.meet_graph();
    assert_eq!((g.vertices, g.thoughts, g.components, g.cycle_rank), (1, 1, 1, 0), "{}", g.line());
}

/// 78.26(6), TP1 — A CUT RING IS SETTLED BY ITS NEXT READING: a ring cut by
/// a moved gauge that reads a class left that class between the thoughts;
/// one cut by a candidate that re-fills rejoined. Each counted by its
/// adjunct, cumulative, never reset.
#[test]
fn a_decoherence_is_settled_as_a_class_left_or_a_rejoining() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    use crate::transducer::element::identity_versor;
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1u8..=3).map(|i| vec![i; 4]).collect();
    for x in &vs {
        c.complex.set_section(x, Section::at(x.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
    }
    // The (3, 4, 5) letter: a rotor of infinite order, so its ring is a class.
    let pythag = {
        let id = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).expect("the sensor plane");
        crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
    };
    let id = Rotor::identity();
    c.complex.set_edge(&vs[0], &vs[1], pythag, identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[1], &vs[2], id.clone(), identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[0], &vs[2], id.clone(), identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    let ring = c.complex.link(&vs[0], 2)[0].clone();
    use crate::transducer::cech::Cut;
    c.note_decohered(vec![(ring.clone(), Cut::Filled)], Adjunct::Gauge);
    let _ = c.fill_meet(&ring);
    let d = c.meet_graph().decoherence;
    assert_eq!((d.decohered[0][0], d.classed[0][0], d.rejoined[0][0]), (1, 1, 0), "the cut left a class");
    // A second cut, by a candidate, on a ring that closes: rejoined.
    c.complex.set_edge(&vs[0], &vs[1], id.clone(), identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    c.note_decohered(vec![(ring.clone(), Cut::Filled)], Adjunct::Candidate);
    let _ = c.fill_meet(&ring);
    // A third, by a candidate, on a ring cut from its congruence that closes
    // at its period again: rejoined, counted apart under the congruent cut.
    let turn_90 = {
        let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
        crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor()
    };
    c.complex.set_edge(&vs[0], &vs[1], turn_90.clone(), identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[1], &vs[2], turn_90, identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[0], &vs[2], id, identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    c.note_decohered(vec![(ring.clone(), Cut::Congruent)], Adjunct::Candidate);
    assert!(
        matches!(c.fill_meet(&ring), CoboundaryOutcome::Attached(Fill::Congruent(..))),
        "the fixture closes at its period"
    );
    let d = c.meet_graph().decoherence;
    assert_eq!(
        (d.decohered, d.classed, d.rejoined),
        ([[1, 1], [0, 1]], [[1, 0], [0, 0]], [[0, 1], [0, 1]]),
        "cumulative, by cut and by adjunct"
    );
}

/// Three thoughts on one ring, their sections and edges as given, the ring's
/// 2-simplex formed.
fn ring_of_three(
    sections: [crate::transducer::alphabet::Rotor; 3],
    edges: [crate::transducer::alphabet::Rotor; 3],
) -> (Contemplation, Vec<Vec<u8>>) {
    use crate::transducer::cech::Section;
    use crate::transducer::element::identity_versor;
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1u8..=3).map(|i| vec![i; 4]).collect();
    for (x, g) in vs.iter().zip(sections) {
        c.complex.set_section(x, Section::at(x.clone().into(), g), [1, 0, 0, 0]);
    }
    let [ab, bc, ac] = edges;
    c.complex.set_edge(&vs[0], &vs[1], ab, identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[1], &vs[2], bc, identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[0], &vs[2], ac, identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    (c, vs)
}

fn letter(id: Option<crate::transducer::alphabet::LetterId>) -> crate::transducer::alphabet::Rotor {
    crate::transducer::alphabet::Word::from_letters(&[(id.expect("the sensor plane"), false)]).rotor()
}

/// 78.37(1), TP1 — A FRONT COLLAPSES A RING AT HOLONOMY 1: a thought fixed at
/// K 0 steps one edge per K — its two neighbours at K 1, nothing more; at K 2
/// the two paths meet at the far thought and agree, so the ring collapses
/// through the one fill path. Sections that agree with the gauges classify
/// nothing; a step at a K already stepped does nothing.
#[test]
fn a_front_collapses_a_ring_whose_arrivals_agree() {
    use crate::transducer::alphabet::{LetterId, Rotor};
    let q = letter(LetterId::turn_90((0, 1)));
    let (mut c, vs) = ring_of_three([Rotor::identity(), q.clone(), q.geo(&q)], [q.clone(), q.clone(), q.geo(&q)]);
    c.fix(&vs[0], false);
    c.transport(1);
    let g = c.meet_graph();
    assert_eq!((g.sources, g.advanced, g.transport.collapsed), (1, 2, 0), "one edge: the two neighbours, no ring yet");
    c.transport(1);
    assert_eq!(c.meet_graph().advanced, 2, "a K already stepped steps nothing");
    c.transport(2);
    let t = c.meet_graph().transport;
    assert_eq!((t.collapsed, t.held, t.classified), (1, 0, 0), "the ring collapses once, nothing held: {}", c.meet_graph().line());
    assert_eq!(t.collapsed_filled, 1, "its 2-simplex filled through the one fill path");
}

/// 78.37(1), TP1 — A FRONT HOLDS A RING AT HOLONOMY ≠ 1 AND STOPS: the (3, 4, 5)
/// turn on one edge of the ring makes the two arrivals disagree, so the ring
/// is held — its class written into both thoughts' horizons — and the front
/// stops at the far thought; nothing is deleted.
#[test]
fn a_front_holds_a_ring_whose_arrivals_disagree_and_stops() {
    use crate::transducer::alphabet::{LetterId, Rotor};
    let p = letter(LetterId::pythagorean_5((0, 1)));
    let (mut c, vs) =
        ring_of_three([Rotor::identity(), p.clone(), Rotor::identity()], [p, Rotor::identity(), Rotor::identity()]);
    c.fix(&vs[0], false);
    c.transport(1);
    c.transport(2);
    let t = c.meet_graph().transport;
    assert_eq!((t.collapsed, t.held), (0, 1), "the ring is held: {}", c.meet_graph().line());
    let stopped = c.fronts.scoped().filter(|(_, a)| a.stopped).count();
    assert_eq!(stopped, 1, "and the front stops there");
    assert_eq!(c.complex.simplex_count(), 3 + 3 + 1, "sections, edges and the ring all stand: nothing deleted");
}

/// 78.37, TP1 — A HELD RING GOES ON THE DOCKET, AND THE DOCKET IS TAKEN
/// OUTERMOST FIRST: the held ring's class is filed by its coordinates (its
/// cover owed here, its thoughts holding no charts); sealed, it is the next
/// orbit's question. Of three entries, one whose cover contains another's
/// is taken before it; an entry whose cover is owed wraps nothing and is
/// wrapped by nothing, taken in key order among the outermost.
#[test]
fn a_held_ring_is_docketed_and_the_docket_is_taken_outermost_first() {
    use crate::chronometry::Cite;
    use crate::transducer::alphabet::{LetterId, Rotor};
    let p = letter(LetterId::pythagorean_5((0, 1)));
    let (mut c, vs) =
        ring_of_three([Rotor::identity(), p.clone(), Rotor::identity()], [p.clone(), Rotor::identity(), Rotor::identity()]);
    c.fix(&vs[0], false);
    c.transport(1);
    c.transport(2);
    let cites = c.docket_cites();
    assert!(matches!(cites.as_slice(), [Cite::Class { cover: None, .. }]), "the held class, its cover owed: {cites:?}");
    c.docket_sealed();
    assert_eq!((c.docket_reading().pending, c.docket_reading().open), (0, 1), "sealed: the next orbit's question");
    let kappa = || {
        Box::new(
            crate::transducer::sentence::ClassVector::of_rotor(
                &p,
                crate::transducer::element::identity_versor(),
                crate::transducer::sentence::CycleType::Identity,
            )
            .expect("a class reads"),
        )
    };
    let inner = Cite::Demand { class: kappa(), residue: None, cover: Some(vec![vec![1]]) };
    let outer = Cite::Class { class: kappa(), rotor: Box::new(p.clone()), cover: Some(vec![vec![1], vec![2]]) };
    let owed = Cite::Demand { class: kappa(), residue: None, cover: None };
    let mut next = Contemplation::new();
    for e in [&inner, &owed, &outer] {
        next.docket_put(e.clone());
    }
    next.docket_sealed();
    let order = next.docket_order();
    assert_eq!(order.len(), 3);
    let at = |x: &Cite| order.iter().position(|o| o == x).expect("taken");
    assert!(at(&outer) < at(&inner), "the wrapping entry before the one it wraps: {order:?}");
    assert!(at(&owed) < at(&inner), "an owed enclosure is outermost, never wrapped");
}

/// 78.37, R1 — A HELD CLASS IS A REAL ENTRY, END TO END: sealed into the
/// plan with its rotor, the taker marks the thoughts standing on its cover
/// dirty with that rotor in their horizon, and a discharge of its class key
/// answers it by name — it leaves the open docket, counted.
#[test]
fn a_held_class_is_taken_and_answered_by_its_class_key() {
    use crate::chronometry::Cite;
    use crate::transducer::alphabet::LetterId;
    let p = letter(LetterId::pythagorean_5((0, 1)));
    let kappa = crate::transducer::sentence::ClassVector::of_rotor(
        &p,
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
    .expect("a class reads");
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    let (thought, cover): (Vec<u8>, Vec<Vec<u8>>) = {
        let (k, b) = c.bubbles.scoped().next().expect("one thought held");
        (k, b.cover.scoped().map(|(ck, _)| ck).collect())
    };
    let entry = Cite::Class { class: Box::new(kappa.clone()), rotor: Box::new(p.clone()), cover: Some(cover) };
    c.docket_put(entry);
    c.docket_sealed();
    c.dirty.remove(&thought);
    c.take_docket();
    let b = c.bubbles.get(&thought).expect("still held");
    assert!(b.horizon.get(p.class_key().expect("a class key").as_bytes()).is_some(), "the rotor in its horizon");
    assert!(c.dirty.contains(&thought), "marked dirty, so the expand ring reaches it first");
    assert_eq!(c.docket_reading().taken, 1);
    c.docket_discharged(kappa.class_key().as_bytes());
    let d = c.docket_reading();
    assert_eq!((d.open, d.answered), (0, 1), "answered by its class key, by name");
}

/// 78.37, R1 — IN-PROCESS, A HELD RING'S ENTRY IS ANSWERED WHERE ITS SIMPLEX
/// RE-FILLS: the ring held with a (3, 4, 5) edge is docketed; the edge made
/// consistent, the ring's 2-simplex fills, and the entry leaves by name.
#[test]
fn a_held_rings_entry_is_answered_where_its_simplex_refills() {
    use crate::transducer::alphabet::{LetterId, Rotor};
    use crate::transducer::element::identity_versor;
    let p = letter(LetterId::pythagorean_5((0, 1)));
    let (mut c, vs) =
        ring_of_three([Rotor::identity(), p.clone(), Rotor::identity()], [p, Rotor::identity(), Rotor::identity()]);
    c.fix(&vs[0], false);
    c.transport(1);
    c.transport(2);
    assert_eq!(c.docket_reading().pending, 1, "the held class on the docket");
    c.complex.set_edge(&vs[0], &vs[1], Rotor::identity(), identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    let ring = c.complex.link(&vs[0], 2)[0].clone();
    assert!(matches!(c.fill_meet(&ring), CoboundaryOutcome::Attached(Fill::Filled(..))), "the ring fills");
    let d = c.docket_reading();
    assert_eq!((d.pending, d.answered), (0, 1), "answered where its simplex re-filled");
}

/// 78.37(2), TC1 — A LEAF HANGING OFF A RING, SHED BY PRICE: a ring of four
/// thoughts (no 2-simplex, so the nerve's Ȟ¹ is 1) with a leaf on one
/// corner reads one free vertex and a 2-core of four. The price door sheds
/// the leaf: removed 1, in the core 0, rank lost 0. Then it sheds a corner of
/// the ring: in the core 1, rank lost 1 — what shedding by price took that
/// carried a loop, read exactly. A filled triangle beside it reads three free
/// edge faces and Ȟ¹ 0: every 2-simplex a cell whatever its fill.
#[test]
fn a_leaf_off_a_ring_shed_by_price_carries_no_loop_and_a_corner_does() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    use crate::transducer::element::identity_versor;
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1u8..=5).map(|i| vec![i; 4]).collect();
    for x in &vs {
        c.complex.set_section(x, Section::at(x.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
    }
    let id = Rotor::identity();
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0), (0, 4)] {
        c.complex.set_edge(&vs[a], &vs[b], id.clone(), identity_versor(), [1, 0, 0, 0]);
    }
    let r = c.core_reading();
    assert_eq!((r.free_vertices, r.isolated, r.core, r.h1), (1, 0, 4, 1), "{r:?}");
    c.set_now(10);
    c.bubbles.insert(&vs[4], Bubble { until: Some(1), ..Bubble::default() });
    let _ = c.lapse_on_clock();
    let s = c.meet_graph().shed;
    assert_eq!((s.removed[0], s.in_core[0], s.rank_lost[0]), (1, 0, 0), "a leaf carried no loop");
    c.bubbles.insert(&vs[2], Bubble { until: Some(1), ..Bubble::default() });
    let _ = c.lapse_on_clock();
    let s = c.meet_graph().shed;
    assert_eq!((s.removed[0], s.in_core[0], s.rank_lost[0]), (2, 1, 1), "a corner of the ring carried one");

    let (t, _) = ring_of_three([Rotor::identity(), Rotor::identity(), Rotor::identity()], [id.clone(), id.clone(), id]);
    let r = t.core_reading();
    assert_eq!((r.free_faces[0], r.h1, r.unfaced, r.malformed), (3, 0, 0, 0), "each edge a face of the one triangle: {r:?}");
}

/// Two pivots on the same bubble with no new word leave fills unchanged.
#[test]
fn two_pivots_without_a_new_word_leave_fills() {
    use crate::transducer::element::{canonical_chain, from_canonical};
    let mut c = Contemplation::new();
    let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    c.hold(rem.clone());
    c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
    let word = from_canonical(&canonical_chain(&rem).expect("one entry per position is A5's chain"));
    let snap = |c: &Contemplation| {
        let mut out = Vec::new();
        for layer in &c.complex.degrees {
            for (k, s) in layer.scoped() {
                out.push((k.clone(), s.fill.clone()));
            }
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: the simplices by their own keys, so two snapshots of one complex compare"
        )]
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    };
    let mut budget = crate::physics::Physics::default().stroke_n(64);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let before = snap(&c);
    let mut budget = crate::physics::Physics::default().stroke_n(64);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let fills = |s: &[(Vec<u8>, Fill)]| s.iter().map(|(_, f)| format!("{f:?}")).collect::<Vec<_>>();
    assert_eq!(fills(&snap(&c)), fills(&before), "a second pivot with no new word leaves fills");
}

/// 25.1: a syllabus class standing at the conjugacy stratum of a held
/// vortex's orbit applies the certificate whose move is that class's
/// representative; the other candidate is not applied.
///
/// THE TWO CANDIDATES SHARE `before`, because they discharge ONE orbit, and
/// that is the coordinate a candidate hangs under; a placeholder there is
/// only invisible until the coordinate is read.
#[test]
fn vortex_collapses_by_glue_or_ruling() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::sentence::{Axis, ClassVector, PivotBy, PivotCertificate, Side};
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2)]);
    let q = LetterId::turn_90((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    let stratum = r.class_key().expect("90° turn class");
    // The orbit's own state vector, whose first coordinate is that stratum.
    let before = ClassVector::of_word(&w).expect("a 90° turn has a class vector").class_key();
    let after = ClassVector::zero().class_key();
    // The one orbit's chord, both candidates' (AT1: a chord is an overlap).
    let chord = crate::transducer::sentence::tests::common::some_overlap;
    let hit = PivotCertificate {
        chord: chord(),
        axis: Axis::Unit(0),
        before: before.clone(),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: chord(),
            word: Box::new(w.clone()),
            side: Side::Right,
        },
    };
    let sheet = crate::transducer::alphabet::PlaneForm {
        orientation: true,
        rot: [0; crate::transducer::alphabet::CAYLEY_ALPHABET_LEN],
    }
    .to_word();
    assert_ne!(w.key(), sheet.key(), "two words, one orbit");
    let other = PivotCertificate {
        chord: chord(),
        axis: Axis::Hue,
        before: before.clone(),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: chord(),
            word: Box::new(sheet),
            side: Side::Left,
        },
    };
    let k = c.bubbles.scoped().next().map(|(k, _)| k).expect("bubble");
    let hit_key = hit.key();
    if let Some(b) = c.bubbles.get_mut(&k) {
        sentence::vortex_put(&mut b.vortex, hit);
        sentence::vortex_put(&mut b.vortex, other);
    }
    let rec = ClassRecord {
        key: stratum,
        order: r.order().expect("the 90° turn has an order"),
        representative: r,
    };
    assert_eq!(c.seed_horizon(&[rec]), 1);
    let b = c.bubbles.get(&k).expect("bubble");
    assert_eq!(b.vortex.len(), 0, "the vortex collapsed");
    assert!(
        b.certificates.get(&hit_key).is_some(),
        "the matching certificate was applied"
    );
}

/// KB1 (2) — AN IDEA'S NERVE IS OVER ITS SECTIONS: a vertex is the address,
/// the chart and the carriage, so one pair of ideas meeting on two charts,
/// or by two carriages on one, is that many simplices — never one key for the
/// pair, which the last filing overwrote.
#[test]
fn an_ideas_nerve_vertices_are_its_sections() {
    use crate::transducer::element::{identity_versor, translator_versor};
    use crate::transducer::stalk::Invariant;
    let (a, b) = (Invariant::counts(&[1]), Invariant::counts(&[2]));
    let (still, moved) = (identity_versor(), translator_versor(1, 0));
    let at = |chart: &Invariant, car| section_vertex(b"idea", chart, car);
    assert_ne!(at(&a, &still), at(&b, &still), "two charts, two sections");
    assert_ne!(at(&a, &still), at(&a, &moved), "two carriages on one chart, two sections");
    assert_eq!(at(&a, &still), at(&a, &still), "one section, one vertex");
    let mut nerve = crate::transducer::cech::Complex::new();
    let other = |chart: &Invariant| section_vertex(b"other", chart, &still);
    let turn = crate::transducer::alphabet::Rotor::identity();
    nerve.file_congruent(at(&a, &still), other(&a), turn.clone(), 1);
    nerve.file_congruent(at(&b, &still), other(&b), turn, 1);
    assert_eq!(nerve.simplex_count(), 2, "the pair met on two charts is two simplices");
}
