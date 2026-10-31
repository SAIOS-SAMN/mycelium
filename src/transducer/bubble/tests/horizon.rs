// A19, 25.4 — the horizon: a seeded class is recognised, never re-derived,
// and it is keyed by the class key.

use crate::transducer::bubble::*;

/// A class seeded into the horizon is recognised in a held bubble under the
/// Ruled standing, never re-derived.
#[test]
fn seed_horizon_keeps_class_ruled_not_rederived() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let rec = ClassRecord {
        key: r.class_key().expect("90° turn class"),
        order: r.order().expect("the 90° turn has an order"),
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
    // too, so every orbit deferred to a horizon entry it had written itself.
    assert_eq!(*st, Standing::Ruled);
}

/// The horizon is keyed by the class key: an entry put for a rotor is found
/// under that rotor's class key with its order and standing.
#[test]
fn horizon_put_keys_entry_by_class_key() {
    use crate::transducer::alphabet::{LetterId, Word};
    let mut c = Contemplation::new();
    c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).rotor();
    let ck = r.class_key().expect("90° turn class");
    let key = {
        let (k, _) = c.bubbles.scoped().next().expect("bubble");
        k
    };
    if let Some(b) = c.bubbles.get_mut(&key) {
        horizon_put(&mut b.horizon, r.clone(), Standing::Held);
    }
    let hb = c.bubbles.get(&key).expect("held");
    let (got, order, st) = hb.horizon.get(ck.as_bytes()).expect("class key");
    assert_eq!(got.class_key(), Some(ck.clone()));
    assert_eq!(*order, r.order().expect("the 90° turn has an order"));
    assert_eq!(*st, Standing::Held);
}
