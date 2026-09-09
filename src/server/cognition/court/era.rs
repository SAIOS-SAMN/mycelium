// 45.8 — THE ERA: the finality reading over a court's carriers, the fold
// behind a fully final prefix, and the archive it leaves cold.
//
//! A hypha folds its ledger behind a prefix below which no listing,
//! provisional word or escrow stands, when its reserve reading says so:
//! the fold writes the ledger's net as `epoch` rows (`Treasury::fold`),
//! seals a receipt of kind `Fold` citing the archived prefix's head, and
//! moves the prefix to `ARC1`, so the chain never breaks and heights
//! continue. A prefix that is not final is `REFUSE_FOLD_OPEN`; a fold
//! that does not conserve is `REFUSE_FOLD_INEXACT`, and neither writes.

use crate::chronometry::{ActKind, Era};
use crate::cognition::Kernel;
use crate::network::nerve::ROOT;

/// What a fold did, read for the line 34.2 writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldDone {
    /// The fold receipt's K and height.
    pub k: u64,
    pub h: u64,
    /// The archived prefix, first K to last.
    pub k0: u64,
    pub k1: u64,
    /// Identities the epoch rows named.
    pub identities: u32,
}

impl Kernel {
    /// THE FINALITY READING (45.8): the greatest K below which nothing is
    /// open — every listing on the page, every provisional word and every
    /// escrow stands at a K above it — or none when something open stands
    /// at or below every hot receipt, or no hot receipt is there to fold.
    pub fn final_prefix_head(&self) -> Option<u64> {
        let head = self.chain.head_k();
        let first = self.chain.receipts().first()?.step_index;
        let mut floor = head;
        // index: the open carriers, each a keyed fiber
        for (_, (_, k)) in self.pending.fiber(ROOT) {
            if *k <= first {
                return None;
            }
            floor = floor.min(k - 1);
        }
        for (_, k) in self.provisional_k.fiber(ROOT) {
            if *k <= first {
                return None;
            }
            floor = floor.min(k - 1);
        }
        for award in self.treasury.escrowed_awards() {
            let k = award.held.anchor.0;
            if k <= first {
                return None;
            }
            floor = floor.min(k - 1);
        }
        (floor >= first).then_some(floor)
    }

    /// FOLD AN ERA (45.8, S13.9): read the finality, read that the ledger
    /// conserves over the prefix, archive the prefix, seal the `Fold`
    /// receipt citing its head, and write the epoch rows anchored to that
    /// receipt. Refuses before it writes: `REFUSE_FOLD_OPEN` with nothing
    /// final, `REFUSE_FOLD_INEXACT` with a ledger that does not conserve.
    pub fn fold_era(&mut self) -> Result<FoldDone, &'static str> {
        let head_k = self.final_prefix_head().ok_or(crate::verdict::REFUSE_FOLD_OPEN)?;
        self.treasury.fold_reading(head_k)?;
        let (cold, root) = self.chain.fold_before(head_k).ok_or(crate::verdict::REFUSE_FOLD_OPEN)?;
        let k0 = cold.first().map_or(root.k, |r| r.step_index);
        let me = self.identity;
        self.append_step(ActKind::Fold, root.hash, me);
        let anchor = self.anchor();
        let folded = match self.treasury.fold(head_k, anchor) {
            Ok(f) => f,
            Err(reason) => {
                eprintln!("  court fold refused {:02x}{:02x}…: {reason}", me[0], me[1]);
                return Err(reason);
            }
        };
        let identities = folded.rows as u32;
        self.archive.push(Era { root, identities, receipts: cold });
        let done = FoldDone { k: self.chain.head_k(), h: self.chain.head_height(), k0, k1: root.k, identities };
        eprintln!(
            "  court fold {:02x}{:02x}… K {} h {} prefix {}..{} identities {}",
            me[0], me[1], done.k, done.h, done.k0, done.k1, done.identities
        );
        self.mark_persist();
        Ok(done)
    }

    /// The eras this court archived, in K order (`ARC1`).
    pub fn archive(&self) -> &[Era] {
        &self.archive
    }
}
