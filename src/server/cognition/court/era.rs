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

    /// THE FOLD'S HEAD, ONE FRAME AT A TIME (45.8, 47.3): the last close at
    /// or below the finality reading within the page a witness fetches in
    /// one `ACT_RECEIPTS` reply — the same read at the same frame. An orbit
    /// longer than a page folds whole at its own close, since a fold cuts
    /// nowhere else.
    pub fn fold_head(&self) -> Option<Place> {
        let head = self.final_prefix_head()?;
        let me = self.chain.vk();
        let after = self.chain.root().map(|r| r.at);
        let (at_most, bytes) = Self::receipts_frame()?;
        // The page is served in line order: walked once, it names the last
        // close on it and the place it reached.
        let (mut cut, mut past) = (None, after);
        for r in self.receipts_page(&me, after, head, at_most, bytes) {
            past = Some(r.stamp.place());
            if r.kind == crate::chronometry::ActKind::Orbit {
                cut = past;
            }
        }
        if cut.is_some() {
            return cut;
        }
        self.chain
            .line_between(&me, past, head, usize::MAX)
            .into_iter()
            .find(|r| r.kind == crate::chronometry::ActKind::Orbit)
            .map(|r| r.stamp.place())
    }

    /// THE PAGE ONE `ACT_RECEIPTS` REPLY CARRIES: at most this many
    /// receipts and this many bytes of their tapes — the frame budget less
    /// the reply's header, and the most of the smallest tape that fits it.
    /// `None` where the frame is smaller than a header.
    pub fn receipts_frame() -> Option<(usize, usize)> {
        let bytes = crate::network::wire::frame_budget().checked_sub(64)?;
        Some((bytes / KStepReceipt::MIN_TAPE, bytes))
    }

    /// THE PEERS' FOLDED LINES LEAVE THE HOT LEDGER (45.8): each peer's
    /// receipts at or below its own fold's root, but for those this court
    /// still reads by them — a parent its next act will cite, an admitting
    /// stamp aliased to a word it holds, an act committing its signer to a
    /// word it holds, which is that word's depth, and the receipt a dialect
    /// transition learned from a peer stands at, which the ledger files it
    /// on.
    pub(crate) fn shed_folded_lines(&mut self) -> u64 {
        let mut stood: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for at in self.dialect_chain.iter().filter_map(|(_, _, at)| *at) {
            stood.insert(at.key(), ());
        }
        for (_, at) in self.cocycle_receipt.scoped() {
            stood.insert(at.key(), ());
        }
        let pending = &self.pending_parents;
        let held = &self.held_at;
        self.chain.shed_folded(|r| {
            let key = r.stamp.key();
            pending.contains(&key)
                || held.contains(&key)
                || stood.contains(&key)
                || r.cites.iter().filter_map(crate::chronometry::Cite::at).any(|at| held.contains(&at.key()))
        })
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
        self.receipts_page(signer, after, upto, budget, usize::MAX)
    }

    /// THE SAME READ, BOUNDED BY THE BYTES A PAGE CAN CARRY (Phase 1 item
    /// 3): no receipt is read past the one whose tape fills `bytes`. The
    /// count bound alone is the frame over the SMALLEST tape, thousands of
    /// receipts, and a page carries what fits — so each served page cloned
    /// and re-read far more than it sent, and hypha b, serving its peers'
    /// catch-up, answered its own console with none of four workers free
    /// (12:05, 128 receipts pages since boot). The work now follows the page.
    pub fn receipts_page(
        &self,
        signer: &[u8; 32],
        after: Option<Place>,
        upto: Place,
        budget: usize,
        bytes: usize,
    ) -> Vec<KStepReceipt> {
        let mut out = Vec::new();
        if after.is_some_and(|a| a >= upto) {
            return out;
        }
        let mut tape = Vec::new();
        let mut used = 0usize;
        // A receipt is taken while the page has room; the one that fills it
        // is the last, so a page is never empty for want of bytes.
        let mut fits = |r: &KStepReceipt| -> bool {
            if used >= bytes {
                return false;
            }
            tape.clear();
            // A tape that will not write, or bytes past the counter, fill
            // the page: it is read no further.
            used = match r.write(&mut tape).and_then(|()| used.checked_add(tape.len())) {
                Some(u) => u,
                None => bytes,
            };
            true
        };
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
        // THE BUDGET BOUNDS THE CLONE, not the result. This cloned every
        // receipt of every era the range named and truncated after: one
        // back-fill request held 561 MB of clones at once on hypha c, which
        // the kernel OOM-killed at its 2 GB cap every six minutes (epoch 114,
        // the jemalloc heap profile at the peak). The run is in line order,
        // so what the budget and the page's bytes leave room for is its head.
        'eras: for e in self.eras_over(signer, after, upto) {
            let Some(page) = self.era_receipts(e) else { continue };
            // A paged era is in line order: the range is its run between
            // the two places.
            for r in page.iter().filter(|r| within(r.stamp.place())) {
                if cold.len() >= budget || !fits(r) {
                    break 'eras;
                }
                cold.push(r.clone());
            }
        }
        // THE DESTINATION IS ALREADY KNOWN (law 3): the hot line is read
        // off the line relation between the two places.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the cold run is in line order, so its tail is the last place it read"
        )]
        let hot_after = cold.last().map(|r| r.stamp.place()).or(after);
        let hot = self.chain.line_between(signer, hot_after, upto, budget);
        let own = *signer == self.identity;
        // Another signer's line is read never past a hole: each receipt
        // names the one before it. The cold run was measured as it was
        // read; the hot run is measured here.
        let mut last = after;
        let cold_len = cold.len();
        for (i, r) in cold.into_iter().map(std::borrow::Cow::Owned).chain(hot.into_iter().map(std::borrow::Cow::Borrowed)).enumerate() {
            if out.len() >= budget || (i >= cold_len && !fits(&r)) {
                break;
            }
            if !own {
                if r.prev.map(|p| p.at.place()) != last {
                    break;
                }
                last = Some(r.stamp.place());
            }
            out.push(r.into_owned());
        }
        out
    }
}
