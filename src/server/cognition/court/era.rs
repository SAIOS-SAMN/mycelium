// 45.8 — THE ERA: the finality reading over a court's carriers, the
// archive a fold leaves cold, and the K-range read over both.
//
//! A hypha folds its ledger behind a prefix below which no listing,
//! provisional word or escrow stands, when its reserve reading says so
//! (`court/fold.rs`, two organs); what it archived is read here by K
//! range, for a gap's back-fill or a court's replay of an era.

use crate::chronometry::{Era, KStepReceipt, Place};
use crate::cognition::Kernel;

impl Kernel {
    /// THE FINALITY READING (45.8): the greatest place below which nothing
    /// is open — every listing on the page, every provisional word and
    /// every escrow stands at a place above it — or none when something
    /// open stands at or below every hot receipt, or no hot receipt is
    /// there to fold.
    pub fn final_prefix_head(&self) -> Option<Place> {
        let head = self.chain.head_place()?;
        let first = self.chain.opening()?.stamp.place();
        let mut floor = head;
        // The act before an open one on our own line: its predecessor, as
        // the node names it.
        let before = |at: Place| -> Option<Place> { Some(self.chain.at_place(at)?.prev?.at.place()) };
        // index: the open carriers, each a keyed fiber
        // A listing stands at the List act that admitted it (78.24); one
        // this court cannot place on its ledger is open at every prefix.
        for (addr, _) in self.pending.scoped() {
            let at = self.admitted_at(&addr)?.place();
            if at <= first {
                return None;
            }
            floor = floor.min(before(at)?);
        }
        // A provisional word stands at the act that admitted it (78.24).
        for (addr, _) in self.provisional.scoped() {
            let at = self.admitted_at(&addr)?.place();
            if at <= first {
                return None;
            }
            floor = floor.min(before(at)?);
        }
        // An escrow's row stands at the head act it was written after:
        // that act and everything before it may fold.
        for award in self.treasury.escrowed_awards() {
            let at = crate::economy::stands_at(&award.held.anchor)?;
            if at < first {
                return None;
            }
            floor = floor.min(at);
        }
        // A FOLD CUTS ONLY AT AN ORBIT'S CLOSE (OK1): the prefix head is the
        // last close at or below the floor, so every orbit is held whole on
        // one side of the cut and its close re-derives on the hot line.
        let vk = self.chain.vk();
        let closes = self.chain.line_between(&vk, self.chain.root().map(|r| r.at), floor, usize::MAX);
        let mut head = None;
        for r in closes {
            if r.kind == crate::chronometry::ActKind::Orbit {
                head = Some(r.stamp.place());
            }
        }
        head.filter(|h| *h >= first)
    }

    /// The eras this court holds, in the order it filed them (`ARC1`): its
    /// own folds and the ones it witnessed, each under its signer's key.
    pub fn archive(&self) -> &[Era] {
        &self.archive
    }

    /// THE ERAS OF ONE SIGNER THAT OVERLAP the places strictly after
    /// `after` up to `upto`, found by the coordinate that orders them.
    ///
    /// `commit_fold` files eras in line order and a fold's root is its
    /// last place, so `archive` is sorted by `root.at` within a signer and
    /// the eras that can answer a range are a CONTIGUOUS RUN of it. The
    /// first is found by bisection on `root.at`; the run then ends at the
    /// first era whose own first place is past `upto`. Nothing reads the
    /// eras before the range, and nothing reads the receipts inside one to
    /// decide whether it is the right era (law 3).
    ///
    /// A court holds its own folds and the ones it witnessed for others,
    /// so the archive interleaves signers; the bisection is over the whole
    /// of it and the signer is checked where the run is read, which keeps
    /// one ordering rather than one per signer.
    fn eras_over(&self, signer: &[u8; 32], after: Option<Place>, upto: Place) -> Vec<&Era> {
        let at = self.archive.partition_point(|e| Some(e.root.at) <= after);
        let mut out: Vec<&Era> = Vec::new();
        let Some(run) = self.archive.get(at..) else {
            return out;
        };
        for e in run {
            if e.first_place() > upto {
                break;
            }
            if e.signer == *signer {
                out.push(e);
            }
        }
        out
    }

    /// THE PLACE-RANGE READ (47.2 step 3, 45.8; S13.10): `signer`'s
    /// receipts strictly after `after` (from its line's start where `None`)
    /// up to and including `upto`, in line order, at most `budget` of them
    /// — this court's own from `ARC1` and its hot chain, another signer's
    /// from the eras this court witnessed for it and what it admitted of
    /// it, never past a hole: each receipt read names the one before it as
    /// its predecessor.
    pub fn receipts_range(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, budget: usize) -> Vec<KStepReceipt> {
        let mut out = Vec::new();
        if after.is_some_and(|a| a >= upto) {
            return out;
        }
        // AN ERA NAMES THE PLACE RANGE IT HOLDS, and the archive is in line
        // order, so the era that answers a place is FOUND, never searched for.
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
        let within = |at: Place| after.is_none_or(|a| a < at) && at <= upto;
        let mut cold: Vec<KStepReceipt> = Vec::new();
        for e in self.eras_over(signer, after, upto) {
            let Some(page) = self.era_receipts(e) else { continue };
            // A paged era is in line order: the range is its run between
            // the two places.
            cold.extend(page.iter().filter(|r| within(r.stamp.place())).cloned());
        }
        cold.truncate(budget);
        // THE DESTINATION IS ALREADY KNOWN (law 3): the hot line is read
        // off the line relation between the two places.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the cold run is in line order, so its tail is the last place it read"
        )]
        let hot_after = cold.last().map(|r| r.stamp.place()).or(after);
        let hot = self.chain.line_between(signer, hot_after, upto, budget);
        if *signer == self.identity {
            for r in cold.iter().chain(hot).take(budget) {
                out.push(r.clone());
            }
            return out;
        }
        // Another signer's line, never past a hole: each receipt names the
        // one before it.
        let mut last = after;
        for r in cold.iter().chain(hot) {
            if out.len() >= budget {
                break;
            }
            let prev = r.prev.map(|p| p.at.place());
            if prev != last {
                break;
            }
            last = Some(r.stamp.place());
            out.push(r.clone());
        }
        out
    }
}
