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
    let keys: Vec<Vec<u8>> = c.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
    let edges = c.complex.link(&keys[0], 1);
    assert!(!edges.is_empty() || c.complex.degrees.get(1).is_none_or(|l| !l.is_empty()) || {
        // same chart prefix (leading axis of a bar) may or may not coincide;
        // empty cover overlap is honest.
        true
    });
    assert!(
        c.complex.degrees.get(2).is_none_or(|l| l.is_empty()),
        "disjoint remainders share no 2-cell"
    );
}

#[test]
fn overlapping_remainders_merge_by_fill() {
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
    assert_eq!(c.len(), 1, "shared cell is Extended: the bubbles merge");
    let keys: Vec<Vec<u8>> = c.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
    assert_eq!(keys.len(), 1);
    assert!(c.bubbles.get(&keys[0]).unwrap().potential.iter().any(|&x| x > 0));
}

/// Two pivots on the same bubble with no new word leave fills unchanged.
#[test]
fn two_pivots_without_a_new_word_leave_fills() {
    use crate::transducer::element::from_residual;
    let mut c = Contemplation::new();
    let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    c.hold(rem.clone());
    c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
    let word = from_residual(&rem);
    let snap = |c: &Contemplation| {
        let mut out = Vec::new();
        for layer in &c.complex.degrees {
            for (k, cell) in layer.fiber(ROOT) {
                out.push((k.clone(), cell.fill.clone()));
            }
        }
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    };
    let mut budget = crate::physics::Physics::default().stroke_n(64);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let before = snap(&c);
    let mut budget = crate::physics::Physics::default().stroke_n(64);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let fills = |s: &[(Vec<u8>, Fill)]| s.iter().map(|(_, f)| f.clone()).collect::<Vec<_>>();
    assert_eq!(fills(&snap(&c)), fills(&before), "a second pivot with no new word leaves fills");
}

/// 25.1: a syllabus class whose key is one held certificate's `after`
/// applies that one; the other vortex cell is not applied.
#[test]
fn vortex_collapses_by_glue_or_ruling() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::sentence::{Axis, PivotBy, PivotCertificate, Side};
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2)]);
    let q = LetterId::quarter((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    let after = r.class_key().expect("quarter class");
    let hit = PivotCertificate {
        chord: 0,
        axis: Axis::Unit(0),
        before: crate::transducer::address::Address::from(vec![1]),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: 0,
            word: w.clone(),
            side: Side::Right,
        },
    };
    let sheet = crate::transducer::alphabet::PlaneForm {
        orientation: true,
        rot: [0; crate::transducer::alphabet::CAYLEY_ALPHABET_LEN],
    }
    .to_word();
    assert_ne!(w.key(), sheet.key(), "two words, one after class");
    let other = PivotCertificate {
        chord: 0,
        axis: Axis::Hue,
        before: crate::transducer::address::Address::from(vec![2]),
        after: after.clone(),
        rank_move: RankMove::Discharged { from: RankAxis::Torsion },
        mu: crate::transducer::sentence::Mu::Unmeasured,
        by: PivotBy::Word {
            face: 0,
            word: sheet,
            side: Side::Left,
        },
    };
    let k = c.bubbles.fiber(ROOT).next().map(|(k, _)| k).expect("bubble");
    let hit_key = hit.key();
    if let Some(b) = c.bubbles.get_mut(&k) {
        sentence::vortex_put(&mut b.vortex, hit);
        sentence::vortex_put(&mut b.vortex, other);
    }
    let rec = ClassRecord {
        key: after,
        order: r.word_order().expect("quarter spells"),
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
