// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §7.1 (#137); MU-RECURSION 47.1, 47.4; MU-XSEPTUM §5.5 — THE
//! SIGNED RECEIPTS A MIND HOLDS, AND THE EQUIVOCATION PROOFS IT SENDS.
//!
//! Every digest a hypha answers carries that hypha's signed position behind
//! it: 47.1's receipt section, its head receipt with the prefix it signed
//! and the signature (`sign_cover_digest`). A receipts reply carries bare
//! receipts, so this section is the one signed receipt a mind is handed.
//! The session reads it off every digest it absorbs, holds each signer's
//! last one, and where a second one stands at the same place of the same
//! line under a different chain hash, the two are 47.4's proof: the mind
//! presents them (`ACT_EQUIVOCATION`) to the hyphas it knows. Proofs are by
//! anyone (MU-XSEPTUM §5.5; MU-HOSTING §5.3).
//!
//! The mind keeps no receipt chain of its own (78.49): this holds the head
//! each signer last showed it, one section per signer, and nothing else.
//! A section is held only where the court's own checks pass on it: its
//! signature verifies over the 47.1 bytes (47.2 step 2), its receipt
//! re-derives its chain hash (`Reconstructed::receipt`), and the prefix and
//! the receipt are one claim (`admission::one_claim`). The fork itself is
//! decided by the court's predicate, `admit_equivocation`, never restated
//! here.

use super::*;
use crate::anastome::dialogue::ReceiptSection;
use crate::chronometry::{KStepReceipt, Place};
use crate::court::admission::{self, Reconstructed};

/// WHAT HOLDING ONE SIGNED RECEIPT DID: a typed reading, never a flag.
#[derive(Debug, Clone)]
pub enum Holding {
    /// The first section held from this signer, or the first on a line of
    /// another origin (a recut restarts the line, 45.5): held.
    Opened,
    /// A later place on the line held: it replaces the head held at `from`.
    Advanced { from: Place },
    /// The receipt held at this place, signed again (another payload, one
    /// chain hash): one receipt, nothing proved.
    Duplicate,
    /// A place before the head held: nothing held at it to compare, so
    /// nothing is held or proved.
    Behind { head: Place },
    /// The section fails a check the court would run on it, named by its
    /// 34.1 constant: nothing held.
    Refused(&'static str),
    /// Two signed receipts under one key at one place with two chain hashes
    /// (47.4): `proof` is the `ACT_EQUIVOCATION` frame presenting them, the
    /// head held first and the one just read.
    Equivocated { at: Place, proof: Vec<u8> },
}

/// EACH SIGNER'S LAST SIGNED RECEIPT, keyed by its verifying key: bounded
/// by the hyphas the session has heard from, never by a constant.
#[derive(Default)]
pub struct SignedReceipts {
    heads: crate::network::nerve::Keyed<ReceiptSection>,
}

impl SignedReceipts {
    pub fn new() -> Self {
        Self::default()
    }

    /// The section held for `signer`, if any.
    pub fn head_of(&self, signer: &[u8; 32]) -> Option<&ReceiptSection> {
        self.heads.get(signer)
    }

    /// HOLD ONE SIGNED RECEIPT, read by the court's own checks. A section
    /// that fails one is refused by name and changes nothing held.
    pub fn hold(&mut self, section: ReceiptSection) -> Holding {
        let signer = section.receipt.stamp.vk;
        let Ok(verified) = admission::verify_signature(&section.prefix, &signer, &section.sig) else {
            return Holding::Refused(crate::verdict::REFUSE_SIGNATURE);
        };
        let Some(read) = Reconstructed::<KStepReceipt>::receipt(&section.receipt) else {
            return Holding::Refused(crate::verdict::REFUSE_RECEIPT);
        };
        if !admission::one_claim(verified.prefix(), &read) {
            return Holding::Refused(crate::verdict::REFUSE_RECEIPT);
        }
        let at = section.receipt.stamp.place();
        let Some(held) = self.heads.get(&signer) else {
            self.heads.insert(signer, section);
            return Holding::Opened;
        };
        // 45.5: a place is a place on ONE origin's line; a section naming
        // another genesis or epoch opens that line afresh.
        if held.prefix.genesis != section.prefix.genesis || held.prefix.epoch != section.prefix.epoch {
            self.heads.insert(signer, section);
            return Holding::Opened;
        }
        let head = held.receipt.stamp.place();
        match at.cmp(&head) {
            std::cmp::Ordering::Greater => {
                self.heads.insert(signer, section);
                Holding::Advanced { from: head }
            }
            std::cmp::Ordering::Less => Holding::Behind { head },
            std::cmp::Ordering::Equal => Self::compare_at(held, &section, verified, &read),
        }
    }

    /// Two sections at one place of one line: 47.4's predicate decides,
    /// the held one re-read by the same checks it was held under.
    fn compare_at(
        held: &ReceiptSection,
        section: &ReceiptSection,
        verified: admission::Verified<'_>,
        read: &Reconstructed<KStepReceipt>,
    ) -> Holding {
        let signer = held.receipt.stamp.vk;
        let (Ok(held_verified), Some(held_read)) = (
            admission::verify_signature(&held.prefix, &signer, &held.sig),
            Reconstructed::<KStepReceipt>::receipt(&held.receipt),
        ) else {
            return Holding::Refused(crate::verdict::REFUSE_RECEIPT);
        };
        match admission::admit_equivocation(held_verified, &held_read, verified, read) {
            Ok(proved) => match dialogue::encode_equivocation(held, section) {
                Some(proof) => Holding::Equivocated { at: proved.at, proof },
                None => Holding::Refused(crate::verdict::REFUSE_RECEIPT),
            },
            Err(_) => Holding::Duplicate,
        }
    }
}

impl Session {
    /// THE SIGNED RECEIPT BEHIND A DIGEST, HELD: `body` is a digest reply
    /// whose cover digest `vk` signed and which verified. The section is
    /// read where the digest ends, held only where it is `vk`'s own, and an
    /// equivocation it proves is presented at once to every hypha this
    /// session holds an address for. A digest with no section holds nothing.
    pub(crate) fn hold_signed_receipt(&self, body: &[u8], vk: &[u8; 32]) -> Option<Holding> {
        let at = dialogue::cover_digest_end(body)?;
        let section = dialogue::decode_receipt_section(body, at)?;
        if section.receipt.stamp.vk != *vk {
            return Some(Holding::Refused(crate::verdict::REFUSE_SIGNATURE));
        }
        let holding = self.signed.borrow_mut().hold(section);
        if let Holding::Equivocated { at, proof } = &holding {
            self.present_equivocation(vk, *at, proof);
        }
        Some(holding)
    }

    /// 47.4 BY ANYONE: the proof spoken to each hypha this session knows,
    /// each court burning the signer's bond it holds. The answer is the
    /// court's `Ruled`, named in the log; nothing in cognition reads it.
    fn present_equivocation(&self, signer: &[u8; 32], at: Place, proof: &[u8]) {
        let addrs: Vec<String> = self.addrs.borrow().scoped().map(|(_, a)| a.clone()).collect();
        for endpoint in addrs {
            let ruled = match self.exchange_at(&endpoint, proof.to_vec()).ok().and_then(|b| dialogue::decode_ruled(&b)) {
                Some(dialogue::Ruled::Applied) => "the bond burned".to_string(),
                Some(dialogue::Ruled::Inert) => "no bond left to burn".to_string(),
                Some(dialogue::Ruled::Refused(why)) => format!("refused: {}", String::from_utf8_lossy(&why)),
                Some(dialogue::Ruled::Gap { .. }) => "a gap named".to_string(),
                None => "unanswered".to_string(),
            };
            eprintln!(
                "  mind equivocation {:02x}{:02x}… at {} presented to {endpoint}: {ruled}",
                signer[0],
                signer[1],
                at.print()
            );
        }
    }

    /// The signed receipts this session holds, one per signer.
    pub fn signed_receipts(&self) -> std::cell::Ref<'_, SignedReceipts> {
        self.signed.borrow()
    }
}
