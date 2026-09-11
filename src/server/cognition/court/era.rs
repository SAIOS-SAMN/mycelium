// 45.8 — THE ERA: the finality reading over a court's carriers, the
// archive a fold leaves cold, and the K-range read over both.
//
//! A hypha folds its ledger behind a prefix below which no listing,
//! provisional word or escrow stands, when its reserve reading says so
//! (`court/fold.rs`, two organs); what it archived is read here by K
//! range, for a gap's back-fill or a court's replay of an era.

use crate::chronometry::{Era, KStepReceipt};
use crate::cognition::Kernel;
use crate::network::nerve::ROOT;

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

    /// The eras this court holds, in the order it filed them (`ARC1`): its
    /// own folds and the ones it witnessed, each under its signer's key.
    pub fn archive(&self) -> &[Era] {
        &self.archive
    }

    /// THE ERAS OF ONE SIGNER THAT OVERLAP `from..=to`, found by the
    /// coordinate that orders them.
    ///
    /// `commit_fold` files eras in K order and a fold's root is its last
    /// K, so `archive` is sorted by `root.k` within a signer and the eras
    /// that can answer a range are a CONTIGUOUS RUN of it. The first is
    /// found by descending on `root.k`; the run then ends at the first era
    /// whose own first K is past `to`. Nothing walks the eras before the
    /// range, and nothing walks the receipts inside one to decide whether
    /// it is the right era (law 3).
    ///
    /// A court holds its own folds and the ones it witnessed for others,
    /// so the archive interleaves signers; the descent is over the whole
    /// of it and the signer is checked where the run is read, which keeps
    /// one ordering rather than one per signer.
    fn eras_over(&self, signer: &[u8; 32], from: u64, to: u64) -> Vec<&Era> {
        let at = self.archive.partition_point(|e| e.root.k < from);
        let mut out: Vec<&Era> = Vec::new();
        let Some(run) = self.archive.get(at..) else {
            return out;
        };
        for e in run {
            match e.first_k() {
                Some(first) if first > to => break,
                Some(_) if e.signer == *signer => out.push(e),
                _ => {}
            }
        }
        out
    }

    /// THE K-RANGE READ (47.2 step 3, 45.8; S13.10): `signer`'s receipts
    /// `from..=to` in K order, at most `budget` of them — this court's
    /// own from `ARC1` and its hot chain, another signer's from the eras
    /// this court witnessed for it and what it admitted of it, never past
    /// a hole.
    pub fn receipts_range(&self, signer: &[u8; 32], from: u64, to: u64, budget: usize) -> Vec<KStepReceipt> {
        let mut out = Vec::new();
        if to < from {
            return out;
        }
        // AN ERA NAMES THE K RANGE IT HOLDS, and the archive is in K
        // order, so the era that answers a K is FOUND, never searched for.
        //
        // This read `archive.iter().flat_map(|e| e.receipts.iter())` and
        // filtered on `step_index` — every receipt of every era, on every
        // request, for a range the era's own `first_k` and `root.k` name.
        // Narrowing that to a filter over eras was the same fault one
        // level up: a linear scan of seventy-one intervals for a
        // coordinate that orders them. Measured on the fleet: a 3561 MiB
        // archive, ~100 back-fill pages a minute, three courts at 50–80%
        // CPU. Law 3 — no loop over a carrier that already names its
        // members — and this carrier names both ends of every interval.
        // THE PAYLOAD IS PAGED, NOT HELD. Only the eras this range names
        // are read from the cold segment, and only when one of them is
        // selected — a court carries descriptors, not history.
        let mut cold: Vec<KStepReceipt> = Vec::new();
        for e in self.eras_over(signer, from, to) {
            let Some(page) = self.era_receipts(e) else { continue };
            let first = e.first;
            cold.extend((from..=to).filter_map(|k| {
                let i = usize::try_from(k.checked_sub(first)?).ok()?;
                let r = page.get(i)?;
                (r.step_index == k).then(|| r.clone())
            }));
        }
        cold.truncate(budget);
        if *signer == self.identity {
            // THE DESTINATION IS ALREADY KNOWN (law 3). The hot chain is
            // contiguous, so `at_k` is arithmetic; a paged era is indexed
            // the same way, by `k − first`.
            let hot = (from..=to).filter_map(|k| self.chain.at_k(k));
            for r in cold.iter().chain(hot).take(budget) {
                out.push(r.clone());
            }
            return out;
        }
        let eras = cold.iter();
        let mut next = from;
        for r in eras {
            if out.len() >= budget || r.step_index > to {
                break;
            }
            if r.step_index == next {
                out.push(r.clone());
                next += 1;
            }
        }
        for k in next..=to {
            if out.len() >= budget {
                break;
            }
            let Some(r) = self.admissions.hash_at(signer, k).and_then(|h| self.chain.known(&h)) else {
                break;
            };
            out.push(r.clone());
        }
        out
    }
}
