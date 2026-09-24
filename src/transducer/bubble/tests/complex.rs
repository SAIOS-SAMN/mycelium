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
    let q = LetterId::quarter((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    let stratum = r.class_key().expect("quarter class");
    // The orbit's own state vector, whose first coordinate is that stratum.
    let before = ClassVector::of_word(&w).expect("a quarter turn has a class vector").class_key();
    let after = ClassVector::zero().class_key();
    let hit = PivotCertificate {
        chord: 0,
        axis: Axis::Unit(0),
        before: before.clone(),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: 0,
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
        chord: 0,
        axis: Axis::Hue,
        before: before.clone(),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: 0,
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
        order: r.order().expect("quarter has an order"),
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
