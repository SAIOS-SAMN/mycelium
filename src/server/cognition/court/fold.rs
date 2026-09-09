// 45.8, 47.3 — THE TWO-ORGAN FOLD: the home proposes, the witness fetches
// the prefix and re-derives it, and the home seals only on both signatures.
//
//! A proposal is the finality reading and the nets over the prefix, held
//! until a witness co-signs the root; the commit archives the prefix, seals
//! the `Fold` receipt citing the root, writes the epoch rows anchored to
//! it, and files the era with the witness and both signatures. The witness
//! re-derives the receipts as a chain ending at the root and the rows'
//! nets as the epochs proposed, co-signs, and holds the same era under the
//! home's key, so every archived prefix has two holders.

use crate::chronometry::{ActKind, Era, KStepReceipt, Root};
use crate::cognition::Kernel;
use crate::economy::{nets_of, Entry};
use crate::verdict::{REFUSE_FOLD_INEXACT, REFUSE_FOLD_OPEN, REFUSE_RECEIPT, REFUSE_WITNESS_DEPENDENT, REFUSE_WITNESS_ROOM};

/// What a fold needs a witness for: the prefix's first K, its root, and
/// the epoch rows the home will write. Held as `fold_pending` until a
/// witness co-signs the root or the reading changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub k0: u64,
    pub root: Root,
    pub epochs: Vec<([u8; 32], [u64; 4])>,
}

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
    pub witness: [u8; 32],
}

impl Kernel {
    /// PROPOSE (45.8, 47.3): the finality reading and the nets over the
    /// prefix, without writing; the root is the hot receipt at the prefix
    /// head. A prefix that is not final proposes nothing
    /// (`REFUSE_FOLD_OPEN`); a ledger that does not conserve over it
    /// proposes nothing (`REFUSE_FOLD_INEXACT`).
    pub fn propose_fold(&mut self) -> Result<Proposal, &'static str> {
        let head_k = self.final_prefix_head().ok_or(REFUSE_FOLD_OPEN)?;
        self.treasury.fold_reading(head_k)?;
        let epochs = nets_of(&self.treasury.rows_below(head_k))?.rows;
        let receipts = self.chain.receipts();
        let k0 = receipts.first().ok_or(REFUSE_FOLD_OPEN)?.step_index;
        let at = receipts.iter().find(|r| r.step_index == head_k).ok_or(REFUSE_FOLD_OPEN)?;
        let root = Root { k: at.step_index, hash: at.chain_hash, height: at.height };
        let p = Proposal { k0, root, epochs };
        self.fold_pending = Some(p.clone());
        Ok(p)
    }

    /// The proposal awaiting a witness, if any.
    pub fn fold_proposal(&self) -> Option<&Proposal> {
        self.fold_pending.as_ref()
    }

    /// COMMIT ON TWO SIGNATURES (47.3): the witness is another key than
    /// this court's, its signed position over the root verifies under its
    /// key (47.2 step 2) with the root hash as payload, the proposal still
    /// reads exact and its prefix still final — then the prefix leaves the
    /// hot chain, the `Fold` receipt is sealed citing the root, the epoch
    /// rows are written anchored to it, and the era is filed with the
    /// witness and both signatures. `court fold …` and `court fold
    /// witnessed …` (34.2).
    pub fn commit_fold(
        &mut self,
        sig_home: [u8; 64],
        witness: &crate::anastome::dialogue::ReceiptSection,
    ) -> Result<FoldDone, &'static str> {
        let p = self.fold_pending.clone().ok_or(REFUSE_FOLD_OPEN)?;
        let me = self.identity;
        let wk = witness.receipt.vk;
        if wk == me {
            return Err(REFUSE_WITNESS_DEPENDENT);
        }
        crate::court::admission::verify_signature(&witness.prefix, &wk, &witness.sig)?;
        if witness.prefix.payload != p.root.hash {
            return Err(REFUSE_RECEIPT);
        }
        let epochs = nets_of(&self.treasury.rows_below(p.root.k))?.rows;
        if epochs != p.epochs {
            self.fold_pending = None;
            return Err(REFUSE_FOLD_INEXACT);
        }
        if self.final_prefix_head().is_none_or(|h| h < p.root.k) {
            self.fold_pending = None;
            return Err(REFUSE_FOLD_OPEN);
        }
        let (cold, root) = self.chain.fold_before(p.root.k).ok_or(REFUSE_FOLD_OPEN)?;
        let k0 = cold.first().map_or(root.k, |r| r.step_index);
        self.append_step(ActKind::Fold, root.hash, me);
        let anchor = self.anchor();
        let folded = match self.treasury.fold(p.root.k, anchor) {
            Ok(f) => f,
            Err(reason) => {
                eprintln!("  court fold refused {:02x}{:02x}…: {reason}", me[0], me[1]);
                self.fold_pending = None;
                return Err(reason);
            }
        };
        let identities = folded.rows as u32;
        self.archive.push(Era {
            root,
            signer: me,
            witness: wk,
            sig_home,
            sig_witness: witness.sig,
            identities,
            receipts: cold,
        });
        let done = FoldDone {
            k: self.chain.head_k(),
            h: self.chain.head_height(),
            k0,
            k1: root.k,
            identities,
            witness: wk,
        };
        eprintln!(
            "  court fold {:02x}{:02x}… K {} h {} prefix {}..{} identities {}",
            me[0], me[1], done.k, done.h, done.k0, done.k1, done.identities
        );
        eprintln!(
            "  court fold witnessed {:02x}{:02x}… co-signed {:02x}{:02x}…",
            wk[0], wk[1], root.hash[0], root.hash[1]
        );
        self.fold_pending = None;
        self.mark_persist();
        Ok(done)
    }

    /// THE WITNESS'S RE-DERIVATION (47.3): this court holds bond
    /// (`REFUSE_WITNESS_ROOM` otherwise); the receipts fetched are the
    /// home's chain from `k0`, each re-deriving and linking to the one
    /// before, ending at the root; the rows' nets are the epochs proposed.
    pub fn verify_fold(
        &self,
        home: [u8; 32],
        p: &Proposal,
        receipts: &[KStepReceipt],
        rows: &[Entry],
    ) -> Result<(), &'static str> {
        if self.treasury.bond_of(&self.identity).is_none() {
            return Err(REFUSE_WITNESS_ROOM);
        }
        let mut prev: Option<&KStepReceipt> = None;
        for (k, r) in (p.k0..).zip(receipts.iter()) {
            if r.step_index != k || r.vk != home || !r.rederives() {
                return Err(REFUSE_RECEIPT);
            }
            if prev.is_some_and(|q| r.prev_receipt_hash != q.chain_hash) {
                return Err(REFUSE_RECEIPT);
            }
            prev = Some(r);
        }
        let last = prev.ok_or(REFUSE_FOLD_OPEN)?;
        if (last.step_index, last.chain_hash, last.height) != (p.root.k, p.root.hash, p.root.height) {
            return Err(REFUSE_RECEIPT);
        }
        if nets_of(rows)?.rows != p.epochs {
            return Err(REFUSE_FOLD_INEXACT);
        }
        Ok(())
    }

    /// The witness files the era it co-signed under the home's key, so
    /// the archived prefix has two holders (45.8).
    pub fn hold_era(&mut self, era: Era) {
        self.archive.push(era);
        self.mark_persist();
    }

    /// THE ROWS READ (`ACT_ROWS`): this court's own ledger rows anchored in
    /// `from..=to`, in journal order, a page of at most `budget` from
    /// `cursor`, with the total the span holds. Another signer's rows are
    /// not this court's to serve.
    pub fn rows_range(&self, signer: &[u8; 32], from: u64, to: u64, cursor: u32, budget: usize) -> (u32, Vec<Entry>) {
        if *signer != self.identity || to < from {
            return (0, Vec::new());
        }
        let all = self.treasury.rows_span(from, to);
        let total = all.len() as u32;
        let page = all.into_iter().skip(cursor as usize).take(budget).collect();
        (total, page)
    }
}
