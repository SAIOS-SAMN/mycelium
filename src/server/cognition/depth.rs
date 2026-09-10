// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L15, 45.1, 1.9 — DEPTH IS READ FROM THE RECEIPTS, and stored nowhere.
//!
//! An address's depth is not a number a carrier holds. It is the count of
//! DISTINCT SIGNERS whose receipts of kind `Install`, `Ruling` or
//! `Adopted` name that address — a reading of the causal poset the chain
//! already is, computed at the read (law 12) and never written down.
//!
//! `Standing` is untouched. Standing says which stratum of one mind's own
//! language a word sits in (24.1); depth says how many independent organs
//! have committed to it. A word can be `Ruled` here at depth 1 and the
//! same word `Ruled` elsewhere at depth 3, and neither reading is the
//! other's — 47.3's arity is depth, not standing.
//!
//! What IS held is the index: address → signer ‖ kind → K. Without it
//! the read walks the whole chain for every address, which is the
//! enumeration law 3 forbids. It is derived and rebuilt from the chain
//! at restore, like the axis and moduli indices.

use super::*;

/// The three kinds that COMMIT an organ to an address (47.3). An attest,
/// a listing, an utterance and a learning are acts about a word; these
/// three are a vantage standing behind it.
fn commits(kind: crate::chronometry::ActKind) -> bool {
    matches!(
        kind,
        crate::chronometry::ActKind::Install
            | crate::chronometry::ActKind::Ruling
            | crate::chronometry::ActKind::Adopted
    )
}

/// Signer ‖ kind, both written out (A21): the coordinate an attestation
/// is named by. Two acts of one signer on one address at two kinds are
/// two entries and one signer, which is what `depth_of` reads.
fn attestor_key(vk: &[u8; 32], kind: crate::chronometry::ActKind) -> Vec<u8> {
    let mut k = Vec::with_capacity(33);
    k.extend_from_slice(vk);
    k.push(kind.to_byte());
    k
}

impl Kernel {
    /// Record one receipt on the depth index. Called wherever a receipt
    /// joins this court's chain — its own acts and every peer receipt it
    /// admits — so the index and the chain cannot disagree.
    pub(crate) fn note_attestor(&mut self, r: &crate::chronometry::KStepReceipt) {
        if r.delta.is_empty() {
            return;
        }
        let key = attestor_key(&r.vk, r.kind);
        match self.attestors.get_mut(&r.delta) {
            Some(bucket) => {
                bucket.insert(key, r.step_index);
            }
            None => {
                let mut bucket = crate::network::nerve::Keyed::new();
                bucket.insert(key, r.step_index);
                self.attestors.insert(&r.delta, bucket);
            }
        }
    }

    /// L15 — THE DEPTH OF AN ADDRESS: how many distinct organs have
    /// committed to it. The walk is over that address's OWN entries — its
    /// boundary, which rule 1 allows — and the distinct signers are read
    /// from a keyed set built for the read and dropped after it, so
    /// nothing about depth is stored (law 12).
    ///
    /// Zero is a true reading: an address no organ has committed to has
    /// depth zero, and that is a fact rather than an absence.
    pub fn depth_of(&self, addr: &[u8]) -> usize {
        let Some(bucket) = self.attestors.get(addr) else {
            return 0;
        };
        let mut signers: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (k, _step) in bucket.fiber(crate::network::nerve::ROOT) {
            // The key is the signer's 32 bytes followed by the kind byte:
            // read both out, never a prefix guess.
            let [vk @ .., kind] = k.as_slice() else {
                continue;
            };
            let Some(kind) = crate::chronometry::ActKind::from_byte(*kind) else {
                continue;
            };
            if !commits(kind) {
                continue;
            }
            signers.insert(vk, ());
        }
        signers.fiber(crate::network::nerve::ROOT).count()
    }

    /// Rebuild the depth index from the chain this court holds. Derived,
    /// never persisted: a restored court reads its own receipts and the
    /// peer receipts it admitted, exactly as it read them when they
    /// arrived.
    pub(crate) fn rebuild_attestors(&mut self) {
        self.attestors = crate::network::nerve::Keyed::new();
        let receipts: Vec<crate::chronometry::KStepReceipt> = self.chain.receipts().to_vec();
        for r in &receipts {
            self.note_attestor(r);
        }
    }

    /// 45.9 — THE GENERATORS THIS COURT HOLDS, BY DEPTH. One row per
    /// depth reached, in depth order, as a Vec of relations: the reading
    /// the instrument prints beside the count of adjoined generators.
    /// Never a total and never an average (law 5).
    pub fn generators_by_depth(&self) -> Vec<(usize, u64)> {
        let mut rows: crate::network::nerve::Keyed<u64> = crate::network::nerve::Keyed::new();
        for (addr, ()) in self.ruled_addrs.fiber(crate::network::nerve::ROOT) {
            let d = self.depth_of(&addr);
            let key = (d as u64).to_be_bytes().to_vec();
            match rows.get_mut(&key) {
                Some(n) => *n += 1,
                None => {
                    rows.insert(key, 1);
                }
            }
        }
        rows.fiber(crate::network::nerve::ROOT)
            .filter_map(|(k, n)| {
                let raw: [u8; 8] = k.as_slice().try_into().ok()?;
                Some((u64::from_be_bytes(raw) as usize, *n))
            })
            .collect()
    }
}
