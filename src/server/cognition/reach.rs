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

use super::Court;
use crate::chronometry::{KStepReceipt, Place, Stamp};

impl Court {
    /// A RECEIPT ADMITTED BY BACK-FILL, held by its place on its signer's
    /// line until a frame the signer signed at or above that place is
    /// admitted.
    pub(crate) fn hold_unreached(&mut self, r: &KStepReceipt) {
        self.unreached.insert(Stamp::line_key(&r.stamp.vk, r.stamp.place()), r.clone());
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
    /// stamp it files), the peers' folded lines where it is a fold, and the
    /// domain registries where it names a founding. On the stroke of the
    /// act that admitted the signed frame (65.5).
    fn read_reached(&mut self, r: &KStepReceipt, budget: &mut crate::physics::Budget) {
        self.note_attestor(r);
        self.alias_admitting(r, budget);
        self.settle_own_attestation(r);
        if r.kind == crate::chronometry::ActKind::Fold {
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
    }
}
