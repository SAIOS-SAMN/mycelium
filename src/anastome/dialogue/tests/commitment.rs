// 78.154 — THE LEDGER'S TWO DIGESTS (Concordance 1.13): the state commitment,
// a sparse Merkle tree over a universe's records, and the events digest,
// positional over the events applied. Proved by decoded values and by the
// form's own rules, never by a golden byte string.

use crate::anastome::dialogue::commitment::{
    commitment, event_leaf, events_digest, leaf, node, path, prove, verify_absence, verify_inclusion, Event, Record, DEPTH, ZERO,
};
use crate::anastome::dialogue::seat::Arg;
use crate::chronometry::tuniversal::{Origin, TPoint};

fn record(key: &str, n: u64) -> Record {
    (key.as_bytes().to_vec(), vec![Arg::Count(n), Arg::Q { num: -3, den: 7 }])
}

fn records() -> Vec<Record> {
    vec![record("world/7/stock/iron", 40), record("world/7/people", 1200), record("vault/2", 9), record("core/0/mass", 5)]
}

/// The bit of a path at `depth`, most significant first.
fn bit(p: &[u8; 32], depth: usize) -> bool {
    (p[depth / 8] >> (7 - depth % 8)) & 1 == 1
}

/// INSERTION ORDER IS NO PART OF THE ROOT: every rotation and the reversal
/// of the records give one root; changing one value changes it; a record
/// deleted restores the root without it exactly.
#[test]
fn the_root_is_the_records_never_their_order() {
    let all = records();
    let root = commitment(&all).expect("the records place");
    for r in 0..all.len() {
        let mut rotated = all.clone();
        rotated.rotate_left(r);
        assert_eq!(commitment(&rotated), Some(root), "rotation {r}");
    }
    let mut reversed = all.clone();
    reversed.reverse();
    assert_eq!(commitment(&reversed), Some(root));
    let mut changed = all.clone();
    changed[1] = record("world/7/people", 1201);
    assert_ne!(commitment(&changed), Some(root), "one value moved the root");
    let without: Vec<Record> = all.iter().filter(|(k, _)| k.as_slice() != b"vault/2").cloned().collect();
    let before = commitment(&without).expect("places");
    assert_ne!(before, root);
    let mut again = without.clone();
    again.insert(1, record("vault/2", 9));
    assert_eq!(commitment(&again), Some(root), "inserted back, the root returns");
    assert_eq!(commitment(&without), Some(before), "deleted, the earlier root exactly");
}

/// THE EDGES OF THE FORM: no records is 32 zero bytes; a key twice is
/// refused; a key past its u16 wall is refused, never widened; one record's
/// root is its leaf folded up its own path against zero siblings.
#[test]
fn empty_duplicate_wall_and_one_record() {
    assert_eq!(commitment(&[]), Some(ZERO));
    assert_eq!(commitment(&[record("a", 1), record("a", 2)]), None, "a key twice is refused");
    let wide = (vec![0u8; usize::from(u16::MAX) + 1], vec![Arg::Count(1)]);
    assert_eq!(commitment(&[wide]), None, "a key past the wall");
    let one = record("world/7/people", 1200);
    let l = leaf(&one.0, &one.1).expect("a leaf");
    let p = path(&one.0);
    let mut cur = l;
    for depth in (0..DEPTH).rev() {
        cur = match bit(&p, depth) {
            false => node(&cur, &ZERO),
            true => node(&ZERO, &cur),
        };
    }
    assert_eq!(commitment(&[one]), Some(cur));
    assert_eq!(node(&ZERO, &ZERO), ZERO, "an empty node is zero");
    assert_ne!(node(&l, &ZERO), node(&ZERO, &l), "left and right are two places");
}

/// INCLUSION AND ABSENCE: every record's proof folds its leaf to the root;
/// an absent key's proof folds zero to it; a wrong value, a tampered
/// sibling, a short proof, or a held key read as absent all fail.
#[test]
fn inclusion_and_absence_proofs_verify_and_wrong_ones_fail() {
    let all = records();
    let root = commitment(&all).expect("places");
    for (k, v) in &all {
        let proof = prove(&all, k).expect("a proof");
        assert_eq!(proof.len(), DEPTH);
        assert!(verify_inclusion(&root, k, v, &proof), "{}", String::from_utf8_lossy(k));
        assert!(!verify_absence(&root, k, &proof), "a held key is not absent");
        assert!(!verify_inclusion(&root, k, &[Arg::Count(0)], &proof), "a wrong value fails");
        let mut tampered = proof.clone();
        tampered[200] = [1; 32];
        assert!(!verify_inclusion(&root, k, v, &tampered), "a tampered sibling fails");
        assert!(!verify_inclusion(&root, k, v, &proof[1..]), "a short proof fails");
    }
    let absent = b"world/9/nobody";
    let proof = prove(&all, absent).expect("a proof of absence");
    assert!(verify_absence(&root, absent, &proof));
    assert!(!verify_inclusion(&root, absent, &[Arg::Count(1)], &proof));
    let other = prove(&all, b"vault/2").expect("a proof");
    assert!(!verify_absence(&root, absent, &other), "another path's siblings prove nothing here");
}

fn at(h: u64) -> TPoint {
    TPoint { origin: Origin { genesis: [4; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0], k: 1 }
}

fn event(h: u64, u: u64, key: &str) -> Event {
    (at(h), u, key.as_bytes().to_vec(), vec![Arg::Flag(true)])
}

/// THE EVENTS DIGEST IS POSITIONAL: order matters; none is zero; one is its
/// leaf; three are `node(node(l0, l1), l2)` — the odd one carried up.
#[test]
fn the_events_digest_keeps_its_order() {
    let (a, b, c) = (event(5, 10, "ship/1"), event(6, 11, "ship/2"), event(6, 12, "ship/1"));
    assert_eq!(events_digest(&[]), Some(ZERO));
    let la = event_leaf(&a).expect("a leaf");
    assert_eq!(events_digest(std::slice::from_ref(&a)), Some(la));
    let (lb, lc) = (event_leaf(&b).expect("leaf"), event_leaf(&c).expect("leaf"));
    assert_eq!(events_digest(&[a.clone(), b.clone(), c.clone()]), Some(node(&node(&la, &lb), &lc)));
    assert_ne!(events_digest(&[a.clone(), b.clone(), c.clone()]), events_digest(&[b.clone(), a.clone(), c.clone()]), "order matters");
    assert_ne!(event_leaf(&a), event_leaf(&(at(5), 11, a.2.clone(), a.3.clone())), "u is a coordinate of the event");
}
