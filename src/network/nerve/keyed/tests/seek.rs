// 78.153 — THE CURSOR SEEKS: a budgeted circuit begins past `from` by
// following `from`'s own bits, never by walking the keys before it.

use crate::network::nerve::keyed::{work, Keyed};

/// Keys of every shape the trie routes: prefixes of one another, a shared
/// stem, single bytes at both ends of the byte order.
fn carrier() -> (Keyed<()>, Vec<Vec<u8>>) {
    let mut keys: Vec<Vec<u8>> = vec![
        vec![0x00],
        vec![0x00, 0x00],
        vec![0x00, 0x01],
        vec![0x01],
        vec![0x7f, 0xff],
        vec![0x80],
        vec![0x80, 0x00],
        vec![0x80, 0x00, 0x00],
        vec![0xff],
        vec![0xff, 0xff],
    ];
    for i in 0u8..64 {
        keys.push(vec![0x40, i]);
        keys.push(vec![0x40, i, i]);
    }
    let mut k: Keyed<()> = Keyed::new();
    for key in &keys {
        k.insert(key, ());
    }
    keys.sort_unstable();
    keys.dedup();
    (k, keys)
}

/// EXACTLY THE KEYS AFTER `from`, IN ORDER: for every key held and for
/// keys between, beside, below and above them, the seek yields the keys a
/// full walk filtered past `from` yields — no fewer, no more, no reorder.
#[test]
fn after_yields_exactly_keys_past_from() {
    let (k, keys) = carrier();
    let mut probes = keys.clone();
    probes.extend([vec![0x00, 0x00, 0x00], vec![0x3f], vec![0x40, 0x10, 0x00, 0x00], vec![0x41], vec![0xff, 0xff, 0xff]]);
    for from in &probes {
        let sought: Vec<Vec<u8>> = k.after(from).map(|(key, _)| key).collect();
        let walked: Vec<Vec<u8>> = keys.iter().filter(|key| key.as_slice() > from.as_slice()).cloned().collect();
        assert_eq!(sought, walked, "after {from:?}");
    }
}

/// THE SEEK WALKS `from`'S PATH, NOT THE KEYS BEFORE IT: from the last
/// key, the walk visits a trie path's worth of nodes — fewer than the keys
/// before it, which the full walk visited one by one.
#[test]
fn after_visits_fewer_nodes_than_full_walk() {
    let (k, keys) = carrier();
    // The keys are sorted: the slice's ends are the order's.
    let [first, .., last] = keys.as_slice() else { panic!("the carrier holds keys") };
    let (first, last) = (first.clone(), last.clone());
    work::reset();
    assert_eq!(k.after(&last).count(), 0, "nothing follows the last key");
    let sought = work::visits();
    work::reset();
    assert_eq!(k.scoped().filter(|(key, _)| key.as_slice() > last.as_slice()).count(), 0);
    let walked = work::visits();
    assert!(sought < walked, "the seek visited {sought} nodes, the walk {walked}");
    // And the cursor reads the same circuit from there: it wraps to the
    // first key at once.
    let mut b = crate::physics::Physics::default().stroke_n(1);
    let mut cur = k.cursor(Some(&last), &mut b);
    assert_eq!(cur.next().map(|(key, _)| key), Some(first), "past the last key, the circuit wraps to the first");
    assert!(cur.circuit(), "and that is a circuit");
}
