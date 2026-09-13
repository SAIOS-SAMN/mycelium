// 28 — persistence: a contemplation restored from its tape holds the same
// bubbles and the same horizon as the one that was written.

use crate::transducer::bubble::*;

#[test]
fn tht1_round_trips_bubbles() {
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
    let mut bytes = Vec::new();
    c.to_bytes(&mut bytes);
    let mut pos = 0usize;
    let back = Contemplation::from_bytes(&bytes, &mut pos).expect("THT3");
    assert_eq!(back.len(), 1);
    assert_eq!(back.potential(), c.potential());
}

#[test]
fn restart_refills_horizon_from_syllabus() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    assert_eq!(c.horizon_cells(), 0);
    let q = LetterId::quarter((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let rec = ClassRecord {
        key: r.class_key().expect("quarter class"),
        order: r.order().expect("quarter has an order"),
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
