// 78.154 — THE LEDGER'S TWO DIGESTS (the coordinator's rulings, 2026-10-09;
// Concordance 1.13): a universe's STATE COMMITMENT, a sparse Merkle tree over
// its canonical records, and the EVENTS DIGEST, a positional tree over the
// input events applied since its parent entry. Each kernel names its own
// records and events (each a coordinate path within its universe); the FORM
// is shared. The court stores both digests and checks neither.
//
//! THE COMMITMENT. A record is a key (the entity's coordinate path, at most
//! `u16::MAX` bytes — the wall, never widened) and its exact values in the
//! wire's ARG encoding (`seat::put_args`).
//!
//! - its PATH is the citation hash of the key's bytes: 256 bits, most
//!   significant bit first, bit value 0 the left child at each depth;
//! - its LEAF is `derive_key("mycelium ledger leaf v1", key_len u16 LE ‖ key
//!   ‖ put_args(values))`, standing at depth 256 of its path, no short-cut;
//! - an EMPTY subtree is 32 zero bytes at every level;
//! - an INNER NODE is zero where both children are zero, else
//!   `derive_key("mycelium ledger node v1", left ‖ right)`;
//! - no records: the root is 32 zero bytes; two records with one key (or one
//!   path) are refused, so the root is never ambiguous.
//!
//! Computed as a SPARSE FOLD — the records split by their path's bit at each
//! depth — so the cost is 256 nodes per record and nothing allocates the
//! empty tree. A PROOF is the 256 siblings from the root down a key's path:
//! folding the key's leaf up against them gives the root where the record is
//! held (INCLUSION), and folding zero up against them gives the root where
//! no record stands at that path (ABSENCE).
//!
//! THE EVENTS DIGEST is positional, never sorted: the events in the order
//! they were applied, each leaf `derive_key("mycelium ledger event v1",
//! TPoint 84 ‖ u u64 LE ‖ key_len u16 LE ‖ key ‖ put_args(values))`, paired
//! left to right level by level with the commitment's node rule, an odd node
//! carried up unchanged (never duplicated); one event's digest is its leaf,
//! none is 32 zero bytes.

use super::seat::{put_args, Arg};
use crate::chronometry::tuniversal::TPoint;

const LEAF: &str = "mycelium ledger leaf v1";
const NODE: &str = "mycelium ledger node v1";
const EVENT: &str = "mycelium ledger event v1";

/// The empty subtree, and the root of no records.
pub const ZERO: [u8; 32] = [0; 32];

/// The depth a leaf stands at: one level per bit of its path.
pub const DEPTH: usize = 256;

/// ONE RECORD: the entity's coordinate path within its universe, and its
/// exact values.
pub type Record = (Vec<u8>, Vec<Arg>);

/// ONE INPUT EVENT: the final point and the universe's `u` it was applied
/// at, the entity's coordinate path, and its exact values.
pub type Event = (TPoint, u64, Vec<u8>, Vec<Arg>);

/// `key_len u16 LE ‖ key ‖ put_args(values)`; `None` past a wall.
fn keyed(out: &mut Vec<u8>, key: &[u8], values: &[Arg]) -> Option<()> {
    out.extend_from_slice(&u16::try_from(key.len()).ok()?.to_le_bytes());
    out.extend_from_slice(key);
    put_args(out, values)
}

/// A record's LEAF; `None` where its key passes the u16 or a value does not
/// write.
pub fn leaf(key: &[u8], values: &[Arg]) -> Option<[u8; 32]> {
    let mut m = Vec::new();
    keyed(&mut m, key, values)?;
    Some(blake3::derive_key(LEAF, &m))
}

/// AN INNER NODE: zero over two empty children, else their derived key.
pub fn node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    if *left == ZERO && *right == ZERO {
        return ZERO;
    }
    let mut m = Vec::with_capacity(64);
    m.extend_from_slice(left);
    m.extend_from_slice(right);
    blake3::derive_key(NODE, &m)
}

/// A key's PATH: the citation hash of its bytes.
pub fn path(key: &[u8]) -> [u8; 32] {
    crate::chronometry::payload_hash(key)
}

/// The path's bit at `depth`, most significant first; `true` is the right.
fn bit(path: &[u8; 32], depth: usize) -> bool {
    path.get(depth / 8).is_some_and(|b| (b >> (7 - depth % 8)) & 1 == 1)
}

/// The records as leaves on their paths, in path order, each path once;
/// `None` where a key repeats (or two keys share a path), a key passes its
/// wall, or a value does not write.
fn placed(records: &[Record]) -> Option<Vec<([u8; 32], [u8; 32])>> {
    let mut at: crate::network::nerve::Keyed<[u8; 32]> = crate::network::nerve::Keyed::new();
    for (key, values) in records {
        let p = path(key);
        if at.contains(&p) {
            return None;
        }
        at.insert(p, leaf(key, values)?);
    }
    Some(at.scoped().filter_map(|(p, l)| Some((<[u8; 32]>::try_from(p.as_slice()).ok()?, *l))).collect())
}

/// The subtree at `depth` over `leaves` — all on one prefix, in path order.
fn fold(leaves: &[([u8; 32], [u8; 32])], depth: usize) -> [u8; 32] {
    match leaves {
        [] => ZERO,
        [(_, l)] if depth == DEPTH => *l,
        _ if depth >= DEPTH => ZERO,
        _ => {
            let split = leaves.partition_point(|(p, _)| !bit(p, depth));
            let (left, right) = leaves.split_at(split);
            node(&fold(left, depth + 1), &fold(right, depth + 1))
        }
    }
}

/// THE STATE COMMITMENT over a universe's records, in any order; `None`
/// where a key repeats, passes its wall, or a value does not write.
pub fn commitment(records: &[Record]) -> Option<[u8; 32]> {
    Some(fold(&placed(records)?, 0))
}

/// A KEY'S PROOF: the 256 siblings down its path, the root's child first;
/// `None` where the records do not place.
pub fn prove(records: &[Record], key: &[u8]) -> Option<Vec<[u8; 32]>> {
    let all = placed(records)?;
    let p = path(key);
    let mut on: &[([u8; 32], [u8; 32])] = &all;
    let mut siblings = Vec::with_capacity(DEPTH);
    for depth in 0..DEPTH {
        let split = on.partition_point(|(q, _)| !bit(q, depth));
        let (left, right) = on.split_at(split);
        let (mine, other) = match bit(&p, depth) {
            false => (left, right),
            true => (right, left),
        };
        siblings.push(fold(other, depth + 1));
        on = mine;
    }
    Some(siblings)
}

/// `from` (what stands at depth 256 of `key`'s path) folded up against the
/// siblings; `None` where there are not exactly 256.
fn fold_up(key: &[u8], from: [u8; 32], siblings: &[[u8; 32]]) -> Option<[u8; 32]> {
    if siblings.len() != DEPTH {
        return None;
    }
    let p = path(key);
    let mut cur = from;
    for depth in (0..DEPTH).rev() {
        let sib = siblings.get(depth)?;
        cur = match bit(&p, depth) {
            false => node(&cur, sib),
            true => node(sib, &cur),
        };
    }
    Some(cur)
}

/// INCLUSION: the record `(key, values)` stands under `root`.
pub fn verify_inclusion(root: &[u8; 32], key: &[u8], values: &[Arg], siblings: &[[u8; 32]]) -> bool {
    leaf(key, values).and_then(|l| fold_up(key, l, siblings)).is_some_and(|r| r == *root)
}

/// ABSENCE: no record stands at `key`'s path under `root`.
pub fn verify_absence(root: &[u8; 32], key: &[u8], siblings: &[[u8; 32]]) -> bool {
    fold_up(key, ZERO, siblings).is_some_and(|r| r == *root)
}

/// One event's leaf; `None` past a wall.
pub fn event_leaf(e: &Event) -> Option<[u8; 32]> {
    let (at, u, key, values) = e;
    let mut m = Vec::with_capacity(TPoint::LEN + 8);
    at.put(&mut m);
    m.extend_from_slice(&u.to_le_bytes());
    keyed(&mut m, key, values)?;
    Some(blake3::derive_key(EVENT, &m))
}

/// THE EVENTS DIGEST, in the order applied; `None` past a wall.
pub fn events_digest(events: &[Event]) -> Option<[u8; 32]> {
    let mut level = events.iter().map(event_leaf).collect::<Option<Vec<_>>>()?;
    if level.is_empty() {
        return Some(ZERO);
    }
    while level.len() > 1 {
        level = level
            .chunks(2)
            .map(|pair| match pair {
                [l, r] => node(l, r),
                [odd] => *odd,
                _ => ZERO,
            })
            .collect();
    }
    level.into_iter().next()
}
