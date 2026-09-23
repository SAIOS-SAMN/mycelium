// Z11 — THE TRIE PERSISTED: written out as the trie it is, and followed
// on disk by the key's own bits, exactly as in memory.
//
//! A WORD IS REACHED BY ITS ADDRESS AND BY NO POSITION (25 as ruled
//! 2026-09-20). The easy persistence is a run of records with a byte span
//! recorded per key, and it is refused: a span stored beside a key is a
//! number naming a place in a carrier on the cognition path, which is rule
//! 15's defect wearing a file's costume, and it is the shape Z9 spent four
//! commits deleting from memory. The genome file is not transport — under
//! Z11 it backs every fiber read — so the same law binds it as binds the
//! carriers.
//!
//! So the file IS the trie. A reader starts at the root and follows the
//! key's own bits; the offsets in a branch are the structure's EDGES, not
//! names of objects. Nothing outside this file ever forms one, no carrier
//! stores one, and a caller holds an address and gets bytes — the same
//! contract `Keyed::get` has, following the same prefix.
//!
//! THE PREFIX READ IS `Node::get`'s, LINE FOR LINE. A second walk that agreed
//! with the first "in principle" would be two readers of one fact (law
//! 12); this one reuses `key_bit` and the same three decisions — a leaf
//! compares its key, a branch whose bit equals the key's length answers
//! from its `end`, a key shorter than the branch's bit is absent — so the
//! on-disk and in-memory answers cannot diverge without the tests saying
//! so.

use super::trie::{key_bit, Node};
use super::Keyed;

/// The file's magic and version. A layout change is a recut (§10) and no
/// reader for an older one exists.
const MAGIC: &[u8; 4] = b"TRI1";

/// Absent child. A trie with `u32::MAX` nodes is not a trie this engine
/// builds, and the sentinel is the file's own, never a key's.
const NONE: u32 = u32::MAX;

const TAG_LEAF: u8 = 0;
const TAG_BRANCH: u8 = 1;

fn put_u32(out: &mut Vec<u8>, n: u32) {
    out.extend_from_slice(&n.to_le_bytes());
}

fn take_u32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn put_bytes(out: &mut Vec<u8>, b: &[u8]) -> Option<()> {
    put_u32(out, u32::try_from(b.len()).ok()?);
    out.extend_from_slice(b);
    Some(())
}

/// Write one node and everything under it, CHILDREN FIRST so a branch
/// knows where its children are when it is written. Returns the node's own
/// offset.
fn write_node(node: &Node<Vec<u8>>, out: &mut Vec<u8>) -> Option<u32> {
    match node {
        Node::Leaf(k, v) => {
            let at = u32::try_from(out.len()).ok()?;
            out.push(TAG_LEAF);
            put_bytes(out, k)?;
            put_bytes(out, v)?;
            Some(at)
        }
        Node::Branch { bit, children, end } => {
            let c0 = match children[0].as_deref() {
                Some(n) => write_node(n, out)?,
                None => NONE,
            };
            let c1 = match children[1].as_deref() {
                Some(n) => write_node(n, out)?,
                None => NONE,
            };
            let at = u32::try_from(out.len()).ok()?;
            out.push(TAG_BRANCH);
            put_u32(out, u32::try_from(*bit).ok()?);
            put_u32(out, c0);
            put_u32(out, c1);
            match end.as_deref() {
                None => out.push(0),
                Some((k, v)) => {
                    out.push(1);
                    put_bytes(out, k)?;
                    put_bytes(out, v)?;
                }
            }
            Some(at)
        }
    }
}

/// THE TRIE AS A FILE. An empty carrier writes its header and a root of
/// `NONE`: a file that holds nothing says so, rather than being an absence
/// the reader has to infer from a length.
pub fn write(k: &Keyed<Vec<u8>>, out: &mut Vec<u8>) -> Option<()> {
    out.extend_from_slice(MAGIC);
    // The root's offset goes in the header, so it is patched after the
    // nodes are written: the nodes' offsets are from the file's start.
    let root_at = out.len();
    put_u32(out, NONE);
    let root = match k.root_node() {
        Some(n) => write_node(n, out)?,
        None => NONE,
    };
    out.get_mut(root_at..root_at + 4)?.copy_from_slice(&root.to_le_bytes());
    Some(())
}

/// The bit-length of a key, as the trie measures it.
fn bits(key: &[u8]) -> Option<usize> {
    key.len().checked_mul(8)
}

/// FOLLOW THE FILE BY THE KEY'S OWN BITS. The three decisions are
/// `Node::get`'s, in its order.
pub fn get<'a>(file: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    if file.get(..4)? != MAGIC {
        return None;
    }
    let mut at = take_u32(file, 4)?;
    // A malformed file must not spin: every step moves to a node written
    // BEFORE this one (children first), so a read whose offsets do not fall
    // strictly is a cycle and refuses (rule 10 — the loop reads its own
    // boundary).
    let mut last = u32::MAX;
    loop {
        if at == NONE || at >= last {
            return None;
        }
        last = at;
        let p = at as usize;
        match *file.get(p)? {
            TAG_LEAF => {
                let n = take_u32(file, p + 1)? as usize;
                let k = file.get(p + 5..p + 5 + n)?;
                if k != key {
                    return None;
                }
                let m = take_u32(file, p + 5 + n)? as usize;
                return file.get(p + 9 + n..p + 9 + n + m);
            }
            TAG_BRANCH => {
                let bit = take_u32(file, p + 1)? as usize;
                let c0 = take_u32(file, p + 5)?;
                let c1 = take_u32(file, p + 9)?;
                let has_end = *file.get(p + 13)?;
                if bits(key) == Some(bit) {
                    if has_end != 1 {
                        return None;
                    }
                    let n = take_u32(file, p + 14)? as usize;
                    let k = file.get(p + 18..p + 18 + n)?;
                    if k != key {
                        return None;
                    }
                    let m = take_u32(file, p + 18 + n)? as usize;
                    return file.get(p + 22 + n..p + 22 + n + m);
                }
                if bits(key).is_some_and(|b| b < bit) {
                    return None;
                }
                at = match key_bit(key, bit) {
                    0 => c0,
                    _ => c1,
                };
            }
            _ => return None,
        }
    }
}

/// Every key the file holds, in the trie's order — what a boot reads to
/// build the resident index. The values are not returned: a boot that
/// wanted them would be loading the genome it is trying not to hold.
pub fn keys(file: &[u8]) -> Option<Vec<Vec<u8>>> {
    if file.get(..4)? != MAGIC {
        return None;
    }
    let mut out = Vec::new();
    let root = take_u32(file, 4)?;
    walk_keys(file, root, &mut out)?;
    Some(out)
}

fn walk_keys(file: &[u8], at: u32, out: &mut Vec<Vec<u8>>) -> Option<()> {
    if at == NONE {
        return Some(());
    }
    let p = at as usize;
    match *file.get(p)? {
        TAG_LEAF => {
            let n = take_u32(file, p + 1)? as usize;
            out.push(file.get(p + 5..p + 5 + n)?.to_vec());
            Some(())
        }
        TAG_BRANCH => {
            let c0 = take_u32(file, p + 5)?;
            let c1 = take_u32(file, p + 9)?;
            if *file.get(p + 13)? == 1 {
                let n = take_u32(file, p + 14)? as usize;
                out.push(file.get(p + 18..p + 18 + n)?.to_vec());
            }
            // A child is written before its parent, so this terminates on
            // any file whose offsets fall — which `get` also requires.
            if c0 < at {
                walk_keys(file, c0, out)?;
            }
            if c1 < at {
                walk_keys(file, c1, out)?;
            }
            Some(())
        }
        _ => None,
    }
}
