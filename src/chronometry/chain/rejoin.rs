// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// #273, MU-ENGINEERING §8 — THE REJOIN: a restarted court takes back the
// receipts of its own line that its peers hold above the head it restored,
// before it signs anything.
//
//! A court's own receipts reach its disk only at intervals; a hard stop
//! loses the newest, while its peers already hold them. A court that signed
//! again at those places would equivocate against itself and burn its own
//! bond (observed on the standing mesh, 2026-11-11: a capped hypha killed
//! by its cgroup re-signed 37 places on restart). Replication is
//! reconstruction: each receipt a peer returns is reconstructed from its
//! own coordinates and held on this chain's line only where it continues
//! the head exactly — the predecessor it names is the head, its place is
//! the one the head's kind says follows, a close re-derives its orbit's
//! measure, and its height re-derives over parents this chain holds.

use super::ReceiptChain;
use crate::chronometry::{KStepReceipt, Parent};
use crate::court::admission::{Predecessor, Reconstructed};

/// WHY A RECEIPT OFFERED TO THE REJOIN IS NOT HELD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejoinRefusal {
    /// Signed by another key: a peer's receipt is admitted, never rejoined.
    NotOwn,
    /// This chain holds a node at its stamp already.
    Held,
    /// It does not continue the head: the predecessor it names is not the
    /// head, or its place is not the one that follows the head.
    Discontinuous,
    /// It cites a parent this chain does not hold, so its height cannot
    /// re-derive yet: the parent's signer and place, to be back-filled.
    MissingParent { vk: [u8; 32], k: u64, i: u64 },
    /// Its height does not re-derive over its predecessor and parents.
    Height,
    /// A close whose measure does not re-derive from its orbit's acts, or an
    /// act of a kind no court seals.
    Unsealed,
}

impl ReceiptChain {
    /// #273 — ONE OF THIS CHAIN'S OWN RECEIPTS TAKEN BACK FROM A PEER after a
    /// restart lost it: held on the line and made the head where it continues
    /// the head exactly, so the next act is signed above it. Refused by name
    /// otherwise, and nothing is held.
    pub fn rejoin(&mut self, r: Reconstructed<KStepReceipt>) -> Result<(), RejoinRefusal> {
        if r.stamp.vk != self.vk {
            return Err(RejoinRefusal::NotOwn);
        }
        if self.nodes.contains(&r.stamp.key()) {
            return Err(RejoinRefusal::Held);
        }
        if !r.kind.sealed() || !self.close_rederives(&r) {
            return Err(RejoinRefusal::Unsealed);
        }
        let continues = match (r.prev, self.line_prev()) {
            (Some(named), Some(head)) => Predecessor::cites(&named, &head),
            (None, None) => true,
            _ => false,
        };
        if !continues || r.stamp.place() != self.next_place() {
            return Err(RejoinRefusal::Discontinuous);
        }
        let want = self.next_stamp(&r.parents).ok_or_else(|| missing(self, &r.parents))?;
        if want.h != r.stamp.h {
            return Err(RejoinRefusal::Height);
        }
        let r = r.into_object();
        let stamp = r.stamp;
        self.mark_reached(stamp.place(), &r.parents);
        self.file_on_line(&stamp);
        self.journal.filed(&stamp.key());
        self.file_signings(&r);
        let n = self.pack(r);
        self.nodes.insert(stamp.key(), n);
        self.head = Some(stamp);
        Ok(())
    }
}

/// The first parent this chain does not hold, named.
fn missing(chain: &ReceiptChain, parents: &[Parent]) -> RejoinRefusal {
    match parents.iter().find(|p| chain.height_of(p).is_none()) {
        Some(p) => RejoinRefusal::MissingParent { vk: p.at.vk, k: p.at.k, i: p.at.i },
        None => RejoinRefusal::Height,
    }
}
