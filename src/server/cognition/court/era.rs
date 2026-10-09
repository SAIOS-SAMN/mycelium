// 45.8 — THE ERA: the finality reading over a court's carriers, the
// archive a fold leaves cold, and the K-range read over both.
//
//! A hypha folds its ledger at its last close, whatever stands open below
//! it (78.65; U12: each open chain is projected forward as a puncture,
//! `court/puncture.rs`), two organs signing (`court/fold.rs`); what it
//! archived is read here by K range, for a gap's back-fill or a court's
//! replay of an era.

use crate::chronometry::{Era, KStepReceipt, Place};
use crate::cognition::Kernel;

impl Kernel {
    /// THE FINALITY READING (45.8 as amended by 78.65): finality is per
    /// item, not per prefix. A listing and a provisional word are located
    /// by the stamp that admitted them, which the fold leaves citable
    /// through its era; an escrow is a claim carrier off the ledger, closed
    /// at the act that releases or slashes it — so nothing open holds the
    /// line's fold back. The head is the line's last close; none where no
    /// hot receipt is there to fold.
    pub fn final_prefix_head(&self) -> Option<Place> {
        let first = self.chain.opening()?.stamp.place();
        // A FOLD CUTS ONLY AT AN ORBIT'S CLOSE (OK1): the prefix head is the
        // line's last close, so every orbit is held whole on one side of the
        // cut and its close re-derives on the hot line.
        //
        // READ OFF THE LAST ORBIT, NOT THE WHOLE LINE (78.69). This walked
        // every hot node from the root to find the last close, and it runs on
        // every exchange cycle (`pending_fold_stands`): on hypha b, 117 387
        // nodes per cycle at 192% of a core, which slowed the very door its
        // witnesses fetch the prefix from, so every fold went unmeasured and
        // the line grew — a load spiral. `ReceiptChain::last_close` reads
        // the open orbit's acts and the one before them.
        self.chain.last_close().map(|r| r.stamp.place()).filter(|h| *h >= first)
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
        // 78.153b — THE FIRST CLOSE PAST THE PAGE, read lazily: it unpacked
        // every node of the hot line from the page to the head into one
        // vector (`usize::MAX`) at every close, to find the first `Orbit` in
        // it; now the nodes are read in line order until the close — one
        // orbit's acts past the page (the lock audit, row 8). The same node
        // is found.
        self.chain
            .first_between(&me, past, head, |r| r.kind == crate::chronometry::ActKind::Orbit)
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
        self.line_page(signer, after, upto, budget, bytes, true)
    }

    /// THE READ, with the cold archive paged or not (78.147): a catch-up is
    /// answered off the hot line alone (`serve_receipts`), an era replay —
    /// `why`, the ledger reader, a citation proved by its era — pages it.
    pub(crate) fn line_page(
        &self,
        signer: &[u8; 32],
        after: Option<Place>,
        upto: Place,
        budget: usize,
        bytes: usize,
        cold_read: bool,
    ) -> Vec<KStepReceipt> {
        line_page(self, signer, after, upto, budget, bytes, cold_read)
    }
}

/// 78.151 — WHAT HOLDS A SIGNER'S LINE for a page to be read from: the
/// court under its kernel, or the reading it published (`Published`), which
/// the door reads without the kernel. One reader of a page over both, so
/// the two answer one question one way (law 12).
pub(crate) trait LineHolder {
    /// The key this holder's own line is signed by.
    fn signs_as(&self) -> [u8; 32];
    /// The fold root held for `signer` with its two signatures.
    fn proof_of(&self, signer: &[u8; 32]) -> Option<&crate::anastome::dialogue::RootProof>;
    /// The root `signer`'s line stands on here: this holder's own fold
    /// root, or the root a peer's own fold archived its line to.
    fn standing(&self, signer: &[u8; 32]) -> Option<Place>;
    /// `signer`'s eras overlapping the places strictly after `after` up to
    /// `upto`, each paged in line order and handed to `each` until it says
    /// stop; an era whose payload does not read is passed over.
    fn page_eras(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, each: &mut dyn FnMut(&[KStepReceipt]) -> bool);
    /// `signer`'s hot line strictly after `after` up to `upto`, in line
    /// order, at most `n`.
    fn hot(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<KStepReceipt>;
}

impl LineHolder for Kernel {
    fn signs_as(&self) -> [u8; 32] {
        self.identity
    }

    fn proof_of(&self, signer: &[u8; 32]) -> Option<&crate::anastome::dialogue::RootProof> {
        self.root_proofs.get(signer)
    }

    fn standing(&self, signer: &[u8; 32]) -> Option<Place> {
        match *signer == self.identity {
            true => self.chain.root().map(|r| r.at),
            false => self.chain.peer_root(signer),
        }
    }

    fn page_eras(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, each: &mut dyn FnMut(&[KStepReceipt]) -> bool) {
        for e in self.eras_over(signer, after, upto) {
            let Some(page) = self.era_receipts(e) else { continue };
            if !each(&page) {
                return;
            }
        }
    }

    fn hot(&self, signer: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<KStepReceipt> {
        self.chain.line_between(signer, after, upto, n)
    }
}

/// THE PAGE OF A SIGNER'S LINE (47.2 step 3, 45.8, 78.147), read off any
/// holder of it: the cold run of its eras where `cold_read`, then its hot
/// line, at most `budget` receipts and the receipt that fills `bytes`.
pub(crate) fn line_page(
    holder: &impl LineHolder,
    signer: &[u8; 32],
    after: Option<Place>,
    upto: Place,
    budget: usize,
    bytes: usize,
    cold_read: bool,
) -> Vec<KStepReceipt> {
    let mut out = Vec::new();
    if after.is_some_and(|a| a >= upto) {
        return out;
    }
    let mut tape = Vec::new();
    let mut used = 0usize;
    // A receipt is taken while the page has room; the one that fills it is
    // the last, so a page is never empty for want of bytes.
    let mut fits = |r: &KStepReceipt| -> bool {
        if used >= bytes {
            return false;
        }
        tape.clear();
        // A tape that will not write, or bytes past the counter, fill the
        // page: it is read no further.
        used = match r.write(&mut tape).and_then(|()| used.checked_add(tape.len())) {
            Some(u) => u,
            None => bytes,
        };
        true
    };
    // AN ERA NAMES THE PLACE RANGE IT HOLDS, and the eras are held in line
    // order, so the eras that answer a range are FOUND, never searched for
    // (law 3). THE PAYLOAD IS PAGED, NOT HELD: only the eras this range names
    // are read, and the budget bounds the clone — the run is in line order,
    // so what the budget and the page's bytes leave room for is its head.
    // The cold work a page spent is counted on the serving court
    // (`Court::serve`): eras paged and receipts read off them.
    let within = |at: Place| after.is_none_or(|a| a < at) && at <= upto;
    let mut cold: Vec<KStepReceipt> = Vec::new();
    let mut eras_paged = 0u64;
    if cold_read {
        holder.page_eras(signer, after, upto, &mut |page| {
            eras_paged += 1;
            // A paged era is in line order: the range is its run between the
            // two places.
            for r in page.iter().filter(|r| within(r.stamp.place())) {
                if cold.len() >= budget || !fits(r) {
                    return false;
                }
                cold.push(r.clone());
            }
            true
        });
    }
    {
        use crate::observe::{note_serve, ServeAxis};
        note_serve(ServeAxis::Pages, 1);
        match eras_paged {
            0 => note_serve(ServeAxis::HotOnly, 1),
            n => {
                note_serve(ServeAxis::Cold, 1);
                note_serve(ServeAxis::ErasPaged, n);
                note_serve(ServeAxis::ReceiptsCold, cold.len() as u64);
                note_serve(ServeAxis::MostEras, n);
            }
        }
    }
    // THE DESTINATION IS ALREADY KNOWN (law 3): the hot line is read off the
    // line relation between the two places.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — FIFO drain head: the cold run is in line order, so its tail is the last place it read"
    )]
    let hot_after = cold.last().map(|r| r.stamp.place()).or(after);
    let hot = holder.hot(signer, hot_after, upto, budget);
    let own = *signer == holder.signs_as();
    // Another signer's line is read never past a hole: each receipt names
    // the one before it. The cold run was measured as it was read; the hot
    // run is measured here.
    let mut last = after;
    let cold_len = cold.len();
    for (i, r) in cold.into_iter().chain(hot).enumerate() {
        if out.len() >= budget || (i >= cold_len && !fits(&r)) {
            break;
        }
        if !own {
            if r.prev.map(|p| p.at.place()) != last {
                break;
            }
            last = Some(r.stamp.place());
        }
        out.push(r);
    }
    out
}
