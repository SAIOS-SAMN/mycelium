// 78.147 — A FOLD ROOT WITH ITS TWO SIGNATURES: what a court answers a
// catch-up range opening below a signer's fold root with, and what the asker
// adopts the root on.
//
//! The proof is the fold as 47.3 sealed it: the home's signed position over
//! the root hash and the witness's, each a receipt section. It is verified
//! exactly as `commit_fold` verifies the witness and the witness verifies the
//! home (`judge/fold.rs`): the home's signature under the signer's key at this
//! node's origin, the witness's under another key, both over the root's hash.
//!
//! `[signer 32][root K‖i ‖ hash 32 ‖ height u64][home section][witness section]`

use super::*;
use crate::chronometry::{Parent, Root, Stamp};

/// A FOLD ROOT AND THE TWO SIGNATURES THAT SEALED IT (47.3).
#[derive(Clone)]
pub struct RootProof {
    /// Whose line the fold archived.
    pub signer: [u8; 32],
    pub root: Root,
    /// The home's signed position over the root hash.
    pub home: ReceiptSection,
    /// The witness's signed position over the root hash.
    pub witness: ReceiptSection,
}

impl RootProof {
    /// The root as a citation: its stamp under the signer's key, and hash.
    pub fn parent(&self) -> Parent {
        Parent { at: Stamp::new(self.root.height, self.signer, self.root.at), hash: self.root.hash }
    }

    /// THE TWO SIGNATURES VERIFIED, as the fold was: the home's under the
    /// signer's key at `origin` (47.2 steps 1 and 2), the witness's under
    /// another key (`REFUSE_WITNESS_DEPENDENT` otherwise), each over exactly
    /// the root's hash (`REFUSE_RECEIPT` otherwise).
    pub fn verify(&self, origin: ([u8; 32], u32)) -> Result<(), &'static str> {
        crate::court::admission::admit_signature(origin, &self.home.prefix, &self.signer, &self.home.sig)?;
        let wk = self.witness.receipt.stamp.vk;
        if wk == self.signer {
            return Err(crate::verdict::REFUSE_WITNESS_DEPENDENT);
        }
        crate::court::admission::verify_signature(&self.witness.prefix, &wk, &self.witness.sig)?;
        if self.home.prefix.payload != self.root.hash || self.witness.prefix.payload != self.root.hash {
            return Err(crate::verdict::REFUSE_RECEIPT);
        }
        Ok(())
    }

    /// The proof's tape; `None` where a section does not frame (LD0).
    #[must_use = "a proof that cannot be written is refused by name, never dropped (LD0)"]
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&self.signer);
        self.root.write(out);
        out.extend_from_slice(&encode_receipt_section(&self.home.prefix, &self.home.sig, &self.home.receipt)?);
        out.extend_from_slice(&encode_receipt_section(&self.witness.prefix, &self.witness.sig, &self.witness.receipt)?);
        Some(())
    }

    /// The inverse of [`RootProof::put`], whole or refused.
    pub fn take(b: &[u8], p: &mut usize) -> Option<RootProof> {
        let signer: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        let root = Root::read(b, p)?;
        let home = decode_receipt_section(b, *p)?;
        *p = receipt_section_end(b, *p)?;
        let witness = decode_receipt_section(b, *p)?;
        *p = receipt_section_end(b, *p)?;
        Some(RootProof { signer, root, home, witness })
    }
}

/// WHAT AN `ACT_RECEIPTS` ASK IS ANSWERED WITH: a page of the line, or the
/// signer's fold root where the range opens below it (78.147).
#[derive(Clone)]
pub enum ReceiptsAnswer {
    Page(Vec<crate::chronometry::KStepReceipt>),
    Root(Box<RootProof>),
}

/// The count a page never carries, marking a root answer: a reader that
/// knows no root answer reads it as no page (`decode_receipts_reply`).
const ROOT_MARK: u32 = u32::MAX;

/// A root answer's tape: the mark, then the proof.
pub fn encode_fold_root_reply(proof: &RootProof) -> Option<Vec<u8>> {
    let mut out = ROOT_MARK.to_le_bytes().to_vec();
    proof.put(&mut out)?;
    Some(out)
}

/// An `ACT_RECEIPTS` reply read back: a root answer by its mark, a page
/// otherwise, whole or refused.
pub fn decode_receipts_answer(body: &[u8]) -> Option<ReceiptsAnswer> {
    let mark = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?);
    if mark != ROOT_MARK {
        return decode_receipts_reply(body).map(ReceiptsAnswer::Page);
    }
    let mut p = 4usize;
    let proof = RootProof::take(body, &mut p)?;
    (p == body.len()).then(|| ReceiptsAnswer::Root(Box::new(proof)))
}
