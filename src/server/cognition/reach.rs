// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-HOSTING §5a.9, §5a.10 — THE SIGNED REACH: the one gate between a
//! receipt admitted by back-fill and every reader of it.
//!
//! A back-filled receipt carries no signature of its own (47.2 step 3). It
//! is admitted by its chain and STORED, because the catch-up links on
//! through it: the next receipt of the page, the signed frame that closes
//! the gap, and the head the chase pages on from all read the stored line.
//! The hash links and never admits (§5a.10), so the stored receipt is
//! marked UNREACHED, held in `unreached` by its place on its signer's line.
//! Its signer's signature reaches it only through a signed frame at or
//! above that place whose predecessor hashes link down to it. When such a
//! frame is admitted (`sign_through`), every receipt held at or below it is
//! released in line order and read once (`read_reached`): the depth index,
//! the stamp alias and the rulings waiting on it, a fold's shed, and a
//! founding's reconstruction. Nothing else reads a back-filled receipt
//! before then: an equivocation is proved only against a reached receipt,
//! and a restored court marks again every peer receipt its own line does
//! not cite through (`hold_unreached_at_restore`) before its readers run.
//!
//! The held receipts are bounded by the process's capacity: each was one
//! read of its page (78.153b), and the reads the process holds are
//! `C = ⌊M_proc / b⌋` (MU-DERIVATIONS D2.5). A receipt past that is refused
//! by name (`unreached_refusal`). A signed frame that contradicts what is
//! held unreached on its line evicts the line's unreached top
//! (`evict_contradicted`): nothing a signature reached is evicted.

use super::Court;
use crate::chronometry::{KStepReceipt, Place, Stamp};

impl Court {
    /// A RECEIPT ADMITTED BY BACK-FILL, held by its place on its signer's
    /// line until a frame the signer signed at or above that place is
    /// admitted.
    pub(crate) fn hold_unreached(&mut self, r: &KStepReceipt) {
        self.unreached.insert(Stamp::line_key(&r.stamp.vk, r.stamp.place()), r.clone());
    }

    /// MU-DERIVATIONS D2.5 — THE HELD RECEIPTS' BOUND: one more receipt held
    /// unreached is refused by name where the receipts held already fill
    /// the process's capacity `C`, the reads its own memory holds at the
    /// measured bytes per read. `None` where one more fits.
    pub(crate) fn unreached_refusal(&self) -> Option<&'static str> {
        let held = u64::try_from(self.unreached.len()).ok();
        match held.is_some_and(|n| n < self.physics.capacity()) {
            true => None,
            false => Some(crate::court::verdict::REFUSE_UNREACHED_PAST_MEMORY),
        }
    }

    /// WHERE A SIGNED FRAME CONTRADICTS ITS SIGNER'S LINE AS HELD UNREACHED:
    /// the frame's own place, held unreached with another hash, or its
    /// predecessor's place, held unreached with a hash other than the one
    /// the frame names. `None` where nothing unreached contradicts it.
    fn contradicted(&self, r: &KStepReceipt) -> Option<Place> {
        let vk = &r.stamp.vk;
        let at = r.stamp.place();
        if self.unreached_at(vk, at).is_some_and(|held| held.chain_hash != r.chain_hash) {
            return Some(at);
        }
        let prev = r.prev?;
        let named = self.unreached_at(vk, prev.at.place())?;
        (prev.at.vk == *vk && named.chain_hash != prev.hash).then_some(prev.at.place())
    }

    /// MU-HOSTING §5a.9 — EVICTION BY SIGNATURE: `r`, a frame its signer
    /// signed, contradicts the signer's line where it is held unreached, so
    /// that stored line is not the signer's above the last place a
    /// signature reached. Its unreached top leaves the chain and the gate:
    /// every receipt of the line, from its top down, while it is held
    /// unreached. A receipt a signature reached ends the walk and stays, so
    /// the reached part of the line only ever moves forward. Answers how
    /// many receipts left.
    pub(crate) fn evict_contradicted(&mut self, r: &KStepReceipt) -> usize {
        if !r.rederives() || self.contradicted(r).is_none() {
            return 0;
        }
        let vk = r.stamp.vk;
        let mut left = 0usize;
        while let Some(top) = self.chain.line_head(&vk) {
            let key = Stamp::line_key(&vk, top.stamp.place());
            if !self.unreached.contains(&key) {
                break;
            }
            if self.chain.evict(&top.stamp).is_none() {
                break;
            }
            self.unreached.remove(&key);
            left += 1;
        }
        if left > 0 {
            self.mark_persist();
        }
        left
    }

    /// The receipt held at `at` on `signer`'s line, where it waits for a
    /// signature: a back-filled receipt no signed frame has reached.
    pub(crate) fn unreached_at(&self, signer: &[u8; 32], at: Place) -> Option<&KStepReceipt> {
        self.unreached.get(&Stamp::line_key(signer, at))
    }

    /// A FRAME `signer` SIGNED AT `upto`, admitted here: every receipt of
    /// the signer's line at or below it that waited for a signature is
    /// released and read, in line order.
    pub(crate) fn sign_through(&mut self, signer: &[u8; 32], upto: Place, budget: &mut crate::physics::Budget) {
        let due: Vec<Vec<u8>> = match self.unreached.fibre_at(signer) {
            Some(line) => line
                .map(|(key, _)| key)
                .filter(|key| {
                    let mut p = signer.len();
                    Place::take_canonical(key, &mut p).is_some_and(|at| at <= upto)
                })
                .collect(),
            None => Vec::new(),
        };
        for key in due {
            if let Some(r) = self.unreached.remove(&key) {
                self.read_reached(&r, budget);
            }
        }
    }

    /// EVERY READER OF A BACK-FILLED RECEIPT, once a signature reaches it:
    /// the depth index, the stamp alias (and the rulings waiting on the
    /// stamp it files), the fold root where it is a fold and the peers'
    /// folded lines it sheds, and the domain registries where it names a
    /// founding. On the stroke of the act that admitted the signed frame
    /// (65.5).
    fn read_reached(&mut self, r: &KStepReceipt, budget: &mut crate::physics::Budget) {
        self.note_attestor(r);
        self.alias_admitting(r, budget);
        self.settle_own_attestation(r);
        if r.kind == crate::chronometry::ActKind::Fold {
            self.chain.file_peer_root(r);
            self.shed_folded_lines();
        }
        self.reconstruct_founding(r);
    }

    /// AT RESTORE, before any reader runs: the held marks are volatile, so
    /// each is derived again from the ledger. A peer receipt is reached
    /// where this court's own hot line cites a receipt of its signer at or
    /// above its place, since a parent is cited only after it was admitted
    /// under its signature (45.2) and back-fill never cites. Every other
    /// peer receipt held is marked unreached and waits for the next signed
    /// frame of its signer. A frame admitted under its signature and not
    /// yet cited reads as unreached here, and is read again when its
    /// signer's next frame is admitted.
    pub(crate) fn hold_unreached_at_restore(&mut self) {
        let mut reach: crate::network::nerve::Keyed<Place> = crate::network::nerve::Keyed::new();
        for r in self.chain.own() {
            for p in &r.parents {
                match reach.get(&p.at.vk) {
                    Some(held) if *held >= p.at.place() => {}
                    _ => {
                        reach.insert(p.at.vk, p.at.place());
                    }
                }
            }
        }
        for r in self.chain.admitted() {
            match reach.get(&r.stamp.vk) {
                Some(upto) if r.stamp.place() <= *upto => {}
                _ => self.hold_unreached(&r),
            }
        }
        // The rehydrated chain filed every held fold's root; a fold held
        // unreached files its root only once a signature reaches it.
        let unreached = &self.unreached;
        self.chain.refile_peer_roots(|r| unreached.contains(&Stamp::line_key(&r.stamp.vk, r.stamp.place())));
    }
}
