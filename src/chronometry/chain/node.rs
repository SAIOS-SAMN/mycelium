// 45, 78.24 — A NODE AS THE LEDGER HOLDS IT: what the act carries, and
// references for what its line already holds.
//
//! The stamp is the node's key, so the node does not carry it. Its
//! predecessor is a place on its own line where that line holds it, and the
//! full citation only where it does not. Its operator is an index into the
//! operators the ledger has met. A receipt is assembled from a node where
//! one is asked for.

use super::{ActKind, Cite, KStepReceipt, Parent, Place, ReceiptChain, Stamp};

/// THE PREDECESSOR a node names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Prev {
    /// None: the line's first act.
    Genesis,
    /// The node at this place on the same line, which the ledger holds.
    Line(Place),
    /// A node the ledger does not hold, by its stamp and hash.
    Named(Box<Parent>),
}

/// ONE NODE HELD.
#[derive(Debug, Clone)]
pub(super) struct Node {
    pub(super) kind: ActKind,
    pub(super) mu: [u64; 4],
    pub(super) prev: Prev,
    pub(super) parents: Box<[Parent]>,
    pub(super) cites: Box<[Cite]>,
    pub(super) operator: usize,
    pub(super) hash: [u8; 32],
}

impl ReceiptChain {
    /// The index of an operator, met once.
    fn operator_index(&mut self, op: [u8; 32]) -> usize {
        if let Some(&i) = self.operator_at.get(&op) {
            return i;
        }
        let i = self.operators.len();
        self.operators.push(op);
        self.operator_at.insert(op, i);
        i
    }

    /// A RECEIPT AS A NODE: its predecessor by place where its own line
    /// holds that node with that hash, by citation otherwise.
    pub(super) fn pack(&mut self, r: KStepReceipt) -> Node {
        let prev = match r.prev {
            None => Prev::Genesis,
            Some(p) if p.at.vk == r.stamp.vk && self.node_at(&p.at).is_some_and(|n| n.hash == p.hash) => Prev::Line(p.at.place()),
            Some(p) => Prev::Named(Box::new(p)),
        };
        Node {
            kind: r.kind,
            mu: r.mu,
            prev,
            parents: r.parents.into_boxed_slice(),
            cites: r.cites.into_boxed_slice(),
            operator: self.operator_index(r.operator_hash),
            hash: r.chain_hash,
        }
    }

    /// The node held at one stamp.
    pub(super) fn node_at(&self, at: &Stamp) -> Option<&Node> {
        self.nodes.get(&at.key())
    }

    /// The stamp of the node a signer holds at one place.
    pub(super) fn stamp_at(&self, vk: &[u8; 32], at: Place) -> Option<Stamp> {
        let h = *self.lines.get(&Stamp::line_key(vk, at))?;
        Some(Stamp::new(h, *vk, at))
    }

    /// The predecessor a node names, as a citation.
    pub(super) fn prev_of(&self, at: &Stamp, n: &Node) -> Option<Option<Parent>> {
        match &n.prev {
            Prev::Genesis => Some(None),
            Prev::Line(place) => {
                let stamp = self.stamp_at(&at.vk, *place)?;
                Some(Some(Parent { at: stamp, hash: self.node_at(&stamp)?.hash }))
            }
            Prev::Named(p) => Some(Some(**p)),
        }
    }

    /// THE RECEIPT A NODE IS, assembled at its stamp. `None` only where its
    /// predecessor by place is not held, which removal never leaves.
    pub(super) fn unpack(&self, at: Stamp, n: &Node) -> Option<KStepReceipt> {
        Some(KStepReceipt {
            stamp: at,
            kind: n.kind,
            mu: n.mu,
            prev: self.prev_of(&at, n)?,
            parents: n.parents.to_vec(),
            cites: n.cites.to_vec(),
            operator_hash: *self.operators.get(n.operator)?,
            chain_hash: n.hash,
        })
    }

    /// The edges out of a held node: its predecessor and its parents.
    pub(super) fn edges_of(&self, at: &Stamp, n: &Node) -> Vec<Stamp> {
        let prev = self.prev_of(at, n).flatten().map(|p| p.at);
        prev.into_iter().chain(n.parents.iter().map(|p| p.at)).collect()
    }

    /// A NODE LEAVES the ledger, returned as its receipt: off its line and
    /// its placement, and the node after it on its line, which named it by
    /// place, names it by citation from now on.
    pub(super) fn take_node(&mut self, at: &Stamp) -> Option<KStepReceipt> {
        let key = at.key();
        let r = self.unpack(*at, self.nodes.get(&key)?)?;
        let next = at.place().after(r.kind);
        if let Some(after) = self.stamp_at(&at.vk, next) {
            if let Some(n) = self.nodes.get_mut(&after.key()) {
                if n.prev == Prev::Line(at.place()) {
                    n.prev = Prev::Named(Box::new(Parent { at: *at, hash: r.chain_hash }));
                }
            }
        }
        let left = self.lines.remove(&Stamp::line_key(&at.vk, at.place())).is_some();
        self.tally(&at.vk, false, left);
        self.placed.remove(&key);
        self.nodes.remove(&key);
        Some(r)
    }
}
