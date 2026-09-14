// A19, 25.4 — the horizon: a seeded class is recognised, never re-derived,
// and it is keyed by the class key.

use crate::transducer::bubble::*;

#[test]
fn horizon_seeded_class_is_not_rederived() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    let q = LetterId::quarter((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let rec = ClassRecord {
        key: r.class_key().expect("quarter class"),
        order: r.order().expect("quarter has an order"),
        representative: r.clone(),
    };
    assert_eq!(c.seed_horizon(std::slice::from_ref(&rec)), 1);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    let b = c.bubbles.scoped().next().map(|(_, b)| b).expect("bubble");
    let (_, _, st) = b.horizon.get(&rec.key).expect("seeded");
    // RULED, not Held. The syllabus is another mind's court-ratified class
    // and it is the one standing on which waiting for the word is right.
    // `Standing::Held` is what this mind's OWN obstructions go under —
    // `paint_standing` and `Fill::Class` both write it — and the pivot's A19
    // suppression could not tell the two apart while the syllabus used it
    // too, so every loop deferred to a horizon entry it had written itself.
    assert_eq!(*st, Standing::Ruled);
}

#[test]
fn horizon_is_keyed_by_class_key_and_round_trips() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
    let q = LetterId::quarter((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let ck = r.class_key().expect("quarter class");
    let key = {
        let (k, _) = c.bubbles.scoped().next().expect("bubble");
        k
    };
    if let Some(b) = c.bubbles.get_mut(&key) {
        horizon_put(&mut b.horizon, r.clone(), Standing::Held);
    }
    assert!(c.bubbles.get(&key).unwrap().horizon.contains(ck.as_bytes()));
    let mut bytes = Vec::new();
    c.to_bytes(&mut bytes);
    assert_eq!(&bytes[..4], Contemplation::THT3);
    let mut pos = 0usize;
    let back = Contemplation::from_bytes(&bytes, &mut pos).expect("THT3");
    let hb = back.bubbles.get(&key).expect("restored");
    let (got, order, st) = hb.horizon.get(ck.as_bytes()).expect("class key");
    assert_eq!(got.class_key(), Some(ck.clone()));
    assert_eq!(*order, r.order().expect("quarter has an order"));
    assert_eq!(*st, Standing::Held);
}
