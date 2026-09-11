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
        let eras = self.archive.iter().filter(|e| e.signer == *signer).flat_map(|e| e.receipts.iter());
        if *signer == self.identity {
            // THE DESTINATION IS ALREADY KNOWN (law 3). This walked the
            // whole hot chain to find a K range, so every back-fill page
            // scanned two thousand receipts for one of them — a loop over
            // a carrier that names each of its members by its own step
            // index. The hot chain is contiguous, so `at_k` is arithmetic;
            // the cold eras are few and are read as the archive names them.
            let hot = (from..=to).filter_map(|k| self.chain.at_k(k));
            let cold = eras.filter(|r| r.step_index >= from && r.step_index <= to);
            for r in cold.chain(hot).take(budget) {
                out.push(r.clone());
            }
            return out;
        }
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
