// 28, 78.27 — a restart: the contemplation is volatile, in memory and never
// persisted, so a mind that comes up empty refills its horizon from the
// syllabus page.

use crate::transducer::bubble::*;

/// A restarted mind comes up with an empty horizon and refills it from the
/// syllabus page, and a later idea inherits the seeded class.
#[test]
fn seed_horizon_refills_empty_mind_from_syllabus() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    assert_eq!(c.horizon_len(), 0);
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let rec = ClassRecord {
        key: r.class_key().expect("90° turn class"),
        order: r.order().expect("the 90° turn has an order"),
        representative: r,
    };
    let n = c.seed_horizon(std::slice::from_ref(&rec));
    assert_eq!(n, 1, "empty mind refills from the page");
    c.hold(vec![(5, 5, 3, 4)]);
    assert!(
        c.bubbles
            .scoped()
            .any(|(_, b)| b.horizon.contains(&rec.key)),
        "a later idea inherits the seeded class"
    );
}
