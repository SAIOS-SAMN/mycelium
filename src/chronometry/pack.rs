// 45.8, 78.24 — THE PACKED ACT: a receipt written as what the act carries
// and references for what its neighbours already hold, read back by
// re-deriving the rest.
//
//! A packed node is its signer and operator by index, its kind and frame,
//! its predecessor — the last node of its own line already written, or the
//! full citation where the stream does not carry it — its parents and its
//! citations. Its stamp is re-derived (the signer from its index, the place
//! from its predecessor's place and kind, the height over its predecessor
//! and parents), and its hash from all of it. A signer or an operator is
//! written once, where it is first used. An ANCHOR names a line's last node
//! by its place and hash; the reader re-derives that hash and refuses the
//! stream where it differs, so one changed byte anywhere in a line is seen.
//! The same stream carries the line on disk (with its roots and the nodes
//! that left) and an era's payload.

use super::chain::height_over;
use super::era::Root;
use super::receipt::{ActKind, Cite, KStepReceipt, Parent, Place, Stamp};
use crate::network::nerve::Keyed;

const SIGNER: u8 = 1;
const OPERATOR: u8 = 2;
const NODE: u8 = 3;
const ANCHOR: u8 = 4;
const ROOT: u8 = 5;
const GONE: u8 = 6;

/// The predecessor as a packed node names it.
const PREV_NONE: u8 = 0;
const PREV_LINE: u8 = 1;
const PREV_NAMED: u8 = 2;

/// A count or a lane, in seven-bit groups, low first.
pub(crate) fn put_var(out: &mut Vec<u8>, mut v: u64) {
    while v >= 0x80 {
        out.push((v & 0x7f).to_le_bytes()[0] | 0x80);
        v >>= 7;
    }
    out.push(v.to_le_bytes()[0]);
}

/// The inverse of [`put_var`]: `None` past the end, or past a `u64`.
pub(crate) fn take_var(b: &[u8], p: &mut usize) -> Option<u64> {
    let mut v = 0u64;
    let mut shift = 0u32;
    loop {
        let byte = *b.get(*p)?;
        *p += 1;
        let part = u64::from(byte & 0x7f);
        if shift >= 64 || (shift == 63 && part > 1) {
            return None;
        }
        v |= part << shift;
        if byte & 0x80 == 0 {
            return Some(v);
        }
        shift += 7;
    }
}

/// A count as a `usize`, no larger than the bytes left could carry.
fn take_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let n = usize::try_from(take_var(b, p)?).ok()?;
    (n <= b.len().checked_sub(*p)?).then_some(n)
}

/// THE LAST NODE OF A LINE the stream carries: what the next node of that
/// line re-derives its place, height and predecessor from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Top {
    at: Stamp,
    kind: ActKind,
    hash: [u8; 32],
}

/// WHAT A STREAM HAS ALREADY WRITTEN: its signers and operators by index,
/// and each line's last node.
#[derive(Debug, Clone, Default)]
pub struct Packer {
    signers: Keyed<u64>,
    operators: Keyed<u64>,
    tops: Keyed<Top>,
}

impl Packer {
    pub fn new() -> Self {
        Packer::default()
    }

    /// A signer, defined in the stream where it is not yet: the first a
    /// stream defines is the key the stream is written for.
    pub fn signer(&mut self, out: &mut Vec<u8>, vk: &[u8; 32]) {
        Self::index(&mut self.signers, out, SIGNER, vk);
    }

    /// An index, defined in the stream where it is first used.
    fn index(table: &mut Keyed<u64>, out: &mut Vec<u8>, tag: u8, key: &[u8; 32]) -> u64 {
        if let Some(&i) = table.get(key) {
            return i;
        }
        let i = table.len() as u64;
        table.insert(key, i);
        out.push(tag);
        out.extend_from_slice(key);
        i
    }

    /// ONE NODE, packed. `None` where a citation does not write (LD0), or
    /// where a node with no predecessor does not stand at genesis.
    pub fn node(&mut self, out: &mut Vec<u8>, r: &KStepReceipt) -> Option<()> {
        let vk = r.stamp.vk;
        let signer = Self::index(&mut self.signers, out, SIGNER, &vk);
        let operator = Self::index(&mut self.operators, out, OPERATOR, &r.operator_hash);
        let mut rec = vec![NODE];
        put_var(&mut rec, signer);
        rec.push(r.kind.byte());
        for lane in r.mu {
            put_var(&mut rec, lane);
        }
        let top = self.tops.get(&vk).copied();
        match (r.prev, top) {
            (None, _) if r.stamp.place() == Place::GENESIS => rec.push(PREV_NONE),
            (None, _) => return None,
            (Some(p), Some(t)) if p == (Parent { at: t.at, hash: t.hash }) && t.at.place().after(t.kind) == r.stamp.place() => {
                rec.push(PREV_LINE);
            }
            (Some(p), _) => {
                rec.push(PREV_NAMED);
                p.put(&mut rec);
                put_var(&mut rec, r.stamp.k);
                put_var(&mut rec, r.stamp.i);
            }
        }
        put_var(&mut rec, r.parents.len() as u64);
        for p in &r.parents {
            p.put(&mut rec);
        }
        put_var(&mut rec, r.cites.len() as u64);
        for c in &r.cites {
            c.put_canonical(&mut rec)?;
        }
        put_var(&mut rec, operator);
        out.extend_from_slice(&rec);
        self.tops.insert(vk, Top { at: r.stamp, kind: r.kind, hash: r.chain_hash });
        Some(())
    }

    /// AN ANCHOR on every line this stream has written: its last node's
    /// place and hash.
    pub fn anchors(&self, out: &mut Vec<u8>) {
        for (vk, t) in self.tops.scoped() {
            let Some(&signer) = self.signers.get(&vk) else { continue };
            out.push(ANCHOR);
            put_var(out, signer);
            put_var(out, t.at.k);
            put_var(out, t.at.i);
            out.extend_from_slice(&t.hash);
        }
    }

    /// A root this chain's own line was folded to.
    pub fn root(&self, out: &mut Vec<u8>, root: &Root) {
        out.push(ROOT);
        root.write(out);
    }

    /// A node that left, by its stamp.
    pub fn gone(&self, out: &mut Vec<u8>, at: &Stamp) {
        out.push(GONE);
        at.put_canonical(out);
    }
}

/// WHAT A STREAM CARRIES, read back.
#[derive(Debug)]
pub enum Record {
    Node(Box<KStepReceipt>),
    Root(Root),
    Gone(Stamp),
    /// A signer, an operator or an anchor: read, and nothing to apply.
    Read,
}

/// THE READER: the signers and operators defined so far, and each line's
/// last node as re-derived.
#[derive(Debug, Clone, Default)]
pub struct Unpacker {
    signers: Vec<[u8; 32]>,
    operators: Vec<[u8; 32]>,
    tops: Keyed<Top>,
    /// The first signer defined: the key the stream is written for.
    own: Option<[u8; 32]>,
}

impl Unpacker {
    pub fn new() -> Self {
        Unpacker::default()
    }

    /// The stream as written so far, for the next write after it: the same
    /// indices and the same last nodes.
    pub fn into_packer(self) -> Packer {
        let mut p = Packer::new();
        for (i, vk) in self.signers.iter().enumerate() {
            p.signers.insert(vk, i as u64);
        }
        for (i, op) in self.operators.iter().enumerate() {
            p.operators.insert(op, i as u64);
        }
        p.tops = self.tops;
        p
    }

    /// The first signer the stream defined: the key it is written for.
    pub fn first_signer(&self) -> Option<[u8; 32]> {
        self.own
    }

    fn key(table: &[[u8; 32]], b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
        table.get(usize::try_from(take_var(b, p)?).ok()?).copied()
    }

    /// ONE RECORD read at `p`. `None` refuses the stream: a record that
    /// does not read whole, an index not defined, a predecessor the stream
    /// does not carry, or an anchor its line does not re-derive.
    pub fn take(&mut self, b: &[u8], p: &mut usize) -> Option<Record> {
        let tag = *b.get(*p)?;
        *p += 1;
        match tag {
            SIGNER | OPERATOR => {
                let key: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                match tag {
                    SIGNER => {
                        self.own = self.own.or(Some(key));
                        self.signers.push(key);
                    }
                    _ => self.operators.push(key),
                }
                Some(Record::Read)
            }
            NODE => self.node(b, p).map(|r| Record::Node(Box::new(r))),
            ANCHOR => {
                let vk = Self::key(&self.signers, b, p)?;
                let at = Place { k: take_var(b, p)?, i: take_var(b, p)? };
                let hash: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                let top = self.tops.get(&vk)?;
                (top.at.place() == at && top.hash == hash).then_some(Record::Read)
            }
            ROOT => Some(Record::Root(Root::read(b, p)?)),
            GONE => Some(Record::Gone(Stamp::take_canonical(b, p)?)),
            _ => None,
        }
    }

    fn node(&mut self, b: &[u8], p: &mut usize) -> Option<KStepReceipt> {
        let vk = Self::key(&self.signers, b, p)?;
        let kind = ActKind::of_sealed(*b.get(*p)?)?;
        *p += 1;
        let mut mu = [0u64; 4];
        for lane in &mut mu {
            *lane = take_var(b, p)?;
        }
        let tag = *b.get(*p)?;
        *p += 1;
        let (prev, at) = match tag {
            PREV_NONE => (None, Place::GENESIS),
            PREV_LINE => {
                let t = self.tops.get(&vk)?;
                (Some(Parent { at: t.at, hash: t.hash }), t.at.place().after(t.kind))
            }
            PREV_NAMED => {
                let prev = Parent::take(b, p)?;
                (Some(prev), Place { k: take_var(b, p)?, i: take_var(b, p)? })
            }
            _ => return None,
        };
        let np = take_count(b, p)?;
        let mut parents = Vec::with_capacity(np);
        for _ in 0..np {
            parents.push(Parent::take(b, p)?);
        }
        let nc = take_count(b, p)?;
        let mut cites = Vec::with_capacity(nc);
        for _ in 0..nc {
            cites.push(Cite::take_canonical(b, p)?);
        }
        let operator = Self::key(&self.operators, b, p)?;
        let heights: Vec<u64> = prev.iter().chain(parents.iter()).map(|q| q.at.h).collect();
        let stamp = Stamp::new(height_over(&heights), vk, at);
        let r = KStepReceipt::seal(stamp, kind, mu, prev, parents, cites, operator)?;
        self.tops.insert(vk, Top { at: r.stamp, kind: r.kind, hash: r.chain_hash });
        Some(r)
    }
}
