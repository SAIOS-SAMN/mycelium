// 78.147 — CATCH-UP FROM A FOLD ROOT: the root a fold sealed, held with its
// two signatures; the serving door answering a range below it with it; the
// asker adopting it and continuing the signer's line hot from it.
//
//! A court whose held head of a signer stood far below that signer's last
//! fold paged the cold archive on every page (66 341 pages on hypha c, all
//! cold, 13 007 answers unwritten) and stopped on citations past its head:
//! every court admitted 0 of every peer's receipts. A fold is final on two
//! signatures (47.3), so its root is what the line below it IS to anyone
//! catching up: the asker adopts it and never back-fills below it, and the
//! cold archive answers only an era replay.

use crate::anastome::dialogue::{ReceiptSection, ReceiptsAnswer, RootProof};
use crate::chronometry::{Era, Place};
use crate::cognition::court::era::{line_page, LineHolder};
use crate::cognition::court::fold::FoldDone;
use crate::cognition::Court;
use crate::observe::{note_replication, note_serve, rep, ServeAxis};

/// WHAT A ROOT ANSWER DID HERE: adopted, the line already standing at or
/// past it, or refused by the reason its proof did not stand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootAdoption {
    Adopted,
    Held,
    Refused(&'static str),
}

impl Court {
    /// THE FOLD COMMITTED WITH ITS PROOF KEPT: `commit_fold` on the home's
    /// signature, and the root it sealed held with the home's and the
    /// witness's signed positions, so a catch-up below it is answered with it.
    pub fn commit_fold_proved(&mut self, home: &ReceiptSection, witness: &ReceiptSection) -> Result<FoldDone, &'static str> {
        let done = self.commit_fold(home.sig, witness)?;
        if let Some(root) = self.chain.root() {
            let signer = self.identity;
            self.file_root_proof(RootProof { signer, root, home: home.clone(), witness: witness.clone() });
        }
        Ok(done)
    }

    /// THE WITNESS HOLDS THE ERA AND ITS PROOF: the era under the home's key
    /// (`hold_era`), and the root with the home's signed position it verified
    /// and its own co-signature — the root's second holder answers with it.
    pub fn hold_era_proved(&mut self, era: Era, home: &ReceiptSection, witness: &ReceiptSection) {
        let (signer, root) = (era.signer, era.root);
        self.hold_era(era);
        self.file_root_proof(RootProof { signer, root, home: home.clone(), witness: witness.clone() });
    }

    /// One signer's proof, kept only where its root stands past the one held.
    fn file_root_proof(&mut self, proof: RootProof) {
        let later = self.root_proofs.get(&proof.signer).is_none_or(|held| held.root.at < proof.root.at);
        if later {
            self.root_proofs.insert(proof.signer, proof);
            self.proofs_mark = crate::chronometry::next_mark();
            self.mark_persist();
        }
    }

    /// The fold root held for a signer with its two signatures, if any.
    pub fn root_proof(&self, signer: &[u8; 32]) -> Option<&RootProof> {
        self.root_proofs.get(signer)
    }

    /// 47.3 — THE CO-SIGNATURE THIS COURT ALREADY GAVE: where it holds `home`'s
    /// fold at `root` with its own signed position as the witness (`own_vk`),
    /// that position — what the witness said once and says again when the
    /// same proposal is asked of it, never re-deriving a prefix it proved.
    /// `None` for another root, or a root whose witness was another court.
    pub fn cosigned_root(&self, home: &[u8; 32], root: &crate::chronometry::Root, own_vk: &[u8; 32]) -> Option<ReceiptSection> {
        let proof = self.root_proofs.get(home)?;
        (proof.root == *root && proof.witness.receipt.stamp.vk == *own_vk).then(|| proof.witness.clone())
    }

    /// THE ROOTS' LEAF (`LEAF_ROOTS`): the proofs' count, then each proof; `None`
    /// where none is held. A proof that does not frame is named and left out
    /// of the count (LD0) — it is proved again at the next fold.
    pub(crate) fn put_root_proofs(&self) -> Option<Vec<u8>> {
        let mut framed: Vec<Vec<u8>> = Vec::new();
        for (_, proof) in self.root_proofs.scoped() {
            let mut v = Vec::new();
            match proof.put(&mut v) {
                Some(()) => framed.push(v),
                None => eprintln!("  root proof refused: a signed section of it cannot be written"),
            }
        }
        if framed.is_empty() {
            return None;
        }
        let mut out = Vec::new();
        crate::transducer::address::put_key_count(&mut out, framed.len());
        for v in framed {
            out.extend_from_slice(&v);
        }
        Some(out)
    }

    /// The roots' leaf read back, whole or refused.
    pub(crate) fn take_root_proofs(b: &[u8], p: &mut usize) -> Option<Vec<RootProof>> {
        let n = crate::transducer::address::take_key_count(b, p)?;
        (0..n).map(|_| RootProof::take(b, p)).collect()
    }

    /// ADOPT A PEER'S FOLD ROOT (78.147): its two signatures verified as the
    /// fold's were (`RootProof::verify`), the signer's line stands on it here
    /// where it stood below — its next receipt names the root and admits hot,
    /// and nothing below it is asked again; what this court held of the line
    /// below it leaves (`shed_folded_lines`). Counted under the signer either
    /// way, never silent.
    pub fn adopt_root(&mut self, origin: ([u8; 32], u32), proof: &RootProof) -> RootAdoption {
        let verdict = match proof.signer == self.identity {
            true => Err(crate::verdict::REFUSE_RECEIPT),
            false => proof.verify(origin),
        };
        if let Err(why) = verdict {
            note_replication(Some(proof.signer), rep::ROOT_REFUSED, 1);
            return RootAdoption::Refused(why);
        }
        // A proof that stands is held whether or not the line needed it, so
        // this court answers a catch-up below the root with it too.
        self.file_root_proof(proof.clone());
        if !self.chain.adopt_root(proof.parent()) {
            return RootAdoption::Held;
        }
        note_replication(Some(proof.signer), rep::ROOT_ADOPTED, 1);
        self.shed_folded_lines();
        self.mark_persist();
        RootAdoption::Adopted
    }

    /// THE SERVING DOOR OF `ACT_RECEIPTS` (78.147). An era replay pages the
    /// cold archive (`receipts_page`). A catch-up opening below the signer's
    /// fold root is answered with the root and its two signatures — a
    /// citation the asker proves — never with cold pages; one past it with
    /// the hot line. A catch-up below a root this court holds no proof of is
    /// answered off the hot line alone, and counted.
    pub fn serve_receipts(
        &self,
        signer: &[u8; 32],
        after: Option<Place>,
        upto: Place,
        replay: bool,
        budget: usize,
        bytes: usize,
    ) -> ReceiptsAnswer {
        serve_receipts(self, signer, after, upto, replay, budget, bytes)
    }
}

/// THE SERVING DOOR OF `ACT_RECEIPTS` over any holder of the line: the
/// court under its kernel, or the reading it published (78.151).
pub(crate) fn serve_receipts(
    holder: &impl LineHolder,
    signer: &[u8; 32],
    after: Option<Place>,
    upto: Place,
    replay: bool,
    budget: usize,
    bytes: usize,
) -> ReceiptsAnswer {
    if replay {
        note_serve(ServeAxis::Replays, 1);
        return ReceiptsAnswer::Page(line_page(holder, signer, after, upto, budget, bytes, true));
    }
    let below = |root: Place| after.is_none_or(|a| a < root);
    if let Some(proof) = holder.proof_of(signer).filter(|p| below(p.root.at)) {
        note_serve(ServeAxis::Pages, 1);
        note_serve(ServeAxis::ServedRoot, 1);
        return ReceiptsAnswer::Root(Box::new(proof.clone()));
    }
    // A ROOT WITH NO PROOF HELD IS PAGED FROM THE ARCHIVE, which links the
    // page to the asker's head below it until a proved fold takes over the
    // door (78.147).
    let unproved = holder.standing(signer).is_some_and(below);
    if unproved {
        note_serve(ServeAxis::UnprovedRoot, 1);
    }
    ReceiptsAnswer::Page(line_page(holder, signer, after, upto, budget, bytes, unproved))
}
