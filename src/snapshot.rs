// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 3, 28 — THE SEALED DOOR: the one place a Keyed yields every entry,
//! in key order, for the wire. Cognition names `to_bytes` / `snapshot` on
//! the carrier, never this trait's supertrait.

use crate::network::nerve::Keyed;

mod sealed {
    pub trait Sealed {
        type Value;
        fn walk<F: FnMut(&[u8], &Self::Value)>(&self, f: F);
    }
}

/// The only place a Keyed yields all entries. The supertrait is private
/// to this module; cognition cannot name it.
pub trait Persist: sealed::Sealed {
    fn persist<F: FnMut(&[u8], &Self::Value)>(&self, f: F) {
        sealed::Sealed::walk(self, f);
    }
}

impl<V, C: crate::network::nerve::Coordinate> sealed::Sealed for Keyed<V, C> {
    type Value = V;
    fn walk<F: FnMut(&[u8], &V)>(&self, mut f: F) {
        crate::network::nerve::keyed_persist_walk(self, &mut f);
    }
}

impl<V, C: crate::network::nerve::Coordinate> Persist for Keyed<V, C> {}

/// Walk `k` in key order. Callers live in this module or in a carrier's
/// `to_bytes` that is the wire of that carrier.
pub fn persist<V, C: crate::network::nerve::Coordinate>(
    k: &Keyed<V, C>,
    f: impl FnMut(&[u8], &V),
) {
    <Keyed<V, C> as Persist>::persist(k, f);
}

/// A KEY ON A LOCAL TAPE: `len u64 ‖ bytes`, and TOTAL — there is no length
/// a key can have that this cannot write.
///
/// [`crate::transducer::address::put_addr`] writes a `u16` length and refuses
/// an address longer than 65 535 bytes by writing NOTHING, which is a wire
/// constraint: the citation prefix of 47.1 is part of the protocol and
/// [`crate::transducer::address::Cited`] is the type that proves an address
/// fits it. The ledger and the genome are neither — they are this node's own
/// tape, every other length on them is a `u64` since MYC6, and the 65 535
/// limit buys nothing there.
///
/// It cost something. A form's address is `form_vertex_key`, which writes
/// every vertex, every member and every gauge, so it grows with the form: the
/// largest pair form on the fleet at e98e8a2 held 677 vertices and 4030
/// faces, and its address does not fit a `u16`. The snapshot dropped exactly
/// one record per persist — 24 577 readings of
/// `snapshot: 1 records could not be framed and were not written` across
/// sections of 47, 71, 96, 158, 162 and 171 — and before the refusal was
/// honoured at all, that record went onto the tape with its address field
/// absent, which is the DEF1 desync that cost hypha a 8.6 MB and hypha b
/// 13.7 MB of history.
///
/// A court holding a form too large to cite is not a fault to be silently
/// dropped, and the size at which an address stops fitting is not a
/// mathematical boundary of anything. The tape carries the key.
pub fn put_key(out: &mut Vec<u8>, key: impl AsRef<[u8]>) {
    let key = key.as_ref();
    out.extend_from_slice(&(key.len() as u64).to_le_bytes());
    out.extend_from_slice(key);
}

/// Read a key written by [`put_key`]. `None` where the length or the bytes
/// run past the tape.
pub fn take_key(b: &[u8], pos: &mut usize) -> Option<Vec<u8>> {
    let n = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let n = usize::try_from(n).ok()?;
    let end = pos.checked_add(n).filter(|e| *e <= b.len())?;
    let out = b.get(*pos..end)?.to_vec();
    *pos = end;
    Some(out)
}

/// 78.27 — A TRIE ON THE TAPE: the carrier written as its own name, its
/// byte length and its count, then its nodes in the trie's own key order,
/// each node its KEY INLINE and its LEAVES — `(kind, value)` pairs in kind
/// order, each kind at most once:
///
/// ```text
/// trie  = name len u64 ‖ name ‖ byte_len u64 ‖ count u64 ‖ node…
/// node  = key len u64 ‖ key ‖ leaves u64 ‖ (kind u8 ‖ value len u64 ‖ value)…
/// ```
///
/// A record's position carries nothing: the reader inserts by key, and a
/// key the trie already holds refuses the trie. Nothing is read by where it
/// stands in a stream.
pub type Leaves = Vec<(u8, Vec<u8>)>;

/// Write `nodes` as the trie `name`. The nodes come off the Keyed in key
/// order, which is the order the trie is read in.
pub fn put_trie(out: &mut Vec<u8>, name: &str, nodes: &Keyed<Leaves>) {
    let mut body = Vec::new();
    let mut n = 0u64;
    persist(nodes, |key, leaves| {
        put_key(&mut body, key);
        body.extend_from_slice(&(leaves.len() as u64).to_le_bytes());
        for (kind, value) in leaves {
            body.push(*kind);
            put_key(&mut body, value);
        }
        n += 1;
    });
    put_key(out, name.as_bytes());
    out.extend_from_slice(&(body.len() as u64).to_le_bytes());
    out.extend_from_slice(&n.to_le_bytes());
    out.append(&mut body);
}

/// File one leaf on a node of a trie being written. A leaf kind already on
/// the node is not written twice: the one write of each kind is the
/// caller's (law 12), and a second answers `None`.
pub fn put_leaf(nodes: &mut Keyed<Leaves>, key: &[u8], kind: u8, value: Vec<u8>) -> Option<()> {
    match nodes.get_mut(key) {
        Some(leaves) => {
            if leaves.iter().any(|(k, _)| *k == kind) {
                return None;
            }
            // Filed at its kind's own place, so the leaves stand in kind
            // order as they are written.
            let at = leaves.partition_point(|(k, _)| *k < kind);
            leaves.insert(at, (kind, value));
        }
        None => {
            nodes.insert(key, vec![(kind, value)]);
        }
    }
    Some(())
}

/// Read the trie `name`, whole or refused: its name must be `name`, its
/// body must end at the byte it declares, every node's leaves must stand in
/// strictly rising kind order and every kind must be one of `kinds`, and no
/// key may come twice. The nodes come back keyed, never as a sequence.
pub fn take_trie(b: &[u8], pos: &mut usize, name: &str, kinds: &[u8]) -> Option<Keyed<Leaves>> {
    if take_key(b, pos)? != name.as_bytes() {
        return None;
    }
    let len = usize::try_from(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?)).ok()?;
    *pos += 8;
    let n = usize::try_from(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?)).ok()?;
    *pos += 8;
    let end = pos.checked_add(len).filter(|e| *e <= b.len())?;
    // Every node writes at least its key's length and its leaf count.
    if n > len / 16 {
        return None;
    }
    let mut nodes: Keyed<Leaves> = Keyed::new();
    for _ in 0..n {
        let key = take_key(b, pos)?;
        let nl = usize::try_from(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?)).ok()?;
        *pos += 8;
        if nl > end.checked_sub(*pos)? / 9 {
            return None;
        }
        let mut leaves: Leaves = Vec::with_capacity(nl);
        for _ in 0..nl {
            let kind = *b.get(*pos)?;
            *pos += 1;
            let rising = match leaves.as_slice() {
                [.., (last, _)] => *last < kind,
                [] => true,
            };
            if !rising || !kinds.contains(&kind) {
                return None;
            }
            leaves.push((kind, take_key(b, pos)?));
        }
        if nodes.contains(&key) {
            return None;
        }
        nodes.insert(key, leaves);
    }
    (*pos == end).then_some(nodes)
}

/// The value of one leaf kind on a node read back, or none where the node
/// carries no leaf of that kind.
pub fn leaf(leaves: &Leaves, kind: u8) -> Option<&[u8]> {
    leaves.iter().find(|(k, _)| *k == kind).map(|(_, v)| v.as_slice())
}
