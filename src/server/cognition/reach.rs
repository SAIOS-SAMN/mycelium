// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-HOSTING §5a.9, §5a.10 — THE SIGNED REACH: the one gate between a
//! receipt admitted by back-fill and every reader of it.
//!
//! A back-filled receipt carries no signature of its own (47.2 step 3). It
//! is admitted by its chain and STORED, because the catch-up links on
//! through it: the next receipt of the page, the signed prefix that closes
//! the gap, and the head the chase pages on from all read the stored line.
//! The hash links and never admits (§5a.10), so the stored receipt is
//! marked UNREACHED, held in `unreached` by its place on its signer's line.
//! Its signer's signature reaches it only through a signed prefix at or
//! above that place whose predecessor hashes link down to it. When such a
//! prefix is admitted (`sign_through`), every receipt held at or below it is
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
//! by name (`unreached_refusal`). A signed prefix that contradicts what is
//! held unreached on its line evicts the line's unreached top
//! (`evict_contradicted`): nothing a signature reached is evicted.
//!
//! A gap longer than `C` receipts closes through a fold root (78.147,
//! #88), carried instead of the receipts it covers: a root reconstructed
//! from its two signatures reaches the receipt held unreached at its place
//! where that receipt is the one it names, and every receipt held below it
//! (`reach_by_root`); the receipts a root's shed takes off the chain leave
//! the gate with it (`unhold_shed`).

use super::Court;
use crate::anastome::dialogue::RootProof;
use crate::chronometry::{KStepReceipt, Parent, Place, Stamp};
use crate::court::admission::{Predecessor, Reconstructed};

impl Court {
    /// A RECEIPT ADMITTED BY BACK-FILL, held by its place on its signer's
    /// line until a prefix the signer signed at or above that place is
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

    /// WHERE A SIGNED PREFIX CONTRADICTS ITS SIGNER'S LINE AS HELD UNREACHED:
    /// the prefix's own place, held unreached with another hash, or its
    /// predecessor's place, held unreached with a hash other than the one
    /// the prefix names. `None` where nothing unreached contradicts it.
    fn contradicted(&self, r: &KStepReceipt) -> Option<Place> {
        let vk = &r.stamp.vk;
        let at = r.stamp.place();
        if self.unreached_at(vk, at).is_some_and(|held| Predecessor::of(held).forks(&Predecessor::of(r))) {
            return Some(at);
        }
        let prev = r.prev?;
        let named = self.unreached_at(vk, prev.at.place())?;
        (prev.at.vk == *vk && !Predecessor::named(&prev).continues(&Predecessor::of(named))).then_some(prev.at.place())
    }

    /// MU-HOSTING §5a.9 — EVICTION BY SIGNATURE: `r`, a prefix its signer
    /// signed, contradicts the signer's line where it is held unreached, so
    /// that stored line is not the signer's above the last place a
    /// signature reached. Its unreached top leaves the chain and the gate:
    /// every receipt of the line, from its top down, while it is held
    /// unreached. A receipt a signature reached ends the walk and stays, so
    /// the reached part of the line only ever moves forward. Answers how
    /// many receipts left. The prefix is taken reconstructed: a receipt
    /// whose chain hash does not re-derive evicts nothing (§5a.10).
    pub(crate) fn evict_contradicted(&mut self, r: &Reconstructed<KStepReceipt>) -> usize {
        if self.contradicted(r).is_none() {
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
    /// signature: a back-filled receipt no signed prefix has reached.
    pub(crate) fn unreached_at(&self, signer: &[u8; 32], at: Place) -> Option<&KStepReceipt> {
        self.unreached.get(&Stamp::line_key(signer, at))
    }

    /// A PREFIX `signer` SIGNED AT `upto`, admitted here: every receipt of
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

    /// 78.147, MU-HOSTING §5a.9 (#88) — WHAT A FOLD ROOT COVERS, REACHED BY
    /// ITS SIGNATURES: where the receipt held unreached at the root's place
    /// is the one the root names (its stamp and its chain hash, the link
    /// sense), the root's two signatures reach it, and through its
    /// predecessors every receipt of the line held below it: all are
    /// released and read in line order (`sign_through`). Answers how many
    /// left the gate. A root that names another receipt there, or none held
    /// there, reaches nothing.
    pub(crate) fn reach_by_root(&mut self, root: &Reconstructed<RootProof>, budget: &mut crate::physics::Budget) -> usize {
        let named = root.parent();
        let (vk, at) = (named.at.vk, named.at.place());
        let covered = self
            .unreached_at(&vk, at)
            .is_some_and(|held| Predecessor::cites(&named, &Parent::of(held)));
        if !covered {
            return 0;
        }
        let held = self.unreached.len();
        self.sign_through(&vk, at, budget);
        held - self.unreached.len()
    }

    /// 78.147, MU-HOSTING §5a.9 (#88) — THE GATE FOLLOWS A ROOT'S SHED: a
    /// receipt held unreached at or below its signer's fold root, whose node
    /// the shed took off the chain (`shed_folded_lines`), leaves the gate
    /// unread. The root stands for it, and it was reached by no signature,
    /// so nothing reached leaves here. Answers how many left.
    pub(crate) fn unhold_shed(&mut self) -> usize {
        let mut gone: Vec<Vec<u8>> = Vec::new();
        for vk in self.chain.rooted_signers() {
            let (Some(root), Some(line)) = (self.chain.peer_root(&vk), self.unreached.fibre_at(&vk)) else { continue };
            for (key, _) in line {
                let mut p = vk.len();
                let shed = Place::take_canonical(&key, &mut p).is_some_and(|at| at <= root && !self.chain.holds_place(&vk, at));
                if shed {
                    gone.push(key);
                }
            }
        }
        for key in &gone {
            self.unreached.remove(key);
        }
        gone.len()
    }

    /// THE LOWEST PLACE HELD UNREACHED on a signer's line: where the
    /// signer's line stops being reached by a signature. `None` where
    /// nothing of the line is held unreached.
    pub(crate) fn unreached_floor(&self, signer: &[u8; 32]) -> Option<Place> {
        let (key, _) = self.unreached.fibre_at(signer)?.next()?;
        let mut p = signer.len();
        Place::take_canonical(&key, &mut p)
    }

    /// How many receipts are held unreached, every signer's.
    pub(crate) fn unreached_held(&self) -> usize {
        self.unreached.len()
    }

    /// EVERY READER OF A BACK-FILLED RECEIPT, once a signature reaches it:
    /// the depth index, the stamp alias (and the rulings waiting on the
    /// stamp it files), the fold root where it is a fold and the peers'
    /// folded lines it sheds, and the domain registries where it names a
    /// founding. On the stroke of the act that admitted the signed prefix
    /// (65.5).
    fn read_reached(&mut self, r: &KStepReceipt, budget: &mut crate::physics::Budget) {
        self.note_attestor(r);
        // §3.4 — a back-filled seal is read once its signature reaches it.
        self.note_points(r);
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
    /// prefix of its signer. A prefix admitted under its signature and not
    /// yet cited reads as unreached here, and is read again when its
    /// signer's next prefix is admitted.
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
