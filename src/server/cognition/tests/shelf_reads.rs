// 78.153b — THE READS MOVED OFF THE KERNEL answer as the kernel answers: off
// the published reading (the court's own cover, room, ruled cover, head
// receipt, a fold's re-derivation), off a page of the journal's prefix, and
// off the language snapshot touched in place — exact values, compared.

use super::common::*;
use crate::chronometry::{ActKind, Place};
use crate::cognition::*;

/// A court with ruled words, a voice's listings and a line of acts.
fn court() -> Kernel {
    let mut k = Kernel::new();
    k.bind_identity([0x7a; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    glue_pair(&mut k, &a, &b);
    let c = vec![(0, 0, 2), (1, 1, 2)];
    let d = vec![(0, 0, 3), (1, 1, 3)];
    glue_pair(&mut k, &c, &d);
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    k
}

/// THE COURT'S OWN READS OFF THE SHELF ARE THE KERNEL'S: its cover charts,
/// its room, the ruled root, the ruled digests to its own voice and none to
/// another, the syllabus seed, and the head receipt — each the exact value
/// the kernel answers at the publication; and a cover that did not move is
/// shared with the last reading, not copied.
#[test]
fn the_courts_own_reads_off_the_shelf_are_the_kernels() {
    let mut k = court();
    let voice = [0x7b; 32];
    k.court_voice = Some(voice);
    let p = k.publish(None).expect("a first reading always publishes");
    assert_eq!(p.own_cover_charts(), k.own_cover_charts().as_slice());
    assert_eq!(p.lane_room(), k.lane_room());
    assert_eq!(p.ruled_root(), k.ruled_root());
    assert_eq!(p.ruled_digests_for(&voice).map(<[_]>::to_vec), k.ruled_digests_for(&voice));
    assert_eq!(p.ruled_digests_for(&[0x7c; 32]), None, "another asker is not the court's voice");
    assert_eq!(p.syllabus_seed(), k.syllabus_seed().as_slice());
    assert!(!p.syllabus_seed().is_empty(), "the court holds ruled words");
    assert_eq!(p.head_receipt(), k.chain().head().as_ref());
    // An act moves the head, nothing else: the next reading's head receipt
    // is the new one, its ruled cover and own cover the same pointers.
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    let q = k.publish(Some(&p)).expect("the head moved");
    assert_eq!(q.head_receipt(), k.chain().head().as_ref());
    assert!(std::sync::Arc::ptr_eq(&p.own.ruled, &q.own.ruled), "the ruled cover did not move: shared");
    assert!(std::sync::Arc::ptr_eq(&p.own.cover, &q.own.cover), "the own cover did not move: shared");
}

/// A FOLD RE-DERIVED OFF THE SHELF IS THE KERNEL'S VERDICT: refused for want
/// of bond as the kernel refuses it, and — bonded — standing or refused on
/// the same prefix exactly as the kernel rules.
#[test]
fn a_fold_verified_off_the_shelf_is_the_kernels_verdict() {
    let home = [0x5d; 32];
    let mut h = Kernel::new();
    h.bind_identity(home);
    for _ in 0..3 {
        h.append_step(ActKind::List, [1; 32], [2; 32]);
        h.orbit();
    }
    let p = h.propose_fold().expect("a final prefix");
    let receipts = h.chain().line_between(&home, p.after, p.root.at, usize::MAX);
    let rows = h.rows_range(&home, p.after, p.root.at, 0, usize::MAX).1;
    let mut w = Kernel::new();
    w.bind_identity([0x5e; 32]);
    let read = w.publish(None).expect("published");
    assert_eq!(read.verify_fold(home, &p, &receipts, &rows), w.verify_fold(home, &p, &receipts, &rows), "unbonded");
    let cover = w.genesis_cover().expect("a measured cover");
    w.enroll([0x5e; 32], cover);
    let read = w.publish(Some(&read)).expect("the bond moved");
    assert_eq!(read.verify_fold(home, &p, &receipts, &rows), w.verify_fold(home, &p, &receipts, &rows), "bonded");
    let short = receipts.get(1..).expect("three acts").to_vec();
    assert_eq!(read.verify_fold(home, &p, &short, &rows), w.verify_fold(home, &p, &short, &rows), "a short prefix");
}

/// A PAGE OF THE ROWS BELOW A HEAD is the page the whole prefix gave: for
/// every place on the line and every cursor, the rows and their total equal
/// those `rows_below` reads, so `ACT_ROWS` answers exactly as it did.
#[test]
fn a_page_of_the_rows_below_is_the_whole_prefix_paged() {
    let mut k = court();
    let me = k.chain().vk();
    let line = k.chain().line_between(&me, None, Place { k: u64::MAX, i: u64::MAX }, usize::MAX);
    for r in &line {
        let anchor = (r.stamp, r.chain_hash);
        let _ = k.treasury.genesis([r.stamp.i as u8 + 1; 32], [1, 0, 0, 0], anchor);
    }
    let after = k.chain().line_between(&me, None, Place { k: u64::MAX, i: u64::MAX }, usize::MAX);
    for r in &after {
        let head = r.stamp.place();
        let below = k.treasury.rows_below(head);
        for skip in 0..=below.len() + 1 {
            for n in [0, 1, 2, usize::MAX] {
                let (total, page) = k.treasury.rows_below_page(head, skip, n);
                let want: Vec<_> = below.iter().skip(skip).take(n).cloned().collect();
                assert_eq!((total, page), (below.len() as u64, want), "head {head:?} skip {skip} n {n}");
            }
        }
    }
}

/// THE CLASS RECORDS ON A STROKE OF ONE READ, resumed by asker: each reading
/// reads the next chart's complex round the nerve, and as many readings as
/// the nerve holds charts carry every record the whole reading carries.
#[test]
fn the_class_records_resume_round_the_nerve_by_asker() {
    let mut k = court();
    let all = k.class_records(&crate::transducer::stalk::Invariant::default());
    let whole = k.class_records_on(&crate::transducer::stalk::Invariant::default(), &[1; 32], &mut budget());
    assert_eq!(whole, all, "on a stroke that reads it all, the whole reading");
    let charts = k.complexes.len();
    assert!(charts > 0, "the court glued words onto charts");
    let mut met: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for _ in 0..charts {
        let mut one = crate::physics::Physics::default().stroke_n(1);
        for c in k.class_records_on(&crate::transducer::stalk::Invariant::default(), &[2; 32], &mut one) {
            met.insert(c.key, ());
        }
    }
    for c in &all {
        assert!(met.contains(&c.key), "every class reached the asker in turn");
    }
    assert_eq!(met.len(), all.len());
}

/// A TOUCHED WORD IS REPLACED IN PLACE: the snapshot after the touch holds,
/// at every address, the word a whole rebuild gives it.
#[test]
fn a_touched_word_is_the_word_a_rebuild_gives() {
    let mut k = court();
    let _ = k.language();
    let addr = k.basis.scoped().next().map(|(a, _)| a).expect("a word held");
    // A held word taken for change makes its Arc the basis's own, as the
    // glue's dislocation record does before it touches.
    assert!(k.word_mut(&addr).is_some());
    k.touch_at(&addr);
    let touched = k.language();
    k.language_rebuild();
    let rebuilt = k.language();
    let keys = |l: &Language| l.words.scoped().map(|(a, _)| a).collect::<Vec<_>>();
    assert_eq!(keys(&touched), keys(&rebuilt));
    for a in keys(&rebuilt) {
        assert_eq!(
            touched.word_at(&a).and_then(|w| w.to_bytes()),
            rebuilt.word_at(&a).and_then(|w| w.to_bytes()),
            "the word at each address is the one a rebuild gives"
        );
    }
    assert!(
        std::sync::Arc::ptr_eq(&touched.words.get(&addr).expect("held").held, k.basis.get(&addr).expect("held")),
        "the touched address holds the basis's own Arc"
    );
}
