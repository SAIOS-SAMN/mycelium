// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CHRONOMETRY — ported from ipsal-autic `chronometry/chain.rs` + CE's
//! CHRONOMETRY_BRIDGE: the K-STEP INDEX is the mesh's temporal authority —
//! algebraic time, never a wall clock. Every state transition stamps a receipt
//! BLAKE3-linked to its parent (LAW-5 continuity); the chain's integrity is
//! verifiable end to end; and temporal validation is PROOF-OF-CONVERGENCE —
//! independent nodes verging to the same irreducible form, not voting. This is
//! the spine the dimensional ledger (Step 4) commits onto.

/// One K-step receipt: `chain_hash = BLAKE3(step_le ‖ delta_fp ‖ op ‖ prev)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    /// Monotonic step index — the algebraic clock.
    pub step_index: u64,
    /// Fingerprint of the state/germ this step committed (e.g. a germ address).
    pub delta_fingerprint: [u8; 32],
    /// Which operation was applied (an act digest — never reasoned on).
    pub operator_hash: [u8; 32],
    /// The parent receipt's chain hash (all zeros at genesis).
    pub prev_receipt_hash: [u8; 32],
    /// This receipt's chain hash.
    pub chain_hash: [u8; 32],
}

impl KStepReceipt {
    fn hash(step_index: u64, delta_fp: &[u8; 32], op: &[u8; 32], prev: &[u8; 32]) -> [u8; 32] {
        let mut buf = Vec::with_capacity(8 + 96);
        buf.extend_from_slice(&step_index.to_le_bytes());
        buf.extend_from_slice(delta_fp);
        buf.extend_from_slice(op);
        buf.extend_from_slice(prev);
        *blake3::hash(&buf).as_bytes()
    }
}

/// The receipt chain — append-only, parent-linked, integrity-verifiable.
#[derive(Debug, Default, Clone)]
pub struct ReceiptChain {
    receipts: Vec<KStepReceipt>,
}

impl ReceiptChain {
    pub fn new() -> Self {
        ReceiptChain::default()
    }

    /// The next step index — the current K (0 at genesis).
    pub fn next_step_index(&self) -> u64 {
        self.receipts.len() as u64
    }

    /// The head's chain hash (all zeros before genesis).
    pub fn head_hash(&self) -> [u8; 32] {
        self.receipts.last().map_or([0u8; 32], |r| r.chain_hash)
    }

    /// Append the next receipt — K advances monotonically, the link is sealed.
    pub fn append(&mut self, delta_fingerprint: [u8; 32], operator_hash: [u8; 32]) -> &KStepReceipt {
        let step_index = self.next_step_index();
        let prev = self.head_hash();
        let chain_hash = KStepReceipt::hash(step_index, &delta_fingerprint, &operator_hash, &prev);
        self.receipts.push(KStepReceipt {
            step_index,
            delta_fingerprint,
            operator_hash,
            prev_receipt_hash: prev,
            chain_hash,
        });
        self.receipts.last().unwrap()
    }

    /// Verify the whole chain: indices monotone from 0, every link's parent hash
    /// matches, every chain hash recomputes. A single tampered byte fails.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = [0u8; 32];
        for (i, r) in self.receipts.iter().enumerate() {
            if r.step_index != i as u64 || r.prev_receipt_hash != prev {
                return false;
            }
            let expect =
                KStepReceipt::hash(r.step_index, &r.delta_fingerprint, &r.operator_hash, &prev);
            if r.chain_hash != expect {
                return false;
            }
            prev = r.chain_hash;
        }
        true
    }

    pub fn receipts(&self) -> &[KStepReceipt] {
        &self.receipts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// K advances monotonically, links seal, integrity verifies — and one
    /// tampered byte anywhere breaks the whole chain.
    #[test]
    fn the_chain_is_temporal_authority_and_tamper_evident() {
        let mut c = ReceiptChain::new();
        assert_eq!(c.next_step_index(), 0, "genesis K = 0");
        c.append([1; 32], [9; 32]);
        c.append([2; 32], [9; 32]);
        let r = c.append([3; 32], [8; 32]);
        assert_eq!(r.step_index, 2, "K is the algebraic clock");
        assert!(c.verify_chain_integrity());
        // Tamper: flip one byte of the middle receipt's fingerprint.
        let mut t = c.clone();
        t.receipts[1].delta_fingerprint[0] ^= 1;
        assert!(!t.verify_chain_integrity(), "a single byte breaks the continuity law");
    }
}
